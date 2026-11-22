//! Puerto público para el historial local de readiness del Core.

use std::sync::Arc;

use shared::public_interface::Clock;
use sqlx::SqlitePool;

use crate::persistence::{self, DesktopStatusJournal};

pub use crate::persistence::{JournalError, ProcessSession, ProcessStatus, StatusRecord};

/// Abre el ledger append-only que permite recuperar transiciones de readiness.
#[derive(Clone)]
pub struct DesktopStatusHistory {
    journal: DesktopStatusJournal,
}

impl DesktopStatusHistory {
    /// Construye el puerto sobre el pool SQLite ya migrado por el Core.
    pub fn new(pool: SqlitePool, clock: Arc<dyn Clock>) -> Self {
        Self {
            journal: DesktopStatusJournal::new(pool, clock),
        }
    }

    /// Registra el inicio de un proceso y recupera el alcance estable de esta base.
    pub async fn begin_process(&self) -> Result<ProcessSession, JournalError> {
        self.journal.begin_process().await
    }

    /// Confirma que el proceso actual terminó el bootstrap y está listo.
    pub async fn mark_ready(&self, session: &ProcessSession) -> Result<StatusRecord, JournalError> {
        self.journal.transition(session, ProcessStatus::Ready).await
    }

    /// Persiste que el proceso actual dejó de aceptar trabajo.
    pub async fn mark_stopped(
        &self,
        session: &ProcessSession,
    ) -> Result<StatusRecord, JournalError> {
        self.journal
            .transition(session, ProcessStatus::Stopped)
            .await
    }

    /// Recupera eventos después del cursor dentro del alcance durable.
    pub async fn events_after(
        &self,
        stream_scope_id: &str,
        after_sequence: u64,
    ) -> Result<Vec<StatusRecord>, JournalError> {
        self.journal
            .events_after(stream_scope_id, after_sequence)
            .await
    }

    /// Devuelve el límite de retención de replay para el contrato de transporte.
    pub fn replay_capacity(&self) -> usize {
        persistence::REPLAY_CAPACITY
    }
}
