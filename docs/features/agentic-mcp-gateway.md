> 🟡 **Parcial** 2026-06-20 · TTR-001 (parcial), TTR-002, TTR-003 (storage), TTR-005 implementados · TTR-001 UI herramientas reales (EPIC-1+), TTR-004 (SaaS terms), TTR-006 (proyección), TTR-007 (presencia), TTR-008 (operación/idempotencia), TTR-009 (reanudación gRPC), TTR-010 (delegación/revocación), TTR-011 (trabajos asíncronos), TTR-012 (filtrado de proyección) y TTR-013 (efectos externos) pendientes · Orden [STORY-010](../execution/STORY-010-agentic-mcp-gateway.md)

# Agentic MCP Gateway (Cabina Dual)

**Carpeta:** `./features/agentic-mcp-gateway/`
**Estado:** En Diseño
**Última actualización:** 2026-06-17
**Decisión Arquitectónica Asociada:** ADR-0123 (Cabina Dual) · ADR-0166 (Operaciones Observables y Recuperables)

## 1. ¿Qué es esta feature?
Es la puerta de control que expone las `public_interface` de las Features a un agente LLM externo conectado vía MCP, con la misma autoridad de dominio que Tauri/React o el cliente Headless gRPC. La autorización se resuelve por puerto público de Feature, rol, recurso y delegación vigente; el pipeline aporta contexto de riesgo, pero no es dueño de la capacidad.

**Problema que resuelve:** Hoy, automatizar el trabajo tedioso de descubrimiento (lanzar barridos, iterar diseños de estrategia, revisar reportes) exige que el usuario opere manualmente la interfaz paso a paso. El usuario quiere poder delegar ese trabajo a un agente — su propio "copiloto de trading" — sin que eso abra una puerta sin control hacia el capital real.

**Por qué la necesitamos:** "Software por resultados": el usuario decide, tarea por tarea, si la hace él desde la interfaz o la delega a un agente. Ninguna de las dos vías sustituye a la otra — es Cabina Dual, no migración a un solo canal.

## Narrativa de Producto

**Capacidad:** `CAP-09` — Una interfaz donde se ve el razonamiento

**Historia de usuario:** Como usuario, quiero poder delegar el trabajo repetitivo de investigación a un agente automatizado sin que eso le abra la puerta a operar mi dinero real sin mi autorización explícita, para ganar tiempo sin perder control.

**El problema (técnico):** automatizar tareas de descubrimiento —lanzar barridos, iterar diseños, revisar reportes— hoy exige operar la interfaz paso a paso a mano. Conectar un agente automatizado directamente a todo el sistema, sin distinción de riesgo, significaría que ese agente podría tocar dinero real con la misma facilidad con la que revisa un reporte.

**La solución (técnica):** una puerta de control que expone al agente las mismas operaciones que usa la interfaz normal, con permisos por puerto de Feature, recurso y riesgo: las fases de investigación se conceden según el rol; tocar capital real exige una delegación explícita, revocable y con vencimiento, creada por el propietario desde su interfaz. Cada mutación aceptada sigue la misma transición de dominio que una acción humana, recibe identidad e idempotencia, se confirma junto con auditoría y se proyecta en Tauri/React mediante el estado canónico del Core, sin simular clics ni controlar el DOM. El usuario puede cortar la conexión del agente en cualquier momento sin afectar la aplicación.

**Explicado sin tecnicismos:** es como darle una llave de la casa a un asistente para que haga las compras y organice el clóset, pero la caja fuerte tiene una segunda llave que solo tú tienes, y que hay que girar a propósito cada vez que quieres que él también pueda tocarla.

El asistente puede investigar, probar y preparar todo lo que quieras sin pedirte permiso a cada rato. Pero si intenta tocar tu dinero real, la puerta no se abre —a menos que tú, y solo tú, hayas girado esa segunda llave—. Y en cualquier momento le puedes quitar la llave de la casa sin que nada se rompa. Además, en la pantalla y en la libreta queda claro qué cambió, quién lo hizo y si fue el asistente o fuiste tú.

