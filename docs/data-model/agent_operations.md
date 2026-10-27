# agent_operations y agent_operation_events

**Feature dueña:** [`agentic-mcp-gateway`](../features/agentic-mcp-gateway.md)
**Naturaleza:** operación mutable + historial append-only de actividad.
**Perfil ADR-0020:** Operación transversal; aplica identidad, soberanía, infraestructura y el rastro causal que corresponda al objeto afectado.

## Propósito

Una llamada mutante de un agente no se representa como una serie de mensajes de interfaz. Se materializa primero como una operación durable identificable. Así una superficie Tauri/React, otro cliente autorizado o un relé remoto pueden mostrar el mismo trabajo real, recuperarse de una desconexión y distinguir un reintento de un segundo efecto.

## `agent_operations` — estado actual recuperable

| Columna | Tipo SQL | Null | Descripción |
|---|---|---|---|
| `id` | TEXT | NO | PK, `operation_id` estable |
| `owner_id` | TEXT | SÍ | Dueño del recurso/operación cuando existe cuenta |
| `agent_identity_id` | TEXT | NO | Identidad estable del agente autorizado |
| `agent_session_id` | TEXT | SÍ | Sesión efímera que presentó el comando |
| `feature_key` | TEXT | NO | Feature cuyo puerto público se invoca |
| `public_port` | TEXT | NO | Puerto público autorizado |
| `target_entity_kind` | TEXT | SÍ | Tipo del objeto afectado |
| `target_entity_id` | TEXT | SÍ | Identificador del objeto afectado |
| `status_code` | TEXT | NO | Código estable de estado, sin texto de interfaz |
| `idempotency_key` | TEXT | NO | Clave del comando original dentro del alcance autorizado |
| `request_fingerprint` | TEXT | NO | Huella canónica de Feature, puerto, objeto y parámetros permitidos; detecta reutilización de clave con otra solicitud |
| `expected_row_version` | INTEGER | SÍ | Versión presentada por el solicitante para mutaciones concurrentes |
| `observed_row_version` | INTEGER | SÍ | Versión observada al aceptar o rechazar la mutación |
| `delegation_id` | TEXT | SÍ | Delegación vigente aplicada al aceptar el comando |
| `correlation_id` | TEXT | SÍ | Grupo causal de la operación |
| `causation_id` | TEXT | SÍ | Comando/evento que la originó |
| `job_id` | TEXT | SÍ | Trabajo asíncrono asociado, cuando la ejecución continúa en segundo plano |
| `row_version` | INTEGER | NO | Concurrencia optimista del registro mutable |
| `created_at` | INTEGER | NO | Inicio de la operación |
| `updated_at` | INTEGER | NO | Última transición durable |
| `completed_at` | INTEGER | SÍ | Fin terminal, si ocurrió |

**Claves e invariantes:** PK `id`; unicidad de `(owner_id, agent_identity_id, idempotency_key)` dentro del alcance de autorización; FK hacia cuenta, delegación y job cuando existan. Un reintento con la misma clave y `request_fingerprint` devuelve la misma operación y no repite el efecto. Reusar la clave con una huella distinta se rechaza: no puede recuperar ni ocultar el resultado de otra solicitud. Una mutación que dependa de versión usa `expected_row_version`; una diferencia devuelve conflicto explícito, no sobrescribe.

## `agent_operation_events` — actividad durable ordenada

| Columna | Tipo SQL | Null | Descripción |
|---|---|---|---|
| `id` | TEXT | NO | PK del evento |
| `operation_id` | TEXT | NO | FK a `agent_operations(id)` |
| `event_sequence_id` | INTEGER | NO | Posición monótona y única dentro de `stream_scope_id` para reanudar streams |
| `stream_scope_id` | TEXT | NO | Alcance del cursor: dueño, nodo y réplica autorizada; no se compara entre alcances distintos |
| `status_code` | TEXT | NO | Cambio de estado o hecho observable, estable y localizable |
| `object_snapshot_ref` | TEXT | SÍ | Referencia segura a datos que la interfaz puede recuperar por su puerto autorizado |
| `created_at` | INTEGER | NO | Momento de la transición |
| `correlation_id` | TEXT | SÍ | Correlación de la cadena de trabajo |
| `causation_id` | TEXT | SÍ | Causa inmediata, cuando se conoce |

Los eventos son append-only. La clave de reanudación es `(stream_scope_id, event_sequence_id)`, no un orden total entre nodos o dueños distintos. Ninguna fila contiene secretos, parámetros sensibles, cadenas controladas por el agente ni texto final de interfaz. La interfaz localiza `status_code` y obtiene el objeto por la frontera pública autorizada.

## Compromiso y difusión

La transición de estado de la Feature, `agent_operations`, su evento append-only, `permission_decisions` y `audit_events` se escriben en una sola transacción cuando pertenecen al mismo efecto. Solo después del commit se publica el evento por gRPC. El stream entrega snapshot inicial y posición de reanudación dentro de `stream_scope_id`; el cliente deduplica por `event_sequence_id` y resuelve huecos con recuperación, no suponiendo entrega perfecta. Antes de emitir o incluir una operación en un snapshot, el servidor filtra por el alcance de lectura vigente del observador sobre la operación y su objeto; una revocación corta emisiones posteriores. Un estado terminal no se reabre por eventos tardíos o duplicados.

La presencia efímera (por ejemplo, que un agente está preparando una solicitud) no se guarda aquí: usa un canal con TTL, no sustituye a la actividad durable ni al Audit Log.

**Trazabilidad:** ADR-0123, ADR-0141, ADR-0165, ADR-0166; TTR-008 a TTR-012 de `agentic-mcp-gateway`.
