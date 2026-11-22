//! Supervisa el proceso local del Core y publica solo su estado de conexión.

use std::{
    net::Ipv4Addr,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use tauri::{
    AppHandle, Emitter, Manager, Runtime, State, Window, WindowEvent, path::BaseDirectory,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, Command},
    sync::{mpsc, watch},
    time::{sleep, timeout},
};
use tonic::{
    Code, Request,
    metadata::MetadataValue,
    transport::{Channel, Endpoint},
};
use zeroize::{Zeroize, Zeroizing};

pub mod wire {
    tonic::include_proto!("drasus.desktop.v1");
}

use wire::{
    ReadinessRequest, StatusEvent, StatusKind, WatchStatusRequest,
    desktop_bootstrap_client::DesktopBootstrapClient,
};

const SESSION_SECRET_LENGTH: usize = 32;
const CORE_STARTUP_TIMEOUT: Duration = Duration::from_secs(10);
const RPC_TIMEOUT: Duration = Duration::from_secs(5);
const RECONNECT_DELAY: Duration = Duration::from_millis(500);
const ORDERLY_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(3);
const CORE_STATUS_EVENT: &str = "core-status";

/// Estado mínimo de conexión que el host comparte con React y sus observables.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionPhase {
    Starting,
    Connected,
    Reconnecting,
    Error,
}

/// Resultado real de la última recuperación del stream.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryResult {
    InitialSnapshot,
    CursorResumed,
    FreshSnapshot,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreStatusSnapshot {
    pub phase: ConnectionPhase,
    pub is_recovering: bool,
    pub recovery: Option<RecoveryResult>,
    pub observed_at_unix_ms: u64,
}

impl CoreStatusSnapshot {
    fn new(phase: ConnectionPhase, is_recovering: bool, recovery: Option<RecoveryResult>) -> Self {
        Self {
            phase,
            is_recovering,
            recovery,
            observed_at_unix_ms: timestamp_unix_ms(),
        }
    }
}

/// Comparte estado serializable y controla reintentos y cierre del proceso supervisado.
#[derive(Clone)]
pub struct DesktopBridge {
    status: Arc<Mutex<CoreStatusSnapshot>>,
    retry: mpsc::Sender<()>,
    shutdown: watch::Sender<bool>,
    terminated: watch::Receiver<bool>,
    closing: Arc<AtomicBool>,
}

impl DesktopBridge {
    /// Inicia el supervisor antes de que React solicite el estado inicial.
    pub fn start(app: AppHandle) -> Self {
        let initial = CoreStatusSnapshot::new(ConnectionPhase::Starting, false, None);
        let status = Arc::new(Mutex::new(initial.clone()));
        let (retry_tx, retry_rx) = mpsc::channel(1);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        let (terminated_tx, terminated_rx) = watch::channel(false);

        tauri::async_runtime::spawn(run_supervisor(
            app,
            status.clone(),
            retry_rx,
            shutdown_rx,
            terminated_tx,
        ));

        Self {
            status,
            retry: retry_tx,
            shutdown: shutdown_tx,
            terminated: terminated_rx,
            closing: Arc::new(AtomicBool::new(false)),
        }
    }

    fn snapshot(&self) -> CoreStatusSnapshot {
        self.status
            .lock()
            .map(|status| status.clone())
            .unwrap_or_else(|_| CoreStatusSnapshot::new(ConnectionPhase::Error, false, None))
    }

    fn request_retry(&self) {
        let _ = self.retry.try_send(());
    }

    fn request_shutdown(&self) -> bool {
        if self.closing.swap(true, Ordering::AcqRel) {
            return false;
        }
        self.shutdown.send_replace(true);
        true
    }

    async fn wait_for_termination(&self) {
        let mut terminated = self.terminated.clone();
        while !*terminated.borrow() {
            if terminated.changed().await.is_err() {
                break;
            }
        }
    }
}

#[tauri::command]
pub fn get_core_status(state: State<'_, DesktopBridge>) -> CoreStatusSnapshot {
    state.snapshot()
}

#[tauri::command]
pub fn retry_core_connection(state: State<'_, DesktopBridge>) {
    state.request_retry();
}

/// Mantiene abierta la ventana hasta que EOF cierre el proceso Core de forma ordenada.
pub fn on_window_event(window: &Window, event: &WindowEvent) {
    let WindowEvent::CloseRequested { api, .. } = event else {
        return;
    };

    api.prevent_close();
    close_after_core(window.app_handle().clone());
}

/// Solicita el mismo cierre ordenado desde la barra propia de la ventana.
#[tauri::command]
pub fn close_desktop_window(app: AppHandle) {
    close_after_core(app);
}

fn close_after_core(app: AppHandle) {
    let bridge = { app.state::<DesktopBridge>().inner().clone() };
    if bridge.request_shutdown() {
        tauri::async_runtime::spawn(async move {
            bridge.wait_for_termination().await;
            app.exit(0);
        });
    }
}

pub fn request_shutdown(app: &AppHandle) {
    if let Some(bridge) = app.try_state::<DesktopBridge>() {
        bridge.request_shutdown();
    }
}

async fn run_supervisor(
    app: AppHandle,
    status: Arc<Mutex<CoreStatusSnapshot>>,
    retry: mpsc::Receiver<()>,
    shutdown: watch::Receiver<bool>,
    terminated: watch::Sender<bool>,
) {
    run_supervisor_with_config(
        app,
        status,
        retry,
        shutdown,
        terminated,
        CoreLaunchConfig::default(),
    )
    .await;
}

