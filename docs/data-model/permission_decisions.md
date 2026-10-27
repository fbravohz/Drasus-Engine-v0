# permission_decisions

**Feature dueña:** [`agentic-mcp-gateway`](../features/agentic-mcp-gateway.md)
**Migración actual:** `migrations/0005_mcp_gateway.sql`; TTR-008 a TTR-012 exigen una evolución aditiva antes de implementar los campos de operación y delegación descritos aquí.
**Naturaleza:** Append-only (`event_sequence_id UNIQUE` + triggers anti UPDATE/DELETE)
**Perfil ADR-0020:** Ops/Auditoría — Grupo I + Grupo II (subset) + Grupo IV

## Columnas

| Columna | Tipo SQL | Null | Descripción |
|---|---|---|---|
| `id` | TEXT | NO | PK, UUID v4 |
| `created_at` | INTEGER | NO | Nanosegundos desde epoch |
| `updated_at` | INTEGER | NO | == `created_at` (append-only) |
| `audit_hash` | TEXT | NO | SHA-256 de campos de dominio |
| `audit_chain_hash` | TEXT | SÍ | NULL solo en fila génesis |
| `event_sequence_id` | INTEGER | NO | Posición monótona, `UNIQUE` |
| `owner_id` | TEXT | SÍ | Propietario de la cuenta/alcance autorizado (nullable en local) |
| `institutional_tag` | TEXT | SÍ | "Live"/"Demo" (solo para Manage) |
| `node_id` | TEXT | NO | Host donde corre el Gateway MCP |
| `process_id` | INTEGER | NO | PID del proceso |
| `agent_session_id` | TEXT | NO | Sesión MCP del agente |
| `agent_identity_id` | TEXT | NO | Identidad estable del agente; no se infiere de la sesión efímera |
| `operation_id` | TEXT | SÍ | Operación observable que originó la evaluación; nulo solo para decisiones históricas sin operación |
| `feature_key` | TEXT | NO | Feature dueña del puerto invocado |
| `public_port` | TEXT | NO | Puerto público de la Feature que se quiere usar |
| `target_entity_kind` | TEXT | SÍ | Tipo del objeto alcanzado por la acción |
| `target_entity_id` | TEXT | SÍ | Objeto concreto alcanzado, cuando aplique |
| `pipeline_context` | TEXT | SÍ | Contexto de riesgo/orquestación; no concede capacidades por sí solo |
| `idempotency_key` | TEXT | SÍ | Clave del intento original, para reconstruir reintentos sin duplicar el efecto |
| `request_fingerprint` | TEXT | SÍ | Huella canónica que evita reutilizar una clave de idempotencia para otra solicitud |
| `correlation_id` | TEXT | SÍ | Agrupa la operación y sus efectos relacionados |
| `causation_id` | TEXT | SÍ | Identifica el evento/comando que causó esta evaluación |
| `expected_row_version` | INTEGER | SÍ | Versión esperada del objeto mutable; permite explicar un conflicto |
| `delegation_id` | TEXT | SÍ | Delegación de producción acotada que aplicó, si existió |
| `permission_outcome` | TEXT | NO | `"granted"` \| `"denied:<razón>"` |

## Claves

- **PK:** `id`
- **FK salientes:** `owner_id → accounts(id) ON DELETE RESTRICT` (nullable); `operation_id → agent_operations(id) ON DELETE RESTRICT` (nullable); `delegation_id → agent_delegations(id) ON DELETE RESTRICT` (nullable).
- **Índices:** `(agent_session_id, created_at)`, `(agent_identity_id, created_at)`, `operation_id`, `owner_id`, `event_sequence_id`.
- **Triggers:** `trg_permission_decisions_no_update`, `trg_permission_decisions_no_delete`

## Quién la referencia (FK entrante)

- Ninguna.

## Notas de diseño

- Esta tabla es el rastro append-only de una decisión, no el estado actual de una operación ni la fuente de autoridad.
- La capacidad se evalúa sobre `feature_key` + `public_port` + objeto y rol; `pipeline_context` únicamente aporta riesgo/contexto. Las delegaciones vigentes y revocables viven en [`agent_delegations`](./agent_delegations.md), nunca en un interruptor global.
- La fila de decisión, la mutación autorizada, el evento de actividad durable y el Audit Log se comprometen en la misma transacción cuando la acción cambia estado (ADR-0141, ADR-0166).
