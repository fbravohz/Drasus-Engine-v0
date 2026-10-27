# Bitácora compacta — Tech-Lead

> Resumen acumulado por sesión: qué avanzó y qué sigue. El detalle, evidencia y comandos viven en las Órdenes de `docs/execution/`; la fase, en [`ROADMAP.md`](../../../docs/ROADMAP.md); las deudas, en [`DEBT.md`](../../../docs/DEBT.md).

## Estado de reanudación

Última sesión resumida: 2026-08-02. Contrasta el estado operativo con las Órdenes y el ROADMAP al reanudar.

- **Fase:** EPIC-1 — Soberanía de Datos (`ingest`), en curso.
- **Orden activa:** [STORY-073 — Asistente de Importación](../../../docs/execution/STORY-073-asistente-importacion-pantalla.md), construida y verificada; registra 13/14 criterios.
- **Para cerrar STORY-073:** el propietario debe completar el criterio 13 importando el archivo real desde la aplicación; el recorrido está en §6.1. Faltan las tres lecciones del Modo Docente (§8).
- **Después:** conectar la salida de barras al almacenamiento Parquet particionado Hive-Style; revisar [`DEBT-036`](../../../docs/debt/DEBT-036.md) y [`DEBT-040`](../../../docs/debt/DEBT-040.md).
- **Validación manual (2026-08-01):** corregidos el selector de archivos en Linux, el parpadeo de la zona de carga y la copia de errores. STORY-073 registra `flutter test` 30/30, `flutter analyze` sin errores y `flutter build linux --debug` correcto.
- STORY-073 pagó DEBT-035, DEBT-037 y DEBT-041. DEBT-045 se pagó el 2026-08-02.

## Pendientes acumulados

- **Importación:** DEBT-036 y DEBT-040 se disparan con la persistencia real. DEBT-039 está parcial: la interfaz muestra fechas reales junto al rango; el análisis profundo de homogeneidad sigue pendiente. DEBT-043 (mensajes con jerga) y DEBT-044 (arrastre de archivos) siguen abiertas.
- **Modelo de ingesta:** falta implementar en `pit-data-validator` los gates de coherencia de calendario y homogeneidad de ADR-0162. El [diseño de TASK-064](../../../docs/execution/TASK-064-politica-spread-fill-model-dst.md) está cerrado; su contraste empírico del spread depende de datos de ticks. El catálogo de [TASK-065](../../../docs/execution/TASK-065-catalogo-broker-instrumento-sesion.md) se entregó en STORY-066.
- **Datos reales:** [TASK-060](../../../docs/execution/TASK-060-muestra-real-forex-cfd.md) sigue bloqueada por el propietario para completar la muestra de 1 minuto y ticks, idealmente de dos brókers; el CSV M1 existente permitió el primer build. Sigue abierta [DEBT-025](../../../docs/debt/DEBT-025.md).
- **Cola de trabajo:** [TASK-052](../../../docs/execution/TASK-052-architect-escalation-market-data-sources.md) sigue abierta (prioridad de conectores y tabla de capacidades); [TASK-053](../../../docs/execution/TASK-053-jurisdiccion-constitucion-empresa.md), [TASK-054](../../../docs/execution/TASK-054-nivel-gratuito-cooperativa-minado.md), [TASK-055](../../../docs/execution/TASK-055-cumplimiento-por-arquitectura.md) y [TASK-056](../../../docs/execution/TASK-056-clausulado-especifico-y-comparado.md) siguen abiertas (jurisdicción, modelo del nivel gratuito, cumplimiento por arquitectura y clausulado); [TASK-059](../../../docs/execution/TASK-059-contrato-momento-de-conocibilidad.md) está pendiente y bloquea `signal-topology`; [TASK-061](../../../docs/execution/TASK-061-techo-magnitud-escala-entera.md) y [TASK-062](../../../docs/execution/TASK-062-catalogo-tipos-de-barra.md) siguen pendientes (rango de escala entera y tipos de barras; `DOLLAR` depende de TASK-061 y [DEBT-033](../../../docs/debt/DEBT-033.md) sigue abierta).
- **Decisiones de ingesta:** [TASK-063](../../../docs/execution/TASK-063-formato-estandar-de-ingesta.md) conserva la política de formato y mapeo; sus decisiones principales están en ADR-0156 y ADR-0114. Revisar la Orden antes de darla por cerrada.
- **Riesgo de registro:** la Orden STORY-073 atribuye nueve supervivientes previos de `bridge` a DEBT-042, pero no existe esa ficha ni fila en `DEBT.md`; reconciliarlo antes del cierre.
- **Estados desfasados:** [TASK-069](../../../docs/execution/TASK-069-architect-escalation-reloj-de-broker.md) aparece pendiente de despacho en su Orden, aunque ADR-0162 y STORY-070 documentan su resolución. TASK-064 y TASK-065 siguen `Abierta` en sus Órdenes aunque el ROADMAP da por cerrado el diseño de TASK-064 y STORY-066 entregó el catálogo de TASK-065. ROADMAP aún marca STORY-051 en curso, aunque su Orden la cierra el 2026-07-18. Alinear esas fuentes antes de reabrir o despachar.
- Otras deudas abiertas permanecen en [`DEBT.md`](../../../docs/DEBT.md). [DEBT-038](../../../docs/debt/DEBT-038.md) (siete timestamps sin sufijo de unidad) debe pagarse antes del primer release distribuido.

