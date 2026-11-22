//! Persistencia append-only de transiciones del Core local.

use std::{sync::Arc, time::Duration};

use sha2::{Digest, Sha256};
use shared::public_interface::Clock;
use sqlx::{Row, SqlitePool};
use tokio::time::sleep;
use uuid::Uuid;

const MAX_RECORD_ATTEMPTS: u32 = 5;
pub(super) const REPLAY_CAPACITY: usize = 64;

/// Transiciones históricas de un proceso; `Ready` no representa readiness viva.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    Starting,
    Ready,
    Stopped,
}

impl ProcessStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Starting => "STARTING",
            Self::Ready => "READY",
            Self::Stopped => "STOPPED",
        }
    }

    fn from_str(value: &str) -> Result<Self, JournalError> {
        match value {
            "STARTING" => Ok(Self::Starting),
            "READY" => Ok(Self::Ready),
            "STOPPED" => Ok(Self::Stopped),
            _ => Err(JournalError::UnknownStatus(value.to_string())),
        }
    }

    pub fn is_ready(self) -> bool {
        self == Self::Ready
    }
}

/// Identidad de una ejecución del Core dentro del alcance estable del almacén.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessSession {
    pub stream_scope_id: String,
    pub process_instance_id: String,
    pub starting_event: StatusRecord,
}

/// Evento append-only persistido y devuelto al adaptador gRPC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusRecord {
    pub id: String,
    pub stream_scope_id: String,
    pub process_instance_id: String,
    pub event_sequence_id: u64,
    pub status: ProcessStatus,
    pub created_at_ns: i64,
    pub updated_at_ns: i64,
    pub audit_hash: String,
    pub audit_chain_hash: Option<String>,
}

impl StatusRecord {
    pub fn ready(&self) -> bool {
        self.status.is_ready()
    }

    pub fn observed_at_unix_ms(&self) -> u64 {
        self.created_at_ns.max(0) as u64 / 1_000_000
    }
}

/// Errores del puerto de historial durable.
#[derive(Debug, thiserror::Error)]
pub enum JournalError {
    #[error("falló la operación de base de datos")]
    Database(#[from] sqlx::Error),
    #[error("contención de escritura tras {attempts} intentos")]
    WriteContention { attempts: u32 },
    #[error("la secuencia de eventos excede el límite permitido")]
    SequenceOverflow,
    #[error("timestamp inválido para persistencia")]
    InvalidTimestamp,
    #[error("estado de readiness desconocido: {0}")]
    UnknownStatus(String),
    #[error("la sesión de proceso ya no es la última sesión activa")]
    StaleProcess,
    #[error("transición de readiness inválida")]
    InvalidTransition,
    #[error("el cursor no pertenece al alcance durable")]
    CursorScopeMismatch,
    #[error("el cursor está después del último evento")]
    CursorAhead,
    #[error("el cursor expiró; el historial disponible inicia en {oldest_sequence}")]
    CursorExpired { oldest_sequence: u64 },
    #[error("la secuencia almacenada es inválida")]
    InvalidSequence,
}

#[derive(Clone)]
pub(super) struct DesktopStatusJournal {
    pool: SqlitePool,
    clock: Arc<dyn Clock>,
}

impl DesktopStatusJournal {
    pub(super) fn new(pool: SqlitePool, clock: Arc<dyn Clock>) -> Self {
        Self { pool, clock }
    }

    pub(super) async fn begin_process(&self) -> Result<ProcessSession, JournalError> {
        for attempt in 1..=MAX_RECORD_ATTEMPTS {
            match self.begin_process_once().await {
                Ok(session) => return Ok(session),
                Err(error)
                    if is_transient_write_conflict(&error) && attempt < MAX_RECORD_ATTEMPTS =>
                {
                    sleep(Duration::from_millis(u64::from(attempt) * 10)).await;
                }
                Err(error) if is_transient_write_conflict(&error) => {
                    return Err(JournalError::WriteContention {
                        attempts: MAX_RECORD_ATTEMPTS,
                    });
                }
                Err(error) => return Err(error),
            }
        }
        Err(JournalError::WriteContention {
            attempts: MAX_RECORD_ATTEMPTS,
        })
    }

