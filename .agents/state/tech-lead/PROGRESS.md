# Bitácora Operativa — Tech-Lead (Drasus Engine)

> Memoria operativa entre sesiones. Registra avances, decisiones, bloqueos y siguiente paso.
> El detalle técnico vive en la Orden de Trabajo, el ADR, la ficha de feature o la deuda enlazada.
> El estado de fase vive en [`docs/ROADMAP.md`](../../../docs/ROADMAP.md).

**Último estado verificado:** 2026-07-27.

## Estado vigente

### Frente activo: EPIC-1 — Soberanía de Datos (`ingest`)

El diseño de ingesta está aprobado. Secuencia vigente:

1. Catálogos `instrument-catalog` y `session-catalog`.
2. Lector de barras CSV de MetaTrader.
3. Sanitización, incluyendo spread y procedencia.
4. PIT y coherencia de calendario.
5. Particionado Hive-Style/DuckDB.

Invariante rector: **guardar el hecho y calcular la interpretación**. La ingesta persiste primitivos. El `ask` de MT5 se reconstruye al ejecutar el fill (`bid + spread elegido`); ningún stress test modifica el dato guardado. Gobiernan [ADR-0162](../../../docs/adr/ADR-0162.md), [ADR-0163](../../../docs/adr/ADR-0163.md), ADR-0114 y ADR-0155.

### Entregas cerradas

- **STORY-066 — Catálogos de instrumento y sesión.** ✅ Cerrada el 2026-07-25, QA APTO y gate de mutación APTO. Entregó migración `0022`, cuatro tablas, semillas Forex/Cripto/Futuros, fichas de modelo de datos y `verify`. Orden: [`STORY-066`](../../../docs/execution/STORY-066-instrument-session-catalog.md). ADR-0162 sigue 🟡 Parcial porque falta implementar el gate de calendario en `pit-data-validator`.
- **STORY-068 — Lector de barras MT5.** ✅ Cerrada el 2026-07-25, QA APTO y gate de mutación APTO. Entregó lector, plantilla MT5 y ledger append-only de declaraciones. `spread_reported_raw` conserva el valor crudo; su unidad declarada es `POINTS_MT5`. Dejó abiertas [DEBT-035](../../../docs/debt/DEBT-035.md) y [DEBT-036](../../../docs/debt/DEBT-036.md). Orden: [`STORY-068`](../../../docs/execution/STORY-068-mt5-bar-reader.md).
- **TASK-069 — Declaración por tramo y reloj de bróker.** ✅ Resuelta el 2026-07-26 por el Architect. ADR-0162 canoniza zona ancla IANA + desplazamiento fijo en minutos, tabla `broker_server_clock`, declaraciones por tramo y Gate de Homogeneidad de Procedencia. Orden: [`TASK-069`](../../../docs/execution/TASK-069-architect-escalation-reloj-de-broker.md).
- **TASK-071 y TASK-072.** ✅ Resueltas. `broker_server_clock` recibió Grupo I y concurrencia optimista; `venue_operational_since_ns` quedó bajo el check `M13` de ADR-0141. El retrofit puro quedó encadenado a STORY-070.
- **STORY-070 — Retrofit de declaración por tramo.** ✅ Cerrada el 2026-07-27, QA APTO, 13/13 criterios y gate de mutación APTO en ambos crates. Orden: [`STORY-070`](../../../docs/execution/STORY-070-declaracion-por-tramo-retrofit.md).

### Próximo hito: primera ingesta de uso real

El propietario eligió el alcance completo. El hito debe entregar las tres piezas juntas:

1. Cableado Canvas: pulsar el nodo abre su Inspector Panel. Esto también rescata el inspector existente de `hybrid_data_transformer`, hoy huérfano.
2. Asistente visual: archivo, mapeo manual, reloj, unidades, rango, vista previa de dato transformado y confirmación.
3. Almacenamiento real: conectar `bars_out` con persistencia y conservar la referencia de la declaración sellada.

