# Bitácora operativa — Tech Lead

> Estado vigente y registro histórico por sesión y ronda. Las Órdenes conservan criterios, evidencia y comandos; el ROADMAP conserva la fase y DEBT.md, las deudas.

## Estado de reanudación

Última actualización: 2026-11-11. Contrasta el estado operativo con las Órdenes y el ROADMAP al reanudar.

- **Fase:** EPIC-1 — Soberanía de Datos (`ingest`), en curso.
- **Órdenes activas de UI:** [STORY-075 — composición del Dashboard](../../../docs/execution/STORY-075-dashboard-oficial-tauri-react.md) y [STORY-076 — conexión Core/Tauri](../../../docs/execution/STORY-076-bootstrap-escritorio-conectado.md), ambas 🟡 parciales. El detalle de esta iteración está en [el reporte de reanudación](../../sessions/2026-11-09-dashboard-conectado-reanudacion.md).
- **Secuencia de escritorio instruida por el propietario:** Dashboard inicial conectado al Core, luego Banco de Verificación y después Canvas. Tras esa base, auditar cada Feature backend y conectar su frontend. Los widgets de Features siguen vacíos hasta sus slices reales.
- **Límite de la iteración anterior:** STORY-076 se acotó al Dashboard; Banco y Canvas no debían iniciarse entonces. El propietario abrió esta ronda para tratar el Banco; el límite anterior no se hereda.
- **Conexión en ejecución:** [SPIKE-006](../../../docs/execution/SPIKE-006-integracion-grpc-local.md) completado. [STORY-076](../../../docs/execution/STORY-076-bootstrap-escritorio-conectado.md) conecta Core–Tauri con sesión local, base estable en `app_data_dir`, historial durable y cursor conservado al relanzar el hijo. QA reprodujo 6/6 pruebas del ledger, 8 unitarias + 2 de proceso del Core y 9/9 de Tauri; aprobó la corrección funcional del cursor. Una captura del propietario muestra `Core conectado` y el primer widget vacío en la ventana nativa. El Dashboard sigue parcial: falta verificar idioma, Tab/foco, los cinco widgets y reflujo a 320 px; el propietario ya confirmó el cierre con X.
- **Siguiente paso de UI:** el propietario considera que el Dashboard ya terminó y plantea avanzar al Banco. Las Órdenes y ROADMAP aún marcan STORY-075/076 parciales; el último registro de QA deja pendientes la revisión visual completa, idioma, teclado/foco y reflujo a 320 px. Contrastar evidencia actual con los criterios antes de cambiar el estado. Si el Dashboard cumple, preparar la entrega del Banco conforme a la secuencia vigente; TTR-001 sigue parcial mientras falten sus superficies.
- **Orden de ingesta pendiente:** [STORY-073 — Asistente de Importación](../../../docs/execution/STORY-073-asistente-importacion-pantalla.md), construida y verificada; registra 13/14 criterios.
- **Para cerrar STORY-073:** el propietario debe completar el criterio 13 importando el archivo real desde la aplicación; el recorrido está en §6.1. Faltan las tres lecciones del Modo Docente (§8).
- **Después:** conectar la salida de barras al almacenamiento Parquet particionado Hive-Style; revisar [`DEBT-036`](../../../docs/debt/DEBT-036.md) y [`DEBT-040`](../../../docs/debt/DEBT-040.md).
- **Validación manual (2026-08-01):** corregidos el selector de archivos en Linux, el parpadeo de la zona de carga y la copia de errores. STORY-073 registra `flutter test` 30/30, `flutter analyze` sin errores y `flutter build linux --debug` correcto.
- STORY-073 pagó DEBT-035, DEBT-037 y DEBT-041. DEBT-045 se pagó el 2026-08-02.

## Pendientes acumulados

- **Base de escritorio:** [STORY-074](../../../docs/execution/STORY-074-cascaron-tauri-react.md) prepara el shell; [STORY-075](../../../docs/execution/STORY-075-dashboard-oficial-tauri-react.md) implementa el Dashboard oficial sin datos simulados. SPIKE-006 cerró y [STORY-076](../../../docs/execution/STORY-076-bootstrap-escritorio-conectado.md) implementa el bootstrap gRPC; falta el recorrido visual y las superficies futuras de TTR-001.

- **Importación:** DEBT-036 y DEBT-040 se disparan con la persistencia real. DEBT-039 está parcial: la interfaz muestra fechas reales junto al rango; el análisis profundo de homogeneidad sigue pendiente. DEBT-043 (mensajes con jerga) y DEBT-044 (arrastre de archivos) siguen abiertas.
- **Modelo de ingesta:** falta implementar en `pit-data-validator` los gates de coherencia de calendario y homogeneidad de ADR-0162. El [diseño de TASK-064](../../../docs/execution/TASK-064-politica-spread-fill-model-dst.md) está cerrado; su contraste empírico del spread depende de datos de ticks. El catálogo de [TASK-065](../../../docs/execution/TASK-065-catalogo-broker-instrumento-sesion.md) se entregó en STORY-066.
- **Datos reales:** [TASK-060](../../../docs/execution/TASK-060-muestra-real-forex-cfd.md) sigue bloqueada por el propietario para completar la muestra de 1 minuto y ticks, idealmente de dos brókers; el CSV M1 existente permitió el primer build. Sigue abierta [DEBT-025](../../../docs/debt/DEBT-025.md).
- **Cola de trabajo:** [TASK-052](../../../docs/execution/TASK-052-architect-escalation-market-data-sources.md) sigue abierta (prioridad de conectores y tabla de capacidades); [TASK-053](../../../docs/execution/TASK-053-jurisdiccion-constitucion-empresa.md), [TASK-054](../../../docs/execution/TASK-054-nivel-gratuito-cooperativa-minado.md), [TASK-055](../../../docs/execution/TASK-055-cumplimiento-por-arquitectura.md) y [TASK-056](../../../docs/execution/TASK-056-clausulado-especifico-y-comparado.md) siguen abiertas (jurisdicción, modelo del nivel gratuito, cumplimiento por arquitectura y clausulado); [TASK-059](../../../docs/execution/TASK-059-contrato-momento-de-conocibilidad.md) está pendiente y bloquea `signal-topology`; [TASK-061](../../../docs/execution/TASK-061-techo-magnitud-escala-entera.md) y [TASK-062](../../../docs/execution/TASK-062-catalogo-tipos-de-barra.md) siguen pendientes (rango de escala entera y tipos de barras; `DOLLAR` depende de TASK-061 y [DEBT-033](../../../docs/debt/DEBT-033.md) sigue abierta).
- **Decisiones de ingesta:** [TASK-063](../../../docs/execution/TASK-063-formato-estandar-de-ingesta.md) conserva la política de formato y mapeo; sus decisiones principales están en ADR-0156 y ADR-0114. Revisar la Orden antes de darla por cerrada.
- **Riesgo de registro:** la Orden STORY-073 atribuye nueve supervivientes previos de `bridge` a DEBT-042, pero no existe esa ficha ni fila en `DEBT.md`; reconciliarlo antes del cierre.
- **Estados desfasados:** [TASK-069](../../../docs/execution/TASK-069-architect-escalation-reloj-de-broker.md) aparece pendiente de despacho en su Orden, aunque ADR-0162 y STORY-070 documentan su resolución. TASK-064 y TASK-065 siguen `Abierta` en sus Órdenes aunque el ROADMAP da por cerrado el diseño de TASK-064 y STORY-066 entregó el catálogo de TASK-065. ROADMAP aún marca STORY-051 en curso, aunque su Orden la cierra el 2026-07-18. Alinear esas fuentes antes de reabrir o despachar.
- Otras deudas abiertas permanecen en [`DEBT.md`](../../../docs/DEBT.md). [DEBT-038](../../../docs/debt/DEBT-038.md) (siete timestamps sin sufijo de unidad) debe pagarse antes del primer release distribuido.

## Formato del registro

Actualiza la bitácora al cerrar cada ronda, no solo al terminar la sesión. Usa encabezados «Sesión AAAA-MM-DD» y «Ronda N — tema». Registra qué cambió, por qué, evidencia o resultado, estado de las Órdenes, pendientes y siguiente paso. Conserva causas y resoluciones útiles para reconocer recurrencias; enlaza la Orden para comandos y detalle técnico. Si no cambió el estado, escribe Sin cambio de estado. Contrasta el tablero con Órdenes y ROADMAP.

## Historial por sesión y ronda

### Sesión 2026-06-12

#### Ronda 1 — Arranque de EPIC-0

- **Plan de EPIC-0:** unificado dentro de `docs/ROADMAP.md` v2.0 (se borró `EXECUTION-PLAN.md`). Dos pistas paralelas: spikes de gates SPIKE-001–SPIKE-006 + tandas de cimentación (STORY-001→STORY-002→STORY-003-9→TASK-001-12). Usuario eligió arrancar por los cimientos.
- **STORY-001 (esqueleto) — Completado y auditado.** Rust-Engineer (Sonnet). Workspace Cargo con 8 crates de módulo (`ingest, generate, validate, incubate, manage, execute, feedback, withdraw`) + `shared`, patrón FCIS, cajas vacías. Auditoría Tech-Lead: `cargo build`/`cargo test` 9/9 verdes, 0 warnings, FCIS verificado por inspección.
  - Descubrimiento: el binario raíz `app` (SAD §4.2) NO se creó (criterio literal de STORY-001 no lo pedía). Decisión: se crea en **STORY-009** junto a la CLI. Anotado en ROADMAP.
- **STORY-002 (base de datos) — Completado y auditado.** Rust-Engineer (Sonnet). `migrations/0001_foundation_master_fields.sql` crea los 25 campos maestros exactos (ADR-0020), SQLite WAL, idempotente. Test en `crates/shared/src/persistence/pool.rs`. Auditoría Tech-Lead: 25 campos verificados contra el ADR, test verde.
- **Escalamiento a Architect (Opus) — Contrato de 25 campos + `transformation_id`.** Veredicto:
  - Los 25 campos son **contrato lógico/vocabulario obligatorio**, NO 25 columnas calcadas en cada tabla. Grupo I (Identidad) universal; grupos II–V por **Filtro de Relevancia por Perfil** (`architect/SKILL.md`, `TEMPLATES.md`). La tabla ancla de EPIC-0 es correcta y se queda.
  - `transformation_id` = identificador (TEXT/UUID), no flag booleano.
  - **Propagado** (aprobado por usuario): ADR-0020 "Aplicación" reescrita; SAD §17.9 y §20 alineados; glosa de `transformation_id` corregida en ADR + 8 módulos; typos "V2 V2" (7 features) y comillas sueltas (2 features) limpiados. Verificado por grep.
  - **Implicación para Sprint 1:** las tablas de STORY-003–STORY-008 aplican el filtro por perfil; NO copian 25 columnas.

#### Ronda 2 — STORY-003 (`clock`) completado y auditado

- Rust-Engineer (Sonnet). Reloj en `crates/shared`: `domain/clock.rs` (trait `Clock` + `DeterministicClock`, núcleo puro), `orchestrator.rs` (`SystemClock`, cáscara con `SystemTime::now()`), re-exportado en `public_interface.rs`.
- Auditoría Tech-Lead: build + clippy 0 warnings; 10 tests verdes incl. determinismo bit-a-bit (`deterministic_clock_same_seed_produces_identical_sequence`); FCIS verificado por grep (núcleo sin acceso a reloj real, cáscara sí).
- **Pendiente diferido a STORY-004:** auditoría del reloj (NTP offset, virtual_process_id, delta real/virtual). Requiere `audit-log` (STORY-004) + que el Architect defina el perfil de persistencia de `clock`. El engineer NO inventó campos (correcto). Anotado en bitácora del ROADMAP.