    async fn begin_process_once(&self) -> Result<ProcessSession, JournalError> {
        let mut transaction = self.pool.begin_with("BEGIN IMMEDIATE").await?;
        let previous = latest_record(&mut *transaction).await?;
        let stream_scope_id = previous
            .as_ref()
            .map(|event| event.stream_scope_id.clone())
            .unwrap_or_else(|| Uuid::now_v7().to_string());
        let process_instance_id = Uuid::now_v7().to_string();
        let event_sequence_id = next_sequence(previous.as_ref())?;
        let now_ns = self.timestamp_ns()?;
        let event = new_record(
            &stream_scope_id,
            &process_instance_id,
            event_sequence_id,
            ProcessStatus::Starting,
            now_ns,
            previous.as_ref().map(|record| record.audit_hash.as_str()),
        )?;
        insert_record(&mut *transaction, &event).await?;
        transaction.commit().await?;

        Ok(ProcessSession {
            stream_scope_id,
            process_instance_id,
            starting_event: event,
        })
    }

    pub(super) async fn transition(
        &self,
        session: &ProcessSession,
        status: ProcessStatus,
    ) -> Result<StatusRecord, JournalError> {
        for attempt in 1..=MAX_RECORD_ATTEMPTS {
            match self.transition_once(session, status).await {
                Ok(event) => return Ok(event),
                Err(error)
                    if is_transient_write_conflict(&error) && attempt < MAX_RECORD_ATTEMPTS =>
                {
                    sleep(Duration::from_millis(u64::from(attempt) * 10)).await;
                }
                Err(error) if is_transient_write_conflict(&error) => {
                    return Err(JournalError::WriteContention {
                        attempts: MAX_RECORD_ATTEMPTS,
                    });
                }
                Err(error) => return Err(error),
            }
        }
        Err(JournalError::WriteContention {
            attempts: MAX_RECORD_ATTEMPTS,
        })
    }

    async fn transition_once(
        &self,
        session: &ProcessSession,
        status: ProcessStatus,
    ) -> Result<StatusRecord, JournalError> {
        let mut transaction = self.pool.begin_with("BEGIN IMMEDIATE").await?;
        let previous = latest_record(&mut *transaction)
            .await?
            .ok_or(JournalError::StaleProcess)?;
        if previous.stream_scope_id != session.stream_scope_id
            || previous.process_instance_id != session.process_instance_id
        {
            return Err(JournalError::StaleProcess);
        }
        if previous.status == status {
            transaction.rollback().await?;
            return Ok(previous);
        }
        let valid = matches!(
            (previous.status, status),
            (ProcessStatus::Starting, ProcessStatus::Ready)
                | (ProcessStatus::Starting, ProcessStatus::Stopped)
                | (ProcessStatus::Ready, ProcessStatus::Stopped)
        );
        if !valid {
            return Err(JournalError::InvalidTransition);
        }

        let event = new_record(
            &session.stream_scope_id,
            &session.process_instance_id,
            next_sequence(Some(&previous))?,
            status,
            self.timestamp_ns()?,
            Some(&previous.audit_hash),
        )?;
        insert_record(&mut *transaction, &event).await?;
        transaction.commit().await?;
        Ok(event)
    }

