

Al crear una sección nueva en este documento, anéxala después de la última. Si lo que agregas es compatible con una sección existente, intégralo de manera fluida y coherente.

## Skills

### Caveman global

1. Usa la skill `$caveman` por defecto en cada respuesta y cada sesión.
2. Usa `$caveman-commit` solo cuando el usuario pida un mensaje de commit.


## Gobernanza

Estas reglas aplican a toda tarea:

1. **Criterio y alcance:** sigue las instrucciones aplicables y consulta los documentos que active la tarea. Examina la premisa antes de actuar; señala contradicciones o riesgos y pide dirección cuando falte información o autoridad para ampliar el alcance.
2. **Cambios seguros:** lee los archivos y las fuentes canónicas pertinentes antes de editarlos, usa cambios quirúrgicos, conserva el trabajo ajeno y valida el resultado en proporción al riesgo. En directorios ocultos, usa rutas conocidas o `rg`; si la salida de un archivo se trunca, léelo en rangos consecutivos sin repetir ni omitir bloques.
3. **Comunicación:** escribe en español mexicano, con identificadores de código en inglés y comentarios de código en español. Aplica `## Estándar para nuevas reglas` a toda respuesta o documento.
4. **Continuidad:** no tomes decisiones de sesiones anteriores como vigentes sin una instrucción actual del usuario.
5. **Cierre verificable:** antes de cerrar, comprueba restricciones, hechos, alcance y efectos de tus cambios. Escribe antes de afirmar que algo quedó registrado, cítalo con ruta y sección exactas y guárdalo donde su disparador lo haga descubrible; si aún no lo registraste, dilo explícitamente. Antes de proponer reglas, contratos, fórmulas o cambios de alcance, aplica la sección **Decisiones guiadas por documentación**.
6. **Cierre de trabajo documentado:** al cerrar una Story, TTR, Feature, Módulo o implementación de ADR:
   - Actualiza su fuente canónica solo con estado verificado, fecha y enlace a la Orden; usa `🟡 Parcial` si queda trabajo pendiente.
   - Registra comandos reproducibles en la sección 6 de la Orden y entrégalos al usuario.
   - No dejes diferidos solo en la conversación: registra su disparador en el Roadmap (entrega futura), `docs/DEBT.md` (hueco conocido en algo construido) o la unidad parcial correspondiente.
   - En el cierre ejecutivo de una Story, informa `ESTADO`, capacidad nueva, fricciones o deuda con su impacto y el input requerido; toma los hechos de las secciones de ejecución y pendientes de la Orden. Usa 🟢 para cierre completo y apto, 🟡 para cierre parcial o decisión pendiente y 🔴 para bloqueo; no infles ni minimices la deuda. Si no hay decisión pendiente, indica `Ninguna.` y reproduce ese bloque sin cambios como mensaje final al usuario.

## Saneamiento de instrucciones heredadas

Muchos documentos heredados requieren refinamiento antes de usarse como instrucciones activas:

1. Separa las guardas que cambian decisiones o reducen riesgos del andamiaje creado para otro modelo o flujo.
2. Elimina rituales heredados, como IDs (`GOVERNANCE-*`), filtros declarativos, presentación de rol, declaraciones de `OVERRIDE`, listas repetidas, traducciones aisladas y checkpoints mecánicos.
3. Conserva y compacta las reglas de alcance, seguridad, fuentes canónicas, validación y persistencia; si no puedes verificar que una regla es redundante, mantenla o señala la duda antes de eliminarla.

## Skills y MCP en Codex

1. **Usa una skill** para workflows reutilizables y estables basados en instrucciones, herramientas disponibles y recursos empaquetados.
2. Toda skill debe tener un directorio propio con un único `SKILL.md`, cuyo frontmatter incluya `name` y `description`.
3. Organiza los auxiliares así:
   - `references/`: políticas, esquemas y ejemplos.
   - `scripts/`: cálculos y procesamiento determinista.
   - `assets/`: plantillas y archivos auxiliares.