#### Ronda 3 — STORY-004 (`audit-log`) TTR-001 completado y auditado

- Rust-Engineer (Sonnet). Registro inmutable en `crates/shared`: `domain/audit_log.rs` (encadenamiento + verificación de hash, núcleo puro), `persistence/audit_log.rs` (repositorio append-only, sin update/delete), migración `0002_audit_log.sql` (tabla `audit_events` perfil Auditoría + triggers que abortan UPDATE/DELETE).
- Auditoría Tech-Lead: build + clippy `-D warnings` limpios; 22 tests verdes incl. detección de mutación de evento histórico, rechazo de UPDATE/DELETE por trigger, determinismo de cadena; FCIS verificado. Decisiones aceptadas: dep `uuid` (Rust puro), perfil Auditoría de `architect/SKILL.md`.
- TTR-002 (reconciliación Nautilus) fuera de alcance → EPIC-2+.

#### Ronda 4 — STORY-003 cerrado (Fase 2: rastro de auditoría del reloj)

- **Escalamiento al Architect (Opus):** las postcondiciones de `clock.md` citaban `ntp_sync_offset`/`virtual_process_id`/delta real-virtual como campos del catálogo ADR-0020, pero NO existen ahí (referencia huérfana). Veredicto del Architect: son **payload de evento** (`details_json`), no columnas; el reloj emite a la bitácora existente, Perfil D; 3 eventos auditables (`CLOCK_NTP_SYNC`, `CLOCK_MODE_TRANSITION`, `CLOCK_SESSION_CLOSE`); sin cambios a ADR-0020. Editó `clock.md`. Verificado por grep.
- **Rust-Engineer (Sonnet):** módulo de cáscara `crates/shared/src/clock_audit.rs` (`emit_ntp_sync`/`emit_mode_transition`/`emit_session_close` vía `AuditLogRepository::append`); dep `serde_json`. Auditoría Tech-Lead: clippy `-D warnings` limpio, 28 tests verdes, FCIS (núcleo `clock.rs` intacto) y granularidad del hot-path verificados. STORY-003 → ✅.

#### Ronda 5 — STORY-005 cerrado (async-job-executor) + política de pruebas

- **1ª ronda Rust-Engineer (Sonnet):** construyó núcleo `domain/job.rs`, `persistence/job.rs`, `orchestrator/job_executor.rs`, migración `0003_jobs.sql` (`jobs` mutable + `job_results` append-only). 60 tests verdes, clippy limpio. **PERO** la auditoría independiente del Tech-Lead detectó que el GATE de EPIC-0 (recuperación tras crash) no estaba probado: `recover_at_startup` con 0 invocaciones en tests y todo sobre `:memory:` (no demuestra durabilidad). Defecto de implementación → regresado.
- **Pregunta del usuario sobre testing → decisión:** se formalizó la política de pruebas (ver sesión 2026-06-21); se instaló `cargo-llvm-cov`. NO QA-Engineer dedicado.
- **2ª ronda Rust-Engineer (Sonnet):** añadió `jobs_survive_simulated_crash_and_are_recovered_on_restart` (DB en archivo, cierra/reabre pool, verifica QUEUED recuperado + RUNNING→QUEUED + nada perdido + evento `JOB_RECOVERED_AT_STARTUP`). Auditoría Tech-Lead: 62 tests verdes, clippy limpio, cobertura 90.80%, test del gate leído e inspeccionado. STORY-005 → ✅ (alcance EPIC-0). TTR-007 y cobertura del worker pool concurrente → EPIC-2+.
- **Nota:** la herramienta de subagentes de esta sesión NO tiene `SendMessage`, así que las correcciones se hacen con un Agent nuevo (contexto fresco) apuntando al código ya escrito, no continuando el agente previo.

#### Ronda 6 — Metodología: rename masivo F/W/G → Jira + sistema de Órdenes de Trabajo

- Se eliminaron los códigos F/W/G de TODO el repo (24 archivos). Nuevo esquema: EPIC-n, SPRINT-n, STORY-###, SPIKE-###, TASK-###, BUG-###. Archivos de ejecución renombrados a `docs/execution/STORY-00n-*.md`.
- Sistema spec-driven: cada trabajo tiene una Orden de Trabajo (`docs/execution/`) con el prompt exacto + comandos de validación + bitácora. Plantilla `_TEMPLATE.md`. Reglas en `base/SKILL.md` (sellado + comandos de validación) y `tech-lead/SKILL.md` (flujo de Órdenes).
- Build sigue verde; código intacto (el rename fue documental).

### Sesión 2026-06-13

#### Ronda 1 — TASK-006 — auditoría de Inundación de Fundaciones

TASK-006 cerró sus cuatro fases: 137 features y 8 módulos auditados; perfiles, contratos y Grupo I alineados. La causa raíz P1 fue una plantilla incompleta, corregida en TEMPLATES. La auditoría de 41 moonshots quedó diferida.

### Sesión 2026-06-16

#### Ronda 1 — Reanudación de sesión: ROADMAP v3.0, ADR-0120 (Modos), Orden STORY-007 redactada

- **Contexto recuperado al arrancar:** entre la sesión anterior (2026-06-12/13) y esta, el usuario hizo varios commits documentales por su cuenta. Releí `docs/ROADMAP.md`, los ADR nuevos y los `SKILL.md` afectados antes de tocar nada (regla de reanudación de `tech-lead/SKILL.md`).
- **ADR-0118 (entrega por módulo completo):** reescribió `docs/ROADMAP.md` a v3.0 — ya no es bitácora, solo guía de orden + estado simple. Reasignó `crash-recovery` fuera de EPIC-0 (va a `execute`/EPIC-5). Esto invalidaba el "siguiente paso" que dejé anotado (STORY-006 crash-recovery) — corregido.
- **ADR-0119:** separación Plano de Control/Ejecución para operación distribuida (EPIC-9+). No afecta el trabajo inmediato de EPIC-0.
- **ADR-0120 (Modos de Acompañamiento):** nuevo mecanismo Autónomo/Mentor/Revisión por Agente, declarado en la Orden de Trabajo. Pregunté al usuario el Modo para el Rust-Engineer de la siguiente Story; eligió **Mentor**.
- **Orden `STORY-007-telemetry.md` creada** (`docs/execution/STORY-007-telemetry.md`): alcance TTR-001 de `telemetry` (buffer no bloqueante + heartbeat + persistencia SQLite con poda), Rust-Engineer en Modo Mentor. Diferidos documentados en §8 de la Orden: TTR-002 (→EPIC-7, necesita `feedback`), Builder ETA/gRPC/WebSocket y Best Strategy Tracker (→EPIC-3/EPIC-8, necesitan `generate`/UI headless), CPU/memoria por proceso (→STORY-008, mismo dominio que `worker-isolation-orchestrator`). Diseño de esquema (`metric_name`/`details_json` fuera del contrato de 25 campos) aplicado por precedente directo de `audit_events` (STORY-004), sin necesidad de escalar al Architect.
- **Bajo Modo Mentor, yo NO despacho** (ADR-0120): la Orden queda lista; el usuario decide cuándo invoca `/rust-engineer` pasándole la ruta de la Orden. Yo retomo auditoría y cierre cuando esa sesión termine.
- ROADMAP actualizado: fila STORY-007 → "en curso (Modo Mentor, Orden lista, pendiente invocación)" con enlace a la Orden.

### Sesión 2026-06-18

#### Ronda 1 — Corrección de integridad de numeración (TASK-004→TASK-006) + protocolo aclarado

- **Origen:** el usuario notó que STORY-004 y TASK-004 compartían el número "4", violando la regla "secuencial global" de `tech-lead/SKILL.md`. Investigación con `git log` (fechas reales de creación de archivos y commits) para reconstruir la cronología antes de tocar nada.
- **Hallazgo:** STORY-001 a 005 se crearon juntas en un solo commit (2026-06-13 02:07). TASK (auditoría inundación) se creó 42 min después (02:49) — le correspondía el **6**, no el 4. "STORY-006" (crash-recovery) nunca tuvo archivo real: solo fue una mención de "siguiente paso" en un commit (17:11, ese mismo día) y una nota histórica en ADR-0118 — no es una colisión real, es una reserva fantasma. STORY-007 (`telemetry`) es trabajo ACTIVO (último commit real del repo) citado textualmente por ADR-0124 (protocolo de lecciones) — moverlo tenía mucho mayor radio de impacto que la única colisión real.
- **Decisión del usuario (Opción A, de un menú de 2):** renumerar SOLO TASK-004 → **TASK-006**. STORY-007 no se toca. `crash-recovery` sigue sin número de Story (correcto: ADR-0118 ya no pre-numera trabajo de épicas futuras).
- **Ejecutado:** `git mv` del archivo de la Orden + ID interno; referencias corregidas en ADR-0020 (línea del Registro de Mantenimiento), ADR-0120 (lista de compatibilidad retroactiva), ADR-0118 (nota histórica de "STORY-006" ahora explícita: nunca tuvo archivo, número liberado y reusado), `CONTENT-STRATEGY.md` (6 menciones, incl. una fila de tabla con placeholder obsoleto "STORY-006 — Próxima" corregida a STORY-007), y esta bitácora. Verificado con `grep -rn "TASK-004"` → 0 resultados reales restantes.
- **Protocolo de numeración aclarado** (pregunta del usuario: "¿por qué los Spikes no respetan la numeración global?" + "quiero poder insertar un Task entre épicas sin renumerar"). Se actualizó `tech-lead/SKILL.md` §"Vocabulario Ágil e Identificadores" con dos reglas que ya eran la práctica real pero no estaban escritas:
  1. **SPIKE-### tiene su PROPIO contador, independiente de Story/Task/Bug.** Los Spikes son una lista fija de 6 riesgos de viabilidad definida de antemano en el ROADMAP §6 (no se despachan incrementalmente como Stories/Tasks) — por diseño no comparten el contador global. Story/Task/Bug SÍ comparten un único contador secuencial entre ellos.
  2. **Solo la épica activa tiene numeración real.** Las épicas futuras (EPIC-1+) se listan en el ROADMAP por nombre de Feature/módulo, SIN número de Story pre-asignado — el número se asigna recién cuando el trabajo se despacha de verdad (se crea su Orden en `docs/execution/`). Esto es intencional: deja espacio para insertar un Task/Bug/Spike entre épicas sin tener que renumerar nada. `crash-recovery` (EPIC-5) es el ejemplo correcto: no tiene número todavía.
- **No se renumeraron los Spikes** (SPIKE-001-006): tienen su propio contador por diseño, no son una colisión real con Story/Task/Bug pese a reusar 1-6; renumerarlos habría sido una cascada masiva (citados en ADR-0107, ADR-0112 a ADR-0117, ROADMAP §6) para corregir algo que ya es coherente bajo la regla aclarada.

#### Ronda 2 — STORY-007 (`telemetry`, TTR-001) cerrada y auditada

