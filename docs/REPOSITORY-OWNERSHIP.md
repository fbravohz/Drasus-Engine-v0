# Propiedad de Repositorios y Frontera de Integración

Esta matriz define qué repositorio es dueño de cada familia de archivos y cómo se integra el contexto de agentes con `Drasus-Engine`. La separación física se ejecuta por fases; mientras tanto, las rutas siguen en el checkout actual.

## Regla de propiedad

`Drasus-Engine` es dueño del producto. `Drasus-Context` es dueño del sistema que entrega y valida contexto a los agentes. La propiedad de una fuente no cambia porque el archivo se proyecte como adaptador en el otro repositorio.

## Matriz objetivo

| Familia | Dueño canónico | Tratamiento en `Drasus-Engine` | Regla de edición |
|---|---|---|---|
| `crates/`, `Cargo.toml`, `Cargo.lock` y `flutter_rust_bridge.yaml` | `Drasus-Engine` | Fuente directa | Se edita en el repositorio de código. |
| `migrations/`, `scripts/`, `ui-old/` y configuración del producto | `Drasus-Engine` | Fuente directa | Se valida con las pruebas del producto. |
| `docs/adr/`, `docs/features/`, `docs/modules/`, `docs/sad/`, `docs/execution/`, `docs/data-model/`, `docs/legal/` y `docs/ROADMAP.md` | `Drasus-Engine` | Fuente directa | Contienen decisiones y órdenes del producto. |
| `docs/REPOSITORY-OWNERSHIP.md` y ADR-0164 | `Drasus-Engine` durante la separación | Fuente directa hasta completar el corte | Fijan la frontera entre ambos repositorios. |
| `.agents/knowledge/` | `Drasus-Context` | Submódulo fijado en `.agents` | No se edita desde el repositorio de código. |
| `.agents/skills/` y `.agents/templates/` | `Drasus-Context` | Submódulo fijado en `.agents` | Las skills y plantillas se versionan separadas del producto. |
| `.agents/hooks/` y `.agents/context-runtime/` | `Drasus-Context` | Submódulo fijado en `.agents` | Incluye fuentes, manifiesto, pruebas y runtime; se excluyen entornos locales, cachés y bytecode. |
| `.agents/debt/`, `.agents/CONTEXT-DEBT.md`, `.agents/plans/`, `.agents/sessions/`, `.agents/state/`, `.agents/memory/` y `.agents/documents/` | `Drasus-Context` | Se trasladan con la extracción inicial y se depuran por tipo | Son artefactos operativos del sistema de agentes, no fuente del producto. |
| `.claude/skills/`, `.claude/rules/` y `.claude/settings.json` | `Drasus-Context` como fuente; `Drasus-Engine` como integración | Proyección generada o adaptador mínimo | No se mantienen copias manuales de las skills ni de los hooks. |
| `AGENTS.md` y `CLAUDE.md` | `Drasus-Engine` para el mapa del proyecto; `Drasus-Context` para el contenido compartido | Adaptadores del producto que apuntan al contexto fijado | El mapa del producto queda aquí; las reglas generales se generan desde el repositorio de contexto. |
| `.claude/settings.local.json`, `.agents/state/.session-reads/`, `.agents/hooks/.fts-index.db`, entornos virtuales y cachés | Ningún repositorio | Solo estado local e ignorado | Nunca se versionan ni se usan como fuente de verdad. |
| `.codex/` y demás proyecciones de plataforma | `Drasus-Context` como fuente | Proyección generada según la plataforma | La capacidad MCP y el procedimiento de la skill permanecen separados. |

## Frontera de cambios

- Cambios de producto: solo `Drasus-Engine`.
- Cambios de contexto, skills, hooks, catálogo o runtime: solo `Drasus-Context`.
- Cambios coordinados: primero se valida y etiqueta `Drasus-Context`; después `Drasus-Engine` actualiza la revisión fijada y ejecuta la compatibilidad.
- Cambios de adaptadores: se generan desde `Drasus-Context`; una excepción manual requiere documentar por qué la plataforma no permite generación directa.

## Secuencia de migración

1. Congelar el estado actual de `Drasus-Engine` sin reescribir su historial.
2. Extraer a `Drasus-Context` las rutas de agentes y conservar el historial relevante en un repositorio separado.
3. Sustituir `.agents` en `Drasus-Engine` por el submódulo fijado.
4. Reducir `AGENTS.md` y `CLAUDE.md` a sus mapas y adaptadores específicos del producto.
5. Generar y validar las proyecciones de Claude y Codex desde el repositorio de contexto.
6. Añadir una comprobación de compatibilidad en ambos repositorios.

## Resultado esperado

Una revisión de código del producto no arrastra cambios de contexto. Una revisión de contexto puede probar el resolver y las skills sin mezclar código de dominio. El motor sigue recibiendo una versión explícita y reproducible del contexto que necesita.