4. En `SKILL.md` define entradas, pasos, salidas, validaciones, límites y cuándo consultar cada auxiliar. Mantenlo conciso y procedural.
5. **Usa MCP** para datos vivos, servicios externos, autenticación, acciones controladas o infraestructura centralizada. La skill orquesta; MCP expone datos y acciones tipadas.
6. No uses MCP para servir recursos estáticos que pueden versionarse junto a la skill ni expongas lectores de archivos o ejecutores arbitrarios.
7. Para PDF, Excel, DOCX y reportes, usa scripts o herramientas MCP deterministas y valídalos; no dejes que el modelo construya manualmente el resultado.
8. El harness descubre `SKILL.md` y usa primero `name`/`description`; carga el cuerpo y auxiliares bajo demanda. `AGENTS.md` contiene reglas globales; el detalle condicional vive junto a la skill o en su fuente canónica.

## Estándar para nuevas reglas

Redacta cada regla para que se entienda en una lectura, conserve la información técnica y sea accionable. Aplica estas pautas:

1. **Forma:** incluye un título claro; elimina relleno, historia de origen, repeticiones, nominalizaciones, voz pasiva evitable y afirmaciones sin respaldo; expresa una idea por oración y párrafo; ante dos o más elementos paralelos, usa una frase breve de contexto seguida de bullets o numerales.
2. **Claridad:** traduce la jerga e identifica los términos internos en su primera aparición; conserva el nombre técnico entre paréntesis cuando ayude a localizarlo. Explica solo los comandos, funciones o herramientas no evidentes.
3. **Orden:** coloca el contexto antes del detalle y adapta la estructura al género: problema, solución, motivo y detalle técnico para cambios; contexto, hechos, estado actual y siguiente paso para estados; respuesta directa para preguntas puntuales; y postura breve con trade-off, sin decidir por el usuario, para consultas exploratorias.
4. **Formato:** mantén los encabezados en su propia línea, usa listas para riesgos, causas, pasos o datos múltiples, y reserva ejemplos o tablas para comparar opciones o datos paralelos.
5. **Explicación:** al presentar un concepto desconocido, usa una analogía solo si aclara; vincula defectos, consecuencias y su cadena causal, y reconoce los errores o la dificultad relevantes. Redacta títulos y descripciones según qué es y cómo se usa, sin historia ni rutas literales.
6. **Verificación:** antes de cerrar, comprueba comprensión, concisión sin pérdida, contexto antes del detalle y formato visual claro; elimina historia, rutas y jerga innecesarias.

## Comentarios de código

1. Documenta las interfaces públicas y las unidades cuyo contrato no sea evidente; explica qué efecto producen, qué devuelven o qué garantizan. No comentes código obvio.
2. Comenta en línea solo decisiones no obvias: condiciones de borde, invariantes, seguridad, concurrencia, cálculos complejos y motivos para conservar una operación riesgosa. Explica el porqué y la consecuencia, no la sintaxis.
3. Justifica cualquier `unwrap`, `expect`, pánico o equivalente que no pueda reemplazarse por manejo de errores; deja clara la invariante que hace seguro el fallo.
4. Registra una simplificación deliberada como `ponytail:` solo si tiene un trade-off y un disparador medible; registra deuda causada por una dependencia externa en `docs/DEBT.md`.
5. No uses comentarios como historial, referencias crudas a tickets o ADR, prosa defensiva ni repetición del código. Define los términos que el lector no pueda inferir del contexto.

## Arquitectura y límites del proyecto

1. **Stack:** usa Rust para core y backend, React/TypeScript dentro de Tauri para UI y gRPC en modo headless. No introduzcas Python ni FastAPI en el diseño.
2. **Diseño:** aplica Functional Core / Imperative Shell y arquitectura hexagonal (ADR-0137): la lógica vive en features con puertos tipados; los módulos solo componen y una feature depende de `shared`, no de otra feature.
3. **Límites de datos:** cada dueño expone puertos; no accedas directamente a tablas de otro módulo. La excepción de infraestructura transversal en `shared` y sus criterios vigentes se definen en ADR-0137.
4. **Ejecución y privacidad:** el core no usa contenedores y se ejecuta en hardware del usuario. La cabina central autentica, licencia e ingiere telemetría, pero no computa trabajo del usuario; las credenciales de bróker e IPs de operaciones nunca salen del entorno.
5. **Soberanía por plan:** en el plan gratuito, los datos de trabajo fluyen al proveedor; en el plan de pago al corriente se suprime esa telemetría; si el pago vence, se reactiva. Los detalles canónicos están en ADR-0143 y ADR-0144.
6. **Invariantes:** ante duda genuina, incluye el campo o capacidad requerida (ADR-0020); `NUNCA` y `SIEMPRE` son invariantes físicos, mientras que umbrales, máximos y mínimos deben ser configurables.
7. **Fase:** el proyecto está en greenfield hasta el primer release distribuido: el esquema base puede corregirse en sitio. Después pasa a brownfield y las migraciones son solo hacia adelante (ADR-0006).
8. **Commits:** solo crea commits con autorización explícita del usuario en el turno actual y agrupa los cambios por tipo (`feat`, `docs`, `chore`, `fix` o `test`).
9. **Dirección de UI:** la UI Flutter existente es legado temporal y será eliminada en una sesión futura. Para toda UI nueva o decisión arquitectónica usa React/TypeScript dentro de Tauri; no amplíes Flutter ni lo uses como referencia de diseño. La migración completa y la actualización de sus documentos históricos quedan pendientes de esa sesión.