- **Lo que pasó fuera de esta sesión del Tech-Lead:** el usuario arrancó en Modo Mentor (tecleó `TelemetrySampleContent`, defecto detectado en relectura: `process_id` duplicado), luego cambió la Story a **Modo Docente** (ADR-0122, nuevo) y el Rust-Engineer terminó la implementación completa con explicación profunda bloque a bloque. Lecciones formales en `docs/lessons/rust/STORY-007-telemetry.md` (ADR-0124: un archivo por Story, no por tema).
- **Auditoría independiente del Tech-Lead (reproducida, no tomada del reporte):** `cargo build --workspace` limpio; `cargo clippy --workspace --all-targets -- -D warnings` 0 warnings; `cargo test -p shared` → 76/76 verdes, verifiqué por nombre los 8 tests mapeados 1-a-1 contra los 8 criterios de la Orden (§5); `cargo llvm-cov --workspace --summary-only` → `domain/telemetry.rs` 100%, `orchestrator/telemetry.rs` 93.19% (coinciden exacto con lo reportado), `persistence/telemetry.rs` 94.29% (reportado 93.55%, variación menor). Inspección manual: `domain/telemetry.rs` sin `SystemTime`/`sqlx` (FCIS limpio); `migrations/0004_telemetry.sql` con columnas exactas del perfil + 2 índices justificados, sin triggers append-only (correcto, esta tabla SÍ borra por poda); 8 exports verificados en `public_interface.rs`.
- **Diseño de esquema** (`metric_name`/`details_json` fuera del contrato de 25 campos): aplicado por precedente directo de `audit_events` (STORY-004) — sin necesidad de escalar al Architect, ya resuelto.
- **Veredicto: APROBADO.** Sellado `docs/features/telemetry.md` (banner 🟡 Parcial, TTR-002 → EPIC-7) y `docs/ROADMAP.md` (fila STORY-007 → "parcial (TTR-002 → EPIC-7)").
- **Diferidos confirmados** (§8 de la Orden, sin cambios): TTR-002 → EPIC-7; Builder ETA/gRPC/WebSocket + Best Strategy Tracker → EPIC-3/EPIC-8; CPU/memoria por proceso → STORY-008.

### Sesión 2026-06-19

#### Ronda 1 — Revisión de commits y staged; ROADMAP actualizado a v3.1

**Commits revisados (HEAD hasta 6353961):**
- `d852540` — auditoría de integridad ADR: 42 hallazgos corregidos en 124 ADRs (documental).
- `c879749` — ADR-0124 lecciones: un archivo por Story/Task, no por tema (ya estaba en PROGRESS).
- `32fcb2e` — `agentic-mcp-gateway` dual-cabine (documental).
- `c03ec68` — STORY-007 (`telemetry`) ya auditada y cerrada (registrada en entrada 2026-06-18).
- `7741e46` — ADR-0122 Modo Docente + protocolo de lecciones (ya en PROGRESS).
- Y commits anteriores ya documentados.

**Staged (nuevas decisiones arquitectónicas del Architect):**
- **ADR-0125-0128** — Capa de datos fundamentales:
  - ADR-0125: Event Study + Surprise como métodos canónicos; NLP a `moonshots`.
  - ADR-0126: Sourcing externo de hecho crudo + scoring 100% propio (Soberanía).
  - ADR-0127: PIT de eventos — arrival timestamp + vintage/as-of (first-print vs revisiones).
  - ADR-0128: Mapa de exposición evento→activo + normalización por instrumento.
- **ADR-0129** — Entradas concurrentes no bloqueantes por defecto + de-duplicación de señal (extiende ADR-0004 y ADR-0081).
- **ADR-0130** — Frecuencia/horizonte de operación como objetivo declarable + agnosticismo de temporalidad (extiende NSGA-II y backtest-engine).
- **4 features nuevas:** `fundamental-event-store` (→ `ingest`/EPIC-1), `event-impact-scorer` + `asset-exposure-map` + `fundamental-indicator-projector` (→ `generate`/EPIC-3).
- **Features modificadas:** `order-fsm`, `advanced-trade-management`, `backtest-engine`, `nsga2-optimizer` (extendidas por ADR-0129/0130).
- **Módulos actualizados:** `ingest`, `generate`, `validate`, `execute`, `manage` (nuevos TTRs incorporados).
- **SAD-21.md** — nueva sección del SAD.
- **Índices actualizados por el Architect:** `docs/ADR.md` (ADR-0125-0130 registrados), `docs/README.md` (4 features nuevas con módulo asignado).

**Qué actualizó el Tech-Lead:**
- `docs/ROADMAP.md` → v3.1 (2026-06-19):
  - EPIC-1: añadido `fundamental-event-store` (ADR-0126/0127).
  - EPIC-2: añadidos ADR-0129 (N posiciones concurrentes) y ADR-0130 (agnosticismo de temporalidad).
  - EPIC-3: añadidas 3 features fundamentales + objetivo de frecuencia/horizonte (ADR-0130).
  - EPIC-5: `order-fsm` y `advanced-trade-management` referenciados con ADR-0129.


### Sesión 2026-06-20

#### Ronda 1 — STORY-010 (`agentic-mcp-gateway`) cerrada y auditada

**Auditoría independiente del Tech-Lead:**
- `cargo build --workspace` ✅ limpio.
- `cargo clippy --workspace --all-targets -- -D warnings` ✅ 0 warnings.
- `cargo test --workspace` → **103 verdes** (91 previos + 12 nuevos MCP).
- 12 tests MCP verificados por nombre (9 requeridos + 3 bonus: withdraw_denied, audit_hash_deterministic, audit_hash_differs).
- FCIS: `grep -n "sqlx\|tokio\|std::io" domain/mcp_gateway.rs` → 0 resultados. ✅
- Cobertura: `domain/mcp_gateway.rs` 93.18% · `persistence/mcp_gateway.rs` 97.91% · `orchestrator/mcp_server.rs` 9.60% (esperado — el servidor stdio no tiene test de integración sin cliente MCP real).
- Lección `docs/lessons/rust/STORY-010-agentic-mcp-gateway.md` creada (202 líneas, 5 bloques de enseñanza con código real).
- Crate MCP elegido: `rmcp` 1.7.0 (SDK oficial modelcontextprotocol, versión estable, 13M descargas).
- ADR-0123 sellado (✅), `agentic-mcp-gateway.md` sellado (🟡 Parcial — TTR-001 UI real + TTR-004 SaaS diferidos).
- **Veredicto: APROBADO.** ROADMAP fila STORY-010 → "🟡 parcial".

#### Ronda 2 — STORY-009 (`cli-app`) cerrada y auditada

**Auditoría independiente del Tech-Lead:**
- `cargo build --workspace` ✅ limpio.
- `cargo clippy --workspace --all-targets -- -D warnings` ✅ 0 warnings.
- `cargo test --workspace` → **93 verdes** (91 previos STORY-001-008 + 1 nuevo gate `kill -9` en `crates/app/tests/` + compilación de 8 módulos).
- Gate EPIC-0 verificado: `job_survives_kill9_and_recovers_on_restart` → OK. Salida real observada: "Recuperados 1 jobs del crash anterior." + evento `JOB_RECOVERED_AT_STARTUP` en BD.
- `drasus version` → `drasus v0.1.0` ✅; `drasus start` → arranca + apagado limpio ✅.
- FCIS: grep de `domain::\|persistence::\|orchestrator::` en `main.rs` → solo líneas de comentarios (7 y 93). ✅
- Cobertura: `app/src/main.rs` 95.77% líneas / 100% funciones. 8 regiones no cubiertas = rama `#[cfg(not(unix))]` (Windows) — inaccesible en Linux, aceptable.
- Lección `docs/lessons/rust/STORY-009-cli-app.md` creada (254 líneas), enlaza a la Orden y cita código real de esta Story.
- Decisión no especificada aceptada: re-exportaciones `create_pool`/`run_migrations` en `shared/public_interface.rs` para mantener ADR-0003 (el binario accede a `shared` solo por su `public_interface`).
- **Veredicto: APROBADO.** ROADMAP fila STORY-009 → "✅ terminado".

#### Ronda 3 — TASK-011 cerrada (Architect: enmienda ADR-0003)

**Escalamiento resuelto:**
- **ADR-0003** enmendado: sección "Persistencia en Features Multi-Consumidor (Regla de Tabla Única)" añadida. Fija 4 reglas FIJO: una Feature → una tabla → un módulo dueño (migración única); TTR de Integración = enchufar puerto, NUNCA migrar; "Consumido por" = accede al puerto; datos propios del consumidor van en tablas propias con referencia.
- **ADR-0118**: referencia cruzada bidireccional con ADR-0003 (Construcción vs Integración y su cara de persistencia).
- **`docs/templates/FEATURE.md`**: nota en "Dependencias y Bloqueantes" aclarando el significado de "Consumido por" para la persistencia.
- **Hallazgo del Architect:** ADR-0006 (Migraciones Centralizadas) ya hacía estructuralmente imposible la "tabla por consumidor" — la regla de ADR-0003 ahora lo hace explícito.
- **ADR a citar en Órdenes de Trabajo:** ADR-0003 §"Persistencia en Features Multi-Consumidor".

#### Ronda 4 — STORY-008 (`worker-isolation-orchestrator`) cerrada + SKILL.md actualizado

**STORY-008 — Auditoría independiente del Tech-Lead:**
- `cargo build --workspace` ✅ limpio.
- `cargo clippy --workspace --all-targets -- -D warnings` ✅ 0 warnings.
- `cargo test -p shared` → **91/91 verdes** (76 previos de STORY-001-007 + 16 nuevos de worker).
- Los 8 criterios del §5 tienen prueba nombrada presente y en verde: `shared_memory_access_latency_under_1ms`, `shared_memory_ram_constant_with_n_workers`, `shared_memory_worker_write_is_rejected`, `worker_graceful_shutdown_under_2s`, `worker_terminates_when_parent_drops`, `worker_jobs_recovered_to_queued_on_restart`, `worker_respects_max_concurrent_workers`, FCIS grep (0 imports de sistema en domain).
- Cobertura: `domain/worker_orchestrator.rs` 92.71% · `orchestrator/worker_runner.rs` 76.89% (aceptable, rutas OS-level parcialmente no ejercibles en CI sin procesos reales).
- FCIS verificado por grep: `domain/worker_orchestrator.rs` — cero imports de `std::process`, `tokio`, `memmap2`, `nix`. ✅
- Decisión de migración verificada: `process_id` ya existía en `jobs` (migración 0003) — no se añadió `0005_worker_pid.sql`. Correcto.
- Feature sellada por el Rust-Engineer durante la sesión Docente. ROADMAP actualizado a "terminado".
- **Modo Docente funcionó:** el Rust-Engineer completó bloques 1-4 con enseñanza bloque a bloque. Lección en `docs/lessons/rust/STORY-008-worker-isolation-orchestrator.md`.

**SKILL.md (tech-lead) — 3 actualizaciones de esta sesión (pendiente autorización para commit):**
1. Regla de git: autorización explícita por turno, sin herencia de sesiones previas.
2. Gate de Coherencia Pre-Despacho: auditoría ADR-0020 en 4 pasos (Grupo I, Perfil, Grupos coherentes, catálogo de 25) + claridad sobre variantes locales de latencia.
3. Modelo de tabla única por feature: una Feature → una tabla → un módulo dueño; consumidores usan el puerto, no duplican esquema.