    pub(super) async fn events_after(
        &self,
        stream_scope_id: &str,
        after_sequence: u64,
    ) -> Result<Vec<StatusRecord>, JournalError> {
        let tail = latest_record(&self.pool)
            .await?
            .ok_or(JournalError::CursorScopeMismatch)?;
        if tail.stream_scope_id != stream_scope_id {
            return Err(JournalError::CursorScopeMismatch);
        }
        let after_sequence =
            i64::try_from(after_sequence).map_err(|_| JournalError::CursorAhead)?;
        let tail_sequence =
            i64::try_from(tail.event_sequence_id).map_err(|_| JournalError::InvalidSequence)?;
        if after_sequence > tail_sequence {
            return Err(JournalError::CursorAhead);
        }

        let oldest: Option<i64> = sqlx::query_scalar(
            "SELECT MIN(event_sequence_id) FROM (\
                SELECT event_sequence_id FROM desktop_status_events \
                ORDER BY event_sequence_id DESC LIMIT ?\
             )",
        )
        .bind(REPLAY_CAPACITY as i64)
        .fetch_one(&self.pool)
        .await?;
        let oldest = oldest.ok_or(JournalError::CursorScopeMismatch)?;
        if after_sequence.saturating_add(1) < oldest {
            return Err(JournalError::CursorExpired {
                oldest_sequence: u64::try_from(oldest)
                    .map_err(|_| JournalError::InvalidSequence)?,
            });
        }

        let rows = sqlx::query(
            "SELECT id, created_at, updated_at, audit_hash, audit_chain_hash, \
                    event_sequence_id, stream_scope_id, process_instance_id, status \
             FROM desktop_status_events \
             WHERE stream_scope_id = ? AND event_sequence_id > ? \
             ORDER BY event_sequence_id ASC LIMIT ?",
        )
        .bind(stream_scope_id)
        .bind(after_sequence)
        .bind(REPLAY_CAPACITY as i64)
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(record_from_row).collect()
    }

    fn timestamp_ns(&self) -> Result<i64, JournalError> {
        let now_ns = self.clock.timestamp_ns();
        if now_ns < 0 {
            return Err(JournalError::InvalidTimestamp);
        }
        Ok(now_ns)
    }
}

fn next_sequence(previous: Option<&StatusRecord>) -> Result<i64, JournalError> {
    previous
        .map(|record| i64::try_from(record.event_sequence_id))
        .transpose()
        .map_err(|_| JournalError::InvalidSequence)?
        .unwrap_or(0)
        .checked_add(1)
        .ok_or(JournalError::SequenceOverflow)
}

fn new_record(
    stream_scope_id: &str,
    process_instance_id: &str,
    event_sequence_id: i64,
    status: ProcessStatus,
    timestamp_ns: i64,
    previous_hash: Option<&str>,
) -> Result<StatusRecord, JournalError> {
    let id = Uuid::now_v7().to_string();
    let audit_hash = compute_audit_hash(
        &id,
        timestamp_ns,
        event_sequence_id,
        stream_scope_id,
        process_instance_id,
        status,
    );
    Ok(StatusRecord {
        id,
        stream_scope_id: stream_scope_id.to_string(),
        process_instance_id: process_instance_id.to_string(),
        event_sequence_id: u64::try_from(event_sequence_id)
            .map_err(|_| JournalError::InvalidSequence)?,
        status,
        created_at_ns: timestamp_ns,
        updated_at_ns: timestamp_ns,
        audit_hash,
        audit_chain_hash: previous_hash.map(str::to_string),
    })
}

fn compute_audit_hash(
    id: &str,
    timestamp_ns: i64,
    event_sequence_id: i64,
    stream_scope_id: &str,
    process_instance_id: &str,
    status: ProcessStatus,
) -> String {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, b"desktop-status-v1");
    hash_field(&mut hasher, id.as_bytes());
    hash_field(&mut hasher, &timestamp_ns.to_be_bytes());
    hash_field(&mut hasher, &event_sequence_id.to_be_bytes());
    hash_field(&mut hasher, stream_scope_id.as_bytes());
    hash_field(&mut hasher, process_instance_id.as_bytes());
    hash_field(&mut hasher, status.as_str().as_bytes());
    let digest = hasher.finalize();
    let mut encoded = String::with_capacity(digest.len() * 2);
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for byte in digest {
        encoded.push(HEX[usize::from(byte >> 4)] as char);
        encoded.push(HEX[usize::from(byte & 0x0f)] as char);
    }
    encoded
}

