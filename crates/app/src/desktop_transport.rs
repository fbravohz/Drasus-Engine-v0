//! Transporte gRPC autenticado para el arranque local de Tauri.

use std::{
    fmt,
    io::Write,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    pin::Pin,
    sync::Arc,
    time::{Duration, Instant},
};

use desktop_packaging_manager::public_interface::{
    DesktopStatusHistory, JournalError, ProcessSession, StatusRecord,
};
use shared::public_interface::{create_pool, run_migrations, SystemClock};
use tokio::{
    io::{AsyncRead, AsyncReadExt},
    net::TcpListener,
    sync::{broadcast, mpsc, oneshot, watch, Mutex},
    task::JoinHandle,
};
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{metadata::MetadataValue, transport::Server, Request, Response, Status};
use zeroize::Zeroize;

pub mod wire {
    tonic::include_proto!("drasus.desktop.v1");
}

use wire::{
    desktop_bootstrap_server::{DesktopBootstrap, DesktopBootstrapServer},
    ReadinessReply, ReadinessRequest, StatusEvent, StatusKind, WatchStatusRequest,
};

const SESSION_SECRET_LENGTH: usize = 32;
const STATUS_HISTORY_CAPACITY: usize = 64;
const AUTH_REQUIRED: &str = "AUTH_REQUIRED";
const CURSOR_GAP: &str = "CURSOR_GAP";
const CURSOR_SCOPE_MISMATCH: &str = "CURSOR_SCOPE_MISMATCH";

/// Contrato de arranque: el host escribe 32 bytes secretos por stdin y lee una sola línea JSON por stdout.
pub(super) async fn serve_from_stdin(
    session_ttl: Duration,
    database_path: &str,
) -> Result<(), TransportError> {
    if session_ttl.is_zero() {
        return Err(TransportError::InvalidConfiguration);
    }

    let mut stdin = tokio::io::stdin();
    let secret = read_session_secret(&mut stdin).await?;
    let auth = SessionAuth::new(secret, session_ttl)?;
    let database_url = format!("sqlite://{database_path}");
    let pool = create_pool(&database_url)
        .await
        .map_err(|_| TransportError::Persistence)?;
    run_migrations(&pool)
        .await
        .map_err(|_| TransportError::Persistence)?;
    let history = DesktopStatusHistory::new(pool, Arc::new(SystemClock::default()));
    let process = history
        .begin_process()
        .await
        .map_err(|_| TransportError::Persistence)?;
    let feed = StatusFeed::new(history, process);
    let (address, shutdown, server) = start_server(auth, feed.clone()).await?;
    feed.set_ready(true).await?;

    let readiness = StartupEndpoint {
        address: address.ip().to_string(),
        port: address.port(),
    };
    let line = serde_json::to_string(&readiness).map_err(|_| TransportError::StartupOutput)?;
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    writeln!(stdout, "{line}").map_err(|_| TransportError::StartupOutput)?;
    stdout.flush().map_err(|_| TransportError::StartupOutput)?;

    let shutdown_cause = tokio::select! {
        result = wait_for_bootstrap_eof(&mut stdin) => result,
        _ = tokio::signal::ctrl_c() => Ok(()),
        _ = sigterm_received() => Ok(()),
    };
    let server_result = stop_server(&feed, shutdown, server).await;
    shutdown_cause?;
    server_result?;
    Ok(())
}

#[derive(serde::Serialize)]
struct StartupEndpoint {
    address: String,
    port: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum TransportError {
    InvalidConfiguration,
    InvalidSessionInput,
    UnexpectedSessionInput,
    SessionPipeRead,
    SessionExpiry,
    Persistence,
    Bind,
    ServerTask,
    StartupOutput,
    Server,
}

impl fmt::Display for TransportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidConfiguration => "configuración de sesión inválida",
            Self::InvalidSessionInput => "entrada de sesión inválida",
            Self::UnexpectedSessionInput => "datos inesperados en el canal de sesión",
            Self::SessionPipeRead => "no se pudo leer el canal de sesión",
            Self::SessionExpiry => "no se pudo calcular el vencimiento de sesión",
            Self::Persistence => "no se pudo inicializar el historial local de readiness",
            Self::Bind => "no se pudo abrir el listener local",
            Self::ServerTask => "falló la tarea del servidor local",
            Self::StartupOutput => "no se pudo publicar el endpoint de readiness",
            Self::Server => "falló el servidor gRPC local",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for TransportError {}

impl From<tonic::transport::Error> for TransportError {
    fn from(_: tonic::transport::Error) -> Self {
        Self::Server
    }
}

/// Guarda el secreto de sesión fuera de argumentos y lo borra al cerrar su dueño.
struct SessionSecret([u8; SESSION_SECRET_LENGTH]);

impl fmt::Debug for SessionSecret {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SessionSecret([REDACTED])")
    }
}