**Auditoría ADR-0020 (143 features barridas):** catálogo sigue en **25 campos exactos** (confirmado en texto ADR + conteo + SQL migración 0001). Campos fuera del catálogo encontrados: 4 falsos positivos del parser (constantes/parámetros CONFIG) + campos locales legítimos ya documentados (agentic-mcp-gateway, variantes de latencia) + 2 sin clasificar explícita (`active_genome_domain`, `phase_id`) → locales válidos por ser de dominio único. Único candidato a vigilar: `latency_ns` en pre-trade-validator (1 feature; necesita 3+ para promover al catálogo).

#### Ronda 5 — TASK-012 y BUG-013: matriz de plataformas

TASK-012 fijó Windows/Linux/macOS nativos, clientes móviles por gRPC y Web como futuro incierto. La optimización prctl quedó exclusiva de Linux; macOS y Windows usan keepalive file. BUG-013 aplicó cfg target_os=linux a prctl en worker_runner.rs. QA APTO, 103/103 pruebas; ADR-0134 y referencias cruzadas actualizadas.

### Sesión 2026-06-21

#### Ronda 1 — Verificación independiente y QA

El Tech Lead quedó obligado a reproducir evidencia y QA a revisar la lógica además de las pruebas. STORY-015 mostró que revisar Dart no detecta incompatibilidades entre bindings generados y código; desde entonces, Flutter exige flutter build. Cada criterio debe mapear a una prueba discriminante.

#### Ronda 2 — STORY-014/015 y gates de viabilidad

STORY-014 aisló NautilusTrader 0.58.0 en nautilus_compat; QA APTO, 110 pruebas y SPIKE-001 cerrado. STORY-015 entregó el Panel Operativo Fundacional; Flutter build y test pasaron, QA APTO 8/8 y SPIKE-006 cerrado. La revisión encontró que Rust u64 se genera como Dart BigInt; los bindings generados quedaron como fuente de tipos y flutter build como gate obligatorio. Siguiente: cerrar SPIKE-002–005 por evidencia documental e iniciar EPIC-1.

### Sesión 2026-06-23

#### Ronda 1 — Repaso de EPIC-0 contra la arquitectura hexagonal (ADR-0135/0136/0137)

**Contexto:** el usuario pidió repasar toda la Épica 0 tras el giro arquitectónico de los últimos commits (`d9d44bf`→`134c21d`). Releí base/tech-lead SKILL, ROADMAP, ADR-0135/0136/0137, estructura física de crates y los 15 commits.

**Verificación independiente reproducida:**
- `cargo test --workspace` → **verde tras la reestructura**: 103 (`shared`) + 1 (kill-9 en `app`) + 1 (smoke Nautilus) + 1 (preset) = 106 tests. `app`/`bridge`/`nautilus_compat` compilan.
- Estructura nueva confirmada: los 8 crates de módulo (cascarones de STORY-001) **fueron demolidos** por `98a8e7c`. Hoy el workspace es `shared` + `crates/features/<dominio>/` (7 dominios vacíos + `_TEMPLATE`) + `crates/presets/standard-pipeline` + `app` + `bridge` + `nautilus_compat`.
- **Todo el código de EPIC-0 sobrevivió** porque vive en `crates/shared` (clock, audit_log, job, telemetry, worker_orchestrator, mcp_gateway) + 5 migraciones. No se tocó.