## 2. Comportamientos Observables
- [ ] El usuario conecta un agente LLM a Drasus Engine vía MCP. El agente ve las herramientas de `ingest`, `generate`, `validate`, `incubate` y `feedback` que su rol y alcance vigente le permiten, sin pedir una nueva aprobación humana por cada llamada.
- [ ] El agente pide a `manage` rebalancear un portafolio con `institutional_tag = Demo`. La operación se ejecuta sin fricción.
- [ ] El agente pide a `manage` rebalancear un portafolio con `institutional_tag = Live` sin delegación vigente. La llamada se rechaza con el mismo código de denegación que usa el Gateway SaaS para un token sin permisos.
- [ ] El agente intenta invocar cualquier frontera de `execute` o `withdraw` sin una delegación vigente que cubra ese puerto y objeto. La llamada se rechaza incondicionalmente.
- [ ] El propietario concede una delegación de producción desde la interfaz local, limitada por agente, Feature, puerto, objeto y vencimiento. Solo las llamadas dentro de ese alcance se conceden.
- [ ] En modo SaaS, el propietario intenta conceder una delegación sin haber aceptado los términos de riesgo de delegar producción a un LLM. El sistema bloquea la concesión y muestra el estado de aceptación pendiente mediante una clave localizada.
- [ ] El propietario cierra la conexión MCP en cualquier momento. El agente pierde acceso de inmediato; la interfaz Tauri/React sigue operando con control 100% manual, sin degradación.
- [ ] Cada acción ejecutada por el agente queda registrada en el log de auditoría con un origen identificado como "agente", distinguible de una acción humana en el mismo registro.
- [ ] Una mutación concedida vía MCP cambia el estado visible de la interfaz Tauri/React sin que el agente controle el mouse, el DOM o un componente visual.
- [ ] La interfaz muestra el resultado, el progreso cuando aplique, el error o el rechazo, junto con el origen agente y la operación relacionada.
- [ ] Si la interfaz estaba desconectada, al reconectar o recargar reconstruye el estado canónico y el historial de la mutación agéntica.
- [ ] Mientras una acción del agente está conectada, pendiente, ejecutándose, progresando, completada, fallida o cancelada, la interfaz muestra su presencia, el agente que la inició, el objeto afectado y el estado actual.
- [ ] La actividad aparece en una superficie global y también sobre el objeto afectado mediante códigos de estado localizados, nunca con frases construidas por el Core ni texto crudo del agente.
- [ ] Una repetición MCP con la misma clave de idempotencia devuelve el resultado de la operación original sin repetir la mutación.
- [ ] Si la versión del objeto cambió desde que el agente leyó su estado, la llamada se rechaza como conflicto y muestra el estado confirmado más reciente.
- [ ] Si el propietario revoca el rol o la delegación durante una operación, ningún compromiso posterior modifica el dominio; la actividad muestra la revocación y conserva la evidencia ya obtenida.
- [ ] Si la operación inicia un job, la interfaz y el agente observan la misma identidad, progreso, cancelación, recuperación y resultado.
- [ ] La interfaz reconecta desde una vista inicial y una posición recuperable; no duplica actividad ni omite una mutación comprometida.