impl Drop for SessionSecret {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

async fn read_session_secret(
    reader: &mut (impl AsyncRead + Unpin),
) -> Result<SessionSecret, TransportError> {
    let mut bytes = [0_u8; SESSION_SECRET_LENGTH];
    if reader.read_exact(&mut bytes).await.is_err() {
        bytes.zeroize();
        return Err(TransportError::InvalidSessionInput);
    }
    Ok(SessionSecret(bytes))
}

async fn wait_for_bootstrap_eof(
    reader: &mut (impl AsyncRead + Unpin),
) -> Result<(), TransportError> {
    let mut trailing = [0_u8; 1];
    match reader
        .read(&mut trailing)
        .await
        .map_err(|_| TransportError::SessionPipeRead)?
    {
        0 => Ok(()),
        _ => {
            trailing.zeroize();
            Err(TransportError::UnexpectedSessionInput)
        }
    }
}

async fn stop_server(
    feed: &StatusFeed,
    shutdown: oneshot::Sender<()>,
    server: JoinHandle<Result<(), tonic::transport::Error>>,
) -> Result<(), TransportError> {
    let readiness_update = feed.set_ready(false).await;
    feed.close_streams();
    let _ = shutdown.send(());
    server.await.map_err(|_| TransportError::ServerTask)??;
    readiness_update
}

struct SessionAuth {
    secret: SessionSecret,
    valid_until: Instant,
}

impl SessionAuth {
    fn new(secret: SessionSecret, ttl: Duration) -> Result<Arc<Self>, TransportError> {
        let valid_until = Instant::now()
            .checked_add(ttl)
            .ok_or(TransportError::SessionExpiry)?;
        Ok(Arc::new(Self {
            secret,
            valid_until,
        }))
    }

    fn accepts(&self, authorization: Option<&MetadataValue<tonic::metadata::Ascii>>) -> bool {
        if Instant::now() >= self.valid_until {
            return false;
        }
        let Some(value) = authorization.and_then(|value| value.to_str().ok()) else {
            return false;
        };
        let Some(encoded) = value.strip_prefix("Bearer ") else {
            return false;
        };
        let Some(mut candidate) = decode_hex_secret(encoded) else {
            return false;
        };
        let matches = constant_time_equal(&self.secret.0, &candidate);
        candidate.zeroize();
        matches
    }
}

#[derive(Clone)]
struct AuthInterceptor(Arc<SessionAuth>);

impl tonic::service::Interceptor for AuthInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        if self.0.accepts(request.metadata().get("authorization")) {
            Ok(request)
        } else {
            Err(Status::unauthenticated(AUTH_REQUIRED))
        }
    }
}

fn decode_hex_secret(encoded: &str) -> Option<[u8; SESSION_SECRET_LENGTH]> {
    if encoded.len() != SESSION_SECRET_LENGTH * 2 {
        return None;
    }

    let mut bytes = [0_u8; SESSION_SECRET_LENGTH];
    for (index, pair) in encoded.as_bytes().chunks_exact(2).enumerate() {
        let high = hex_nibble(pair[0])?;
        let low = hex_nibble(pair[1])?;
        bytes[index] = (high << 4) | low;
    }
    Some(bytes)
}

fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn constant_time_equal(
    left: &[u8; SESSION_SECRET_LENGTH],
    right: &[u8; SESSION_SECRET_LENGTH],
) -> bool {
    left.iter()
        .zip(right)
        .fold(0_u8, |difference, (left, right)| {
            difference | (left ^ right)
        })
        == 0
}