fn hash_field(hasher: &mut Sha256, field: &[u8]) {
    hasher.update((field.len() as u64).to_be_bytes());
    hasher.update(field);
}

async fn insert_record(
    connection: &mut sqlx::SqliteConnection,
    record: &StatusRecord,
) -> Result<(), JournalError> {
    sqlx::query(
        "INSERT INTO desktop_status_events (\
            id, created_at, updated_at, audit_hash, audit_chain_hash, event_sequence_id, \
            stream_scope_id, process_instance_id, status\
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&record.id)
    .bind(record.created_at_ns)
    .bind(record.updated_at_ns)
    .bind(&record.audit_hash)
    .bind(&record.audit_chain_hash)
    .bind(i64::try_from(record.event_sequence_id).map_err(|_| JournalError::InvalidSequence)?)
    .bind(&record.stream_scope_id)
    .bind(&record.process_instance_id)
    .bind(record.status.as_str())
    .execute(connection)
    .await?;
    Ok(())
}

async fn latest_record<'e, E>(executor: E) -> Result<Option<StatusRecord>, JournalError>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let row = sqlx::query(
        "SELECT id, created_at, updated_at, audit_hash, audit_chain_hash, \
                event_sequence_id, stream_scope_id, process_instance_id, status \
         FROM desktop_status_events ORDER BY event_sequence_id DESC LIMIT 1",
    )
    .fetch_optional(executor)
    .await?;
    row.as_ref().map(record_from_row).transpose()
}

fn record_from_row(row: &sqlx::sqlite::SqliteRow) -> Result<StatusRecord, JournalError> {
    let sequence: i64 = row.try_get("event_sequence_id")?;
    Ok(StatusRecord {
        id: row.try_get("id")?,
        created_at_ns: row.try_get("created_at")?,
        updated_at_ns: row.try_get("updated_at")?,
        audit_hash: row.try_get("audit_hash")?,
        audit_chain_hash: row.try_get("audit_chain_hash")?,
        event_sequence_id: u64::try_from(sequence).map_err(|_| JournalError::InvalidSequence)?,
        stream_scope_id: row.try_get("stream_scope_id")?,
        process_instance_id: row.try_get("process_instance_id")?,
        status: ProcessStatus::from_str(&row.try_get::<String, _>("status")?)?,
    })
}

fn is_transient_write_conflict(error: &JournalError) -> bool {
    let JournalError::Database(sqlx::Error::Database(db)) = error else {
        return false;
    };
    let message = db.message().to_lowercase();
    message.contains("database is locked")
        || message.contains("database table is locked")
        || (db.is_unique_violation() && message.contains("event_sequence_id"))
}

#[cfg(test)]
mod tests {
    use std::{
        str::FromStr,
        sync::atomic::{AtomicI64, Ordering},
    };

    use shared::public_interface::{create_pool, run_migrations};
    use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
    use tokio::task::JoinSet;

    use super::*;

    struct TestClock(AtomicI64);

    impl Clock for TestClock {
        fn timestamp_ns(&self) -> i64 {
            self.0.fetch_add(1, Ordering::SeqCst)
        }
    }

    async fn setup() -> (tempfile::TempDir, DesktopStatusJournal) {
        let directory = tempfile::tempdir().expect("crear directorio temporal de prueba");
        let path = directory.path().join("desktop-status.sqlite");
        let database_url = format!("sqlite://{}", path.display());
        let pool = create_pool(&database_url).await.expect("abrir base SQLite");
        run_migrations(&pool).await.expect("aplicar migraciones");
        let clock = Arc::new(TestClock(AtomicI64::new(1_800_000_000_000_000_000)));
        (directory, DesktopStatusJournal::new(pool, clock))
    }