## Resumen de sesiones

- **2026-06-12:** arrancó EPIC-0. Se levantaron el workspace Rust y las primeras bases de clock, audit-log y async-job-executor; se formalizaron Órdenes de Trabajo y pruebas discriminantes.
- **2026-06-16–21:** avanzaron telemetry, worker isolation, CLI y MCP; se fijaron modos de acompañamiento, QA independiente y portabilidad entre sistemas operativos. STORY-007–015 quedaron entregadas o auditadas.
- **2026-06-23–24:** la revisión de EPIC-0 alineó el código con ADR-0137: features hexagonales, infraestructura transversal en `shared` y puertos documentados. EPIC-0 quedó coherente para su cierre.
- **2026-06-25–2026-07-03:** se formalizó y estandarizó la biblioteca de componentes UI, el tema y la galería (STORY-016–026); se corrigieron déficits de registro y diseño.
- **2026-06-27–2026-07-12:** EPIC-0 cerró y arrancó EPIC-1 con [STORY-024](../../../docs/execution/STORY-024-sovereign-data-fetcher.md). Se completaron los 14 cimientos de monetización, la auditoría retroactiva y el Banco de Verificación ([STORY-050](../../../docs/execution/STORY-050-verification-bench.md)).
- **2026-07-18–19:** [STORY-051](../../../docs/execution/STORY-051-hybrid-data-transformer.md) cerró el backend del transformador. Se fijó que Drasus aporta conectores y el usuario sus datos; se reforzó conservar hechos crudos y calcular interpretaciones. El análisis detectó límites de escala y vacíos de procedencia; se abrieron tareas y deudas. Se fijaron Doc-First y el orden Quant→Architect.
- **2026-07-22–24:** con datasets reales se definieron la propiedad de los datos, los campos canónicos, spread y reloj de origen. El Architect selló ADR-0162/0163 y las features de catálogo e ingesta.
- **2026-07-25–27:** [STORY-066](../../../docs/execution/STORY-066-instrument-session-catalog.md) entregó los catálogos; [STORY-068](../../../docs/execution/STORY-068-mt5-bar-reader.md), el lector MT5; [STORY-070](../../../docs/execution/STORY-070-declaracion-por-tramo-retrofit.md), la declaración por tramo y el reloj de bróker. El análisis halló ticks unilaterales y procedencia heterogénea; [TASK-067](../../../docs/execution/TASK-067-identificacion-archivos-dat.md) descartó importar `.dat` de StrategyQuant X. Se resolvieron TASK-069, TASK-071 y TASK-072. STORY-070 cerró con QA y gate de mutación aptos; quedaron DEBT-038 y DEBT-039.
- **2026-07-31–2026-08-02:** [STORY-073](../../../docs/execution/STORY-073-asistente-importacion-pantalla.md) entregó la interfaz y el mapeo manual. La prueba del propietario encontró tres defectos de frontera; se corrigieron y verificaron. Sigue pendiente importar el archivo real y cerrar las lecciones Docente.

## Cómo extender esta bitácora

Al cerrar cada sesión, agrega una entrada breve con fecha, logros, verificación resumida, pendientes y enlaces. Mantén el detalle técnico y los comandos en la Orden. Para reanudar, lee esta bitácora, la fase activa del ROADMAP y la Orden correspondiente; aplica [`tech-lead/SKILL.md`](../../skills/tech-lead/SKILL.md).