#[derive(Clone)]
struct StatusFeed {
    history: DesktopStatusHistory,
    process: ProcessSession,
    current: Arc<Mutex<StatusEvent>>,
    updates: broadcast::Sender<StatusEvent>,
    stopping: watch::Sender<bool>,
}

impl StatusFeed {
    fn new(history: DesktopStatusHistory, process: ProcessSession) -> Self {
        let initial =
            status_event_from_record(&process.starting_event, StatusKind::ReadinessChanged);
        let (updates, _) = broadcast::channel(STATUS_HISTORY_CAPACITY);
        let (stopping, _) = watch::channel(false);
        Self {
            history,
            process,
            current: Arc::new(Mutex::new(initial)),
            updates,
            stopping,
        }
    }

    fn scope(&self) -> &str {
        &self.process.stream_scope_id
    }

    async fn current(&self) -> Result<StatusEvent, Status> {
        Ok(self.current.lock().await.clone())
    }

    async fn set_ready(&self, ready: bool) -> Result<(), TransportError> {
        let mut current = self.current.lock().await;
        if current.ready == ready {
            return Ok(());
        }
        let record = if ready {
            self.history.mark_ready(&self.process).await
        } else {
            self.history.mark_stopped(&self.process).await
        }
        .map_err(|_| TransportError::Persistence)?;
        let event = status_event_from_record(&record, StatusKind::ReadinessChanged);
        *current = event.clone();
        let _ = self.updates.send(event);
        if !ready {
            self.stopping.send_replace(true);
        }
        Ok(())
    }

    fn close_streams(&self) {
        self.stopping.send_replace(true);
    }

    async fn events_after(&self, request: &WatchStatusRequest) -> Result<Vec<StatusEvent>, Status> {
        if request.stream_scope.is_empty() {
            if request.after_sequence != 0 {
                return Err(Status::invalid_argument("CURSOR_SCOPE_REQUIRED"));
            }
            let mut snapshot = self.current().await?;
            snapshot.kind = StatusKind::InitialSnapshot as i32;
            return Ok(vec![snapshot]);
        }
        if request.stream_scope != self.scope() {
            return Err(Status::out_of_range(CURSOR_SCOPE_MISMATCH));
        }
        self.history
            .events_after(&request.stream_scope, request.after_sequence)
            .await
            .map(|records| {
                records
                    .iter()
                    .map(|record| status_event_from_record(record, StatusKind::ReadinessChanged))
                    .collect()
            })
            .map_err(cursor_status)
    }
}

fn status_event_from_record(record: &StatusRecord, kind: StatusKind) -> StatusEvent {
    StatusEvent {
        stream_scope: record.stream_scope_id.clone(),
        sequence: record.event_sequence_id,
        kind: kind as i32,
        ready: record.ready(),
        observed_at_unix_ms: record.observed_at_unix_ms(),
    }
}

fn cursor_status(error: JournalError) -> Status {
    match error {
        JournalError::CursorScopeMismatch => Status::out_of_range(CURSOR_SCOPE_MISMATCH),
        JournalError::CursorAhead | JournalError::CursorExpired { .. } => {
            Status::out_of_range(CURSOR_GAP)
        }
        _ => Status::internal("INTERNAL"),
    }
}

#[derive(Clone)]
struct BootstrapService {
    feed: StatusFeed,
    valid_until: Instant,
}

#[tonic::async_trait]
impl DesktopBootstrap for BootstrapService {
    async fn check_readiness(
        &self,
        _request: Request<ReadinessRequest>,
    ) -> Result<Response<ReadinessReply>, Status> {
        let current = self.feed.current().await?;
        Ok(Response::new(ReadinessReply {
            ready: current.ready,
            stream_scope: current.stream_scope,
            sequence: current.sequence,
            observed_at_unix_ms: current.observed_at_unix_ms,
        }))
    }

    type WatchStatusStream = Pin<Box<dyn Stream<Item = Result<StatusEvent, Status>> + Send>>;