## 3. Restricciones
- PROHIBIDO crear un camino de validación o de lógica de negocio exclusivo para el canal MCP: consume la misma `public_interface` que ya usan Tauri/React y el gRPC Headless (FCIS, Soberanía de Datos).
- La autoridad pertenece a la Feature que expone el puerto; los módulos solo agrupan y orquestan la composición del pipeline, no son dueños de la lógica ni de sus entidades.
- NUNCA una conexión MCP nace con permiso sobre `execute` o `withdraw` por defecto.
- NUNCA se concede una llamada de `manage` sobre un portafolio `institutional_tag = Live` sin delegación vigente y acotada.
- NUNCA se concede una delegación de producción en modo SaaS sin que el usuario haya aceptado expresamente los términos de riesgo correspondientes.
- NUNCA una acción ejecutada por el agente queda indistinguible de una acción humana en el registro de auditoría.
- El cierre del canal MCP por el propietario es inmediato y no requiere reinicio de la interfaz local.
- NUNCA el agente controla la interfaz mediante mouse, DOM, selectores o eventos sintéticos de componentes.
- NUNCA una mutación aceptada vía MCP queda aplicada en el Core sin una proyección recuperable para la interfaz.
- NUNCA la interfaz calcula reglas de negocio o crea una agregación exclusiva para hacer visible una acción agéntica.
- NUNCA cada pantalla implementa su propia lectura de presencia o actividad; la interfaz consume una proyección común del Core.
- NUNCA un permiso se concede solo porque un preset de pipeline lo contiene; la unidad de capacidad es el puerto público de la Feature y el objeto objetivo.
- NUNCA el listado de herramientas MCP o una actividad visible se interpreta como autoridad: la llamada y la lectura se validan contra el alcance vigente al momento de servirlas.
- NUNCA una delegación de producción es global o sin vencimiento: queda limitada por cuenta, agente, capacidad y alcance de recurso.
- NUNCA un evento de actividad se publica antes de confirmar la transacción de dominio y auditoría.
- NUNCA un texto, parámetro sensible, secreto o instrucción no confiable del agente se replica crudo a la UI, el stream o la auditoría.
- NUNCA un observador sin permiso de lectura sobre el objeto recibe su identidad, metadatos o presencia agéntica.

## 4. Parámetros Configurables
| Parámetro | Default | Rango | Qué hace | FIJO/CONFIG |
|---|---|---|---|---|
| PRODUCTION_DELEGATION_INDICATOR | sin delegaciones | con/sin delegaciones | Indicador visual agregado de delegaciones de producción vigentes; no sustituye su alcance por cuenta, agente, Feature, objeto y vencimiento | CONFIG |
| TERMS_ACCEPTED_SAAS | false | true/false | Aceptación expresa de los términos de riesgo en modo SaaS; precondición para conceder delegaciones de producción en esa cuenta | [FIJO] una vez aceptado, no se reescribe — se revoca por separado |
| MCP_RATE_LIMIT | 120 req/min | 10-1000 | Límite de llamadas del agente por ventana temporal, reutiliza el mecanismo del Gateway SaaS | CONFIG |
| AGENT_SESSION_EXPIRY | 60 minutos | 5-1440 minutos | Vigencia de la sesión MCP antes de requerir reautenticación | CONFIG |
| AGENT_PRESENCE_TTL | 60 segundos | 15-300 segundos | Tiempo máximo sin renovación antes de presentar al agente como estado desconocido | CONFIG |
| DELEGATION_MAX_TTL | 15 minutos | 1-60 minutos | Duración máxima de una delegación de producción antes de requerir renovación del propietario | CONFIG |

## 5. Estructura Interna (FCIS — ADR-0002)
- **Core (Lógica Pura):** Evaluador de permisos: dado el operador, el puerto público de la Feature, el objeto, su versión esperada, el riesgo del recurso, la delegación vigente y el contexto de pipeline opcional, decide conceder, denegar o reportar conflicto. Función pura, sin IO.
- **Shell (Infraestructura):** Servidor MCP que traduce las herramientas invocables hacia la `public_interface` real de cada Feature; persiste identidad de operación, delegaciones, actividad y auditoría; reutiliza autenticación y roles. Antes de cualquier compromiso, reevalúa autorización y versión.
- **Proyección de cliente:** La Shell confirma transición, auditoría y actividad durable en una sola transacción. Después publica el evento por gRPC desde el registro comprometido. Tauri/React se reanuda desde una vista inicial y una posición monotónica; conserva solo estado de presentación y selección.
- **Actividad compartida:** El Core representa estado durable de operación y presencia efímera de sesión. La interfaz muestra progreso, procedencia y conflicto sin introducir lógica de negocio ni interpretar texto del agente.
- **Frontera Pública:** Puerto de evaluación de permisos que cualquier Feature puede consultar antes de aceptar una llamada proveniente del canal MCP.

## 6. Ciclo de Vida de la Feature — Agentic MCP Gateway

