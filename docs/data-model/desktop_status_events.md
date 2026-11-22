# desktop_status_events

**Feature dueña:** [`desktop-packaging-manager`](../features/desktop-packaging-manager.md)
**Migración:** `migrations/0024_desktop_status_events.sql`
**Naturaleza:** Append-only atómica (`event_sequence_id UNIQUE` + triggers anti UPDATE/DELETE)
**Perfil ADR-0020:** Grupo I + identidad del alcance local y proceso Core; no contiene datos de usuario ni de operaciones.

## Columnas

| Columna | Tipo SQL | Null | Descripción |
|---|---|---|---|
| `id` | TEXT | NO | PK, UUIDv7 |
| `created_at` | INTEGER | NO | Instante de persistencia en nanosegundos UTC |
| `updated_at` | INTEGER | NO | Igual a `created_at`; el ledger no se actualiza |
| `audit_hash` | TEXT | NO | SHA-256 del evento canónico, 64 caracteres hexadecimales |
| `audit_chain_hash` | TEXT | SÍ | `audit_hash` anterior; `NULL` en la fila génesis |
| `event_sequence_id` | INTEGER | NO | Posición monótona global del historial, `UNIQUE` |
| `stream_scope_id` | TEXT | NO | UUIDv7 estable asociado a esta base local |
| `process_instance_id` | TEXT | NO | UUIDv7 de la ejecución del Core que produjo el evento |
| `status` | TEXT | NO | `STARTING`, `READY` o `STOPPED`; describe una transición histórica |

## Claves

- **PK:** `id`
- **Únicas:** `event_sequence_id` — secuencia global sin duplicados.
- **Índices:** `event_sequence_id`, `(stream_scope_id, event_sequence_id)`
- **Triggers:** `trg_desktop_status_events_no_update`, `trg_desktop_status_events_no_delete`
- **FK salientes:** ninguna.
- **CHECK:** hashes de 64 caracteres; secuencia positiva; timestamps no negativos e iguales; estado restringido a `STARTING`, `READY` o `STOPPED`.
- **Tabla:** `STRICT`.

## Quién la referencia (FK entrante)

- Ninguna.

## Notas de diseño

- El dueño `desktop-packaging-manager` inicia cada proceso con `STARTING`, registra `READY` después del bind local y agrega `STOPPED` al cierre ordenado. Tras un cierre abrupto, el último evento puede seguir siendo `READY`; no es el estado vivo. El unary `CheckReadiness` responde desde el proceso actual.
- `stream_scope_id` se crea al insertar el primer evento y se conserva entre reinicios de la misma base. `event_sequence_id` avanza dentro de una transacción `BEGIN IMMEDIATE` y el hash encadena cada evento con el anterior.
- La retención de replay es lógica: el stream reanuda desde una ventana de 64 eventos y devuelve `OutOfRange` si el cursor quedó fuera. La tabla conserva el historial físico y sus triggers prohíben borrarlo.
- El contenido se limita al alcance, UUID del proceso, estado y metadatos de integridad. No almacena sesión gRPC, credenciales, endpoint, datos de trading ni datos de usuario.