    async fn watch_status(
        &self,
        request: Request<WatchStatusRequest>,
    ) -> Result<Response<Self::WatchStatusStream>, Status> {
        let cursor = request.into_inner();
        let mut updates = self.feed.updates.subscribe();
        let mut stopping = self.feed.stopping.subscribe();
        let replay = self.feed.events_after(&cursor).await?;
        let mut last_sequence = replay
            .last()
            .map(|event| event.sequence)
            .unwrap_or(cursor.after_sequence);
        let feed = self.feed.clone();
        let valid_until = tokio::time::Instant::from_std(self.valid_until);
        let (sender, receiver) = mpsc::channel(STATUS_HISTORY_CAPACITY);

        tokio::spawn(async move {
            for event in replay {
                if sender.send(Ok(event)).await.is_err() {
                    return;
                }
            }
            if *stopping.borrow() {
                return;
            }
            loop {
                let update = tokio::select! {
                    biased;
                    changed = stopping.changed() => {
                        if changed.is_err() || *stopping.borrow() {
                            let resume_request = WatchStatusRequest {
                                stream_scope: feed.scope().to_string(),
                                after_sequence: last_sequence,
                            };
                            if let Ok(events) = feed.events_after(&resume_request).await {
                                for event in events {
                                    if sender.try_send(Ok(event)).is_err() {
                                        return;
                                    }
                                }
                            }
                            return;
                        }
                        continue;
                    }
                    _ = tokio::time::sleep_until(valid_until) => {
                        let _ = sender.send(Err(Status::unauthenticated(AUTH_REQUIRED))).await;
                        return;
                    }
                    update = updates.recv() => update,
                };
                match update {
                    Ok(event)
                        if event.stream_scope == feed.scope() && event.sequence > last_sequence =>
                    {
                        last_sequence = event.sequence;
                        if sender.send(Ok(event)).await.is_err() {
                            return;
                        }
                    }
                    Ok(_) => {}
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        let resume_request = WatchStatusRequest {
                            stream_scope: feed.scope().to_string(),
                            after_sequence: last_sequence,
                        };
                        match feed.events_after(&resume_request).await {
                            Ok(events) => {
                                for event in events {
                                    last_sequence = event.sequence;
                                    if sender.send(Ok(event)).await.is_err() {
                                        return;
                                    }
                                }
                            }
                            Err(error) => {
                                let _ = sender.send(Err(error)).await;
                                return;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Closed) => return,
                }
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(receiver))))
    }
}

async fn start_server(
    auth: Arc<SessionAuth>,
    feed: StatusFeed,
) -> Result<
    (
        SocketAddr,
        oneshot::Sender<()>,
        JoinHandle<Result<(), tonic::transport::Error>>,
    ),
    TransportError,
> {
    let listener = TcpListener::bind(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0)))
        .await
        .map_err(|_| TransportError::Bind)?;
    let address = listener.local_addr().map_err(|_| TransportError::Bind)?;
    if !address.ip().is_loopback() {
        return Err(TransportError::Bind);
    }
    let (shutdown_sender, shutdown_receiver) = oneshot::channel();
    let service = DesktopBootstrapServer::with_interceptor(
        BootstrapService {
            feed,
            valid_until: auth.valid_until,
        },
        AuthInterceptor(auth),
    );
    let server = tokio::spawn(async move {
        Server::builder()
            .add_service(service)
            .serve_with_incoming_shutdown(
                tokio_stream::wrappers::TcpListenerStream::new(listener),
                async {
                    let _ = shutdown_receiver.await;
                },
            )
            .await
    });
    Ok((address, shutdown_sender, server))
}

#[cfg(unix)]
async fn sigterm_received() {
    use tokio::signal::unix::{signal, SignalKind};
    match signal(SignalKind::terminate()) {
        Ok(mut signal) => {
            signal.recv().await;
        }
        Err(_) => std::future::pending::<()>().await,
    }
}