    async fn low_timeout_pool(path: &std::path::Path, timeout: Duration) -> SqlitePool {
        let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", path.display()))
            .expect("URL SQLite válida")
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(timeout)
            .create_if_missing(true);
        SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await
            .expect("pool de prueba con timeout acotado")
    }

    #[tokio::test]
    async fn concurrent_process_starts_have_dense_unique_sequences() {
        let (_directory, history) = setup().await;
        let mut writers = JoinSet::new();
        for _ in 0..16 {
            let history = history.clone();
            writers.spawn(async move {
                history
                    .begin_process()
                    .await
                    .expect("el escritor agrega su evento de inicio")
            });
        }

        let mut sessions = Vec::with_capacity(16);
        while let Some(result) = writers.join_next().await {
            sessions.push(result.expect("la tarea de escritura termina"));
        }
        let scope = sessions[0].stream_scope_id.clone();
        let mut sequences: Vec<_> = sessions
            .iter()
            .map(|session| session.starting_event.event_sequence_id)
            .collect();
        sequences.sort_unstable();
        assert_eq!(sequences, (1..=16).collect::<Vec<_>>());
        assert!(sessions
            .iter()
            .all(|session| session.stream_scope_id == scope));

        let chain = history
            .events_after(&scope, 0)
            .await
            .expect("replay completo");
        assert_eq!(chain.len(), 16);
        assert!(chain.windows(2).all(|pair| {
            pair[1].event_sequence_id == pair[0].event_sequence_id + 1
                && pair[1].audit_chain_hash.as_deref() == Some(pair[0].audit_hash.as_str())
        }));
    }

    #[tokio::test]
    async fn returned_event_has_new_hash_link_and_nanosecond_timestamp() {
        let (_directory, history) = setup().await;
        let session = history.begin_process().await.expect("inicio persistido");
        let ready = history
            .transition(&session, ProcessStatus::Ready)
            .await
            .expect("ready persistido");

        assert_eq!(ready.status, ProcessStatus::Ready);
        assert!(ready.created_at_ns > session.starting_event.created_at_ns);
        assert_eq!(ready.updated_at_ns, ready.created_at_ns);
        assert_ne!(ready.audit_hash, session.starting_event.audit_hash);
        assert_eq!(
            ready.audit_chain_hash.as_deref(),
            Some(session.starting_event.audit_hash.as_str())
        );
        assert_eq!(
            ready.event_sequence_id,
            session.starting_event.event_sequence_id + 1
        );
        assert_eq!(ready.audit_hash.len(), 64);
    }

    #[tokio::test]
    async fn old_cursor_expires_at_logical_retention_boundary_without_deleting_history() {
        let (_directory, history) = setup().await;
        let first = history.begin_process().await.expect("alcance inicial");
        history
            .transition(&first, ProcessStatus::Ready)
            .await
            .expect("primer evento ready");
        for _ in 0..=REPLAY_CAPACITY {
            history
                .begin_process()
                .await
                .expect("nuevo evento de inicio");
        }

        let expired = history
            .events_after(
                &first.stream_scope_id,
                first.starting_event.event_sequence_id,
            )
            .await
            .expect_err("cursor anterior a la ventana debe expirar");
        assert!(matches!(expired, JournalError::CursorExpired { .. }));

        let pool = history.pool.clone();
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM desktop_status_events")
            .fetch_one(&pool)
            .await
            .expect("historial físico consultable");
        assert_eq!(count, 2 + REPLAY_CAPACITY as i64 + 1);
    }