### Entrada
- Llamada entrante del agente vía MCP: identidad del agente y sesión, Feature, puerto público, objeto, parámetros permitidos, clave de idempotencia, huella canónica de solicitud y versión esperada.
- Delegación vigente, rol del operador, términos aceptados y contexto de pipeline cuando aplique.
- `institutional_tag` y demás condiciones de riesgo del objeto afectado.

### Proceso
- Asigna o recupera la identidad de operación según clave y huella de idempotencia; rechaza reutilizar la misma clave para una solicitud distinta.
- Evalúa rol por puerto de Feature, alcance de objeto, `institutional_tag`, delegación y contexto de pipeline.
- Si la versión esperada no coincide, detiene la mutación y entrega conflicto explícito.
- Confirma, en una transacción, la mutación permitida, la auditoría y el evento de actividad; solo entonces se publica por gRPC.
- Si inicia un job, vincula el job a la operación y publica sus transiciones sin crear un segundo historial.
- Reevalúa autorización antes de cada compromiso posterior y marca la presencia sin renovación como desconocida.

### Salida
- La llamada se enruta hacia la Feature responsable y regresa el mismo `operation_id` al agente, con resultado, rechazo o conflicto codificados.
- Registro inmutable de la decisión de permiso y de cada transición de actividad.
- Si la llamada muta estado, el Core confirma una transición persistida y la publica para la interfaz junto con procedencia, posición de recuperación y referencia al objeto. La interfaz puede reconstruirla después de una desconexión o recarga.
- El ciclo de actividad termina en estado localizado visible, sin requerir control del mouse o del DOM.

### Contextos de Uso
**Contexto 1: Descubrimiento delegado**
- El usuario delega a su agente todo el ciclo de `ingest` → `feedback`, dedicando su atención solo a las decisiones que de verdad quiere tomar él.
**Contexto 2: Producción supervisada**
- El propietario concede una delegación temporal y acotada para que el agente ejecute una tarea puntual sobre producción; esta vence o se revoca al terminar.

## 7. Tareas (TTRs)

### TTR-001: Servidor MCP expone las fronteras públicas de las Features como herramientas
* **¿Cuál es el problema?** Un agente LLM no puede invocar directamente la `public_interface` interna de Rust; necesita un protocolo estándar de herramientas.
* **¿Qué tiene que pasar?** El agente, al conectarse vía MCP, ve listadas las operaciones disponibles de cada Feature según la matriz de permisos vigente en ese momento.
* **¿Cómo sé que está hecho?**
  - [ ] Conecto un agente MCP y veo las herramientas de `ingest`/`generate`/`validate`/`incubate`/`feedback` disponibles de inmediato.
  - [ ] Las herramientas de `execute`/`withdraw` no aparecen listadas sin una delegación vigente que cubra su puerto y alcance.
* **¿Qué no puede pasar?** El servidor MCP no puede exponer una operación que no exista ya en la `public_interface` real de la Feature responsable.

### TTR-002: Evaluador de permisos por puerto de Feature, objeto y riesgo
* **¿Cuál es el problema?** Un preset de pipeline no es dueño de una Feature. Hay que decidir en cada llamada si el agente tiene permiso sobre el puerto y objeto concretos, sin duplicar lógica de negocio.
* **¿Qué tiene que pasar?** Antes de enrutar cualquier llamada MCP, el sistema evalúa el rol del operador, el puerto de Feature, el objeto, su riesgo y el contexto de pipeline opcional.
* **¿Cómo sé que está hecho?**
  - [ ] Una llamada a `manage` sobre un portafolio Demo se concede si el rol cubre ese puerto y objeto.
  - [ ] La misma llamada sobre un portafolio Live se rechaza sin delegación vigente y acotada.
* **¿Qué no puede pasar?** No puede existir una ruta de Feature que evite esta evaluación ni una capacidad concedida solo por pertenecer a un pipeline.