Este hito paga o evalúa juntas DEBT-035, DEBT-036 y DEBT-039. La forma final del formulario ya está fijada por ADR-0162. Después sigue el lector de ticks en `data-normalization-layer` y el Gate de Homogeneidad en `pit-data-validator`.

### Deuda y tareas abiertas relevantes

- **DEBT-039 🔴:** el rango declarado todavía no se coteja con las fechas reales del archivo. Evaluar antes de sellar declaraciones desde la interfaz.
- **DEBT-038 🟠:** siete columnas antiguas incumplen M13. Pagar antes del primer release distribuido.
- **DEBT-035/036/037 🟠:** pantalla y vista previa del mapeo manual; trazabilidad física hacia persistencia; vocabulario obsoleto `Glow*` en `DESIGN.md`.
- **DEBT-003/004 🟡:** STORY-024 aún requiere cerrar los gaps de su SVF y su nodo Canvas/Inspector. Detalle en [DEBT-003](../../../docs/debt/DEBT-003.md) y [DEBT-004](../../../docs/debt/DEBT-004.md).
- **DEBT-025/026/027 y DEBT-029–034:** deuda transversal de datos, licencias, contratos, identidad, ledger, rango numérico y gate de mutación. El índice canónico es [`docs/DEBT.md`](../../../docs/DEBT.md).
- **TASK-052:** replanteada por ADR-0154; ahora debe ordenar conectores y tabla de capacidades, no seleccionar proveedores para Drasus.
- **TASK-053–056:** abiertas y pendientes de despacho; cubren jurisdicción, nivel gratuito, cumplimiento por arquitectura y clausulado sectorial. Sus Órdenes son la fuente de alcance.
- **TASK-067:** ✅ cerrada. Los `.dat` son exportación de StrategyQuant X; no se construirá plantilla. Formatos confirmados: CSV MT5, Binance con credenciales y futuros CQG/NinjaTrader cuando lleguen.
- **DEBT-025:** el CSV MT5 basta para el primer build Forex. Sigue pendiente una segunda fuente real; cTrader queda como alternativa investigada.

No hay permiso para crear commits implícito en esta bitácora. Los commits requieren autorización explícita del propietario en el turno actual.

## Reglas operativas que siguen activas

- Cada trabajo usa una Orden de Trabajo en [`docs/execution/`](../../../docs/execution/). La Orden contiene prompt, criterios, comandos y cierre.
- El Tech-Lead reproduce evidencia antes de cerrar. No cierra una Story con base solo en el reporte del ingeniero.
- QA-Engineer es gate obligatorio. En Flutter, `flutter build <platform>` es condición mínima.
- El gate de mutación lo corre únicamente el Tech-Lead con `scripts/mutation-gate.sh`; exige cero supervivientes. `timeout` y `unviable` cuentan como capturas solo bajo la política vigente de ADR-0133.
- El modo de acompañamiento vive en la Orden. Docente y Autónomo los despacha el Tech-Lead; Mentor y Revisión esperan la invocación del usuario.
- Antes de proponer una regla, fórmula, contrato o cambio de alcance se busca en `docs/` y se cita la fuente. Si falta cobertura, se declara como pregunta abierta.
- Un hueco en algo construido es `DEBT-XXX`; una decisión o insumo pendiente es `TASK-XXX`. Todo bloqueo del propietario se marca en la ficha, la bitácora y el ROADMAP.
- El detalle de un ledger append-only sigue el molde canónico de [`.agents/knowledge/ledger-pattern.md`](../../knowledge/ledger-pattern.md), no la implementación más antigua.

## Mapa de recuperación

