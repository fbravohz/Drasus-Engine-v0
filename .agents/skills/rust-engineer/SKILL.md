---
name: rust-engineer
description: Implementa y revisa dominio, persistencia, backend Rust y transporte gRPC entre el Core y clientes Tauri/React o headless. Úsala para Órdenes en esas capas.
---

# Rust Engineer

Implementa y revisa dominio, persistencia, backend y transporte gRPC dentro del alcance de la Orden, bajo despacho del Tech Lead.

Usa la skill compañera [Rust Best Practices](../rust-best-practices/SKILL.md) para criterios idiomáticos de implementación y revisión. El canon y el flujo de Drasus siguen gobernados por esta skill, la Orden, `AGENTS.md` y los ADR aplicables.

## Flujo de trabajo

1. Usa [`docs/README.md`](../../../docs/README.md) como índice y lee la Orden completa. Obtén de ella el alcance, el modo, los criterios, los entregables y los comandos de validación.
2. Antes de cambiar código, lee el TTR del módulo, la Feature, los ADR citados y el código o la migración afectados. Si falta contrato, perfil de persistencia o propiedad de datos, escala el bloqueo al Tech Lead.
3. Implementa dentro de los puertos y límites de la Feature Rust conforme a [ADR-0137](../../../docs/adr/ADR-0137.md). Estabiliza y verifica `public_interface` antes de adaptar el transporte.

## Transporte gRPC

Cuando la Orden incluya transporte:

- No implementes servidor, cliente ni bindings hasta que cierre SPIKE-006, como exige [ADR-0142](../../../docs/adr/ADR-0142.md). Si sigue abierto, informa el bloqueo al Tech Lead y continúa solo con trabajo de Core independiente incluido en la Orden. Sigue las decisiones del Spike sobre generación, ubicación de Protobuf y bindings, y mapeo de errores.
- Usa Protobuf como contrato de transporte y mapea explícitamente sus mensajes a `public_interface` conforme a [ADR-0165](../../../docs/adr/ADR-0165.md). Mantén compatibilidad aditiva; no dupliques DTOs ni lógica de dominio.
- Para el primer slice, implementa una llamada unary y el stream requerido por ADR-0166. Valida autenticación, errores, cancelación y recuperación que apliquen; prueba llamadas y streams contra el Core real.
- Entrega bindings compilables, comandos, evidencia y riesgos en la Orden antes de que Frontend consuma el contrato.

## Modos de Acompañamiento

Usa el modo asignado a Rust Engineer en la sección 3 de la Orden; si no declara uno, trabaja en Autónomo. Sigue el comportamiento y registra el plan o checklist en la sección 4 según [AGENTS.md, «Modos de acompañamiento y lecciones»](../../../AGENTS.md) y la [plantilla de Orden](../../../docs/execution/_TEMPLATE.md). Registra una lección por Story/Task y dominio: `docs/lessons/rust/` para el Core y `docs/lessons/ffi-grpc/` para el transporte, conforme a [ADR-0124](../../../docs/adr/ADR-0124.md).

## Determinismo y rendimiento

- Si serializas o calculas hashes sobre datos de dominio, fija un orden canónico; no alimentes un hash reproducible con un `HashMap` sin ordenar. Consulta [ADR-0002](../../../docs/adr/ADR-0002.md).
- Si cambias una ruta sensible a latencia, aplica los límites vigentes de [ADR-0047](../../../docs/adr/ADR-0047.md) y la sección «KPIs por Fase» de [docs/ROADMAP.md](../../../docs/ROADMAP.md).

## 4. Atomicidad de ledgers append-only

Al crear o modificar un ledger, lee la secuencia y el hash previos e inserta dentro de una sola transacción `BEGIN IMMEDIATE`; limita los reintentos a contención transitoria. Aplica el esquema, la inmutabilidad, la documentación y las pruebas de [`AGENTS.md`, «Ledgers append-only»](../../../AGENTS.md#ledgers-append-only) y [ADR-0141](../../../docs/adr/ADR-0141.md). No tomes una implementación existente como plantilla completa.

## Pruebas y entrega

- Relaciona cada criterio de la Orden con una prueba nombrada y entrega la evidencia y los comandos de su sección 6. Aplica los disparadores de prueba pertinentes de [ADR-0133](../../../docs/adr/ADR-0133.md) y las reglas discriminantes de [`AGENTS.md`](../../../AGENTS.md#simplicidad-y-pruebas-discriminantes).
- Si la Feature expone `verify()`, usa el comando real registrado en la Orden o [`docs/TEST.md`](../../../docs/TEST.md), conforme a [ADR-0142](../../../docs/adr/ADR-0142.md); el CLI complementa las pruebas, no las sustituye.
- Si el cambio activa el Filtro de mutación en `domain/` o `persistence`, prepara las pruebas y evidencia; el Tech Lead ejecuta `scripts/mutation-gate.sh` conforme a [`AGENTS.md`, «Filtro de mutación»](../../../AGENTS.md).