### TTR-003: Delegación acotada de producción
* **¿Cuál es el problema?** El propietario necesita delegar autoridad de producción sin habilitar globalmente a todos los agentes ni dejar el permiso activo sin límite.
* **¿Qué tiene que pasar?** La interfaz presenta una delegación explícita y reversible, limitada por cuenta, agente, puerto, objeto y vencimiento; puede representarse con un interruptor, pero ese control no amplía el alcance persistido.
* **¿Cómo sé que está hecho?**
  - [ ] Creo una delegación para un agente, un puerto y un objeto; una llamada dentro del alcance se concede.
  - [ ] El mismo agente intenta otro objeto o puerto y se rechaza.
  - [ ] Revoco o dejo vencer la delegación y una llamada posterior se rechaza de inmediato.
* **¿Qué no puede pasar?** El agente no puede crear, ensanchar ni renovar su propia delegación.

### TTR-004: Aceptación de términos de riesgo en modo SaaS
* **¿Cuál es el problema?** En modo SaaS, delegar producción a un LLM no puede ser una decisión tomada a la ligera ni revertible solo por el operador de la plataforma.
* **¿Qué tiene que pasar?** La creación de una delegación de producción no está disponible para una cuenta SaaS hasta que el usuario acepta expresamente el texto de riesgo correspondiente.
* **¿Cómo sé que está hecho?**
  - [ ] Intento crear una delegación sin haber aceptado los términos y el sistema la bloquea, mostrando el estado localizado pendiente.
  - [ ] Tras aceptar, la creación de delegación queda disponible para esa cuenta.
* **¿Qué no puede pasar?** No puede inferirse una aceptación implícita (ej. por seguir usando la plataforma).

### TTR-005: Auditoría de operación y procedencia
* **¿Cuál es el problema?** Sin trazabilidad, una acción del agente y una acción humana serían indistinguibles en el historial.
* **¿Qué tiene que pasar?** Cada entrada originada por MCP queda ligada a identidad de agente, sesión, operación, Feature, puerto, objeto, correlación y decisión de autorización.
* **¿Cómo sé que está hecho?**
  - [ ] Reviso el log de auditoría y cada acción del agente está marcada con `operation_id`, identidad de agente y `agent_session_id`.
* **¿Qué no puede pasar?** No puede existir una entrada de auditoría sin procedencia identificada.

### TTR-006: Proyección transaccional de mutaciones agénticas en la interfaz
* **¿Cuál es el problema?** El agente puede ejecutar una operación válida contra el Core, pero el usuario puede no ver el cambio, su resultado o su procedencia en la interfaz.
* **¿Qué tiene que pasar?** Toda mutación concedida vía MCP usa la misma transición de dominio que la interfaz humana. Su cambio, auditoría y actividad durable se confirman en una sola transacción antes de publicarse por el canal de estado. Tauri/React muestra resultado, progreso, error, rechazo o conflicto con origen agéntico y reconstruye desde el Core.
* **¿Cómo sé que está hecho?**
  - [ ] Ejecuto una mutación desde MCP y observo el cambio correspondiente en Tauri/React sin interacción sintética.
  - [ ] Desconecto la interfaz durante la mutación, la reconecto y observo el estado y el historial correctos.
  - [ ] La misma operación iniciada por una persona y por un agente usa la misma validación, persistencia y representación de estado.
* **¿Qué no puede pasar?** El canal MCP no puede tener una ruta de negocio paralela, depender de control del DOM, publicar antes de confirmar ni dejar una mutación aplicada que la interfaz no pueda reconstruir.

### TTR-007: Presencia y actividad agéntica compartida
* **¿Cuál es el problema?** El usuario puede ver el resultado final de una operación, pero no sabe si el agente está conectado, trabajando, detenido o modificando un objeto en ese momento.
* **¿Qué tiene que pasar?** El Core publica el ciclo durable de cada operación agéntica y la presencia efímera de su sesión. La interfaz muestra la actividad en una superficie global y en el objeto afectado, con identidad presentada del agente, operación, estado, progreso y resultado. La presencia vence si no se renueva; la actividad se conserva para reconstrucción.
* **¿Cómo sé que está hecho?**
  - [ ] Inicio una operación por MCP y veo el estado localizado de inicio antes del resultado.
  - [ ] El objeto afectado muestra actividad agéntica mientras la operación está activa y conserva la procedencia después de terminar.
  - [ ] Desconecto y reconecto la interfaz y la actividad queda coherente con el estado del Core.
