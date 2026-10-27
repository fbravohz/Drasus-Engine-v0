---
name: context-refiner
description: Refine AGENTS.md and other instruction or context files by removing redundancy and stale detail, preserving operational invariants, and auditing the standalone file plus any available diff. Use when a user asks to compact, reorganize, audit, or clean up an instruction document.
metadata:
  short-description: Compact and audit instruction and context files
---

# Refinador de instrucciones y contexto

Refina un archivo de instrucciones o contexto sin debilitar su comportamiento. Produce un documento más pequeño, claro y fácil de recorrer, cuyas reglas críticas sigan siendo ejecutables por personas y agentes.

## Elige el modo de auditoría

- Para `SKILL.md`, lee [$skill-creator](/var/home/fbravohz/.codex/skills/.system/skill-creator/SKILL.md) antes de editar. Aplica sus reglas de alcance, descubrimiento, revelación progresiva, recursos de apoyo y validación.
- Para `AGENTS.md` u otro archivo de instrucciones, usa solo esta skill, salvo que el objetivo incorpore explícitamente un contrato de skill.
- No cargues [$skill-creator](/var/home/fbravohz/.codex/skills/.system/skill-creator/SKILL.md) para archivos de instrucciones no relacionados. Mantén el descubrimiento proporcional al objetivo.
- Si el usuario pide auditar o revisar, informa los hallazgos sin editar. Si pide refinar, actualizar o limpiar, edita únicamente el objetivo solicitado.

## Flujo de trabajo

1. **Delimita el objetivo.** Resuelve cuál es el archivo exacto, cuál es su repositorio y qué documentos canónicos referencia. Un diff es contexto opcional, no el objetivo de auditoría. No edites archivos ajenos ni crees commits salvo que el usuario lo solicite explícitamente.

2. **Lee antes de editar.** Lee y audita el archivo objetivo completo, incluso si no tiene diff. Usa `rg` para localizar reglas relacionadas, secciones duplicadas, detalles volátiles y referencias a fuentes canónicas. Inspecciona el `git diff` actual solo cuando el archivo esté bajo control de versiones y ya tenga modificaciones, como complemento de la revisión del cambio.

3. **Construye una lista de preservación.** Extrae invariantes operativas, restricciones de seguridad, reglas de autoridad y orden, rutas, flujos de trabajo, reglas de escalamiento, permisos y semántica específica del dominio. Marca cada elemento como conservar, combinar o eliminar intencionalmente. Trata las fechas, cantidades, ejemplos y prosa como eliminables solo cuando no cambien el comportamiento.

4. **Delimita la frontera de contexto.** Clasifica cada pasaje como salvaguarda local, referencia de enrutamiento, detalle canónico o prosa no operativa. Conserva las salvaguardas locales que siempre estén activas; reemplaza el detalle canónico por una referencia precisa; elimina la prosa duplicada o no operativa. Para `SKILL.md`, prioriza conservar solo la misión, el disparador, las restricciones obligatorias, los criterios de decisión, el flujo breve, las condiciones de escalamiento y la validación necesarias para realizar la tarea. Traslada las variantes, ejemplos, historia, catálogos, esquemas, procedimientos extensos y políticas de dominio a una referencia específica o al documento propietario de ese contenido. Para archivos de instrucciones del repositorio, consulta primero su índice o documentación rectora cuando exista; después dirige los detalles de arquitectura, producto, ejecución y políticas a sus propietarios canónicos. Conserva una invariante local de una línea cuando omitir su detalle pueda permitir un error grave.

5. **Elige una estructura funcional.** Para una skill, prioriza: propósito y disparador; alcance y exclusiones; invariantes; flujo de trabajo; escalamiento y validación; referencias. Para un archivo de instrucciones, prioriza, cuando corresponda: propósito; decisiones previas al trabajo; invariantes; mapa de documentos y conocimiento; flujo de recuperación; gobernanza, memoria y comunicación. Usa encabezados breves, listas compactas y tablas solo cuando mejoren la consulta.

6. **Compacta semánticamente.** Elimina repeticiones, historia obsoleta, prosa ornamental, detalles de implementación duplicados en documentos canónicos y metadatos que puedan desincronizarse. Reemplaza las reglas repetidas por una única declaración autoritativa y una ruta o referencia precisa. Conserva las condiciones, excepciones, umbrales, propietarios, límites de seguridad y comportamientos ante fallos exactos.

7. **Resuelve la ambigüedad.** No combines silenciosamente reglas contradictorias. Da prioridad a la fuente explícitamente autoritativa, a la instrucción actual del usuario o a la decisión confirmada más reciente. Si el conflicto afecta el comportamiento o la seguridad, detente e infórmalo antes de editar.

8. **Edita quirúrgicamente.** Usa `apply_patch`. Conserva los cambios ajenos del usuario. No reescribas un alcance mayor que el solicitado; se permite reorganizar por completo cuando el usuario lo haya pedido explícitamente.

9. **Audita el resultado dos veces.** Primero audita el archivo final para detectar problemas de estructura, contradicciones, rutas faltantes, precedencia confusa, referencias huérfanas y legibilidad. Después compáralo con la lista de preservación y, cuando exista, con la versión o el diff anterior para detectar invariantes perdidas. Restaura las omisiones con la formulación clara más pequeña y repite la auditoría.

