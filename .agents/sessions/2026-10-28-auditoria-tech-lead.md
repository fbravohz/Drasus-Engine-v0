# Auditoría comparativa de la skill Tech Lead

Fecha: 2026-10-28

Se comparó el archivo anterior con la skill compactada y con las reglas que esta consulta. La revisión no modifica la skill ni resuelve las discrepancias documentales previamente identificadas.

- **Original:** contenido preparado en Git, blob `53ba13919a680d17767d999264b84eeb430da469`; puede consultarse con `git show 53ba13919a680d17767d999264b84eeb430da469`.
- **Versión auditada:** [tech-lead/SKILL.md](../skills/tech-lead/SKILL.md), hash de contenido `ffaa9015ff10aa79c0524fa83c74825b75722c6e`.
- **Fuentes contrastadas:** [AGENTS.md](../../AGENTS.md), [Roadmap](../../docs/ROADMAP.md), [plantilla de Orden](../../docs/execution/_TEMPLATE.md), ADR-0117/0135/0136/0137/0165 y skills de Architect, Quant, QA, Rust y Frontend.

## Hallazgos

1. **Cáscara Visual antes del Core — pérdida confirmada.** El original ejecutaba la Etapa 0.5 antes de la implementación Rust. La skill actual solo exige verificar la Cáscara antes de frontend. [ADR-0135](../../docs/adr/ADR-0135.md) conserva expresamente la etapa antes de Rust; su enmienda de ADR-0165 mantiene esa etapa. Si se empieza el Core sin ella, el diseño visual y sus necesidades llegan tarde. El rol que sustituye al antiguo `ui-designer` necesita precisarse con las capacidades activas; no se propone restaurar la tecnología retirada.

2. **Viabilidad al reevaluar — pérdida confirmada.** El original activaba la revisión de viabilidad al crear una Orden o reevaluar un desarrollo. La skill actual solo la dispara antes de redactar una Orden. Una reevaluación de trabajo existente puede omitir el filtro cuantitativo o legal cuando corresponda. El original permitía `No Aplica`; no exigía inventar una revisión de un área inexistente.

3. **Clasificación de superficie frente a plomería — criterio retirado.** El original prohibía considerar plomería una Feature que produce un tipo de dominio o recibe configuración del usuario. La versión actual aplica requisitos distintos una vez declarada la clasificación, sin conservar ese contraste explícito. ADR-0137 distingue los tipos de dominio y ADR-0136 describe el inspector del nodo, pero las fuentes consultadas no confirman expresamente que cualquier configuración de usuario, por sí sola, siempre implique superficie propia. Suprimir el contraste deja abierta una clasificación equivocada; el alcance exacto del criterio requiere análisis antes de restaurarlo.

4. **Decisiones arquitectónicas convertidas en verificaciones — pérdida confirmada.** El original exigía traducir cada regla auditable de un ADR nuevo o modificado a checks del filtro, o explicar por qué no aplica. La versión actual pide localizar ADR y registrar preguntas y documentos, pero no exige convertir cada obligación aplicable en un criterio verificable. Que una Orden tenga pruebas por criterio no demuestra que haya incluido todos los criterios derivados de una decisión.

5. **Despacho y separación de responsabilidades — límite debilitado.** El original asignaba el Core al Rust Engineer y distinguía al implementador del Tech Lead que coordina y audita. La skill actual no menciona al Rust Engineer ni explica quién implementa en Autónomo; «conecta Core, Bridge y frontend» permite más de una lectura. QA sigue siendo obligatorio, pero la función de auditoría independiente del Tech Lead queda menos definida. El original también admitía correcciones puntuales, por lo que no se infiere que deba restaurarse una prohibición absoluta de toda edición.

6. **APTO cuantitativo por etapa — ambigüedad.** El original bloqueaba la implementación ante `NO APTO` de diseño y exigía `APTO` después de QA para cerrar. El texto actual exige ambas revisiones y dice que un `NO APTO` impide el cierre; su regla general de viabilidad también dice que `NO APTO` bloquea implementación, pero aparece en el paso previo a crear la Orden. No queda inequívoco que la Etapa 1 posterior requiera `APTO` antes del Core ni que la Etapa 6 requiera un veredicto positivo explícito.

7. **Selección de TTR y precondiciones — detalle operativo perdido.** El original hacía recorrer los TTR del módulo y su cadena de precondiciones y postcondiciones para seleccionar un candidato ejecutable. El texto actual consulta fase y dependencias del Roadmap, pero no dirige expresamente esa comprobación entre TTR. [El Roadmap](../../docs/ROADMAP.md) identifica las tablas de TTR de los módulos como fuente del alcance; una fase activa por sí sola no prueba que un TTR esté desbloqueado. No se propone recuperar el filtro histórico por prioridad aislada, que el Roadmap descarta.

8. **Continuidad durante un escalamiento — garantía parcial.** El original actualizaba la continuidad también al escalar o tomar una decisión relevante, sin esperar al cierre. La skill actual registra explícitamente el bloqueo por el propietario y actualiza los registros al cerrar etapas, pero no hace igual de explícito dónde dejar el estado y siguiente paso de un escalamiento técnico pendiente antes de cerrar. La Orden conserva pendientes y AGENTS define el destino del siguiente paso; falta el disparador específico cuando el trabajo queda a medias.

9. **Aprendizaje al terminar una iteración — pérdida parcial.** El original pedía identificar fallas, causa raíz y filtro que las detectó o dejó pasar, y proponer una mejora. La compactación quitó también esa revisión junto con la edición automática de skills. La edición automática sí debe permanecer sujeta a autorización explícita conforme a AGENTS; identificar y reportar una causa raíz no requiere editar skills.

## Controles que permanecen

La skill actual conserva la verificación independiente por evidencia, QA antes del cierre de código, el filtro de mutación, la ventana de verificación de plomería y los veredictos especializados. AGENTS y las fuentes citadas conservan, además, propiedad de datos, privacidad, criterios de aceptación, pruebas discriminantes, migraciones y cierre documental. La comprobación sintáctica no demuestra por sí sola equivalencia operativa.

## Estado de la revisión

El original se comparó con el contenido preparado en Git y el archivo actual completo. Se revisaron las fuentes y los disparadores mencionados en los hallazgos. No se modificó el índice de Git ni la skill auditada. Las discrepancias previas sobre la plantilla y QA quedan para una revisión posterior.