* **¿Qué no puede pasar?** La presencia no puede depender de eventos de mouse, del DOM, de una pantalla particular ni de un WebSocket separado del contrato de estado del Core. Tampoco puede permanecer activa después de que su concesión expire.

### TTR-008: Identidad, idempotencia y concurrencia de operación
* **¿Cuál es el problema?** MCP puede repetir una llamada y un usuario o segundo agente puede cambiar el mismo objeto mientras la primera operación sigue viva.
* **¿Qué tiene que pasar?** Toda llamada recibe o recupera un `operation_id` por clave y huella de idempotencia. Las mutaciones sobre estado mutable comprueban versión esperada y devuelven conflicto explícito si cambió.
* **¿Cómo sé que está hecho?**
  - [ ] Repito la misma llamada y recibo el mismo `operation_id` y resultado, sin segunda mutación.
  - [ ] Reutilizo la misma clave para otra solicitud y recibo rechazo explícito, sin recuperar ni alterar la operación original.
  - [ ] Cambio el objeto desde otra sesión y la llamada con versión previa devuelve conflicto con referencia al estado vigente.
* **¿Qué no puede pasar?** Una repetición o carrera no puede producir cambios duplicados ni resolución silenciosa de último escritor.

### TTR-009: Stream gRPC recuperable de operaciones
* **¿Cuál es el problema?** Un stream en vivo por sí solo puede perder, duplicar o desordenar actividad cuando la interfaz se desconecta.
* **¿Qué tiene que pasar?** El stream entrega vista inicial y continúa desde una posición monotónica dentro de su alcance de dueño, nodo y réplica autorizada. Detecta huecos, tolera duplicados y conserva orden por operación y objeto dentro de ese alcance.
* **¿Cómo sé que está hecho?**
  - [ ] Desconecto la interfaz durante una operación, reanudo desde la última posición y veo exactamente las transiciones comprometidas que faltaban.
  - [ ] Un consumidor lento recibe una reconstrucción coherente en vez de congelar el Core.
  - [ ] Un cursor de otro alcance se rechaza o reinicia con snapshot; nunca se compara como si fuera la misma secuencia.
* **¿Qué no puede pasar?** El stream no puede ser la única copia de actividad ni convertir una notificación efímera en prueba de cambio confirmado.

### TTR-010: Revocación y autorización al comprometer
* **¿Cuál es el problema?** Una llamada puede ser válida al inicio y perder autorización antes de terminar.
* **¿Qué tiene que pasar?** El Gateway revalida rol, delegación y riesgo antes de cada compromiso de estado. Revocar bloquea compromisos posteriores y deja actividad durable de la decisión.
* **¿Cómo sé que está hecho?**
  - [ ] Revoco una delegación mientras corre una operación; ningún cambio posterior se confirma.
  - [ ] Un cálculo que ya produjo evidencia conserva su resultado, pero no promueve ni toca producción sin autorización vigente.
* **¿Qué no puede pasar?** Cerrar sesión, revocar rol o vencer delegación no puede dejar una mutación posterior autorizada por una decisión vieja.

### TTR-011: Integración de operaciones con trabajos asíncronos
* **¿Cuál es el problema?** Un trabajo largo no puede depender de que la petición MCP siga abierta ni mantener un historial separado de la operación que lo creó.
* **¿Qué tiene que pasar?** El job conserva el `operation_id` de origen y publica sus estados, progreso, cancelación, recuperación y resultado como actividad de esa operación.
* **¿Cómo sé que está hecho?**
  - [ ] Inicio un job por MCP, cierro la conexión y al reconectar recupero el mismo progreso y resultado por `operation_id`.
  - [ ] Cancelo o revoco la operación y el job aplica el límite seguro de cancelación definido por su Feature.
* **¿Qué no puede pasar?** Un job originado por agente no puede quedar huérfano, perder procedencia ni promover resultados después de una revocación.
* **¿Qué no puede pasar?** Un evento tardío o duplicado del job no puede reabrir una operación terminal ni sustituir su resultado confirmado.

