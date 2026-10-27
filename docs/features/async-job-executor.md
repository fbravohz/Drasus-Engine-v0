# Async Job Executor — Patrón Asincrónico de Operaciones Costosas

**Carpeta:** `./features/async-job-executor/`  
**Estado:** 🟡 Parcial — núcleo durable + recuperación tras crash (criterio EPIC-0) implementados y probados; TTR-007 (integración con Features costosas) secuenciado a EPIC-2+.
**Última actualización:** 2026-06-12  
**Decisión Arquitectónica Asociada:** ADR-0011 (Operaciones Asincrónicas), ADR-0165 (Tauri/React + gRPC), ADR-0166 (Operaciones Observables)

> 🟡 **Parcial** 2026-06-12 · Orden [STORY-005](../execution/STORY-005-async-job-executor.md) · TTR-001..006 implementados (cola Tokio+SQLite, worker pool, persistencia `jobs`/`job_results` append-only, **recuperación en startup**, progreso, cancelación). **Gate EPIC-0 demostrado:** test de recuperación tras crash sobre DB en archivo (`jobs_survive_simulated_crash_and_are_recovered_on_restart`). Auditado Tech-Lead: 62 tests verdes, clippy `-D warnings` limpio, cobertura workspace 90.80%. Pendiente: TTR-007 (integración con `generate`/`validate`/etc.) a EPIC-2+; cobertura del worker pool concurrente se completa con cargas reales.

---

## ¿Qué es?

Async Job Executor implementa un patrón de tres fases para ejecutar operaciones computacionalmente costosas (backtesting, generación, optimización) sin bloquear la interfaz del usuario.

**Problema:** Backtesting masivo puede tardar minutos/horas. Si ejecuta sincrónico, usuario está bloqueado, no puede hacer nada más.

**Solución:** La Feature dueña solicita un job durable y conserva su `operation_id`. El sistema lo procesa en segundo plano y publica estados por stream gRPC recuperable; cualquier cliente autorizado puede reconstruir el progreso tras reconectarse. Al terminar, consulta el resultado por el puerto de la Feature dueña.

**Resultado observable:** Usuario dispara job costoso, interfaz sigue responsiva, puede monitorear progreso, obtiene resultados cuando estén listos.

---

## Narrativa de Producto

**Capacidad:** `CAP-09` — Una interfaz donde se ve el razonamiento

**Infraestructura.** Encola las operaciones pesadas (un backtest, una generación masiva) para que corran en segundo plano, con progreso consultable y recuperación automática si el sistema se reinicia a medio camino.

El usuario nunca la ve. La nota como que **la interfaz nunca se congela mientras corre algo pesado**, y que un backtest que iba a la mitad sobrevive a un reinicio inesperado.

---

## Comportamientos Observables

- [ ] Usuario o agente autorizado solicita un backtest de estrategia (operación costosa):
  → La Feature crea una operación identificable y un job durable (`state=QUEUED`)
  → La respuesta gRPC incluye `operation_id`, `job_id` y código de estado
  → Usuario no está bloqueado

- [ ] Un cliente autorizado observa el job mientras se ejecuta:
  → Recibe snapshot inicial más cambios ordenados de estado/progreso por gRPC
  → Tras reconexión, reanuda desde su posición o pide snapshot; deduplica cambios sin ejecutar de nuevo el job

- [ ] Job completa exitosamente:
  - Estado cambia a `COMPLETED`
  - Resultados se guardan en tabla `job_results` (append-only, nunca se modifican)
  - Usuario obtiene: `{ "state": "COMPLETED", "result_uuid": "xyz-456" }`

- [ ] Un cliente autorizado obtiene resultados de job completado:
  → Consulta el resultado por la frontera pública de la Feature dueña
  → Resultado es inmutable (snapshot de cuando completó)

- [ ] Job falla (ej: error de datos):
  - Estado cambia a `FAILED`
  - Mensaje de error se guarda: "Invalid date range: start > end"
  - Usuario obtiene: `{ "state": "FAILED", "error": "Invalid date range: ..." }`

- [ ] Usuario o agente autorizado cancela un job en progreso:
  → Sistema valida la capacidad vigente y marca el job como `CANCELLED` en una frontera segura de cancelación
  → Worker actual detiene ejecución gracefully
  → Estado retorna: `{ "state": "CANCELLED" }`