async fn run_supervisor_with_config<R: Runtime>(
    app: AppHandle<R>,
    status: Arc<Mutex<CoreStatusSnapshot>>,
    mut retry: mpsc::Receiver<()>,
    mut shutdown: watch::Receiver<bool>,
    terminated: watch::Sender<bool>,
    launch_config: CoreLaunchConfig,
) {
    let mut cursor = None;
    loop {
        if *shutdown.borrow() {
            break;
        }

        let phase = if status_phase(&status) == ConnectionPhase::Starting {
            ConnectionPhase::Starting
        } else {
            ConnectionPhase::Reconnecting
        };
        publish_status(
            &app,
            &status,
            phase,
            phase == ConnectionPhase::Reconnecting,
            None,
        );

        match launch_core(&app, &launch_config).await {
            Ok(mut core) => {
                match monitor_core(&app, &status, &mut core, &mut shutdown, &mut cursor).await {
                    SessionExit::Stop => {
                        close_core(core).await;
                        break;
                    }
                    SessionExit::Restart => {
                        publish_status(&app, &status, ConnectionPhase::Reconnecting, true, None);
                        close_core(core).await;
                    }
                    SessionExit::Terminal => {
                        close_core(core).await;
                        publish_status(&app, &status, ConnectionPhase::Error, false, None);
                        if !wait_for_retry(&mut retry, &mut shutdown).await {
                            break;
                        }
                        publish_status(&app, &status, ConnectionPhase::Reconnecting, true, None);
                    }
                }
            }
            Err(_) => {
                publish_status(&app, &status, ConnectionPhase::Error, false, None);
                if !wait_for_retry(&mut retry, &mut shutdown).await {
                    break;
                }
                publish_status(&app, &status, ConnectionPhase::Reconnecting, true, None);
            }
        }
    }

    terminated.send_replace(true);
}

fn status_phase(status: &Arc<Mutex<CoreStatusSnapshot>>) -> ConnectionPhase {
    status
        .lock()
        .map(|current| current.phase)
        .unwrap_or(ConnectionPhase::Error)
}

fn publish_status<R: Runtime>(
    app: &AppHandle<R>,
    store: &Arc<Mutex<CoreStatusSnapshot>>,
    phase: ConnectionPhase,
    is_recovering: bool,
    recovery: Option<RecoveryResult>,
) {
    let next = CoreStatusSnapshot::new(phase, is_recovering, recovery);
    if let Ok(mut current) = store.lock() {
        *current = next.clone();
    }
    let _ = app.emit(CORE_STATUS_EVENT, next);
    if matches!(phase, ConnectionPhase::Connected | ConnectionPhase::Error) {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.show();
        }
    }
}

async fn wait_for_retry(
    retry: &mut mpsc::Receiver<()>,
    shutdown: &mut watch::Receiver<bool>,
) -> bool {
    loop {
        if *shutdown.borrow() {
            return false;
        }
        tokio::select! {
            retry_result = retry.recv() => return retry_result.is_some(),
            shutdown_result = shutdown.changed() => {
                if shutdown_result.is_err() || *shutdown.borrow() {
                    return false;
                }
            }
        }
    }
}