### TTR-012: Filtrado de proyección por alcance de lectura
* **¿Cuál es el problema?** La presencia compartida puede filtrar que existe una operación, un agente o un objeto a un operador que no puede consultarlo.
* **¿Qué tiene que pasar?** Antes de servir snapshot, evento o presencia, la proyección verifica el alcance de lectura vigente del observador sobre la operación y su objeto. Una revocación corta entregas posteriores y la interfaz reconstruye su vista autorizada.
* **¿Cómo sé que está hecho?**
  - [ ] Un operador sin acceso al portafolio no ve su actividad, identidad de agente ni identificadores en la superficie global.
  - [ ] Al revocar lectura durante un stream, la operación desaparece de la vista del observador tras la siguiente reconstrucción autorizada.
* **¿Qué no puede pasar?** Una marca de presencia o metadato de actividad no puede revelar un recurso fuera del alcance del observador.

### TTR-013: Efectos externos idempotentes y reconciliables
* **¿Cuál es el problema?** Un reintento de MCP puede llegar a una Feature que envía una orden, una solicitud remota u otro efecto fuera del Core; confirmar éxito antes de que el destino responda puede duplicar el efecto o presentar un resultado ficticio.
* **¿Qué tiene que pasar?** La Feature que cruza una frontera externa propaga la identidad idempotente o persiste una intención reconciliable antes de emitir el efecto. Si el destino no confirma, la operación queda en un estado explícito de reconciliación y conserva la procedencia.
* **¿Cómo sé que está hecho?**
  - [ ] Repito una solicitud después de perder la respuesta y el destino recibe como máximo un efecto.
  - [ ] Simulo ausencia de confirmación del destino y la interfaz muestra reconciliación, no éxito.
* **¿Qué no puede pasar?** Un reintento, una desconexión o una respuesta tardía no pueden duplicar un efecto externo ni cambiar a éxito una operación sin confirmación.

## Puertos de Integración (ADR-0137)

> Obligatorio en toda feature. Define los tipos de dato que la feature acepta (inputs) y produce (outputs).
> Los IDs de tipo deben pertenecer al catálogo de ADR-0137. Un puerto sin tipo declarado es inválido en el Canvas [Forge/Reactor].

| Puerto | ID de tipo | Dirección | Cardinalidad | Descripción |
|---|---|---|---|---|
| `mcp_call_in` | `(interno)` | Input | 1 | Llamada MCP: identidad y sesión del agente, Feature, puerto público, objeto, parámetros permitidos, contexto de pipeline opcional, clave y huella de idempotencia, versión esperada. No es cableable en el canvas. |
| `permission_outcome_out` | `(interno)` | Output | 1 | Decisión codificada: concedido, denegado o conflicto, con `operation_id`, razón estable y posición de actividad. Es respuesta síncrona MCP, no puerto de canvas. |
| `audit_event_out` | `AuditEvent` | Output | `1..N` | Evento auditado que registra decisión, transición de operación, revocación y procedencia sin copiar secretos ni parámetros sensibles. |
> **Cardinalidad:** `1` = exactamente uno · `0..1` = opcional · `0..N` = múltiple · `1..N` = al menos uno.
> **Puertos internos (ADR-0137 Enmienda 2026-06-24):** `mcp_call_in` y `permission_outcome_out` son puertos internos — plomería `textLabel`, payload específico del protocolo MCP de esta feature, nunca cableables en el canvas. No requieren tipo de catálogo.