- [ ] Sistema reinicia inesperadamente (crash):
  - Jobs en estado `QUEUED`/`RUNNING` se recuperan de tabla SQLite
  - En startup, worker re-encolam jobs recuperados
  - Auditoría registra: "SYSTEM_RESTART: recovered 3 jobs"

- [ ] Múltiples jobs se disparan en paralelo:
  ```
  Job 1: backtest estrategia A
  Job 2: generar 100 candidatas
  Job 3: optimizar portafolio
  ```
  → Los 3 se encolan
  → Se ejecutan concurrentemente (máximo `max_concurrent_jobs`)
  → Usuario monitorea todos sin esperar

---

## Restricciones

- **NUNCA un job se modifica después de completar.** Resultado es inmutable.
- **NUNCA se pierden job results.** Append-only en tabla SQLite.
- **NUNCA se ejecutan más de `max_concurrent_jobs` jobs simultáneamente.** Hard limit de recursos.
- **NUNCA un job QUEUED permanece indefinidamente.** Timeout configurable (default: sin límite, pero recomendado 1 hora).
- **NUNCA se olvidan jobs después de crash.** Recuperación automática en startup.
- **NUNCA un job retorna resultado parcial.** Es TODO exitoso o TODO fallido (atómico).
- **NUNCA** se acepta una cancelación, reintento o modificación de un job sin validar identidad, rol, puerto de Feature, objeto y versión esperada cuando aplique.
- **NUNCA** un stream gRPC es la única fuente de estado: la operación y el job deben poder reconstruirse desde almacenamiento durable.

---

## Parámetros Configurables

| Parámetro | Default | Rango | Qué hace | CONFIG/FIJO |
|-----------|---------|-------|----------|------------|
| `max_concurrent_jobs` | 3 | 1-16 | Máximo de jobs ejecutándose simultáneamente | CONFIG |
| `job_timeout` | 3600 | 60-86400 | Timeout en segundos para un job (0 = sin límite) | CONFIG |
| `job_queue_size` | 1000 | 10-10000 | Máximo de jobs encolados antes de rechazar nuevos | CONFIG |
| `progress_interval` | 5 | 1-60 | Segundos entre actualizaciones de progreso | CONFIG |
| `result_retention_days` | 365 | 1-3650 | Cuántos días guardar job results antes de archivar | CONFIG |
| `persist_to_disk` | true | true/false | Si true, jobs se guardan en SQLite para recuperación en crash | FIJO |

---

## Ciclo de Vida de la Feature

### Entrada
- **Quién llama:** Features costosas (generate, validate, manage, incubate, feedback) a través de sus puertos públicos.
- **Qué recibe:** Solicitud con tipo de operación, parámetros seguros, actor/origen, `operation_id`, clave de idempotencia y expectativa de versión cuando aplique.

### Proceso

#### Fase 1: Disparo (Submit)
1. Sistema crea registro en tabla `jobs` ligado a `operation_id`: identificador, actor/origen, tipo, parámetros seguros, estado `QUEUED` y timestamp.
2. Job entra a `tokio.Queue` (in-memory)
3. Sistema retorna identidad estable de operación y job inmediatamente, sin texto de interfaz.

#### Fase 2: Monitoreo (Stream recuperable)
1. Worker thread toma job de queue cuando hay capacity
2. Worker cambia estado a RUNNING, inicializa progreso=0
3. Clientes autorizados reciben snapshot y cambios por stream gRPC; pueden reanudar desde posición conocida.
4. Cada cambio entrega estado, progreso (0-100%) y estimación; la interfaz localiza los códigos.
5. Worker actualiza progreso cada `progress_interval` segundos

#### Fase 3: Recuperación (Fetch)
1. Job completa (exitoso o con error)
2. Estado cambia a COMPLETED o FAILED
3. Resultado se guarda en tabla `job_results` (append-only)
4. Cliente autorizado consulta el resultado por el puerto público de la Feature dueña.
5. Retorna resultado inmutable + timestamp de finalización

### Salida
- **Produce:** UUID de job, estado, progreso, resultado eventual

### Contextos de Uso
- **Generate:** Generar N candidatas con algoritmo evolutivo (minutos a horas)
- **Validate:** Backtesting robusto WFA+MC (minutos)
- **Manage:** Optimización de portafolio (minutos)
- **Incubate:** Forward testing histórico (minutos)
- **Feedback:** Análisis diario de anomalías (segundos a minutos)