| Necesidad | Fuente |
|---|---|
| Orden de fase y siguiente entrega | [`docs/ROADMAP.md`](../../../docs/ROADMAP.md) |
| Decisión arquitectónica | [`docs/ADR.md`](../../../docs/ADR.md) y `docs/adr/` |
| Decisión legal y ruta hacia T&C | [`docs/LEGAL.md`](../../../docs/LEGAL.md) y `docs/legal/ROADMAP-TYC.md` |
| Ejecución, criterios y comandos | `docs/execution/<ID>-<slug>.md` |
| Estado y contrato de una feature | `docs/features/<feature>.md` |
| Deuda conocida | [`docs/DEBT.md`](../../../docs/DEBT.md) y `docs/debt/DEBT-XXX.md` |
| Decisiones de la sesión de ingesta | [`sesion-2026-07-22-23-ingesta-datasets-diseno-catalogo.md`](../../plans/sesion-2026-07-22-23-ingesta-datasets-diseno-catalogo.md) |
| Cierre de STORY-051 y aprendizajes | [`sesion-2026-07-19-cierre-story-051-y-gate-aprendizajes.md`](../../plans/sesion-2026-07-19-cierre-story-051-y-gate-aprendizajes.md) |

## Bitácora histórica resumida

El detalle retirado de aquí no desaparece: permanece en las Órdenes, ADRs, fichas, planes y commits enlazados. Esta lista conserva la secuencia para retomar contexto.

- **2026-06-12:** arrancó EPIC-0. Se construyeron reloj, auditoría, jobs y se fijó el esquema de 25 campos como contrato lógico, no como 25 columnas universales.
- **2026-06-16–18:** ADR-0118 movió `crash-recovery` a EPIC-5; ADR-0120 fijó modos de acompañamiento; `TASK-004` se corrigió a `TASK-006`; STORY-007 quedó cerrada.
- **2026-06-19–21:** se cerraron STORY-008, STORY-009, STORY-010, TASK-011, TASK-012, BUG-013, STORY-014 y STORY-015. QA pasó a ser gate obligatorio.
- **2026-06-23–24:** ADR-0137 confirmó la arquitectura hexagonal, mantuvo infraestructura transversal en `shared`, y completó puertos de integración.
- **2026-06-25–26:** se formalizó la biblioteca funcional de componentes UI; STORY-020, STORY-021 y STORY-022 quedaron cerradas y auditadas.
- **2026-06-27–28:** EPIC-0 cerró. Arrancó EPIC-1 con STORY-024. ADR-0141 ratificó `STRICT`, UUIDv7, enteros escalados, nanosegundos UTC y la fase greenfield.
- **2026-07-04–08:** se cerró el substrato de monetización 14/14, se pagaron retrabajos de fundaciones y ADR-0133 formalizó el gate de mutación con cero supervivientes.
- **2026-07-10–12:** se auditó EPIC-0, se cerró STORY-050 y el Banco de Pruebas quedó como gate permanente de Definición de Terminado.
- **2026-07-18:** ADR-0154 fijó que Drasus provee conectores, no datos. Se aprobó la doctrina “guardar el hecho y calcular la interpretación”; STORY-051 quedó auditada y DEBT-028 pagada.
- **2026-07-19:** se formalizaron el Gate Doc-First, la precedencia Quant→Architect, la distinción TASK/DEBT y el requisito de no precocinar soluciones en los despachos.
- **2026-07-22–23:** se analizaron datasets reales MT5, spread, fill y DST. Se diseñaron ADR-0162/0163, los catálogos y la declaración de procedencia.
- **2026-07-25–27:** se cerraron STORY-066, STORY-068 y STORY-070; TASK-067, TASK-069, TASK-071 y TASK-072 quedaron resueltas; el siguiente hito es conectar Canvas, asistente y almacenamiento real.

El estado completo de deuda no se mantiene aquí. Consultar [`docs/DEBT.md`](../../../docs/DEBT.md), que es el índice canónico. Los historiales extensos de sesiones viven en `../../plans/`; esta bitácora conserva solo contexto de recuperación.
