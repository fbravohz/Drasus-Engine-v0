use std::{fmt, path::Path, process::Stdio, time::Duration};

use serde::Deserialize;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, Command},
    time::timeout,
};
use tonic::{
    transport::{Channel, Endpoint},
    Request,
};

pub mod wire {
    tonic::include_proto!("drasus.desktop.v1");
}

use wire::{
    desktop_bootstrap_client::DesktopBootstrapClient, ReadinessRequest, StatusKind,
    WatchStatusRequest,
};

const SESSION_SECRET: [u8; 32] = [0x5a; 32];

#[derive(Deserialize)]
struct StartupEndpoint {
    address: String,
    port: u16,
}

fn authorized<T>(mut request: Request<T>) -> Request<T> {
    let mut bearer = String::from("Bearer ");
    for byte in SESSION_SECRET {
        use fmt::Write as _;
        write!(&mut bearer, "{byte:02x}").expect("escribir en String no falla");
    }
    request.metadata_mut().insert(
        "authorization",
        bearer.parse().expect("metadata ASCII válida"),
    );
    request
}

#[tokio::test]
async fn bootstrap_process_pipe_eof_closes_active_stream_and_exits() {
    let directory = tempfile::tempdir().expect("crear almacenamiento temporal");
    let database_path = directory.path().join("desktop.sqlite");
    let mut child = Command::new(env!("CARGO_BIN_EXE_drasus"))
        .arg("desktop-server")
        .arg("--db")
        .arg(&database_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .expect("el binario Core inicia");
    let mut stdin = child.stdin.take().expect("stdin del Core disponible");
    stdin
        .write_all(&SESSION_SECRET)
        .await
        .expect("el Core recibe la sesión por pipe");

    let stdout = child.stdout.take().expect("stdout de readiness disponible");
    let mut lines = BufReader::new(stdout).lines();
    let endpoint_line = timeout(Duration::from_secs(3), lines.next_line())
        .await
        .expect("el Core publica readiness a tiempo")
        .expect("stdout del Core puede leerse")
        .expect("el Core publica el endpoint");
    let endpoint: StartupEndpoint =
        serde_json::from_str(&endpoint_line).expect("endpoint JSON válido");
    assert_eq!(endpoint.address, "127.0.0.1");
    assert_ne!(endpoint.port, 0);

    let channel: Channel = timeout(
        Duration::from_secs(3),
        Endpoint::from_shared(format!("http://{}:{}", endpoint.address, endpoint.port))
            .expect("endpoint local válido")
            .connect(),
    )
    .await
    .expect("canal gRPC conecta a tiempo")
    .expect("canal gRPC conecta");
    let mut client = DesktopBootstrapClient::new(channel);
    let readiness = client
        .check_readiness(authorized(Request::new(ReadinessRequest {})))
        .await
        .expect("readiness autentica la sesión real del proceso")
        .into_inner();
    assert!(readiness.ready);

    let mut stream = client
        .watch_status(authorized(Request::new(WatchStatusRequest {
            stream_scope: String::new(),
            after_sequence: 0,
        })))
        .await
        .expect("stream autenticado abre en el proceso Core real")
        .into_inner();
    let initial = timeout(Duration::from_secs(3), stream.message())
        .await
        .expect("snapshot llega a tiempo")
        .expect("stream no reporta error")
        .expect("vista inicial presente");
    assert_eq!(initial.kind, StatusKind::InitialSnapshot as i32);

    drop(stdin);
    let closing = timeout(Duration::from_secs(3), stream.message())
        .await
        .expect("EOF termina el stream activo a tiempo")
        .expect("stream termina sin error")
        .expect("el Core publica el último estado antes de cerrar");
    assert!(!closing.ready);
    assert!(timeout(Duration::from_secs(3), stream.message())
        .await
        .expect("stream cierra tras el estado final")
        .expect("cierre del stream sin error")
        .is_none());

    let status = timeout(Duration::from_secs(3), child.wait())
        .await
        .expect("el proceso Core termina tras EOF")
        .expect("proceso Core puede esperarse");
    assert!(status.success(), "el cierre ordenado termina con éxito");
}

async fn launch_core(database_path: &Path) -> (Child, ChildStdin, DesktopBootstrapClient<Channel>) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_drasus"))
        .arg("desktop-server")
        .arg("--db")
        .arg(database_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .expect("el binario Core inicia");
    let mut stdin = child.stdin.take().expect("stdin del Core disponible");
    stdin
        .write_all(&SESSION_SECRET)
        .await
        .expect("el Core recibe la sesión por pipe");
    let stdout = child.stdout.take().expect("stdout de readiness disponible");
    let mut lines = BufReader::new(stdout).lines();
    let endpoint_line = timeout(Duration::from_secs(3), lines.next_line())
        .await
        .expect("el Core publica readiness a tiempo")
        .expect("stdout del Core puede leerse")
        .expect("el Core publica el endpoint");
    let endpoint: StartupEndpoint =
        serde_json::from_str(&endpoint_line).expect("endpoint JSON válido");
    let channel = timeout(
        Duration::from_secs(3),
        Endpoint::from_shared(format!("http://{}:{}", endpoint.address, endpoint.port))
            .expect("endpoint local válido")
            .connect(),
    )
    .await
    .expect("canal gRPC conecta a tiempo")
    .expect("canal gRPC conecta");
    (child, stdin, DesktopBootstrapClient::new(channel))
}

#[tokio::test]
async fn bootstrap_process_restart_recovers_durable_cursor_without_reusing_live_readiness() {
    let directory = tempfile::tempdir().expect("crear almacenamiento temporal");
    let database_path = directory.path().join("desktop.sqlite");
    let (mut first_child, _first_stdin, mut first_client) = launch_core(&database_path).await;

    let first_readiness = first_client
        .check_readiness(authorized(Request::new(ReadinessRequest {})))
        .await
        .expect("el primer Core autentica readiness")
        .into_inner();
    assert!(first_readiness.ready);
    assert_eq!(first_readiness.sequence, 2);

    let mut first_history = first_client
        .watch_status(authorized(Request::new(WatchStatusRequest {
            stream_scope: first_readiness.stream_scope.clone(),
            after_sequence: 1,
        })))
        .await
        .expect("cursor anterior a READY recupera el evento durable")
        .into_inner();
    let historical_ready = timeout(Duration::from_secs(3), first_history.message())
        .await
        .expect("READY histórico llega a tiempo")
        .expect("stream histórico responde")
        .expect("READY histórico está persistido");
    assert_eq!(historical_ready.sequence, 2);
    assert!(historical_ready.ready);

    first_child
        .kill()
        .await
        .expect("terminar primer proceso Core");
    let first_status = timeout(Duration::from_secs(3), first_child.wait())
        .await
        .expect("primer proceso termina a tiempo")
        .expect("primer proceso puede esperarse");
    assert!(
        !first_status.success(),
        "el primer proceso terminó abruptamente"
    );

    let (mut second_child, second_stdin, mut second_client) = launch_core(&database_path).await;
    let current_readiness = second_client
        .check_readiness(authorized(Request::new(ReadinessRequest {})))
        .await
        .expect("el segundo Core recalcula readiness viva")
        .into_inner();
    assert!(current_readiness.ready);
    assert_eq!(current_readiness.stream_scope, first_readiness.stream_scope);
    assert_eq!(current_readiness.sequence, 4);

    let mut resumed = second_client
        .watch_status(authorized(Request::new(WatchStatusRequest {
            stream_scope: first_readiness.stream_scope,
            after_sequence: 1,
        })))
        .await
        .expect("el segundo proceso acepta el cursor durable del primero")
        .into_inner();
    let mut replayed = Vec::new();
    for _ in 0..3 {
        replayed.push(
            timeout(Duration::from_secs(3), resumed.message())
                .await
                .expect("evento durable llega a tiempo")
                .expect("stream durable responde")
                .expect("evento esperado en replay"),
        );
    }
    assert_eq!(
        replayed
            .iter()
            .map(|event| (event.sequence, event.ready))
            .collect::<Vec<_>>(),
        vec![(2, true), (3, false), (4, true)]
    );
    assert!(
        current_readiness.ready,
        "el snapshot vivo viene del segundo proceso"
    );

    drop(second_stdin);
    let second_status = timeout(Duration::from_secs(3), second_child.wait())
        .await
        .expect("segundo proceso termina tras EOF")
        .expect("segundo proceso puede esperarse");
    assert!(
        second_status.success(),
        "el cierre ordenado termina con éxito"
    );
}