---

## Tareas (TTRs)

### **TTR-ASYNC-EXECUTOR-001: Implementar Job Queue con Tokio + SQLite**

**Qué hace:** Crear JobExecutor que maneja jobs en memoria (rápido) + SQLite (durabilidad).

**Entrada:**
- Request de job: tipo, parámetros seguros, actor/origen, `operation_id` y clave de idempotencia.

**Salida:**
- UUID único del job
- Job guardado en tabla `jobs` y ligado a la operación durable
- Job encolado en `tokio.Queue`

**Reglas de Negocio:**
- UUID debe ser único e irrepetible (usar uuid.uuid4())
- Job, transición de operación y auditoría se guardan antes de retornar sus identidades (garantía de persistencia)
- Queue en memoria para velocidad, disco para durabilidad

**Precondiciones:**
- JobExecutor inicializado
- Tabla `jobs` creada en SQLite

**Postcondiciones:**
- UUID retornado
- Job en cola esperando ejecución

---

### **TTR-ASYNC-EXECUTOR-002: Implementar Worker Pool (Ejecutores Paralelos)**

**Qué hace:** N workers que toman jobs de la queue y los ejecutan.

**Entrada:**
- Job de la queue
- Funciones callback que ejecutar (función que hace el trabajo real)

**Salida:**
- Job completado o con error
- Estado actualizado a COMPLETED/FAILED

**Reglas de Negocio:**
- Máximo `max_concurrent_jobs` workers corriendo simultáneamente
- Si queue está vacía, workers esperan sin consumir CPU
- Si job tira excepción, se captura y guarda como FAILED

**Precondiciones:**
- JobExecutor inicializado
- `max_concurrent_jobs` configurado

**Postcondiciones:**
- Job ejecutado o fallido
- Estado reflejado en tabla `jobs`

---

### **TTR-ASYNC-EXECUTOR-003: Implementar Persistencia en SQLite**

**Qué hace:** Tabla `jobs` y tabla `job_results` que guardan jobs y resultados.

**Entrada:**
- Job object (id, `operation_id`, actor/origen, tipo, parámetros seguros, estado, progreso, timestamps)
- Result object (job_uuid, result_data, error_message, completed_at)

**Salida:**
- Rows en SQLite con datos persistidos

**Reglas de Negocio:**
- Tabla `jobs`: cada fila es un job, se actualiza conforme progresa (state, progress)
- Tabla `job_results`: append-only, nunca se modifica después de insertar
- Ambas tablas tienen timestamp para auditoría

**Precondiciones:**
- SQLite inicializado
- Migraciones ejecutadas (SQLx Migrator)

**Postcondiciones:**
- Datos durables en disco
- Recuperables ante crash

---

### **TTR-ASYNC-EXECUTOR-004: Implementar Recuperación en Startup**

**Qué hace:** Cuando sistema arranca, recuperar jobs incompletos de SQLite.

**Entrada:**
- Tabla `jobs` con registros en estado QUEUED o RUNNING

**Salida:**
- Jobs reencolados en `tokio.Queue`
- Estado actualizado a QUEUED (no RUNNING, porque no sabemos si completó)

**Reglas de Negocio:**
- QUEUED jobs se recuperan directamente
- RUNNING jobs: cambiar a QUEUED (asumir que no completó, porque si completó estaría en COMPLETED)
- Registrar en auditoría un código de recuperación y la referencia del job, sin texto de interfaz ni parámetros sensibles.

**Precondiciones:**
- SQLite disponible
- Sistema arrancando

**Postcondiciones:**
- Jobs recuperados y esperando ejecución
- Auditoría registrada

---

### **TTR-ASYNC-EXECUTOR-005: Implementar Progreso y Estimación de Tiempo**

**Qué hace:** Permitir que job report progreso, sistema estima tiempo restante.

**Entrada:**
- Job en ejecución
- Callback de progreso: `on_progress(percent: float, elapsed_seconds: float)`

**Salida:**
- Estado durable actualizado con progreso y estimación; evento ordenado para el stream recuperable.

**Reglas de Negocio:**
- Progreso es 0-100%
- Estimación = (elapsed_time / progress) × (100 - progress)
- Se actualiza cada `progress_interval` segundos

**Precondiciones:**
- Job ejecutándose

**Postcondiciones:**
- Cliente autorizado puede reconstruir progreso en cualquier momento