## Documentación y recuperación de contexto

1. Usa `docs/README.md`, `docs/ADR.md`, `docs/SAD.md`, `docs/ROADMAP.md` y `docs/templates/TEMPLATES.md` como índices; abre después solo el ADR, sección, fase o plantilla pertinente.
2. Los módulos viven en `docs/modules/`, las features en `docs/features/`, los experimentos en `docs/moonshots/`, las órdenes en `docs/execution/` y las lecciones por dominio en `docs/lessons/`.
3. Localiza primero con `rg`, lee la sección necesaria y evita recargar archivos que no cambiaron. Para un barrido amplio, delimita el alcance y delega solo cuando esté justificado y autorizado.

## Razonamiento de dominio especializado

Antes de canonizar una decisión cuantitativa, estadística, financiera, criptográfica, de microestructura, fiscal o legal:

1. **Límites:** lleva los parámetros a sus extremos y registra el resultado; si produce una contradicción, costo sin cota, bloqueo total o división conceptual por cero, el modelo está incompleto.
2. **Supuestos:** al adoptar una métrica, teorema o patrón externo, deriva sus supuestos y verifica cada uno en el contexto del proyecto; cita esas condiciones, no solo su conclusión.
3. **Modelo:** separa los datos primitivos de la política que los interpreta y distingue los casos cuyos tipos, ciclo de vida o propósito cambian su tratamiento.
4. **Alcance:** localiza todos los puntos donde ocurre el evento o decisión antes de resolverlo; una corrección local no basta para un concepto transversal.
5. **Revisión:** somete a revisión adversarial las afirmaciones que sostienen una decisión crítica; su apariencia de rigor no es evidencia.
6. **Especialista:** si un supuesto falla o la decisión requiere pericia del dominio, no la canonices hasta obtener la revisión correspondiente y documentarla en el ADR o documento canónico.

## Decisiones guiadas por documentación

Antes de proponer una regla, contrato, fórmula o cambio de alcance:

1. **Búsqueda:** localiza con `rg` las decisiones existentes en `docs/`; no decidas desde la memoria ni reabras una decisión ya canónica.
2. **Trazabilidad:** cita el documento que sostiene o contradice la propuesta. Si no hay cobertura, declara las rutas y patrones que buscaste; solo entonces trátala como una pregunta abierta.
3. **Iniciativa:** la ausencia documentada permite proponer una solución, pero no inventar una decisión previa.
4. **Dominio:** si la propuesta pertenece a un dominio especializado, aplica también **`## Razonamiento de dominio especializado`**.

## Memoria y correcciones de skills

1. **Memoria heredada:** no leas, escribas, migres ni sustituyas `.agents/memory/`; el usuario resolverá su rescate y eliminación en otra sesión. Documenta decisiones del proyecto solo en su fuente canónica cuando la tarea lo requiera.
2. **Correcciones reutilizables:** si una corrección del usuario revela una regla de comportamiento para una skill, propón una edición quirúrgica de su `SKILL.md`; aplícala solo con autorización explícita del usuario en el turno actual.

## Modos de acompañamiento y lecciones

Cuando una orden de trabajo declare un modo, aplícalo por agente; sin declaración, trabaja en modo autónomo.