    #[tokio::test]
    async fn sustained_sqlite_contention_returns_exact_retry_count_and_writes_nothing() {
        let directory = tempfile::tempdir().expect("crear directorio temporal de prueba");
        let path = directory.path().join("contention.sqlite");
        let database_url = format!("sqlite://{}", path.display());
        let migrated_pool = create_pool(&database_url).await.expect("abrir base SQLite");
        run_migrations(&migrated_pool)
            .await
            .expect("aplicar migraciones");

        let holder = low_timeout_pool(&path, Duration::from_millis(2)).await;
        let writer = low_timeout_pool(&path, Duration::from_millis(2)).await;
        let lock = holder
            .begin_with("BEGIN IMMEDIATE")
            .await
            .expect("adquirir bloqueo de escritura");
        let history = DesktopStatusJournal::new(
            writer.clone(),
            Arc::new(TestClock(AtomicI64::new(1_800_000_000_000_000_000))),
        );

        let error = history
            .begin_process()
            .await
            .expect_err("la contención sostenida agota los reintentos");
        assert!(matches!(
            error,
            JournalError::WriteContention {
                attempts: MAX_RECORD_ATTEMPTS
            }
        ));
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM desktop_status_events")
            .fetch_one(&writer)
            .await
            .expect("no se escribió un evento parcial");
        assert_eq!(count, 0);

        lock.rollback().await.expect("liberar el bloqueo");
        let session = history
            .begin_process()
            .await
            .expect("escritura funciona al liberar el lock");
        assert_eq!(session.starting_event.event_sequence_id, 1);
    }

    #[tokio::test]
    async fn retry_classifier_distinguishes_sequence_collision_from_other_failures() {
        let (_directory, history) = setup().await;
        let session = history.begin_process().await.expect("fila génesis");
        let pool = history.pool.clone();
        let duplicate_id = sqlx::query(
            "INSERT INTO desktop_status_events (\
                id, created_at, updated_at, audit_hash, audit_chain_hash, event_sequence_id, \
                stream_scope_id, process_instance_id, status\
             ) VALUES (?, 1, 1, ?, NULL, 10, ?, ?, 'STARTING')",
        )
        .bind(&session.starting_event.id)
        .bind("a".repeat(64))
        .bind(&session.stream_scope_id)
        .bind("process-duplicate-id")
        .execute(&pool)
        .await
        .expect_err("PK duplicada se clasifica como error permanente");
        assert!(!is_transient_write_conflict(&JournalError::Database(
            duplicate_id
        )));

        let duplicate_sequence = sqlx::query(
            "INSERT INTO desktop_status_events (\
                id, created_at, updated_at, audit_hash, audit_chain_hash, event_sequence_id, \
                stream_scope_id, process_instance_id, status\
             ) VALUES (?, 1, 1, ?, NULL, ?, ?, ?, 'STARTING')",
        )
        .bind(Uuid::now_v7().to_string())
        .bind("b".repeat(64))
        .bind(i64::try_from(session.starting_event.event_sequence_id).expect("secuencia válida"))
        .bind(&session.stream_scope_id)
        .bind("process-duplicate-sequence")
        .execute(&pool)
        .await
        .expect_err("colisión de secuencia debe ser reintentable");
        assert!(is_transient_write_conflict(&JournalError::Database(
            duplicate_sequence
        )));
        assert!(!is_transient_write_conflict(&JournalError::Database(
            sqlx::Error::RowNotFound
        )));
    }

    #[tokio::test]
    async fn append_only_triggers_reject_update_and_delete() {
        let (_directory, history) = setup().await;
        let session = history.begin_process().await.expect("registro inicial");
        let pool = history.pool.clone();
        let update =
            sqlx::query("UPDATE desktop_status_events SET status = 'STOPPED' WHERE id = ?")
                .bind(&session.starting_event.id)
                .execute(&pool)
                .await
                .expect_err("el trigger impide reescribir un evento");
        assert!(update.to_string().contains("append-only"));
        let delete = sqlx::query("DELETE FROM desktop_status_events WHERE id = ?")
            .bind(&session.starting_event.id)
            .execute(&pool)
            .await
            .expect_err("el trigger impide borrar un evento");
        assert!(delete.to_string().contains("append-only"));
    }
}
