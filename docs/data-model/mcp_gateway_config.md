# mcp_gateway_config

**Feature dueña:** [`agentic-mcp-gateway`](../features/agentic-mcp-gateway.md)
**Migración actual:** `migrations/0005_mcp_gateway.sql`; cualquier cambio para TTR-008 a TTR-012 será aditivo y no convierte esta tabla en fuente de autoridad.
**Naturaleza:** Configuración clave-valor mutable de alcance técnico; no es autoridad de permisos.
**Perfil ADR-0020:** No aplica el contrato de 25 campos — es una tabla de configuración pura.

## Columnas

| Columna | Tipo SQL | Null | Descripción |
|---|---|---|---|
| `key` | TEXT | NO | PK |
| `value` | TEXT | NO | Valor de la clave |

## Claves

- **PK:** `key`
- **FK salientes:** ninguna

## Quién la referencia (FK entrante)

- Ninguna.

## Notas de diseño

- Esta tabla contiene defaults técnicos del Gateway, por ejemplo límites de presencia, retención o ventana de reintento. No contiene un permiso global, ni un interruptor de producción, ni una excepción por cuenta.
- La autorización de producción es una delegación explícita, con dueño, agente, puerto de Feature, objeto/alcance y vencimiento. Su estado vigente vive en [`agent_delegations`](./agent_delegations.md); cada evaluación queda en [`permission_decisions`](./permission_decisions.md).
- Una superficie puede representar el estado agregado de delegaciones como un switch para facilitar comprensión humana, pero ese control debe crear, limitar, vencer o revocar delegaciones concretas. Nunca concede acceso por sí mismo (ADR-0123, ADR-0166).