1. **Mentor:** el usuario escribe el código; explica el concepto, dicta el fragmento exacto y verifica cada bloque antes de continuar.
2. **Docente:** implementa un bloque lógico, explica sus decisiones y resuelve las preguntas sobre ese bloque antes de avanzar al siguiente.
3. **Profundidad:** en Mentor y Docente, explica desde cero qué es el concepto, por qué existe y qué problema resuelve; no supongas conocimientos previos.
4. **Calidad:** el modo cambia quién escribe y cuánto se explica, nunca los criterios de aceptación ni las validaciones.
5. **Lecciones:** en estos modos, consolida lo enseñado en `docs/lessons/<dominio>/<ID>.md`: un archivo por Story/Task y dominio, extensible sin duplicar, vinculado a la orden y anclado al código real; incluye `## Concepto` y `## Trucos de Senior` solo si hay atajos reales que documentar.

## Gestión de deuda técnica

1. **Registro:** usa una deuda (`DEBT-XXX`) solo para un hueco conocido en algo ya construido que no puede resolverse ahora; regístrala en `docs/DEBT.md` y `docs/debt/DEBT-XXX.md` con origen, impacto, causa raíz, severidad, disparador de pago, plan y estado.
2. **Clasificación:** usa una tarea (`TASK-XXX`) para una decisión, insumo o trabajo aún pendiente; si depende del usuario, márcala explícitamente como bloqueada. Usa `ponytail:` solo para una simplificación local bajo control del módulo y con umbral medible. Un defecto de correctitud no es deuda: corrígelo ahora.
3. **Promoción:** al cerrar un hito o decidir el siguiente trabajo, revisa los disparadores de `docs/DEBT.md` y promueve los cumplidos; un hueco de definición de terminado tiene prioridad sobre el siguiente hito y no puede tener severidad baja.
4. **Pago:** salda una deuda mediante una Story o entregable verificable y actualiza su ficha y el índice canónico con el enlace correspondiente.
5. **Referencia:** cuando una orden pida seguir un patrón, consulta el documento canónico que lo define; usa una implementación existente solo para un punto específico ya auditado.

## Ledgers append-only

Un ledger append-only es una bitácora inmutable de eventos, como una auditoría o una declaración de usuario; esta sección solo aplica al crearlo o modificarlo. Este es el molde operativo: no uses la primera implementación existente como referencia canónica.

1. **Esquema:** aplica ADR-0141: tabla `STRICT`, PK `TEXT` con UUIDv7, timestamps en `INTEGER` de nanosegundos UTC y precio o volumen en `INTEGER` escalado ×10⁸ cuando existan. Define `event_sequence_id INTEGER NOT NULL UNIQUE` con índice, `audit_hash` y `audit_chain_hash`; la fila génesis usa `NULL`, nunca un centinela.
2. **Escritura e inmutabilidad:** dentro de una sola transacción `BEGIN IMMEDIATE`, lee la última secuencia y hash, e inserta la fila. Reintenta solo la contención con límite y error tipado; no reintentes errores permanentes. La migración crea triggers que rechazan `UPDATE` y `DELETE` con `RAISE(ABORT, ...)` y un motivo explícito.
3. **Documentación:** en el mismo cambio, crea o actualiza `docs/data-model/<tabla>.md` y su entrada en `docs/DATA-MODEL.md`; ambos deben coincidir columna por columna con la migración.
4. **Pruebas discriminantes:** verifica 16 escritores concurrentes sin huecos ni duplicados; contención sostenida hasta agotar reintentos con el conteo exacto; el clasificador llamado directamente ante PK duplicada, colisión de secuencia y error ajeno a la base; y una fila devuelta con hash, eslabón y timestamp nuevos.
5. **Mantenimiento:** si el molde mejora, actualiza esta sección antes de aplicarlo. Una implementación existente solo ilustra el punto concreto que ya fue auditado.

## Gate de mutación

Al cerrar una Story que añade o cambia lógica de correctitud en `domain/` o `persistence`, aplica ADR-0133:

1. El responsable técnico del cierre ejecuta únicamente `scripts/mutation-gate.sh <crate> [jobs]`; quien implementa o QA prepara y juzga la evidencia, pero no consume la corrida.
2. El veredicto es la línea textual `GATE APTO` o `GATE NO APTO`, no el código de salida de una tubería. No canalices el script; si es inevitable, conserva el estado de salida del script.
3. Exige cero supervivientes. `timeout` y `unviable` cuentan como capturas; solo acepta un equivalente genuino con una demostración escrita de que el cambio no puede afectar el resultado.
4. Corrige pruebas autorreferenciales y aserciones sin justificación de dominio: una prueba debe cambiar de resultado cuando cambia el comportamiento que declara cubrir.