---

### **TTR-ASYNC-EXECUTOR-006: Implementar Cancellación de Jobs**

**Qué hace:** Usuario puede cancelar un job en QUEUED o RUNNING.

**Entrada:**
- Job UUID
- Señal de cancelación

**Salida:**
- Job marcado como CANCELLED
- Worker detiene ejecución
- Resultado parcial se descarta

**Reglas de Negocio:**
- QUEUED jobs se cancelan inmediatamente (no se ejecutan)
- RUNNING jobs se cancelan con cancel token (worker verifica periódicamente)
- Una vez CANCELLED, no se puede reanudar

**Precondiciones:**
- Job en estado QUEUED o RUNNING

**Postcondiciones:**
- Job en estado CANCELLED
- No hay resultado (o resultado parcial se ignora)

---

### **TTR-ASYNC-EXECUTOR-007: Integrar con Features Costosas (generate, validate, manage, etc.)**

**Qué hace:** Cada Feature costosa usa JobExecutor en lugar de ejecutar sincrónico.

**Entrada:**
- Feature que necesita ejecutar operación costosa

**Salida:**
- Job UUID retornado al usuario

**Reglas de Negocio:**
- La Feature no ejecuta lógica costosa en la llamada inicial; la delega a job
- La Feature recibe progreso ligado a `operation_id` y publica estado durable
- El job puede cancelarse por un actor todavía autorizado, respetando sus fronteras seguras

**Precondiciones:**
- JobExecutor inicializado
- Feature integrada al patrón asíncrono

**Postcondiciones:**
- Usuario puede monitorear progreso
- Interfaz responsiva

---

## Puertos de Integración (ADR-0137)

> Obligatorio en toda feature. Define los tipos de dato que la feature acepta (inputs) y produce (outputs).
> Los IDs de tipo deben pertenecer al catálogo de ADR-0137. Un puerto sin tipo declarado es inválido en el Canvas [Forge/Reactor].

| Puerto | ID de tipo | Dirección | Cardinalidad | Descripción |
|---|---|---|---|---|
| `job_request_in` | `(interno)` | Input | 1 | Solicitud de encolar: `operation_kind`, parámetros seguros, actor/origen, `operation_id`, idempotencia y versión esperada cuando aplique. No es un tipo de canvas; es la firma de la API del executor. |
| `job_out` | `Job` | Output | 1..N | Job con su ciclo de vida completo (PENDING → RUNNING → DONE / FAILED / CANCELLED). Representa la unidad de trabajo durable gestionada por el executor. |

> **Cardinalidad:** `1` = exactamente uno · `0..1` = opcional · `0..N` = múltiple · `1..N` = al menos uno.
> **Puerto interno (ADR-0137 Enmienda 2026-06-24):** `job_request_in` es un puerto interno — plomería `textLabel`, payload específico de esta feature, nunca cableable en el canvas. No requiere tipo de catálogo.

---

## Gobernanza y Estándares (Fijos)

- **Local-First (ADR-0016):** 100% Local. La cola de trabajos y sus resultados se gestionan íntegramente en el hardware local.
- **Inundación de Fundaciones (ADR-0020):** 
    - Aplica el **Grupo I (universal)** + solo los campos de su Perfil Técnico listados abajo (Filtro de Relevancia, ADR-0020); NO el catálogo completo de 25 campos.
    - Metadatos de concurrencia e integridad: `process_id` (Worker ID), `session_id`, `node_id` (Hardware Fingerprint), `audit_chain_hash`, `logic_hash` (Executor version), `event_sequence_id`.
    - Soberanía: `owner_id`, `access_token_id`.


---

## Dependencias y Bloqueantes

- **Requiere:** ADR-0011 (patrón definido)
- **Requiere:** SQLite con WAL (persistencia)
- **Requiere:** SQLx Migrator (migraciones para tablas jobs/results)
- **Requiere:** Tokio (runtime async de Rust)
- **Habilita:** Operaciones costosas sin bloquear UI
- **Habilita:** ADR-0012 (multi-pipeline si se usa job executor para pipelines)

---

## Referencias

- `ADR.md` → ADR-0011: Operaciones Asincrónicas
- `ADR.md` → ADR-0012: Arquitectura Multi-Pipeline (usa job executor)
- `features/infrastructure-setup/` → SQLite + SQLx Migrator