async fn wait_before_reconnect(shutdown: &mut watch::Receiver<bool>) -> bool {
    tokio::select! {
        _ = sleep(RECONNECT_DELAY) => !*shutdown.borrow(),
        shutdown_result = shutdown.changed() => shutdown_result.is_ok() && !*shutdown.borrow(),
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StartupEndpoint {
    address: String,
    port: u16,
}

struct SessionSecret([u8; SESSION_SECRET_LENGTH]);

impl SessionSecret {
    fn generate() -> Result<Self, CoreFailure> {
        let mut secret = Self([0; SESSION_SECRET_LENGTH]);
        OsRng
            .try_fill_bytes(&mut secret.0)
            .map_err(|_| CoreFailure::SessionEntropy)?;
        Ok(secret)
    }
}

impl Drop for SessionSecret {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

struct ActiveCore {
    child: Child,
    stdin: Option<ChildStdin>,
    channel: Channel,
    secret: SessionSecret,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cursor {
    scope: String,
    sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EventDecision {
    InitialSnapshot,
    Update,
    Duplicate,
    Resync,
    NotReady,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SessionExit {
    Stop,
    Restart,
    Terminal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CoreFailure {
    CoreUnavailable,
    SessionEntropy,
    SessionPipe,
    StartupEndpoint,
    InvalidEndpoint,
    AppDataPath,
    GrpcChannel,
    Authentication,
    InvalidStatus,
}

#[derive(Clone, Default)]
struct CoreLaunchConfig {
    binary: Option<PathBuf>,
    database_path: Option<PathBuf>,
    session_ttl_seconds: Option<u64>,
}

async fn launch_core<R: Runtime>(
    app: &AppHandle<R>,
    config: &CoreLaunchConfig,
) -> Result<ActiveCore, CoreFailure> {
    let binary = match &config.binary {
        Some(binary) => binary.clone(),
        None => core_binary_path(app)?,
    };
    let database_path = match &config.database_path {
        Some(database_path) => database_path.clone(),
        None => core_database_path(app)?,
    };
    launch_core_binary_with_options(binary, database_path, config.session_ttl_seconds).await
}

#[cfg(test)]
async fn launch_core_binary(
    binary: PathBuf,
    database_path: PathBuf,
) -> Result<ActiveCore, CoreFailure> {
    launch_core_binary_with_options(binary, database_path, None).await
}

async fn launch_core_binary_with_options(
    binary: PathBuf,
    database_path: PathBuf,
    session_ttl_seconds: Option<u64>,
) -> Result<ActiveCore, CoreFailure> {
    let mut command = Command::new(binary);
    command
        .arg("desktop-server")
        .arg("--db")
        .arg(database_path.as_os_str());
    if let Some(session_ttl_seconds) = session_ttl_seconds {
        command
            .arg("--session-ttl-seconds")
            .arg(session_ttl_seconds.to_string());
    }
    let mut child = command
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .map_err(|_| CoreFailure::CoreUnavailable)?;

    let secret = SessionSecret::generate()?;
    let Some(mut stdin) = child.stdin.take() else {
        return Err(CoreFailure::SessionPipe);
    };
    if stdin.write_all(&secret.0).await.is_err() {
        close_unstarted_child(child, stdin).await;
        return Err(CoreFailure::SessionPipe);
    }

    let Some(stdout) = child.stdout.take() else {
        close_unstarted_child(child, stdin).await;
        return Err(CoreFailure::StartupEndpoint);
    };
    let mut lines = BufReader::new(stdout).lines();
    let endpoint_line = match timeout(CORE_STARTUP_TIMEOUT, lines.next_line()).await {
        Ok(Ok(Some(line))) => line,
        _ => {
            close_unstarted_child(child, stdin).await;
            return Err(CoreFailure::StartupEndpoint);
        }
    };
    let endpoint: StartupEndpoint = match serde_json::from_str(&endpoint_line) {
        Ok(endpoint) => endpoint,
        Err(_) => {
            close_unstarted_child(child, stdin).await;
            return Err(CoreFailure::StartupEndpoint);
        }
    };
    let uri = match endpoint_uri(endpoint) {
        Ok(uri) => uri,
        Err(error) => {
            close_unstarted_child(child, stdin).await;
            return Err(error);
        }
    };
    let channel = match Endpoint::from_shared(uri) {
        Ok(endpoint) => match timeout(CORE_STARTUP_TIMEOUT, endpoint.connect()).await {
            Ok(Ok(channel)) => channel,
            _ => {
                close_unstarted_child(child, stdin).await;
                return Err(CoreFailure::GrpcChannel);
            }
        },
        Err(_) => {
            close_unstarted_child(child, stdin).await;
            return Err(CoreFailure::InvalidEndpoint);
        }
    };

    Ok(ActiveCore {
        child,
        stdin: Some(stdin),
        channel,
        secret,
    })
}

fn core_database_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, CoreFailure> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|_| CoreFailure::AppDataPath)?;
    core_database_path_in(&app_data_dir)
}

/// Prepara una ruta persistente y privada para la base local que abre el Core.
fn core_database_path_in(app_data_dir: &Path) -> Result<PathBuf, CoreFailure> {
    if !app_data_dir.is_absolute() {
        return Err(CoreFailure::AppDataPath);
    }

    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder
        .create(app_data_dir)
        .map_err(|_| CoreFailure::AppDataPath)?;

    let app_data_dir = std::fs::canonicalize(app_data_dir).map_err(|_| CoreFailure::AppDataPath)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&app_data_dir, std::fs::Permissions::from_mode(0o700))
            .map_err(|_| CoreFailure::AppDataPath)?;
    }
    Ok(app_data_dir.join("drasus.db"))
}

fn core_binary_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, CoreFailure> {
    let packaged_name = format!("drasus{}", std::env::consts::EXE_SUFFIX);
    if let Ok(resource) = app.path().resolve(&packaged_name, BaseDirectory::Resource) {
        if resource.is_file() {
            return Ok(resource);
        }
    }

    let workspace_binary = PathBuf::from(env!("DRASUS_CORE_BINARY"));
    workspace_binary
        .is_file()
        .then_some(workspace_binary)
        .ok_or(CoreFailure::CoreUnavailable)
}

fn endpoint_uri(endpoint: StartupEndpoint) -> Result<String, CoreFailure> {
    let address = endpoint
        .address
        .parse::<Ipv4Addr>()
        .map_err(|_| CoreFailure::InvalidEndpoint)?;
    if !address.is_loopback() || endpoint.port == 0 {
        return Err(CoreFailure::InvalidEndpoint);
    }
    Ok(format!("http://{address}:{}", endpoint.port))
}

fn authorized_request<T>(message: T, secret: &SessionSecret) -> Result<Request<T>, CoreFailure> {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut authorization = Zeroizing::new(String::with_capacity(7 + SESSION_SECRET_LENGTH * 2));
    authorization.push_str("Bearer ");
    for byte in secret.0 {
        authorization.push(HEX[(byte >> 4) as usize] as char);
        authorization.push(HEX[(byte & 0x0f) as usize] as char);
    }
    let metadata =
        MetadataValue::try_from(authorization.as_str()).map_err(|_| CoreFailure::Authentication)?;
    let mut request = Request::new(message);
    request.metadata_mut().insert("authorization", metadata);
    Ok(request)
}

async fn monitor_core<R: Runtime>(
    app: &AppHandle<R>,
    status: &Arc<Mutex<CoreStatusSnapshot>>,
    core: &mut ActiveCore,
    shutdown: &mut watch::Receiver<bool>,
    cursor: &mut Option<Cursor>,
) -> SessionExit {
    let mut next_recovery = if cursor.is_some() {
        RecoveryResult::CursorResumed
    } else {
        RecoveryResult::InitialSnapshot
    };

    loop {
        if *shutdown.borrow() {
            return SessionExit::Stop;
        }

        let mut client = DesktopBootstrapClient::new(core.channel.clone());
        let readiness_request = match authorized_request(ReadinessRequest {}, &core.secret) {
            Ok(request) => request,
            Err(_) => return SessionExit::Terminal,
        };
        let readiness = match timeout(RPC_TIMEOUT, client.check_readiness(readiness_request)).await
        {
            Ok(Ok(reply)) => reply.into_inner(),
            Ok(Err(error)) if error.code() == Code::Unauthenticated => {
                return SessionExit::Restart;
            }
            Ok(Err(error)) if error.code() == Code::Unavailable => {
                if matches!(core.child.try_wait(), Ok(Some(_))) {
                    return SessionExit::Restart;
                }
                publish_status(app, status, ConnectionPhase::Reconnecting, true, None);
                if !wait_before_reconnect(shutdown).await {
                    return SessionExit::Stop;
                }
                continue;
            }
            _ => return SessionExit::Terminal,
        };
        if !readiness.ready {
            publish_status(app, status, ConnectionPhase::Reconnecting, true, None);
            if !wait_before_reconnect(shutdown).await {
                return SessionExit::Stop;
            }
            continue;
        }

        let cursor_is_current = cursor.as_ref().is_some_and(|current| {
            current.scope == readiness.stream_scope && current.sequence == readiness.sequence
        });
        let watch_request = WatchStatusRequest {
            stream_scope: cursor
                .as_ref()
                .map(|current| current.scope.clone())
                .unwrap_or_default(),
            after_sequence: cursor.as_ref().map(|current| current.sequence).unwrap_or(0),
        };
        let authorized = match authorized_request(watch_request, &core.secret) {
            Ok(request) => request,
            Err(_) => return SessionExit::Terminal,
        };
        let response = match timeout(RPC_TIMEOUT, client.watch_status(authorized)).await {
            Ok(Ok(response)) => response,
            Ok(Err(error)) if error.code() == Code::Unauthenticated => {
                return SessionExit::Restart;
            }
            Ok(Err(error)) if error.code() == Code::OutOfRange && cursor.is_some() => {
                *cursor = None;
                next_recovery = RecoveryResult::FreshSnapshot;
                publish_status(app, status, ConnectionPhase::Reconnecting, true, None);
                continue;
            }
            Ok(Err(error)) if error.code() == Code::Unavailable => {
                if matches!(core.child.try_wait(), Ok(Some(_))) {
                    return SessionExit::Restart;
                }
                publish_status(app, status, ConnectionPhase::Reconnecting, true, None);
                if !wait_before_reconnect(shutdown).await {
                    return SessionExit::Stop;
                }
                continue;
            }
            _ => return SessionExit::Terminal,
        };

        if cursor_is_current {
            publish_status(
                app,
                status,
                ConnectionPhase::Connected,
                false,
                Some(RecoveryResult::CursorResumed),
            );
        }

        let mut stream = response.into_inner();
        loop {
            let message = tokio::select! {
                shutdown_result = shutdown.changed() => {
                    if shutdown_result.is_err() || *shutdown.borrow() {
                        return SessionExit::Stop;
                    }
                    continue;
                }
                child_result = core.child.wait() => {
                    let _ = child_result;
                    publish_status(app, status, ConnectionPhase::Reconnecting, true, None);
                    return SessionExit::Restart;
                }
                result = stream.message() => result,
            };

            match message {
                Ok(Some(event)) => {
                    let decision = match advance_cursor(cursor, &event) {
                        Ok(decision) => decision,
                        Err(_) => return SessionExit::Terminal,
                    };
                    match decision {
                        EventDecision::InitialSnapshot | EventDecision::Update => {
                            publish_status(
                                app,
                                status,
                                ConnectionPhase::Connected,
                                false,
                                Some(next_recovery),
                            );
                            next_recovery = RecoveryResult::CursorResumed;
                        }
                        EventDecision::Duplicate => {}
                        EventDecision::Resync => {
                            *cursor = None;
                            next_recovery = RecoveryResult::FreshSnapshot;
                            publish_status(app, status, ConnectionPhase::Reconnecting, true, None);
                            break;
                        }
                        EventDecision::NotReady => {
                            // STOPPED y STARTING pueden ser eventos históricos del replay.
                            continue;
                        }
                    }
                }
                Ok(None) => {
                    publish_status(app, status, ConnectionPhase::Reconnecting, true, None);
                    if matches!(core.child.try_wait(), Ok(Some(_))) {
                        return SessionExit::Restart;
                    }
                    break;
                }
                Err(error) if error.code() == Code::Unauthenticated => {
                    return SessionExit::Restart;
                }
                Err(error) if error.code() == Code::OutOfRange => {
                    *cursor = None;
                    next_recovery = RecoveryResult::FreshSnapshot;
                    publish_status(app, status, ConnectionPhase::Reconnecting, true, None);
                    break;
                }
                Err(error) if error.code() == Code::Unavailable => {
                    if matches!(core.child.try_wait(), Ok(Some(_))) {
                        return SessionExit::Restart;
                    }
                    publish_status(app, status, ConnectionPhase::Reconnecting, true, None);
                    break;
                }
                Err(_) => return SessionExit::Terminal,
            }
        }

        if !wait_before_reconnect(shutdown).await {
            return SessionExit::Stop;
        }
    }
}

fn advance_cursor(
    cursor: &mut Option<Cursor>,
    event: &StatusEvent,
) -> Result<EventDecision, CoreFailure> {
    if event.stream_scope.is_empty() || event.sequence == 0 {
        return Err(CoreFailure::InvalidStatus);
    }
    match cursor {
        None if event.kind == StatusKind::InitialSnapshot as i32 => {
            *cursor = Some(Cursor {
                scope: event.stream_scope.clone(),
                sequence: event.sequence,
            });
            if event.ready {
                Ok(EventDecision::InitialSnapshot)
            } else {
                Ok(EventDecision::NotReady)
            }
        }
        None => Err(CoreFailure::InvalidStatus),
        Some(current) if event.stream_scope != current.scope => Ok(EventDecision::Resync),
        Some(current) if event.sequence <= current.sequence => Ok(EventDecision::Duplicate),
        Some(current) if event.sequence != current.sequence.saturating_add(1) => {
            Ok(EventDecision::Resync)
        }
        Some(_) if event.kind != StatusKind::ReadinessChanged as i32 => {
            Err(CoreFailure::InvalidStatus)
        }
        Some(current) => {
            current.sequence = event.sequence;
            if !event.ready {
                Ok(EventDecision::NotReady)
            } else {
                Ok(EventDecision::Update)
            }
        }
    }
}

async fn close_unstarted_child(mut child: Child, stdin: ChildStdin) {
    drop(stdin);
    if timeout(ORDERLY_SHUTDOWN_TIMEOUT, child.wait())
        .await
        .is_err()
    {
        let _ = child.kill().await;
        let _ = child.wait().await;
    }
}

async fn close_core(mut core: ActiveCore) -> bool {
    if let Some(stdin) = core.stdin.take() {
        drop(stdin);
    }
    let closed_by_eof = timeout(ORDERLY_SHUTDOWN_TIMEOUT, core.child.wait())
        .await
        .is_ok();
    if !closed_by_eof {
        let _ = core.child.kill().await;
        let _ = core.child.wait().await;
    }
    closed_by_eof
}

fn timestamp_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tauri::Listener;

    fn contract_json() -> serde_json::Value {
        let source = include_str!("../../src/bridge/coreStatusContract.ts");
        let marker = "export const coreStatusContract = ";
        let start = source
            .find(marker)
            .expect("el contrato TS conserva su export")
            + marker.len();
        let end = source[start..]
            .find(" as const;")
            .expect("el contrato TS conserva su literal")
            + start;
        serde_json::from_str(&source[start..end])
            .expect("el descriptor TypeScript sigue siendo JSON válido")
    }

    fn rust_enum_variants(source: &str, enum_name: &str) -> Vec<String> {
        let marker = format!("pub enum {enum_name}");
        let marker_start = source.find(&marker).expect("el enum Rust existe");
        let attributes_start = source[..marker_start]
            .rfind("#[derive")
            .expect("el enum declara sus atributos serde");
        assert!(
            source[attributes_start..marker_start].contains("rename_all = \"snake_case\""),
            "la serialización enum debe conservar snake_case"
        );
        let enum_source = &source[marker_start..];
        let body_start = enum_source.find('{').expect("el enum tiene cuerpo") + 1;
        let body_end = enum_source[body_start..]
            .find('}')
            .expect("el enum cierra su cuerpo")
            + body_start;
        enum_source[body_start..body_end]
            .split(',')
            .filter_map(|variant| {
                let name = variant.trim().split_whitespace().next()?;
                (!name.is_empty()).then(|| {
                    let mut value = String::new();
                    for (index, character) in name.chars().enumerate() {
                        if character.is_ascii_uppercase() && index > 0 {
                            value.push('_');
                        }
                        value.push(character.to_ascii_lowercase());
                    }
                    value
                })
            })
            .collect()
    }

    fn temporary_app_data_dir(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("el reloj del sistema es posterior al epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "drasus-desktop-{label}-{}-{nonce}",
            std::process::id()
        ))
    }

    fn status_event(scope: &str, sequence: u64, kind: StatusKind) -> StatusEvent {
        StatusEvent {
            stream_scope: scope.to_owned(),
            sequence,
            kind: kind as i32,
            ready: true,
            observed_at_unix_ms: 1,
        }
    }

    #[test]
    fn bootstrap_typescript_adapter_matches_rust_serialized_contract() {
        let source = include_str!("desktop_core.rs");
        let contract = contract_json();
        let sample = serde_json::to_value(CoreStatusSnapshot {
            phase: ConnectionPhase::Starting,
            is_recovering: false,
            recovery: None,
            observed_at_unix_ms: 1,
        })
        .expect("el host serializa su contrato IPC");
        let actual_fields = sample
            .as_object()
            .expect("el snapshot serializa como objeto")
            .keys()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        let contract_fields = contract
            .as_object()
            .expect("el descriptor TS serializa como objeto")
            .keys()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(actual_fields, contract_fields, "campos IPC deben coincidir");

        let contract = contract.as_object().expect("objeto de contrato");
        for (field, descriptor) in contract {
            let descriptor = descriptor.as_object().expect("descriptor de campo");
            let value = &sample[field];
            match descriptor["type"].as_str() {
                Some("boolean") => assert!(value.is_boolean(), "tipo de {field}"),
                Some("number") => assert!(value.is_u64(), "tipo de {field}"),
                Some("enum") => {
                    assert!(value.is_string(), "tipo de {field}");
                    assert_eq!(
                        descriptor["values"],
                        serde_json::to_value(rust_enum_variants(source, "ConnectionPhase"))
                            .expect("las fases Rust se serializan")
                    );
                }
                Some("nullable_enum") => {
                    assert!(value.is_null(), "{field} debe admitir null");
                    let mut expected = rust_enum_variants(source, "RecoveryResult")
                        .into_iter()
                        .map(serde_json::Value::String)
                        .collect::<Vec<_>>();
                    expected.push(serde_json::Value::Null);
                    let mut actual = descriptor["values"]
                        .as_array()
                        .expect("estados recovery")
                        .clone();
                    expected.sort_by_key(serde_json::Value::to_string);
                    actual.sort_by_key(serde_json::Value::to_string);
                    assert_eq!(actual, expected, "estados de {field}");
                }
                other => panic!("descriptor TypeScript no reconocido: {other:?}"),
            }
        }
    }

    #[test]
    fn bootstrap_database_path_creates_private_nested_directory_and_is_stable() {
        let root = temporary_app_data_dir("database-path");
        let app_data_dir = root.join("nested").join("app-data");
        let first = core_database_path_in(&app_data_dir).expect("crea la ruta persistente");
        let second = core_database_path_in(&app_data_dir).expect("reutiliza la ruta persistente");
        assert!(
            app_data_dir.is_dir(),
            "la ruta se crea antes de iniciar Core"
        );
        assert_eq!(first, second, "el mismo app_data_dir conserva la DB");
        assert_eq!(
            first.file_name().and_then(|name| name.to_str()),
            Some("drasus.db")
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                std::fs::metadata(&app_data_dir)
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o700
            );
            std::fs::set_permissions(&app_data_dir, std::fs::Permissions::from_mode(0o755))
                .expect("prepara un directorio heredado permisivo");
            core_database_path_in(&app_data_dir).expect("endurece directorio existente");
            assert_eq!(
                std::fs::metadata(&app_data_dir)
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o700,
                "una ruta existente también queda privada"
            );
        }
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn bootstrap_endpoint_rejects_non_loopback_and_zero_port() {
        assert_eq!(
            endpoint_uri(StartupEndpoint {
                address: "127.0.0.1".to_owned(),
                port: 43123,
            })
            .as_deref(),
            Ok("http://127.0.0.1:43123")
        );
        assert_eq!(
            endpoint_uri(StartupEndpoint {
                address: "0.0.0.0".to_owned(),
                port: 43123,
            }),
            Err(CoreFailure::InvalidEndpoint)
        );
        assert_eq!(
            endpoint_uri(StartupEndpoint {
                address: "127.0.0.1".to_owned(),
                port: 0,
            }),
            Err(CoreFailure::InvalidEndpoint)
        );
    }

    #[test]
    fn bootstrap_cursor_accepts_snapshot_and_next_event_but_detects_gaps() {
        let mut cursor = None;
        assert_eq!(
            advance_cursor(
                &mut cursor,
                &status_event("scope-a", 1, StatusKind::InitialSnapshot)
            ),
            Ok(EventDecision::InitialSnapshot)
        );
        assert_eq!(
            advance_cursor(
                &mut cursor,
                &status_event("scope-a", 2, StatusKind::ReadinessChanged)
            ),
            Ok(EventDecision::Update)
        );
        assert_eq!(
            advance_cursor(
                &mut cursor,
                &status_event("scope-a", 2, StatusKind::ReadinessChanged)
            ),
            Ok(EventDecision::Duplicate)
        );
        assert_eq!(
            advance_cursor(
                &mut cursor,
                &status_event("scope-a", 4, StatusKind::ReadinessChanged)
            ),
            Ok(EventDecision::Resync)
        );
        assert_eq!(
            advance_cursor(
                &mut cursor,
                &status_event("scope-b", 3, StatusKind::ReadinessChanged)
            ),
            Ok(EventDecision::Resync)
        );
    }

    #[test]
    fn bootstrap_cursor_consumes_historical_not_ready_events_before_resume() {
        let mut cursor = Some(Cursor {
            scope: "scope-a".to_owned(),
            sequence: 2,
        });
        let mut stopped = status_event("scope-a", 3, StatusKind::ReadinessChanged);
        stopped.ready = false;
        assert_eq!(
            advance_cursor(&mut cursor, &stopped),
            Ok(EventDecision::NotReady)
        );
        assert_eq!(cursor.as_ref().map(|current| current.sequence), Some(3));

        let mut starting = status_event("scope-a", 4, StatusKind::ReadinessChanged);
        starting.ready = false;
        assert_eq!(
            advance_cursor(&mut cursor, &starting),
            Ok(EventDecision::NotReady)
        );
        assert_eq!(cursor.as_ref().map(|current| current.sequence), Some(4));
        assert_eq!(
            advance_cursor(
                &mut cursor,
                &status_event("scope-a", 5, StatusKind::ReadinessChanged)
            ),
            Ok(EventDecision::Update)
        );
        assert_eq!(cursor.as_ref().map(|current| current.sequence), Some(5));
    }

    #[tokio::test]
    async fn bootstrap_supervisor_authenticates_real_core_and_closes_by_eof() {
        let binary = PathBuf::from(env!("DRASUS_CORE_BINARY"));
        assert!(binary.is_file(), "primero compila el binario Core");
        let root = temporary_app_data_dir("authenticated-core");
        let database_path = core_database_path_in(&root).expect("ruta de DB de prueba");
        let core = launch_core_binary(binary, database_path)
            .await
            .expect("el Core publica un endpoint local");

        let mut client = DesktopBootstrapClient::new(core.channel.clone());
        let readiness = client
            .check_readiness(
                authorized_request(ReadinessRequest {}, &core.secret)
                    .expect("la credencial se codifica como metadata gRPC"),
            )
            .await
            .expect("readiness acepta la credencial de sesión")
            .into_inner();
        assert!(readiness.ready);
        assert!(!readiness.stream_scope.is_empty());

        let mut stream = client
            .watch_status(
                authorized_request(
                    WatchStatusRequest {
                        stream_scope: String::new(),
                        after_sequence: 0,
                    },
                    &core.secret,
                )
                .expect("la solicitud del stream se autentica"),
            )
            .await
            .expect("el Core abre el stream de estado")
            .into_inner();
        let initial = timeout(RPC_TIMEOUT, stream.message())
            .await
            .expect("el Core entrega la vista inicial a tiempo")
            .expect("el stream no falla")
            .expect("la vista inicial existe");
        assert_eq!(initial.kind, StatusKind::InitialSnapshot as i32);
        assert_eq!(initial.stream_scope, readiness.stream_scope);
        assert_eq!(initial.sequence, readiness.sequence);

        assert!(
            close_core(core).await,
            "EOF debe cerrar el Core de forma ordenada"
        );
        let _ = std::fs::remove_dir_all(root);
    }

    #[tokio::test]
    async fn bootstrap_host_restart_reuses_durable_database_and_live_readiness() {
        let binary = PathBuf::from(env!("DRASUS_CORE_BINARY"));
        assert!(binary.is_file(), "primero compila el binario Core");
        let root = temporary_app_data_dir("host-restart");
        let app_data_dir = root.join("nested").join("app-data");
        let first_path = core_database_path_in(&app_data_dir).expect("ruta inicial de DB");
        let first_core = launch_core_binary(binary.clone(), first_path.clone())
            .await
            .expect("el primer host inicia Core con DB persistente");
        let mut first_client = DesktopBootstrapClient::new(first_core.channel.clone());
        let first_readiness = first_client
            .check_readiness(
                authorized_request(ReadinessRequest {}, &first_core.secret)
                    .expect("credencial del primer proceso"),
            )
            .await
            .expect("readiness autenticada del primer proceso")
            .into_inner();
        assert!(first_readiness.ready);
        let first_scope = first_readiness.stream_scope.clone();
        let first_sequence = first_readiness.sequence;

        let first_stream = first_client
            .watch_status(
                authorized_request(
                    WatchStatusRequest {
                        stream_scope: String::new(),
                        after_sequence: 0,
                    },
                    &first_core.secret,
                )
                .expect("cursor inicial autenticado"),
            )
            .await
            .expect("stream inicial del primer proceso")
            .into_inner();
        drop(first_stream);
        assert!(close_core(first_core).await, "EOF cierra el primer Core");
        assert!(
            first_path.is_file(),
            "el Core creó la DB en la ruta del host"
        );

        let second_path = core_database_path_in(&app_data_dir).expect("ruta de DB tras reinicio");
        assert_eq!(
            first_path, second_path,
            "el host reutiliza la DB del app_data_dir"
        );
        let second_core = launch_core_binary(binary, second_path)
            .await
            .expect("el host reiniciado vuelve a abrir la misma DB");
        let mut second_client = DesktopBootstrapClient::new(second_core.channel.clone());
        let current = second_client
            .check_readiness(
                authorized_request(ReadinessRequest {}, &second_core.secret)
                    .expect("credencial nueva del segundo proceso"),
            )
            .await
            .expect("readiness autenticada del proceso actual")
            .into_inner();
        assert!(
            current.ready,
            "solo la readiness del proceso actual es viva"
        );
        assert_eq!(
            current.stream_scope, first_scope,
            "el alcance sobrevive reinicio"
        );
        assert!(
            current.sequence > first_sequence,
            "el cursor avanza tras reinicio"
        );

        let mut replay = second_client
            .watch_status(
                authorized_request(
                    WatchStatusRequest {
                        stream_scope: first_scope,
                        after_sequence: first_sequence,
                    },
                    &second_core.secret,
                )
                .expect("cursor previo autenticado con sesión nueva"),
            )
            .await
            .expect("el historial durable acepta el cursor previo")
            .into_inner();
        let stopped = timeout(RPC_TIMEOUT, replay.message())
            .await
            .expect("el replay durable llega a tiempo")
            .expect("el replay no falla")
            .expect("el evento posterior al cursor existe");
        assert_eq!(stopped.sequence, first_sequence + 1);
        assert!(!stopped.ready, "STOPPED histórico no es readiness viva");
        drop(replay);
        assert!(
            close_core(second_core).await,
            "EOF cierra el Core reiniciado"
        );
        let _ = std::fs::remove_dir_all(root);
    }

    #[tokio::test]
    async fn bootstrap_supervisor_resumes_cursor_across_expired_child_and_historical_stopped() {
        let binary = PathBuf::from(env!("DRASUS_CORE_BINARY"));
        assert!(binary.is_file(), "primero compila el binario Core");
        let root = temporary_app_data_dir("supervisor-resume");
        let database_path = core_database_path_in(&root).expect("ruta durable de prueba");
        let app = tauri::test::mock_app();
        let app_handle = app.handle().clone();
        let (event_tx, mut events) = mpsc::unbounded_channel();
        let listener_id = app_handle.listen(CORE_STATUS_EVENT, move |event| {
            if let Ok(status) = serde_json::from_str::<CoreStatusSnapshot>(event.payload()) {
                let _ = event_tx.send(status);
            }
        });

        let status = Arc::new(Mutex::new(CoreStatusSnapshot::new(
            ConnectionPhase::Starting,
            false,
            None,
        )));
        let (_retry_tx, retry_rx) = mpsc::channel(1);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        let (terminated_tx, _terminated_rx) = watch::channel(false);
        let supervisor = tokio::spawn(run_supervisor_with_config(
            app_handle.clone(),
            status,
            retry_rx,
            shutdown_rx,
            terminated_tx,
            CoreLaunchConfig {
                binary: Some(binary),
                database_path: Some(database_path),
                session_ttl_seconds: Some(2),
            },
        ));

        let deadline = tokio::time::Instant::now() + Duration::from_secs(12);
        let mut initial_snapshot_seen = false;
        let mut cursor_resumed_seen = false;
        let mut initial_snapshot_count = 0;
        while !cursor_resumed_seen {
            let status = tokio::time::timeout_at(deadline, events.recv())
                .await
                .expect("el supervisor llega a la recuperación autenticada")
                .expect("el supervisor publica estados");
            if status.phase != ConnectionPhase::Connected {
                continue;
            }
            match status.recovery {
                Some(RecoveryResult::InitialSnapshot) => {
                    initial_snapshot_count += 1;
                    assert_eq!(
                        initial_snapshot_count, 1,
                        "un reinicio del Core no debe reemplazar el cursor por snapshot"
                    );
                    initial_snapshot_seen = true;
                }
                Some(RecoveryResult::CursorResumed) => {
                    assert!(
                        initial_snapshot_seen,
                        "la reanudación sucede después del snapshot del primer Core"
                    );
                    cursor_resumed_seen = true;
                }
                _ => {}
            }
        }

        let stable_until = tokio::time::Instant::now() + Duration::from_millis(250);
        while let Ok(Some(status)) = tokio::time::timeout_at(stable_until, events.recv()).await {
            assert!(
                !(status.phase == ConnectionPhase::Connected
                    && status.recovery == Some(RecoveryResult::InitialSnapshot)),
                "STOPPED/STARTING históricos no reinician la recuperación"
            );
        }

        shutdown_tx.send_replace(true);
        timeout(Duration::from_secs(5), supervisor)
            .await
            .expect("el supervisor termina tras shutdown")
            .expect("la tarea del supervisor termina sin pánico");
        app_handle.unlisten(listener_id);
        let _ = std::fs::remove_dir_all(root);
    }

    #[tokio::test]
    async fn bootstrap_monitor_requests_fresh_snapshot_after_out_of_range_cursor() {
        let binary = PathBuf::from(env!("DRASUS_CORE_BINARY"));
        assert!(binary.is_file(), "primero compila el binario Core");
        let root = temporary_app_data_dir("out-of-range-cursor");
        let database_path = core_database_path_in(&root).expect("ruta durable de prueba");
        let core = launch_core_binary(binary, database_path)
            .await
            .expect("el Core inicia con historial durable vacío");
        let app = tauri::test::mock_app();
        let app_handle = app.handle().clone();
        let (event_tx, mut events) = mpsc::unbounded_channel();
        let listener_id = app_handle.listen(CORE_STATUS_EVENT, move |event| {
            if let Ok(status) = serde_json::from_str::<CoreStatusSnapshot>(event.payload()) {
                let _ = event_tx.send(status);
            }
        });
        let status = Arc::new(Mutex::new(CoreStatusSnapshot::new(
            ConnectionPhase::Starting,
            false,
            None,
        )));
        let (shutdown_tx, mut shutdown_rx) = watch::channel(false);
        let monitor_app = app_handle.clone();
        let mut cursor = Some(Cursor {
            scope: "expired-scope".to_owned(),
            sequence: 4,
        });
        let monitor = tokio::spawn(async move {
            let mut core = core;
            let result = monitor_core(
                &monitor_app,
                &status,
                &mut core,
                &mut shutdown_rx,
                &mut cursor,
            )
            .await;
            (result, close_core(core).await)
        });

        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        let snapshot = loop {
            let status = tokio::time::timeout_at(deadline, events.recv())
                .await
                .expect("el monitor reemplaza el cursor vencido por snapshot")
                .expect("el monitor publica estados");
            if status.phase == ConnectionPhase::Connected
                && status.recovery == Some(RecoveryResult::FreshSnapshot)
            {
                break status;
            }
        };
        assert_eq!(snapshot.phase, ConnectionPhase::Connected);
        shutdown_tx.send_replace(true);
        let (result, closed) = timeout(Duration::from_secs(5), monitor)
            .await
            .expect("el monitor atiende shutdown")
            .expect("la tarea del monitor termina sin pánico");
        assert_eq!(result, SessionExit::Stop);
        assert!(closed, "el Core cierra por EOF tras el test");
        app_handle.unlisten(listener_id);
        let _ = std::fs::remove_dir_all(root);
    }
}
