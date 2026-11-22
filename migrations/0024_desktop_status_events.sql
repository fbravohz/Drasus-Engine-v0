-- Historial durable del estado de readiness, propiedad de desktop-packaging-manager.
-- El estado vivo se mantiene en memoria por proceso; el historial permite
-- reanudar el stream y nunca se usa como sustituto de CheckReadiness.

CREATE TABLE IF NOT EXISTS desktop_status_events (
    id                  TEXT    NOT NULL PRIMARY KEY, -- UUIDv7
    created_at          INTEGER NOT NULL,             -- Nanosegundos UTC
    updated_at          INTEGER NOT NULL,             -- Igual a created_at; ledger inmutable
    audit_hash          TEXT    NOT NULL CHECK (length(audit_hash) = 64),
    audit_chain_hash    TEXT    CHECK (audit_chain_hash IS NULL OR length(audit_chain_hash) = 64),
    event_sequence_id   INTEGER NOT NULL UNIQUE CHECK (event_sequence_id > 0),
    stream_scope_id     TEXT    NOT NULL,             -- UUIDv7 estable para esta base local
    process_instance_id TEXT    NOT NULL,             -- UUIDv7 único por proceso Core
    status              TEXT    NOT NULL
        CHECK (status IN ('STARTING', 'READY', 'STOPPED')),
    CHECK (created_at >= 0 AND updated_at = created_at)
) STRICT;

CREATE INDEX IF NOT EXISTS idx_desktop_status_events_event_sequence_id
    ON desktop_status_events (event_sequence_id);
CREATE INDEX IF NOT EXISTS idx_desktop_status_events_scope_sequence
    ON desktop_status_events (stream_scope_id, event_sequence_id);

CREATE TRIGGER IF NOT EXISTS trg_desktop_status_events_no_update
BEFORE UPDATE ON desktop_status_events
BEGIN
    SELECT RAISE(ABORT, 'desktop_status_events is append-only: UPDATE is forbidden');
END;

CREATE TRIGGER IF NOT EXISTS trg_desktop_status_events_no_delete
BEFORE DELETE ON desktop_status_events
BEGIN
    SELECT RAISE(ABORT, 'desktop_status_events is append-only: DELETE is forbidden');
END;