10. **Valida.** Ejecuta validaciones de sintaxis, Markdown o del repositorio cuando estén disponibles y sean pertinentes. Para `SKILL.md`, ejecuta el `scripts/quick_validate.py` de [$skill-creator](/var/home/fbravohz/.codex/skills/.system/skill-creator/SKILL.md) cuando esté disponible; después revisa manualmente el descubrimiento, las referencias y el comportamiento. Si el archivo está bajo control de versiones, ejecuta `git diff --check` e inspecciona el diff y el estado finales como revisión adicional del cambio. No exijas un diff para declarar coherente y completo el archivo independiente.

## Criterios de calidad de las reglas

Antes de añadir o conservar una regla, aplica estas comprobaciones:

1. Conserva una regla en `AGENTS.md` solo si aplica ampliamente, previene un error relevante y cabe en unas pocas líneas. Consérvala en `SKILL.md` solo si cambia las decisiones o la ejecución de esa skill.
2. Indica el disparador de la tarea y señala el documento canónico; no exijas lecturas globales no relacionadas. Coloca las variantes, ejemplos, historia, catálogos y procedimientos extensos en una referencia específica o en el documento canónico propietario de ese contenido.
3. Combina las reglas con el mismo efecto, elimina el duplicado menos preciso y resuelve la ambigüedad o el conflicto antes de conservar cualquiera de ellas.
4. Elimina barreras genéricas, como pedir permiso, detenerse o esperar, cuando el criterio normal del agente ya las gestione; conserva únicamente los límites explícitos de seguridad, autoridad o arquitectura.
5. Prueba cada regla contra una edición pequeña no relacionada. Si obliga a realizar lecturas, usar herramientas o tomar decisiones irrelevantes sin cambiar el resultado de esa edición, elimínala o dirígela mediante un disparador.
6. Rechaza las reglas que sean únicamente históricas, ornamentales, subjetivas o no verificables. Convierte los objetivos subjetivos en criterios de aceptación, scripts o referencias cuando afecten la calidad.
7. Prefiere un principio estable a una narración de incidente. Conserva la salvaguarda resultante, pero traslada las fechas, anécdotas, historial de commits y ejemplos puntuales a lecciones, deuda o al registro canónico de decisiones.

## Criterios de calidad específicos para skills

Cuando el objetivo sea un `SKILL.md`, verifica que:

- su descripción identifique la capacidad y el disparador sin convertirse en un catálogo de funciones;
- tenga una misión principal en lugar de combinar rol, gobierno, arquitectura, sistema de diseño, QA e historia;
- cada regla conservada cambie una decisión, proteja una invariante real o defina una validación necesaria;
- las restricciones obligatorias se distingan de las recomendaciones, los ejemplos y las convenciones locales;
- el detalle condicional se dirija progresivamente en lugar de cargarse en cada invocación;
- las referencias sean precisas, fáciles de descubrir y pertenezcan al documento adecuado;
- el punto de entrada pueda entenderse sin reconstruir contexto no relacionado del proyecto.

## Reglas de preservación

- Conserva las rutas de las fuentes canónicas, las referencias de políticas, las reglas de seguridad y privacidad, la propiedad de los datos, los límites de permisos, las restricciones de migración, los estados del ciclo de vida, los criterios de escalamiento y los comportamientos obligatorios de reporte o puntos de control.
- Si la fuente canónica no está verificada, conserva el pasaje hasta encontrar a su propietario e informa la propiedad no resuelta.
- Para archivos de instrucciones de todo el repositorio, conserva las invariantes de arquitectura, la estructura del repositorio, las reglas de recuperación de contexto, las reglas de selección de agentes o modelos, los límites de los subagentes, la política de memoria, la precedencia de la gobernanza, el idioma y la autorización de commits cuando estén presentes y sean operativos.
- Sustituye los detalles volátiles, como fechas, recuentos de archivos, recuentos de líneas, redacción histórica y resúmenes duplicados, por la ruta canónica o la referencia de decisión cuando el detalle no sea necesario para el comportamiento.
- En las skills, mantén el punto de entrada lo bastante pequeño para guiar la ejecución en una sola lectura. Traslada las instrucciones específicas de modo, esquemas, ejemplos, catálogos y procedimientos extensos a `references/`, `scripts/` o al documento propietario del proyecto cuando esos recursos realmente apoyen el flujo de trabajo.
- No inventes nombres, rutas, ejemplos, requisitos ni capacidades. Si una ruta o fuente referenciada no es clara, verifícala antes de cambiar la instrucción. Si falta una referencia o está huérfana, infórmalo o repárala solo cuando la fuente canónica sea clara.
- Mantén el documento final directo y operativo. Evita lenguaje promocional, advertencias repetidas, explicaciones decorativas y reglas que solo repitan el encabezado que las contiene.

## Informe final

Informa:

- el archivo objetivo y el alcance cambiado;
- qué se eliminó, consolidó y preservó;
- cualquier información omitida deliberadamente por ser no operativa;
- para un `SKILL.md`, qué permanece en el punto de entrada y qué se dirigió a una referencia o propietario canónico;
- las validaciones realizadas y los riesgos o ambigüedades restantes.

Si la auditoría detecta una omisión relevante, indícala explícitamente y corrígela antes de declarar completada la tarea cuando el usuario haya solicitado editar el archivo.
