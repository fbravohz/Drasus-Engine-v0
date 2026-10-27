# agent_delegations

**Feature dueña:** [`agentic-mcp-gateway`](../features/agentic-mcp-gateway.md)
**Naturaleza:** autorización mutable, acotada y revocable; sus cambios se auditan append-only.

## Propósito

Una acción de producción de un agente exige una delegación concreta. No existe un permiso global que una conexión MCP pueda heredar por estar activa, por pertenecer a un pipeline o por una opción clave-valor.

## Columnas

| Columna | Tipo SQL | Null | Descripción |
|---|---|---|---|
| `id` | TEXT | NO | PK, UUID de la delegación |
| `owner_id` | TEXT | NO | Cuenta dueña que la concede |
| `agent_identity_id` | TEXT | NO | Agente al que se limita |
| `feature_key` | TEXT | NO | Feature permitida |
| `public_port` | TEXT | NO | Puerto público permitido |
| `target_scope_kind` | TEXT | SÍ | Tipo de alcance del objeto: concreto, colección u otro alcance documentado |
| `target_scope_id` | TEXT | SÍ | Objeto o alcance concreto; nulo solo cuando el puerto no opera sobre entidad |
| `institutional_tag` | TEXT | SÍ | Entorno al que se limita, cuando aplique |
| `granted_by_operator_id` | TEXT | NO | Operador humano que otorgó la delegación |
| `expires_at` | INTEGER | NO | Vencimiento obligatorio |
| `revoked_at` | INTEGER | SÍ | Momento de revocación, si ocurrió |
| `revoked_by_operator_id` | TEXT | SÍ | Operador que revocó |
| `row_version` | INTEGER | NO | Concurrencia optimista para modificar/revocar |
| `created_at` | INTEGER | NO | Creación |
| `updated_at` | INTEGER | NO | Último cambio |

## Invariantes

- La delegación solo existe si el operador que la emite posee la capacidad del mismo puerto de Feature y objeto.
- Debe estar vigente tanto al aceptar la solicitud como justo antes del commit del efecto. La revocación detiene trabajos que aún no cruzan una frontera segura de cancelación; los ya comprometidos se conservan como actividad y auditoría, no se reescriben.
- El rol del agente, la jerarquía de cuenta, el alcance del objeto y la política de riesgo siguen evaluándose: la delegación no sustituye ninguna de esas capas.
- Cada creación, modificación, vencimiento efectivo o revocación produce decisión y auditoría; las superficies muestran el resultado con claves localizadas.

**Trazabilidad:** ADR-0123, ADR-0149, ADR-0166; [`permission_decisions`](./permission_decisions.md), [`agent_operations`](./agent_operations.md).