## 8. Gobernanza y Estándares (Fijos)
- **Local-First (ADR-0016):** El canal MCP corre contra el mismo Core local; en modo SaaS hereda las garantías de `saas-gateway`.
- **Inundación de Fundaciones (ADR-0020): Perfil D (Ops / Auditoría)** — Identidad (I) + Soberanía (II) + Hardware (IV).

  | Categoría | Campo | Descripción |
  | :--- | :--- | :--- |
  | **I. Identidad** | `id` | Identificador único de la decisión de permiso |
  | | `created_at` | Timestamp de la llamada |
  | | `updated_at` | Timestamp de última modificación del registro |
  | | `audit_hash` | Hash de auditoría de la llamada (pipeline + parámetros + resultado) |
  | | `audit_chain_hash` | Hash encadenado del historial de decisiones de permiso |
  | | `event_sequence_id` | Secuencia de recuperación (event-sourcing) |
  | **II. Soberanía** | `owner_id` | Propietario que concede o revoca delegaciones acotadas de producción |
  | | `institutional_tag` | Entorno del objeto afectado (Live / Demo) cuando aplica |
  | **IV. Hardware** | `node_id` | Host donde corre el Gateway MCP |
  | | `process_id` | Proceso de la sesión del agente |

  Campos propios de la feature: `operation_id` (acción correlacionable), `agent_identity_id` y `agent_session_id`, `feature_key`, `public_port`, referencia de objeto, `idempotency_key`, correlación, versión esperada, contexto de pipeline, decisión y delegación vigente al evaluar.

## Persistencia (Inundación de Fundamentos — ADR-0020)

| Categoría | Campo | Descripción |
| :--- | :--- | :--- |
| **I. Identidad** | `id` | Identificador de la decisión de permiso |
| | `created_at` | Timestamp de la llamada |
| | `updated_at` | Timestamp de última modificación |
| | `audit_hash` | Hash de auditoría de la llamada |
| | `audit_chain_hash` | Hash encadenado del historial |
| | `event_sequence_id` | Secuencia de recuperación |
| **II. Soberanía** | `owner_id` | Propietario de la delegación acotada |
| | `institutional_tag` | Entorno del objeto afectado (Live/Demo) |
| **IV. Hardware** | `node_id` | Host del Gateway MCP |
| | `process_id` | Proceso de la sesión del agente |
| **Dominio propio** | `operation_id` | Acción identificable a través de permiso, job, auditoría y actividad |
| | `agent_identity_id` / `agent_session_id` | Agente estable y sesión MCP que originó la llamada |
| | `feature_key` / `public_port` | Frontera concreta cuya capacidad se evalúa |
| | `target_entity_kind` / `target_entity_id` | Objeto afectado por la llamada |
| | `idempotency_key` / `expected_row_version` | Defensa contra repetición y conflicto |
| | `pipeline_context` | Contexto de riesgo opcional, nunca sustituto de la capacidad |
| | `permission_outcome` / `delegation_id` | Decisión y alcance de producción vigente al evaluar |

**Rastro de Evidencia:** Emite a `feedback` cada denegación y cada creación, vencimiento o revocación de delegación de producción, para detectar patrones de uso anómalo del canal agéntico.

## 9. Dependencias y Bloqueantes
**Consumido por:** toda Feature que expone un puerto público a humanos o agentes; los presets de pipeline solo aportan una agrupación de riesgo y conveniencia.
**Depende de:** [`operator-roles`](./operator-roles.md) (capacidad por puerto), [`async-job-executor`](./async-job-executor.md) (trabajos vinculados a operación), [`audit-log`](./audit-log.md) (evidencia atómica), [`saas-gateway`](../moonshots/saas-gateway.md) (autenticación y límite de tasa), [`remote-portfolio-access-protocol`](./remote-portfolio-access-protocol.md) (patrón de acceso de referencia).
**Bloqueantes:** Ninguno — las fronteras públicas de las Features ya existen; esta feature añade la capa de permisos sobre ellas.

## Contrato de Integración UI
**Superficie propia:** sección "Cabina Dual" en el Panel Operativo Fundacional, donde el propietario ve sesiones MCP, delegaciones acotadas, procedencia agente/humano y operaciones iniciadas por cada agente.
- SVF: el control de delegación crea o revoca una autorización real sobre un puerto y objeto; la sección muestra en vivo identidad del agente, `operation_id`, estado, conflicto o resultado confirmado. Tras recargar, delegaciones, mutaciones y actividad se reconstruyen desde el Core.
- La superficie verifica que el agente no controla la interfaz: la misma mutación se observa por el canal de estado, sin mouse, DOM ni eventos sintéticos.
- La actividad agéntica se presenta como presencia compartida: superficie global, marca sobre el objeto afectado, procedencia persistida y estado localizado. Si una Feature no tiene superficie propia, la representación genérica mantiene visible su actividad.