#[cfg(not(unix))]
async fn sigterm_received() {
    std::future::pending::<()>().await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpStream;
    use tonic::service::Interceptor;
    use tonic::Code;
    use wire::desktop_bootstrap_client::DesktopBootstrapClient;

    const TEST_SECRET: [u8; SESSION_SECRET_LENGTH] = [0x5a; SESSION_SECRET_LENGTH];

    fn authorization(
        secret: &[u8; SESSION_SECRET_LENGTH],
    ) -> MetadataValue<tonic::metadata::Ascii> {
        let mut encoded = String::from("Bearer ");
        for byte in secret {
            use fmt::Write as _;
            write!(&mut encoded, "{byte:02x}").expect("escribir en String no falla");
        }
        encoded
            .parse()
            .expect("token hexadecimal válido en metadata ASCII")
    }

    async fn test_feed() -> (StatusFeed, tempfile::TempDir) {
        let directory = tempfile::tempdir().expect("crear almacenamiento temporal");
        let database_path = directory.path().join("bootstrap.sqlite");
        let database_url = format!("sqlite://{}", database_path.display());
        let pool = create_pool(&database_url)
            .await
            .expect("abrir base temporal");
        run_migrations(&pool)
            .await
            .expect("aplicar esquema de prueba");
        let history = DesktopStatusHistory::new(pool, Arc::new(SystemClock::default()));
        let process = history.begin_process().await.expect("persistir inicio");
        let feed = StatusFeed::new(history, process);
        feed.set_ready(true)
            .await
            .expect("persistir readiness viva");
        (feed, directory)
    }

    async fn test_client(
        ttl: Duration,
    ) -> (
        SocketAddr,
        oneshot::Sender<()>,
        JoinHandle<Result<(), tonic::transport::Error>>,
        StatusFeed,
        DesktopBootstrapClient<tonic::transport::Channel>,
        tempfile::TempDir,
    ) {
        let auth = SessionAuth::new(SessionSecret(TEST_SECRET), ttl).expect("TTL de prueba válido");
        let (feed, directory) = test_feed().await;
        let (address, shutdown, server) = start_server(auth, feed.clone())
            .await
            .expect("listener local disponible");
        let endpoint = format!("http://{address}");
        let client = DesktopBootstrapClient::connect(endpoint)
            .await
            .expect("Core local acepta conexiones gRPC");
        (address, shutdown, server, feed, client, directory)
    }

    fn authorized<T>(value: T) -> Request<T> {
        let mut request = Request::new(value);
        request
            .metadata_mut()
            .insert("authorization", authorization(&TEST_SECRET));
        request
    }

    #[tokio::test]
    async fn bootstrap_auth_rejects_missing_and_invalid_session_without_echoing_secret() {
        let (_address, shutdown, server, _feed, mut client, _directory) =
            test_client(Duration::from_secs(30)).await;

        let missing = client
            .check_readiness(Request::new(ReadinessRequest {}))
            .await
            .expect_err("sesión ausente debe rechazarse");
        assert_eq!(missing.code(), Code::Unauthenticated);
        assert_eq!(missing.message(), AUTH_REQUIRED);

        let mut invalid = Request::new(ReadinessRequest {});
        invalid.metadata_mut().insert(
            "authorization",
            MetadataValue::try_from(
                "Bearer 0000000000000000000000000000000000000000000000000000000000000000",
            )
            .expect("token de rechazo válido en metadata"),
        );
        let rejected = client
            .check_readiness(invalid)
            .await
            .expect_err("sesión inválida debe rechazarse");
        assert_eq!(rejected.code(), Code::Unauthenticated);
        assert_eq!(rejected.message(), AUTH_REQUIRED);
        assert!(!rejected.message().contains("5a"));

        let reply = client
            .check_readiness(authorized(ReadinessRequest {}))
            .await
            .expect("sesión vigente debe aceptarse")
            .into_inner();
        assert!(reply.ready);
        assert_eq!(reply.sequence, 2);
        assert!(!reply.stream_scope.is_empty());

        let _ = shutdown.send(());
        server
            .await
            .expect("servidor termina")
            .expect("servidor cierra limpio");
    }

    #[test]
    fn bootstrap_auth_rejects_expired_session() {
        let mut auth = SessionAuth::new(SessionSecret(TEST_SECRET), Duration::from_secs(1))
            .expect("TTL de prueba válido");
        Arc::get_mut(&mut auth)
            .expect("no hay clones antes de iniciar")
            .valid_until = Instant::now() - Duration::from_millis(1);
        let mut request = Request::new(());
        request
            .metadata_mut()
            .insert("authorization", authorization(&TEST_SECRET));
        assert_eq!(
            AuthInterceptor(auth)
                .call(request)
                .expect_err("sesión vencida")
                .code(),
            Code::Unauthenticated
        );
    }

    #[tokio::test]
    async fn bootstrap_recovery_replays_from_cursor_and_rejects_wrong_scope() {
        let (_address, shutdown, server, feed, mut client, _directory) =
            test_client(Duration::from_secs(30)).await;

        let initial = client
            .watch_status(authorized(WatchStatusRequest {
                stream_scope: String::new(),
                after_sequence: 0,
            }))
            .await
            .expect("primera vista disponible")
            .into_inner()
            .message()
            .await
            .expect("stream inicial responde")
            .expect("stream inicial contiene snapshot");
        assert_eq!(initial.sequence, 2);
        assert_eq!(initial.kind, StatusKind::InitialSnapshot as i32);

        let mut stream = client
            .watch_status(authorized(WatchStatusRequest {
                stream_scope: initial.stream_scope.clone(),
                after_sequence: initial.sequence,
            }))
            .await
            .expect("reanudación en cursor vigente")
            .into_inner();
        feed.set_ready(false)
            .await
            .expect("estado de cierre se registra");
        let resumed = stream
            .message()
            .await
            .expect("stream sigue activo")
            .expect("evento posterior al cursor");
        assert_eq!(resumed.sequence, 3);
        assert!(!resumed.ready);

        let wrong_scope = client
            .watch_status(authorized(WatchStatusRequest {
                stream_scope: "scope-from-another-core".to_owned(),
                after_sequence: 1,
            }))
            .await
            .expect_err("no compara cursores de otro alcance");
        assert_eq!(wrong_scope.code(), Code::OutOfRange);
        assert_eq!(wrong_scope.message(), CURSOR_SCOPE_MISMATCH);

        let gap = client
            .watch_status(authorized(WatchStatusRequest {
                stream_scope: initial.stream_scope,
                after_sequence: 4,
            }))
            .await
            .expect_err("cursor futuro debe rechazarse");
        assert_eq!(gap.code(), Code::OutOfRange);
        assert_eq!(gap.message(), CURSOR_GAP);

        drop(stream);
        let _ = shutdown.send(());
        server
            .await
            .expect("servidor termina")
            .expect("servidor cierra limpio");
    }

    #[tokio::test]
    async fn bootstrap_recovery_rejects_cursor_outside_durable_retention() {
        let (feed, _directory) = test_feed().await;
        for _ in 0..(STATUS_HISTORY_CAPACITY / 2) {
            let process = feed
                .history
                .begin_process()
                .await
                .expect("registrar reinicio en historial compartido");
            feed.history
                .mark_ready(&process)
                .await
                .expect("registrar readiness del proceso");
        }

        let auth = SessionAuth::new(SessionSecret(TEST_SECRET), Duration::from_secs(30))
            .expect("TTL de prueba válido");
        let (address, shutdown, server) = start_server(auth, feed.clone())
            .await
            .expect("listener local disponible");
        let mut client = DesktopBootstrapClient::connect(format!("http://{address}"))
            .await
            .expect("Core local acepta conexiones gRPC");
        let expired = client
            .watch_status(authorized(WatchStatusRequest {
                stream_scope: feed.scope().to_owned(),
                after_sequence: 0,
            }))
            .await
            .expect_err("el cursor previo a los últimos 64 eventos expiró");
        assert_eq!(expired.code(), Code::OutOfRange);
        assert_eq!(expired.message(), CURSOR_GAP);

        let _ = shutdown.send(());
        server
            .await
            .expect("servidor termina")
            .expect("servidor cierra limpio");
    }

    #[tokio::test]
    async fn bootstrap_historical_ready_does_not_mark_new_process_live() {
        let (previous_feed, _directory) = test_feed().await;
        let process = previous_feed
            .history
            .begin_process()
            .await
            .expect("registrar nuevo proceso después de READY histórico");
        let feed = StatusFeed::new(previous_feed.history.clone(), process);
        let auth = SessionAuth::new(SessionSecret(TEST_SECRET), Duration::from_secs(30))
            .expect("TTL de prueba válido");
        let (address, shutdown, server) = start_server(auth, feed.clone())
            .await
            .expect("listener local disponible");
        let mut client = DesktopBootstrapClient::connect(format!("http://{address}"))
            .await
            .expect("Core local acepta conexiones gRPC");

        let before_ready = client
            .check_readiness(authorized(ReadinessRequest {}))
            .await
            .expect("consultar readiness del proceso actual")
            .into_inner();
        assert!(!before_ready.ready);
        assert_eq!(before_ready.sequence, 3);

        feed.set_ready(true)
            .await
            .expect("confirmar readiness del proceso nuevo");
        let after_ready = client
            .check_readiness(authorized(ReadinessRequest {}))
            .await
            .expect("consultar readiness tras completar bootstrap")
            .into_inner();
        assert!(after_ready.ready);
        assert_eq!(after_ready.sequence, 4);

        let _ = shutdown.send(());
        server
            .await
            .expect("servidor termina")
            .expect("servidor cierra limpio");
    }

    #[tokio::test]
    async fn bootstrap_session_input_requires_32_bytes_from_private_pipe() {
        let (mut host_pipe, mut core_pipe) = tokio::io::duplex(SESSION_SECRET_LENGTH);
        host_pipe
            .write_all(&TEST_SECRET)
            .await
            .expect("escribir secreto de prueba");
        drop(host_pipe);
        let secret = read_session_secret(&mut core_pipe)
            .await
            .expect("32 bytes de sesión aceptados");
        assert_eq!(secret.0, TEST_SECRET);

        let (mut host_pipe, mut core_pipe) = tokio::io::duplex(SESSION_SECRET_LENGTH);
        host_pipe
            .write_all(&TEST_SECRET[..SESSION_SECRET_LENGTH - 1])
            .await
            .expect("escribir secreto incompleto de prueba");
        drop(host_pipe);
        assert_eq!(
            read_session_secret(&mut core_pipe)
                .await
                .expect_err("secreto corto"),
            TransportError::InvalidSessionInput
        );
    }

    #[tokio::test]
    async fn bootstrap_pipe_eof_shuts_down_and_joins_the_server() {
        let (mut host_pipe, mut core_pipe) = tokio::io::duplex(SESSION_SECRET_LENGTH);
        host_pipe
            .write_all(&TEST_SECRET)
            .await
            .expect("escribir secreto de prueba");
        let secret = read_session_secret(&mut core_pipe)
            .await
            .expect("secreto completo por el pipe");
        let auth = SessionAuth::new(secret, Duration::from_secs(30)).expect("TTL de prueba válido");
        let (feed, _directory) = test_feed().await;
        let (address, shutdown, server) = start_server(auth, feed.clone())
            .await
            .expect("listener local disponible");
        let probe = TcpStream::connect(address)
            .await
            .expect("listener activo antes del cierre");
        drop(probe);

        drop(host_pipe);
        wait_for_bootstrap_eof(&mut core_pipe)
            .await
            .expect("EOF del host solicita cierre ordenado");
        stop_server(&feed, shutdown, server)
            .await
            .expect("el servidor termina y su tarea queda unida");
        assert!(
            TcpStream::connect(address).await.is_err(),
            "el listener debe quedar cerrado después de unir la tarea"
        );
    }

    #[tokio::test]
    async fn bootstrap_listener_is_ipv4_loopback_only_and_port_is_ephemeral() {
        let auth = SessionAuth::new(SessionSecret(TEST_SECRET), Duration::from_secs(30))
            .expect("TTL de prueba válido");
        let (feed, _directory) = test_feed().await;
        let (address, shutdown, server) = start_server(auth, feed.clone())
            .await
            .expect("listener local disponible");
        assert!(address.ip().is_loopback());
        assert_eq!(address.ip(), Ipv4Addr::LOCALHOST);
        assert_ne!(address.port(), 0);
        assert_ne!(address.ip(), Ipv4Addr::UNSPECIFIED);
        let _ = shutdown.send(());
        server
            .await
            .expect("servidor termina")
            .expect("servidor cierra limpio");
    }
}
