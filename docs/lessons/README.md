# Lecciones — Aprendizaje Acumulado por Story/Task

Esta carpeta registra lo que el usuario aprende mientras construye el proyecto, anclado a código real. Cualquier agente puede consultar las lecciones; solo en Mentor o Docente registra lo enseñado, conforme a [AGENTS.md](../../AGENTS.md), [ADR-0120](../adr/ADR-0120.md) y [ADR-0122](../adr/ADR-0122.md). [ADR-0124](../adr/ADR-0124.md) fija un archivo por Story/Task, nombrado con el ID de su Orden en `docs/execution/`.

## Reglas

El modo de acompañamiento se declara en la Orden según AGENTS y ADR-0120/ADR-0122; la organización de lecciones sigue ADR-0124.

- Un archivo por Story/Task, nombrado igual que su Orden (`STORY-007-telemetry.md`), nunca por tema de lenguaje suelto.
- Cada concepto explicado dentro del archivo cita el código real de esa Story (ruta + fragmento), no un ejemplo de manual.
- El archivo enlaza a su Orden en `docs/execution/<ID>.md` al inicio — trazabilidad en ambos sentidos.
- Si la misma Story se retoma en una sesión posterior, no se crea un segundo archivo: se añade al ya existente, debajo de lo escrito.
- Cada archivo tiene como mínimo `## Concepto` (una subsección por cada concepto enseñado, cero-conocimiento, anclada a código real) y `## Trucos de Senior` (atajos/azúcar sintáctica reales de esa Story — solo si hay algo real que destacar).

## Carpetas por dominio

| Carpeta | Ingeniero(s) que escriben aquí | Dominio |
|---|---|---|
| [`rust/`](./rust/) | `rust-engineer`, `refactoring-engineer` | Lenguaje Rust, Cargo, build/release |
| `ffi-grpc/` | `rust-engineer` | Integración de contratos Protobuf/gRPC, adaptadores y streams |
| [`quant/`](./quant/) | `quant-engineer` | Estadística, matemática financiera, sesgos |
| [`testing/`](./testing/) | `qa-engineer` | Patrones de prueba (unitaria, propiedad, oráculo, SLA) |