**Desajustes encontrados (lo construido vs ADR-0137):**
1. **🔑 (duda madre, ESCALADA al Architect):** las 6 features de Fundación son módulos dentro de `shared`, no crates hexagonales bajo `crates/features/infrastructure/`. ADR-0137 + invariante de CLAUDE.md exigen "un crate hexagonal por feature", pero la *nota* de ADR-0137 bendice que los *tipos* de infra vivan en `shared` y calla sobre las *features*. Decisión del usuario: **escalar al Architect** para que enmiende el ADR y zanje si migran o se quedan.
2. Ninguna de las 6 features de Fundación tiene `## Puertos de Integración` (verificado con grep). Pendiente backfill — **bloqueado por el veredicto** (#1 define en qué contexto se declaran).
3. Ficha EPIC-0 del ROADMAP desactualizada ("8 módulos + shared", "monolito modular FCIS"). Pendiente reescritura — se bundle con el post-veredicto para no editar dos veces.
4. **SPIKE-002/003 cerrados hoy** (erradicación de `tch`/`libtorch`/`pysr`/`python` verificada ausente por grep; ROADMAP §6 actualizado). **SPIKE-004/005:** su validación residual NO es ejecutable en EPIC-0 (desempeño del motor → EPIC-2; LLM → EPIC-8). Esto choca con el criterio de salida literal de EPIC-0 ("los 6 SPIKE con su validación residual ejecutada") → señalado al usuario como wrinkle del criterio.

**Decisión del usuario (2 preguntas):** (a) duda madre → escalar al Architect; (b) alcance → re-alinear docs + plan de migración. El plan de migración queda **bloqueado** hasta el veredicto (no se especula su contenido).

**VEREDICTO DEL ARCHITECT (subagente Opus, 2026-06-23) — auditado y APROBADO por el Tech-Lead:**
- **Problema 1 → SE QUEDAN EN `shared`.** Las 6 features de infra crosscutting NO migran. ADR-0137 enmendado (bloque "Enmienda 2026-06-23 — Excepción bendecida", línea 106) con criterio anti-coladero de 3 condiciones (produce tipo `textLabel` del catálogo + consumida por ≥2 dominios + sin puerto de Alpha en canvas). `crates/features/infrastructure/` queda vacía a propósito. **La Orden de migración queda DESCARTADA** (no hay nada que migrar).
- **Problema 2 → criterio de salida de EPIC-0 reformulado** (ROADMAP línea 73): gate de Fundación = veredicto en ADR para los 6 + residual ejercible en EPIC-0 (smoke SPIKE-001/006, erradicación grep SPIKE-002/003); desempeño (SPIKE-004) → EPIC-2, LLM (SPIKE-005) → EPIC-8.
- **Archivos del Architect:** `docs/adr/ADR-0137.md`, `CLAUDE.md` §1, `docs/ADR.md`, `docs/ROADMAP.md` línea 73. **Auditoría:** grep G1=0 contradicciones, G2=enmienda en los 3 archivos, alcance respetado (cero cambios en `crates/`/código/git).
- **Tech-Lead además:** selló SPIKE-002/003 en ROADMAP §6; reescribió ficha EPIC-0 (Objetivo + Alcance hexagonal) + nota de reestructura bajo la tabla de estado.

**✅ Backfill de puertos COMPLETADO y auditado (2026-06-23, subagente Sonnet).** Las 6 features de Fundación (`clock`, `audit-log`, `async-job-executor`, `telemetry`, `worker-isolation-orchestrator`, `agentic-mcp-gateway`) ya tienen `## Puertos de Integración` (posición correcta Tareas→Puertos→Gobernanza/Persistencia en las 6, verificado por grep). Puertos OUTPUT declarados con tipos válidos del catálogo: `timestamp_ns` (clock), `AuditEvent` (audit-log, agentic-mcp-gateway), `Job` (async-job-executor out / worker-isolation in), `TelemetrySample` (telemetry).

**✅ HUECO RESUELTO (2026-06-24, Architect Sonnet, auditado y APROBADO):** los 6 puertos sin tipo de catálogo NO eran un defecto sino una categoría faltante. ADR-0137 enmendado (Enmienda 2026-06-24, línea 165) crea la categoría **"puerto interno"**: la firma Rust que una feature `textLabel` de infraestructura expone a sus llamadores directos, que NO es un conector del canvas y por tanto NO requiere tipo de catálogo (se documenta con `(interno)`). Tres criterios para calificar + regla de promoción (3+ features de dominios distintos → se promueve al catálogo). Regla FIJO reescrita: "todo puerto DE CANVAS requiere tipo" (antes "todo puerto"); puertos internos exentos; un nodo sin puertos de canvas no conecta (los internos no cuentan); ante la duda → puerto de canvas (más restrictivo). Las 6 features actualizadas con sus puertos internos completos; flags de escalamiento eliminados. Auditoría: grep A1=0 flags obsoletos, A2=6 puertos internos presentes, A3=regla antigua reemplazada + 1 sola sección FIJO, alcance respetado (cero cambios en `ui/`/`crates/`). Seguimiento natural: cuando se construya `agentic-mcp-gateway` en su épica, el Rust-Engineer confirma que `mcp_call_in` coincide con la firma real del protocolo MCP (ajustable sin enmienda, es firma interna).

**➡️ SIGUIENTE PASO:** EPIC-0 queda coherente con la arquitectura hexagonal (código verde + docs realineados). Pendiente para cerrar formalmente EPIC-0: nada bloqueante. Listo para arrancar EPIC-1 (`ingest`) cuando el usuario lo autorice. Recordatorio: hay cambios sin commitear (regla: git solo si el usuario lo pide; agrupar por tipo `docs`/`chore`).

### Sesión 2026-06-25

#### Ronda 1 — Sub-fase de Estandarización de la Biblioteca de Componentes UI (formalización + arranque)

- **Origen:** el usuario reportó que la galería (biblioteca de componentes de producción) se construyó sin estándar entre varios modelos: colores/radios/padding hardcodeados, "glass mejorado" no seleccionable globalmente, color de fuente no configurable (texto invisible sobre fondo claro), bordes con glow fijo en vez del énfasis, cero comentarios de bloque, y bugs de interacción al hacer clic. Pidió estandarizar componente por componente (cobertura 100%) y formalizarlo dentro de EPIC-0 con el flujo completo de Tech-Lead.
- **Hallazgo de gobernanza:** el plan previo `tengo-feedback-1-en-peaceful-breeze.md` (2026-06-24) había definido STORY-016 (tema dinámico), 017 (Dashboard Shell), 018 (Canvas Shell) y 019 (Design System, ADR-0138) — **todas construidas en código pero nunca registradas** en el ROADMAP ni con Orden. Por eso ADR-0138 citaba "STORY-019" inexistente. El usuario eligió **formalización completa**.
- **Decisiones del usuario:** (1) modos de superficie N-extensibles (no un 4º fijo); (2) color de fuente auto por paleta + override manual; (3) selector de color híbrido (swatches + rueda HSV) uniforme; (4) refactor en 4 lotes Sonnet en paralelo, Modo **Autónomo**; (5) cobertura 100% sin muestreo + arreglo de bugs de interacción; (6) grabar la disciplina en los skills; (7) autorización al Tech-Lead para enmendar ADR-0138 sin escalar.
- **Hecho en esta sesión (gobernanza):**
  - Enmienda ADR-0138 (2026-06-25): "Tema Extensible — registro abierto de N propiedades". Índice `docs/ADR.md` completado (faltaban 0138 y 0139).
  - Órdenes retro `STORY-016..019` creadas en `docs/execution/` (estado real 🟡 parcial, nota ad-hoc).
  - Órdenes nuevas `STORY-020` (contrato de tokens, prompt de despacho listo) y `STORY-021` (estandarización 4 lotes + bugs, prompt común listo).
  - ROADMAP ficha EPIC-0: filas STORY-016 a 021 + nota de registro retroactivo.
  - Plan de trabajo: `.agents/plans/estamos-teniendo-problemas-importantes-hazy-cloud.md`.
- **✅ STORY-020 CERRADA (2026-06-25):** contrato de tokens congelado. 1ª entrega Flutter (Sonnet) → QA NO APTO (2 footguns `const` de superficie preexistentes, 1 borde hardcodeado introducido en el drawer, comentario inexacto, duplicación de mapa de paletas) → 5 correcciones aplicadas → Tech-Lead APROBADO (build linux verde reproducido; grep confirma 0 const de superficie, drawer usa `Gx.borderBase`, `kPalettes` única fuente). STORY-016 marcada ✅ (su pendiente de color de fuente lo entregó STORY-020). Artefactos nuevos: enum `enhancedGlass` + `kSurfaceModeRegistry` (registro N-extensible), `kTextDefaults` (texto oscuro para slate/paper), espejos estáticos `_globalAccent`/`_globalTextColor`, tokens `Gx.textBase*`/`borderBase`/`accentDynamic`/`borderHairline`/`borderFocus`/`space4..64`, widget `ui/lib/widgets/color_picker.dart` (swatches + rueda HSV reusable).

- **✅ STORY-021 CERRADA Y AUDITADA (2026-06-25):** estandarización total de las 13 secciones de la galería contra el contrato de tokens, build linux verde, QA APTO, cobertura 100%. Recorrido: 1ª tanda de 4 lotes cortada por límite de sesión a media tarea (recuperada sin perder trabajo) → re-tanda de 3 lotes sobre lo incompleto → QA NO APTO (hallazgo clave que el grep de literales NO ve: ~19 `Gx.borderPanel` estático en bordes de chrome → `Gx.borderBase`; tokens de texto estáticos en chrome; 3 radios sin token) → remediación → Tech-Lead APROBADO (greps de verificación en 0; 3 residuos `textLabel` en botones cerrados por el Tech-Lead). Skills actualizados (tarea de vigilancia): `flutter-engineer/SKILL.md` §"Biblioteca de Componentes — Contrato de Tokens" y `qa-engineer/SKILL.md` §"Gate de UI". **Registro histórico del corte (referencia):**
  - **Estado medido por sección** (grep de hardcodes residuales `Colors.white/black|Color(0x` y `BorderRadius.circular(` literal ≠999, y presencia de tokens nuevos `Gx.textBase/borderBase/accentDynamic/space`):
    - ✅ Avanzadas: `section_dataviz_quant.dart` (tokens=11, 0 hardcodes), `section_dataviz_extended.dart` (tokens=6, 0), `section_dag_nodes.dart` (tokens=11; quedan 3 radios literales).
    - 🟡 Parciales: `section_nav.dart` (tokens=2; 2 radios), `section_buttons_extended.dart` (1 Colors.white residual en loading button), `section_inputs_extended.dart` (3 radios), `section_data_display_extended.dart` (2 radios).
    - ❌ Sin tocar / casi nada: `section_dataviz_new.dart` (6 radios, 0 tokens), `section_std_missing.dart` (6 radios, 0 tokens), `section_feedback_extended.dart` (1 color, 2 radios, 0 tokens), `section_drasus_core_extended.dart` (1 color, 5 radios, 0 tokens), `section_animations.dart` (2 colores hardcoded — AccentAbSection, 0 tokens), `section_trade_tape.dart` (2 Colors.black en gradiente, 0 tokens).
  - (Resuelto: el reinicio re-despachó re-lotes A/B/C sobre lo incompleto; QA + remediación cerraron la Story. Lo de arriba queda como registro de cómo se recuperó el corte.)
  1. `grep -rnE "Colors\.(white|black)|Color\(0x" ui/lib/gallery/sections/` y `grep -rn "BorderRadius.circular(" ui/lib/gallery/sections/` (literales ≠ 999) → ver qué secciones ya están limpias y cuáles no.
  2. Por cada lote incompleto, re-despacha usando el prompt YA ESCRITO en `docs/execution/STORY-021-component-standardization.md` §4 (lotes: L1 inputs/buttons/std-missing · L2 nav/feedback/data-display · L3 las 3 dataviz · L4 dag-nodes/animations/trade-tape/drasus-core + widgets de componente de gallery_fx.dart).
  3. `cd ui && export PATH="$HOME/.flutter/bin:$PATH" && flutter analyze && flutter build linux --debug` definitivos (yo, no el reporte del agente).
  4. Gate de QA de STORY-021 (obligatorio): cobertura nominal 100% (checklist por lote vs código), hardcodes cero, reactividad a los N modos sobre paleta `paper` (claro) y `bunker` (oscuro), bordes/títulos en énfasis, bugs de interacción corregidos.
  5. Sellar STORY-021 (Estado ✅ + §7) y actualizar fila del ROADMAP.
  6. **Tarea pendiente 8:** grabar la disciplina en `.claude/skills/flutter-engineer/SKILL.md` y `.claude/skills/qa-engineer/SKILL.md` (contrato de tokens + comentarios + interacción probada + cobertura 100% sin muestreo; gate de UI de QA).
  - Contexto recuperable sin re-derivar: la Orden STORY-021 tiene los prompts completos y el mecanismo de checklist nominal permite ver qué falta.
- **➡️ SIGUIENTE PASO:** sub-fase de UI (STORY-016 a 021) **COMPLETA y auditada**. Biblioteca de componentes estandarizada y formalizada en EPIC-0. Pendiente: (a) decisión del usuario sobre commit (ver Git); (b) residuos de estilos no estandarizados (el usuario reporta que aún hay componentes fuera del estándar — pendiente diagnóstico con grep); (c) reanudar EPIC-1 (`ingest`) cuando el usuario lo autorice.
- **STORY-022 — Galería navegable y aislable (✅ cerrada 2026-06-26):** el usuario reportó que la galería de componentes era "un monolito" que impedía depurar componentes individuales. Diagnóstico real: no era un solo archivo (ya eran 14), sino que `gallery_tab.dart` renderizaba las 21 categorías (~150 widgets) juntas en un scroll único → imposible aislar. Solución (Opción 1, aprobada por el usuario): catálogo navegable maestro-detalle. Creado `lib/gallery/gallery_registry.dart` (modelo `GalleryEntry`/`GalleryCategory` + `buildGalleryCatalog`, builders bajo demanda); `gallery_tab.dart` reescrito de 1200→393 líneas (cáscara con panel lateral + buscador + detalle aislado). Despachado a Flutter-Engineer (Autónomo, Sonnet). En auditoría salió que las pruebas quedaron rojas por (1) overflow PREEXISTENTE de `AccentAbSection` (lo verifiqué aislando el widget a 380px → desborda 66px; no lo causó el refactor), (2) smoke test atado al diseño viejo, (3) goldens por regenerar. Re-despacho (alcance ampliado con autorización del usuario): overflow corregido (Row mainAxisSize.min + Flexible + ellipsis), smoke test reescrito con navegación real, goldens regenerados. Re-auditado: `flutter test` 3/3 verde, `flutter build linux` verde, `flutter analyze` 0 errores nuevos. Gate QA-Engineer → **APTO**. Orden `docs/execution/STORY-022-gallery-isolation.md` sellada ✅. **Fase 2 (planificada, NO ejecutada):** extraer ~40-50 componentes reutilizables a `lib/widgets/` (inventario en §8 de la Orden). **Pendiente git:** todo sin commitear; worktree huérfano `.claude/worktrees/agent-a1a9cd6a95fe207bf` por limpiar (requiere autorización).
- **Configuración de agentes opencode (2026-06-25):** creados 8 agentes en `.opencode/agents/` (rust-engineer, flutter-engineer, bridge-engineer, qa-engineer, quant-engineer, refactoring-engineer, ui-designer, architect) con modelos baratos asignados. SKILL de tech-lead actualizado con doble bloque de despacho (Bloque A = Claude Code, Bloque B = opencode). Ambos bloques incluyen lectura de `CLAUDE.md` + `base/SKILL.md` + SKILL de rol.
- **Git:** árbol sucio a propósito (ADR-0138, índice ADR, 6 Órdenes STORY-016..021, ROADMAP, PROGRESS, plan, 2 skills, tech-lead SKILL, 8 agentes opencode, + código UI de STORY-020/021). NADA commiteado. Git solo con autorización explícita del usuario, agrupando por tipo (`docs` para ADR/Órdenes/ROADMAP/skills, `feat(ui)` para el código).

### Sesión 2026-06-27

#### Ronda 1 — Cierre de EPIC-0 + arranque de EPIC-1 (STORY-024 Sovereign Fetcher)

- **Worktrees de git revisados (a petición del usuario):** un solo worktree huérfano `.claude/worktrees/agent-a1a9cd6a95fe207bf` en commit `61c6d3a` (ancestro de `main` → ya integrado, cero commits propios, solo una carpeta `ui/lib/gallery/` sin seguimiento duplicada de lo que `main` ya tiene). **Eliminado con autorización del usuario** (`git worktree remove --force`). Árbol principal limpio.
- **EPIC-0 cerrada:** todos los criterios de salida cumplidos (esqueleto verde, migración de 25 campos, recuperación tras `kill -9`, 6 spikes con veredicto en ADR, Panel Operativo Fundacional). Corregida fila obsoleta de TASK-011 en el ROADMAP (decía "pendiente"; estaba cerrada desde 2026-06-20). EPIC-0 marcada ✅ en el mapa de entregas.
- **EPIC-1 arrancada.** Primera historia por la cadena de precondiciones de `ingest`: **STORY-024 `sovereign-data-fetcher`** (descarga híbrida Bulk+Delta, módulo TTR-006 = feature TTR-001+TTR-002). Es el primer crate hexagonal de dominio del proyecto (puebla `crates/features/data/`).
- **Gate de Coherencia Pre-Despacho corrido sobre `sovereign-data-fetcher.md`** (correcciones in-situ, ninguna requirió escalar al Architect):
  1. Persistencia: eliminada fila duplicada de `data_snapshot_id`; el "URL/endpoint de la fuente" pasó a campo propio `source_endpoint` (provenance, fuera del catálogo de 25). Eliminado `execution_latency_ms` (Grupo V, ajeno al Perfil A) — la duración la lleva el `Job`/telemetría. Nota de perfil añadida.
  2. Puertos: añadida `## Puertos de Integración` (faltaba). Salidas `ticks_out` (`Tick`) / `bars_out` (`Bars`); nodo fuente sin input de canvas. Tipos ya en el catálogo ADR-0137 → sin escalamiento.
  3. Decisión de tipos: `Tick`/`Bars` ya existen como STUBS en `crates/shared/src/types/mod.rs`; se completan ahí (no en el crate de la feature) por el invariante "cada crate de feature depende solo de `shared`".
- **Modo de Acompañamiento (decisión del usuario):** Rust-Engineer en **Modo Docente** → **lo despacho yo** (subagente Sonnet); el Ingeniero implementa y escribe la lección en `docs/lessons/rust/STORY-024-...md`. Etapas que no aplican: 0.5 + 3-4 (sin superficie UI propia; la barra de progreso es de `background-download-manager`) y 1+6 (Quant — no es fórmula/estrategia/métrica).
- **Barrido ADR ampliado del Gate (2026-06-27):** incorporados a la Orden ADR-0011 (descarga = Job durable), ADR-0105 + SAD §8 (datos crudos crudos → Polars en el transformador; **corrigió que la Orden original pedía parsear a structs**, contradiciendo el SAD), ADR-0093 (credenciales diferidas — datos públicos), ADR-0008/0012 (config + concurrencia consciente). SAD sin cambio (la Orden se alineó hacia él).
- **Diferidos** (§8 de la Orden): TTR-003 (webhook listener) + TTR-004 (conversor de datos alternativos) → datos alternativos, alcance mayor, EPIC-3+. Prueba de integración contra Binance Vision real → manual opcional (no CI); las pruebas usan adaptadores falsos de los puertos.
- **Pendiente git:** cambios sin commitear de esta sesión (feature spec corregida, Orden STORY-024 nueva, ROADMAP, este PROGRESS). Git solo con autorización explícita, agrupando por tipo (`docs`).

### Sesión 2026-06-28

#### Ronda 1 — STORY-024 — superficie propia bloqueada

La vista del Banco para sovereign-data-fetcher se diseñó, pero faltaban tokens y componentes configurables para implementar el tema. El build verde no suplía esa capacidad; el hallazgo bloqueó la superficie y motivó revisar ADR-0138.

### Sesión 2026-06-29

#### Ronda 1 — ADR-0138 y deuda de Canvas

ADR-0138 se amplió para permitir propiedades de tema extensibles y separar componentes funcionales del estilo. La vista del Banco dejó de estar bloqueada por diseño; el nodo Canvas y su inspector siguieron pendientes.

### Sesión 2026-07-03

#### Ronda 1 — Orden del pipeline de ingesta

Se fijó descarga → transformación → sanitización → validación temporal/PIT → particionado. STORY-024 abrió crates/features/data; el lector no se adelantó a sus contratos y precondiciones.

### Sesión 2026-07-04

#### Ronda 1 — Cimientos 1–5 y DEBT-001

STORY-027–031 cerraron identidad, licencias, cuotas, medición de uso y consentimiento con QA APTO. En STORY-030, QA detectó que el ledger calculaba event_sequence_id y escribía sin proteger leer–incrementar–insertar en una transacción. UNIQUE evitaba corrupción, pero una carrera podía perder el evento; se abrió DEBT-001 y se fijó escritura atómica como regla.

### Sesión 2026-07-05

#### Ronda 1 — Pago de DEBT-001 y cimientos 6–9

STORY-032 corrigió los ledgers con BEGIN IMMEDIATE, reintentos acotados y error tipado; pagó DEBT-001/007. STORY-033–036 añadieron eventos, reportes, gateway y agregación. Un commit externo mezcló trabajo; la auditoría confirmó que nada se perdió y se reconstruyeron cambios agrupados por tipo.

### Sesión 2026-07-06

#### Ronda 1 — Cimiento 10 y corrección de ejes

STORY-037 entregó cuentas verificadas. El Architect detectó que capital_reality duplicaba institutional_tag; STORY-038 separó autoría y realidad del capital como ejes ortogonales. QA aprobó el retrabajo; STORY-035/036 también cerraron APTO.

### Sesión 2026-07-07

#### Ronda 1 — Cimientos 11–12 y retrabajos

STORY-039/040 cerraron continuidad de instancia y jerarquía. La auditoría de ADR-0020 halló un campo duplicado en el cimiento 10 y falta de max_child_accounts en el 12. STORY-041/042 corrigieron ambos; QA APTO y DEBT-016/017 pagadas.

### Sesión 2026-07-08

#### Ronda 1 — Substrato 14/14 y gate de mutación

STORY-043/044 completaron portabilidad y roles; el substrato quedó 14/14 con QA APTO. Como la suite podía pasar con conductas mutables sin prueba discriminante, se formalizó cargo-mutants en ADR-0133: cero supervivientes en dominio/persistencia y pruebas específicas para ledgers. El Tech Lead ejecuta el gate.

### Sesión 2026-07-10

#### Ronda 1 — Auditoría retroactiva de EPIC-0

El contraste ADR/código halló problemas en el esquema previo a ADR-0141 y en cobertura: FK inerte, tablas sin STRICT, identificadores/versiones y cadena de auditoría incompletos. STORY-045–048 corrigieron esquema, atomicidad y tokens; QA APTO. TASK-049 escaló decisiones documentales y se abrió DEBT-019 para mutación residual.

### Sesión 2026-07-12

#### Ronda 1 — STORY-050 — primera entrega del Banco

[STORY-050](../../../docs/execution/STORY-050-verification-bench.md) cerró APTO y estableció el Banco como gate de Definition of Done: harness genérico, selector, entrada JSON y salida de verify_feature para los 14 cimientos. Esa entrega histórica precede la migración de UI; la superficie vigente debe cumplir Tauri/React y gRPC, según ADR-0165.

### Sesión 2026-07-17–18

#### Ronda 1 — STORY-051 y propiedad de datos

STORY-051 entregó y auditó el backend del transformador. El propietario fijó: Drasus provee conectores, no datos; se conservan hechos crudos y se calculan interpretaciones al consumirlos. TASK-052–056 y TASK-059–062 registraron decisiones o insumos pendientes; TASK-060 quedó bloqueada hasta recibir muestras de bróker.

### Sesión 2026-07-19

#### Ronda 1 — 2026-07-19 · STORY-051 CERRADA + Gate de Aprendizajes aplicado

**Volcado íntegro de la sesión:** [`.agents/plans/sesion-2026-07-19-cierre-story-051-y-gate-aprendizajes.md`](../../plans/sesion-2026-07-19-cierre-story-051-y-gate-aprendizajes.md). Todo lo de abajo es el resumen; el detalle vive ahí.

**Cerrado:** STORY-051 (QA APTO · Quant APTO-CON-CONDICIONES) — primera pieza real de EPIC-1. También STORY-057 (identidad de serie en la barra) y STORY-058 (migración de tipos; el marcador `Tick` genérico se retiró del catálogo). DEBT-028 saldada.

**🔴 BLOQUEADO EN EL PROPIETARIO — TASK-060:** muestra real de Forex/CFD. Se necesitan **archivos CSV exportados de terminal, sin credenciales**: un par de divisas, dos granularidades (1 minuto y ticks), sin limpiar, e idealmente **de dos brókers distintos** del mismo par para probar que la tripleta los mantiene separados. No avanza sin él; salda DEBT-025 y alimenta TASK-052.

**Hallazgo de arquitectura (TASK-061 + DEBT-033):** ADR-0141 eligió la escala ×10⁸ por *precisión decimal* y nunca comprobó el *rango*. El techo de `i64` son ~92 200 millones de unidades; **una barra horaria de SHIB a volumen normal lo desborda 1,13×** (4,52× en día activo). Verificado aritméticamente. Directiva del propietario: **no se descarta la clase de activo** — se decide cómo soportarla.

**Dos decisiones que ya existían y nadie recordaba** (ambas detectadas por el propietario, no por los agentes): la regla del cierre de vela está **FIJA en ADR-0114** ("Bar-Open Alignment"), y la agregación a tipos de barra ya está especificada en `algorithmic-bars.md` (Renko, Rango, Volumen, Tick). Es la causa raíz que motivó la **Etapa 0.3**.

**Etapa 0.3 — Barrido de Coherencia Pre-Orden (NUEVA, obligatoria).** El Architect revisa toda feature ANTES de que el TL redacte su Orden: produce las preguntas que un ingeniero se hará, las responde citando documento y línea, y clasifica lo que queda en resoluble / contradicción / **bloqueo duro que se consulta al propietario**. Entrega una sección "Preguntas resueltas antes de implementar" que viaja con el despacho. Nació del caso del precio medio ponderado: se construyó, debatió y auditó algo que nunca debió existir en esa fase, porque la doctrina existía dispersa en prosa y nunca como respuesta localizable. El propietario lo describió como "un teléfono descompuesto que empeora en cada paso".

**Gate de mutación:** lo corre **solo el TL**, con `scripts/mutation-gate.sh`. Tres agentes agotaron su presupuesto esperando corridas sin entregar nada. El script resuelve además el fallo de RAM (`/tmp` es tmpfs; el enlazado de Polars la agota y muere con un Bus error que aparenta fallo de compilación). Doctrina completa en `.agents/knowledge/mutation-gate.md`.

**Molde canónico del ledger:** `.agents/knowledge/ledger-pattern.md`. Regla nueva: "sigue el patrón de ledger" se refiere a ESE documento, nunca a "la feature que ya lo tiene". Elegir referencia por antigüedad propagó dos huecos durante meses (DEBT-032).

**Deuda abierta:** DEBT-025, 029, 030, 031, 032, 033. **Tasks abiertas:** 052, 053, 054, 055, 056, 059, **060 (bloqueada en el propietario)**, 061, 062.

**Gate de aprendizajes: las nueve mejoras aplicadas**, incluidas la sincronía documental por etapa y correr mutación durante el trabajo del ingeniero (no solo en el gate final: en STORY-051 eso costó tres rondas donde bastaba una — QA detectó 14 supervivientes, el TL corrió el gate completo y encontró 32 más).

---

#### Ronda 2 — 2026-07-19 (tarde) · Gate Doc-First, reorden Quant→Architect, y tres frentes despachados

**Volcado de la sesión actualizado:** §7 de [`sesion-2026-07-19-cierre-story-051-y-gate-aprendizajes.md`](../../plans/sesion-2026-07-19-cierre-story-051-y-gate-aprendizajes.md).

**Commiteado** (6 commits agrupados por tipo, `d2a78bc`..`9eb38d8`): backend de datos, puente FFI, panel Flutter, pruebas de UI, documentación de ejecución, y skills/knowledge. **Sin push.**

**Cambio de orquestación (directiva del propietario).** El Quant deja de ser solo cazador de vacíos cuantitativos: **tiene acceso documental completo** (ADRs, SAD, features, módulos, moonshots, TTRs) y **obligación de citar** — cada hallazgo se entrega como "ya resuelto por X", "en contradicción entre X e Y", o "vacío real, busqué en Z". Y **va primero**: las etapas se reordenaron a **0.3 = Quant/Legal**, **0.4 = Architect**, con los hallazgos del Quant entregados íntegros al Architect como insumo. Razón: el Quant detecta el hueco de dominio mejor que nadie; que pregunte primero y el Architect confirme, corrija o amplíe.

**Gate Doc-First (`base.md` §7.4), obligatorio para TODOS incluido el TL.** Buscar en la documentación con `grep` antes de proponer una regla, contrato, fórmula o cambio de alcance. **"No encontré nada" solo vale declarando dónde se buscó** — una afirmación de vacío sin búsqueda es tan cara como inventar una regla. NO prohíbe improvisar: prohíbe improvisar sobre preguntas ya respondidas por escrito.

**Ponytail NO se reparte al equipo.** Gobierna *cuánto construir*, no *cuánto leer*; dos de sus reglas ("la escalera es un reflejo, no un proyecto de investigación", "si la explicación es más larga que el código, bórrala") empeorarían el fallo real. Se mantiene donde ya sirve: decisiones de alcance.

**DEBT vs TASK — criterio escrito** (`debt-management.md`). Era deriva real: el criterio no existía en ningún documento. **Hueco en algo que ya corre → DEBT** (se gobierna por disparador). **Pregunta que alguien debe contestar → TASK** (se gobierna por bloqueante). No se fusionan: si nadie toca una deuda el sistema sigue andando; si nadie toca una tarea, el trabajo que depende de ella no empieza. Un hallazgo con las dos caras genera las dos y se enlazan (caso real: DEBT-033 + TASK-061). **Todo accionable que dependa del propietario se marca `🔴 BLOQUEADO POR EL USUARIO`** en ficha, bitácora y ROADMAP — nunca en prosa dentro de un chat.

**TASK-063 (nueva) — formato estándar de ingesta y doctrina compra/venta.** Cuatro directivas del propietario: (D1) no se ingesta formato arbitrario, se prefiere que el usuario transforme afuera; (D2) plantillas por plataforma + mapeo declarado con vista previa, nunca adivinanza por nombre de columna; (D3) **la ejecución simulada solo lee compra o venta, jamás un campo genérico de precio** — matiz del TL: el hecho negociado se conserva como materia prima para barras Renko/Tick/Volumen/Rango, que se construyen contando operaciones reales; (D4) **la declaración del mapeo es evidencia inmutable** — ledger append-only siguiendo `ledger-pattern.md`, sellando archivo origen, mapeo mostrado, plantilla aplicada, instante y vista previa aceptada. D4 es lo que cierra la exposición legal de D1.

**Tres frentes despachados en paralelo** (2026-07-19, autorización explícita del propietario):
1. **General-Counsel** → veredicto sobre si transformar el archivo del usuario transfiere responsabilidad, con autorización para redactar clausulado en su dominio. Hipótesis a validar: transformar con mapeo declarado y aceptado NO transfiere; adivinar SÍ.
2. **Quant** → TASK-061 (techo de escala) + TASK-062 (catálogo de tipos de barra), estrenando el mandato documental. Su salida alimenta al Architect.
3. **Rust** → retrofit de DEBT-032 (prueba real del clasificador en `sovereign-data-fetcher` + candados en la migración 0006 + barrido de los seis ledgers restantes).

**Siguiente paso:** recibir los tres informes, correr el gate de mutación sobre lo del Rust, y despachar al Architect con los hallazgos del Quant en la mano. **Sigue bloqueado en el propietario:** TASK-060 (muestras reales de Forex/CFD) — el propietario las está buscando.

---

### Sesión 2026-07-22–23

#### Ronda 1 — 2026-07-22 / 07-23 · Frente de ingesta: datasets reales, spread/fill/DST, catálogo de entidades, y disciplina de despacho

**Commiteado al arrancar la sesión** (3 commits, `18e4cf0`..`5814034`): `fix(data)` retrofit del ledger (DEBT-032/034), `docs(ingesta)` escala entera + barras + formato estándar, `chore(agents)` plan + `datasets/` en `.gitignore`. **Sin push.**

**Datasets reales entregados por el propietario** (`datasets/`, ~900 MB, ignorados en git). Agente Explore los analizó: el CSV M1 de EURUSD 25 años (2000–2025, barras MT5 con columna SPREAD) es la fuente principal; el tick CSV (abril 2025, bid/ask real) el propietario lo **va a borrar**; los tres `.dat` son binario propietario de QuantDataManager, **no ingestibles sin re-exportar a CSV** (uno mal nombrado: es H1, no M1). Ningún archivo declara zona horaria ni bróker.

**Reconciliación bid/ask (doctrina D3 / ADR-0114 regla c).** El propietario planteó "operamos en el open y aplicamos el spread". **NO revierte el invariante Bid/Ask Exclusivo — lo cumple por reconstrucción:** open MT5 = bid, ask = open + `spread_reported`, compra al ask, vende al bid, nunca punto medio. La columna SPREAD del CSV es lo que hace ingestible los 25 años para fills (antes se creyó que no servía).

**TASK-064 (Quant) — política de spread + fill + DST.** Corregida por el propietario 2026-07-23: (1) el piso de spread NO se calibra contra abril 2025 (dataset fugaz); se calibra **por instrumento** desde el `default_spread` declarado del catálogo + la distribución de spreads no-cero del propio archivo. (2) DST: la zona horaria **se declara** (selector mundial con bandera DST, patrón SQX confirmado por el propietario), NO se infiere de la noticia de Nóminas ni de la volatilidad; hueco de fin de semana solo como verificación estructural opcional. (3) Spread nulo → forward-fill acotado; spread cero → sospechoso, piso por instrumento. Selector configurable: real/aleatorio/fijo con semilla obligatoria.

**TASK-065 (Architect) — catálogo Bróker × Instrumento × Sesión.** Destapó hueco real: **no existe hoy tabla de bróker/instrumento/sesión**; `Instrument` (ADR-0140) nunca se activó; los costos (spread/comisión/swap) viven como CONFIG global (incorrecto: el día de triple swap varía por bróker). Corregido 2026-07-23: se productiza como **FEATURES hexagonales, no tablas** (`instrument-catalog`, `session-catalog`, capa nueva en `pit-data-validator`); esquema de 3 tablas = su persistencia. Gate de coherencia sesión/activo es **N×N por huella de calendario** (no lista de pares), simétrico y con veredicto INDETERMINADO si el archivo es muy corto.

**Colisión de numeración corregida:** ambos agentes crearon `TASK-064` en paralelo; el del Architect se renumeró a **TASK-065**.

**Disciplina de despacho — dos reglas canónicas nuevas (causa raíz: el TL precocinó los prompts).** Los dos subagentes ejecutaron premisas defectuosas que yo les puse en la mano (dataset fugaz + noticia; tablas en vez de features). No fue capacidad de Sonnet, ni contexto, ni la división de governance. Canonizado: (a) `tech-lead/SKILL.md` §"Antes de despachar" — **no precocines la solución en el prompt**, entrega problema + restricciones + doc + entregable y deja que el especialista derive el método; (b) **`GOVERNANCE-4.7 — Reta la Premisa Recibida`** (imperativa para TODO agente) — un senior empuja contra una premisa masticada antes de ejecutarla. Memoria: `despacho-no-precocinar-solucion.md`.

**Estado de bloqueos (2026-07-23):** el enfoque está **aprobado por el propietario**; la zona horaria quedó resuelta (declarada); el dataset M1 está disponible → **TASK-060 efectivamente satisfecho para el primer build de Forex** (queda como follow-up re-exportar los `.dat`). **No hay bloqueo para el paso de diseño.**

**Siguiente paso:** despachar al **Architect** para que selle los ADRs y redacte las features. → **HECHO (2026-07-23), ver sesión 2026-07-23.**

---

### Sesión 2026-07-23

#### Ronda 1 — 2026-07-23 (tarde) · Architect completó el diseño de ingesta — HANDOFF para el próximo Tech-Lead

> ⚠️ **El chat de origen se elimina.** Handoff completo en [`.agents/plans/sesion-2026-07-22-23-ingesta-datasets-diseno-catalogo.md`](../../plans/sesion-2026-07-22-23-ingesta-datasets-diseno-catalogo.md). Esto es el resumen de retoma.

**Estado: frente de diseño de ingesta SELLADO, sin commitear, listo para revisión del propietario.** El Architect (despachado con prompt bien enmarcado, sin precocinar) entregó todo antes de que la API lo cortara por límite de sesión; verificado íntegro en disco.

**Producido (SIN COMMITEAR):**
- **`docs/adr/ADR-0162.md`** (nuevo) — Catálogo Instrumento × Bróker × Sesión; en índice `docs/ADR.md`. Activa ADR-0140, extiende 0154/0155/0137. Contiene 3 "Corrección del Architect" que retan la premisa (GOVERNANCE-4.7): UUIDv7 técnico vs. `instrument_id` de negocio; `row_version` (tablas mutables, no `event_sequence_id`); zona horaria **IANA** (`America/New_York`) en vez de la convención frágil `EST+07`.
- **5 ADRs enmendados:** 0114 (reconstrucción ask=open+spread), 0140 (activado), 0154, 0155, 0137.
- **2 features nuevas:** `instrument-catalog.md`, `session-catalog.md`.
- **4 features extendidas:** `pit-data-validator` (gate coherencia N×N), `data-import-wizard` (selector zona + gate), `data-sanitizer-pipeline` (piso de spread), `backtest-engine` (`SPREAD_SOURCE_MODE`).
- **`scripts/now.sh`** — reloj de pared determinista (correr antes de estimar cualquier reset de límite; la hora del sistema es la fuente de verdad).

**Lote completo sin commitear** (para agrupar cuando el propietario autorice): TASK-064, TASK-065, ADR-0162 + 5 ADRs + `ADR.md`, 2 features nuevas + 4 extendidas → `docs`; `governance.md` (regla GOVERNANCE-4.7), `tech-lead/SKILL.md` (no precocinar despacho), memoria `despacho-no-precocinar-solucion.md` + `MEMORY.md`, `scripts/now.sh`, `PROGRESS.md`, el plan de sesión → `chore(agents)`. **NO push salvo orden explícita.**

**Siguiente paso real (es CÓDIGO):** despachar ingenieros a construir el **adaptador de ingesta MetaTrader** (CSV M1 en `datasets/`, gitignored) → esquema canónico: parser TAB, escala ×10⁸, timestamps ns UTC con zona IANA declarada, `ask = open + spread_reported`, política de spread nulo/cero por instrumento, gate de coherencia de calendario. **En cada despacho aplicar GOVERNANCE-4.7:** entregar problema + restricciones + doc + entregable, nunca el método precocinado.

**Bloqueos:** alcance de día 1 de plantillas de sesión (qué brókers) `🔴 pendiente del propietario`; TASK-060 satisfecho para el primer build de Forex (CSV M1 basta); los `.dat` necesitan re-export a CSV (follow-up). El tick de abril 2025 el propietario lo borra — nada depende de él.

---

### Sesión 2026-07-25

#### Ronda 1 — Diseño de ingesta y STORY-066

El propietario aprobó ADR-0162/0163 y la política de spread/gaps: persistir hechos crudos y reconstruir ask al ejecutar; los stress tests no alteran datos guardados. STORY-066 entregó catálogos de instrumento/sesión; el TL reprodujo pruebas, esquema y CLI; gate en cero y QA APTO. Opciones, swap_type y una arista del SAD quedaron diferidos.

#### Ronda 2 — STORY-068 — datos reales y unidad del spread

Dos despachos fallaron por error 529 antes de escribir; el tercero partió de árbol limpio. Quant encontró que 60.9% de unos 5 M de ticks tenía un solo lado y dos barras de 9.2 M eran físicamente imposibles. Architect definió arrastre acotado y fuente explícita. El gate encontró 21 supervivientes en plantilla, hash y cierre de barra; 17 pruebas nuevas los mataron. El propietario detectó que faltaba declarar la unidad del spread: se verificó que MT5 reporta puntos, no pips, y se selló POINTS_MT5 sin escalar el dato crudo.

#### Ronda 3 — STORY-068 — cierre y lección

QA contrastó una anomalía contra el CSV real; STORY-068 cerró APTO. Se corrigieron tres desfases documentales y se abrieron DEBT-035/036 para mapeo manual y trazabilidad al almacenamiento. La causa recurrente fue no barrer todos los campos de una familia: se declaró unidad para volumen, pero se omitió spread. Se agregó el barrido de campos crudos a las verificaciones.

### Sesión 2026-07-26

#### Ronda 1 — TASK-067, reloj y procedencia

TASK-067 descartó los .dat de StrategyQuant X: no eran formato oficial ni tenían especificación pública. El análisis de zona horaria invirtió una conclusión: el patrón constante de IC Markets ocurría porque el reloj seguía el DST estadounidense. Además, el archivo empezaba antes de existir el bróker y mezclaba fuentes; atribuir todo su rango a IC Markets violaba la procedencia. Ambos problemas se escalaron como TASK-069.

#### Ronda 2 — TASK-069 y alcance de importación

El Architect resolvió reloj con ancla IANA y desplazamiento en minutos, y declaración por tramo para separar cambios de convención y datos de otro origen. Rechazó versionar la unidad de volumen en el catálogo: la heterogeneidad medida era del archivo. El propietario priorizó un hito de uso real con pantalla, mapeo manual y almacenamiento antes del lector de ticks.

#### Ronda 3 — STORY-070 — gate inicial y TASK-071

El gate halló tres supervivientes: el inicio predeterminado podía retroceder hasta 1970 sin fallar. Se exigió comparar ambos extremos con fechas reales, no repetir la constante en el esperado. TASK-071 escaló la omisión del Grupo I en broker_server_clock: la lista literal del ADR contradecía el patrón de sus tablas hermanas; no correspondía al TL resolverlo por inferencia.

### Sesión 2026-07-27

#### Ronda 1 — TASK-071/072, M13 y cierre de STORY-070

El Architect confirmó Grupo I y se añadió concurrencia optimista/auditoría a broker_server_clock. TASK-072 halló que venue_operational_since perdió _ns al cruzar puerto, orquestador y SQL; con unidad equivocada, el gate siempre pasaría. ADR-0141 y el gate del TL fijaron M13 para exigir _ns; DEBT-038 registra siete columnas antiguas. Ambos crates cerraron gate en cero; STORY-070 terminó 13/13, QA APTO.

#### Ronda 2 — DEBT-039 y acceso al inspector Canvas

QA encontró que el rango firmado no se cotejaba con las fechas del archivo: podía declarar un siglo sobre un día, falsificar evidencia y romper idempotencia. Se abrió DEBT-039 Alta. Canvas permitía soltar nodos, pero ninguno abría inspector; otro inspector ya construido tampoco era alcanzable. El propietario decidió incluir ese cableado en el hito.

### Sesión 2026-07-31

#### Ronda 1 — Separar pantalla y almacenamiento

Se confirmó que el transformador leía Parquet, pero solo escribía dentro de pruebas; faltaba escritura productiva particionada. El propietario separó pantalla primero y almacenamiento después. DEBT-035/037/039 corresponden a la pantalla; DEBT-036, al almacenamiento.

#### Ronda 2 — 2026-07-31 · STORY-073 construida — pantalla del asistente de importación

**Estado: 13/14 criterios cumplidos. El único pendiente (#13) solo lo puede ejecutar el propietario** — importar el archivo real de 9,2 M de filas desde la aplicación de escritorio; no hay forma de manejar la GUI desde el entorno de agentes.

**Entregado:** vía de mapeo manual con su pantalla de seis pasos y vista previa de dato ya transformado (LDR-0008 condición 1); mecanismo genérico de paneles del lienzo (`canvas_inspector_registry.dart`) — dar de alta un panel cuesta una entrada de mapa, y con él se adoptó el panel huérfano del transformador híbrido; cotejo del rango declarado contra el contenido real; soporte de fecha/hora en columnas separadas de punta a punta.

**Deudas:** `DEBT-035` y `DEBT-037` ✅ pagadas · `DEBT-041` ✅ pagada · `DEBT-039` 🟠 parcial (contención cerrada; el caso "declarado más ancho que el contenido" se resolvió mostrándolo al usuario, sin inventar umbral) · `DEBT-040`, `DEBT-042` abiertas · **`DEBT-043` nueva** (🟡 jerga de programador en mensajes de rechazo, ADR-0156 R1).

**Verificación:** gate de mutación `data-import-wizard` **APTO, 0 supervivientes** (160 mutantes; el único *timeout* está diagnosticado — `attempt += 1` → `*= 1` cuelga el bucle de reintento, es detección por cuelgue, no superviviente). `bridge` NO APTO por 9 supervivientes **previos** → `DEBT-042` con justificación escrita. `cargo test`: 96 + 33. `flutter test`: **22/22**. `flutter analyze`: 0 errores.

**Dos hallazgos ajenos al encargo, ambos por verificar en vez de confiar:**
1. **Cadena de pánico de 3 eslabones** — una fecha fuera del rango representable en ns (año 9999 pasa cualquier validación de formato) reventaba el Núcleo, cruzaba FFI como excepción, y la pantalla no capturaba: botón colgado en "Analizando…" para siempre. Cortada en ambos extremos, con pruebas demostradas discriminantes.
2. **`flutter test` llevaba >1 mes en rojo permanente y no estaba registrado.** `docs/DESIGN.md` documentaba tres tipografías que STORY-023 sustituyó por Rajdhani borrando sus `.ttf`; el golden seguía cargándolos y reventaba en `setUpAll`. Al regenerar el PNG **lo inspeccioné**: media pantalla salía en cajas grises porque la escala tipográfica nace sin familia hasta que `load()` corre, cosa que el test nunca hacía. Corregido; el golden por fin sirve de línea base visual.

**Lección transversal, tercera vez en esta cadena:** un criterio verificable por `grep` no es, por eso solo, un criterio correcto. Falló el criterio 12 original (escrito por mí), y falló el criterio de pago de `DEBT-037` (`grep -c "Glow" → 0`, imposible sin destruir contenido legítimo y premiando borrar las notas históricas útiles). Cuando la verificación mecánica sale más barata que pensar el criterio, se acaba midiendo lo que la herramienta sabe medir.

**Saneado en el momento (no es deuda):** lenguaje mortuorio en `DESIGN.md` y tres archivos Dart — `autopsy-header` → `ceremonial-header`, `_autopsyHeader()` → `_closingReportHeader()`.

**Siguiente entrega:** conectar la pantalla con el almacenamiento real (escritura Parquet + particionado) → salda `DEBT-036` y `DEBT-040`. Es la segunda mitad de la decisión del propietario "pantalla primero, almacenamiento inmediatamente después".

**Sin commitear y SIN PUSH.** 41+ commits locales pendientes de autorización de subida.

### Sesión 2026-08-01

#### Ronda 1 — 2026-08-01 · Primera prueba manual del propietario — tres defectos de frontera

**Los tres estaban fuera del alcance de todo lo que sabemos automatizar hoy** (30 pruebas de widget, gate de mutación 0 supervivientes, `analyze` limpio): viven en la frontera con el sistema operativo y con el cuerpo humano.

1. **El diálogo de archivos no abría.** `file_picker` no abre diálogo propio en Linux — lanza el binario externo `zenity`, ausente en Fedora Silverblue. Inaceptable en producto (KDE mínimo tampoco lo trae; imagen atómica = reiniciar para instalarlo). Sustituido por `file_selector`, que compila el diálogo GTK dentro del plugin. Verificado con build real: plugin registrado y `.so` en el bundle.
2. **Parpadeo infinito de la zona de carga.** La geometría dependía del hover (texto más corto + borde más grueso, que en Flutter consume espacio interior) → la caja encogía, su borde pasaba bajo el cursor, `onExit`, crecía, `onEnter`. Bucle. **Prueba de regresión con rojo→verde demostrado** (198,8×109 px contra 400×128 px). Canonizado como regla irrompible en `DESIGN.md`, **corrigiendo su matriz de estados, que decía "hover = leve escala"** — el propio documento de diseño prescribía la clase de defecto.
3. **El error no se podía copiar**, hubo que transcribirlo a mano. `CopyButton`/`CopyableText` nuevos + `copyable` en `Banner` y `KeyValue`. Regla en `DESIGN.md`: etiquetas y botones no seleccionables; errores, huellas, identificadores y rutas siempre copiables.

**Deudas nuevas:** `DEBT-044` (🟡 arrastre real nunca cableado; el texto que lo prometía se saneó en el momento) · `DEBT-045` (🟠 los nodos del lienzo no dicen qué consume ni qué produce cada feature).

**Fallo de proceso propio, con regla permanente.** El propietario abrió el *Transformador de Datos de Mercado* en vez del asistente y concluyó que la pantalla era inservible — con razón sobre lo que veía. La causa no fue suya: el criterio 13 se le asignó con "solo lo puede ejecutar el propietario" **y ninguna instrucción de cómo**. Regla nueva: *todo criterio cuyo ejecutor sea el propietario se entrega con el recorrido literal*. **Un criterio que el TL no puede ejecutar es un criterio que debe poder dictar.** Recorrido escrito en §6.1 de la Orden. Mismo patrón que los tres criterios `grep` de esta Story: escribir el criterio salió más barato que pensar cómo se cumple.

**Hueco de protocolo detectado y NO saldado:** las tres etapas corrieron en modo Docente y ninguna dejó archivo en `docs/lessons/` (ADR-0122/0124). Pendiente antes de cerrar la Story.

**Verificación:** `flutter test` 30/30 · `flutter analyze` 0 errores · `flutter build linux --debug` correcto · goldens sin cambio · filas de `DESIGN.md` actualizadas para las cuatro firmas que cambiaron (el criterio 12 se rompería el mismo día en que se cerró).

### Sesión 2026-08-02

#### Ronda 1 — DEBT-045 pagada

El registro vigente marca DEBT-045 como pagada. La prueba manual del asistente y sus defectos de frontera están en la ronda del 2026-08-01.

### Sesión 2026-11-09

#### Ronda 1 — STORY-075/076 — Dashboard conectado

Dashboard Tauri/React quedó integrado con el Core local por gRPC. QA reprodujo el cursor persistente y 9/9 pruebas de Tauri; Core pasó 8 unitarias y 2 de proceso. CUA no expuso la ventana, así que quedaron pendientes idioma, teclado/foco, reflujo y revisión visual. El propietario confirmó conexión, primer widget vacío y cierre con X. Detalle: [reporte](../../sessions/2026-11-09-dashboard-conectado-reanudacion.md), [STORY-075](../../../docs/execution/STORY-075-dashboard-oficial-tauri-react.md), [STORY-076](../../../docs/execution/STORY-076-bootstrap-escritorio-conectado.md).

### Sesión 2026-11-11

#### Ronda 1 — Banco de Verificación

El propietario aclaró que el límite anterior terminaba en Dashboard y abrió el trabajo del Banco. Considera terminado el Dashboard; Órdenes y ROADMAP lo marcan parcial por verificaciones visuales pendientes. El tablero mantiene ambos hechos y condiciona el despacho a contrastar evidencia.

#### Ronda 2 — actualización por ronda

El propietario pidió actualizar esta memoria en cada ronda. Se editó tech-lead/SKILL.md para exigir fecha, resultados verificados, estado de Órdenes, pendientes/bloqueos y siguiente paso. STORY-075/076 siguen parciales; el siguiente paso no cambió: contrastar evidencia visual antes de despachar el Banco.

#### Ronda 3 — recuperar y ordenar la bitácora

El propietario restauró el histórico y pidió ordenarlo por sesión/ronda, conservando causas, retos, resoluciones y patrones. Se retiró la sección antigua de reglas para Claude Code/opencode: ya no gobierna el flujo; las reglas vigentes están en AGENTS.md y las skills. Se reorganizó el histórico por sesión y ronda; se retiraron duplicados y reglas obsoletas, y se conservaron causas, correcciones, evidencia y patrones. STORY-075/076 siguen parciales; falta contrastar evidencia visual y decidir el despacho del Banco. git diff --check pasó. No se usó context-refiner.
