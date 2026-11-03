# Revisión y refinamiento visual

Usa este procedimiento para una crítica de diseño o un pase de refinamiento solicitado. Examina una superficie y su recorrido completo dentro del alcance acordado. El objetivo es identificar problemas observables y orientar una corrección; una revisión experta no equivale a una prueba con usuarios ni a una auditoría de conformidad.

## Preparación

1. Identifica la superficie, la tarea, las decisiones aprobadas y los estados relevantes. Lee el canon y las guías de los dominios implicados.
2. Reúne la evidencia que exista: referencia aprobada, prototipo o aplicación, contenido representativo, tamaños de ventana y restricciones. Una imagen sin contenido visible, recortada o de otro estado no prueba el requisito que se intenta revisar.
3. Anota qué se puede examinar directamente y qué falta. Si no puedes abrir una superficie o verificar un método de entrada, informa el límite en vez de presentar una revisión completa.

## Dos perspectivas complementarias

Primero recorre la experiencia como diseño, sin partir de una lista mecánica: tarea, lectura, agrupación, ubicación de acciones, claridad del estado, tono, consistencia y adaptación. Después revisa las propiedades comprobables con las herramientas disponibles: contraste compuesto, foco, orden de teclado, overflow, nombres accesibles, estados y errores visibles, según el alcance.

Puedes realizar ambas pasadas en una misma sesión. No las presentes como evaluaciones independientes si las hizo la misma persona o agente. Una medición es una pista que debe comprobarse en contexto; la ausencia de hallazgos no demuestra calidad visual o cumplimiento total.

## Recorre la tarea

- **Llegada:** ¿el usuario sabe dónde está y cuál es el siguiente paso? ¿La información importante se distingue de lo secundario?
- **Acción:** ¿los controles se reconocen, describen su efecto y ofrecen respuesta apropiada? ¿La información necesaria está junto a la decisión?
- **Progreso y resultado:** ¿la interfaz comunica qué ocurre y qué terminó? ¿Un progreso mostrado es real y su significado es claro?
- **Desvíos:** ¿qué sucede con datos vacíos, filtros sin resultados, permisos insuficientes, contenido largo, interrupciones, errores y recuperación?
- **Continuidad:** ¿el usuario conserva contexto y trabajo al pasar entre paneles o al volver? ¿Se distinguen los entornos simulados y los reales cuando corresponde?
- **Repetición:** ¿las acciones frecuentes, los mensajes y el movimiento siguen siendo útiles después de muchos usos?

Para una evaluación heurística, reutiliza los diez criterios y la escala de gravedad del [catálogo de métodos de investigación](../research/references/research-methods-catalog.md). Selecciona escenarios pertinentes para personas que empiezan, personas con experiencia y métodos de acceso relevantes. Son perspectivas de inspección, no personas investigadas ni resultados de usuarios.

## Comparación con una referencia aprobada

Cuando el encargo exige respetar una composición, compara lado a lado el material aprobado y el resultado en un estado y tamaño equivalentes. Revisa:

- estructura, orden de lectura, navegación, regiones y distribución;
- escala relativa, jerarquía tipográfica, densidad y separación;
- materiales, transparencia, bordes, sombras, color de fondo y continuidad del halo cuando formen parte de la dirección;
- apariencia y ubicación de controles, datos, estados y contenido;
- diferencias introducidas para reflujo, accesibilidad o contenido real.

Clasifica cada diferencia relevante como correspondencia, adaptación justificada, elemento faltante, contradicción o adición. Cita la decisión, restricción o evidencia que justifica una adaptación; no la inventes. Una diferencia menor de píxeles no tiene la misma importancia que cambiar una jerarquía o retirar una función.

## Prioridad y corrección

Ordena los hallazgos por impacto real: primero tareas bloqueadas, pérdida de trabajo, información engañosa o barreras de acceso; después estados y recuperación; luego jerarquía, adaptación y coherencia; al final, detalles visuales. Ajusta la prioridad al riesgo y al alcance, sin convertir una calificación agregada en prueba de que todo es seguro o usable.

Para cada hallazgo registra ubicación, estado o condición, evidencia, consecuencia para el usuario y una recomendación concreta. Señala también lo que funciona y debe conservarse. Distingue una decisión de diseño pendiente de un defecto en la maqueta o la implementación.

En una solicitud de pulido, preserva la dirección, el contenido, los comportamientos y las áreas fuera del alcance. Si el problema requiere una nueva composición o una decisión de producto, muéstralo en vez de introducir un rediseño silencioso.

## Entrega y nueva revisión

Entrega los hallazgos ordenados, las comparaciones efectuadas, las adaptaciones justificadas y los límites de la evidencia. Tras un cambio, vuelve a recorrer las condiciones afectadas y conserva cualquier problema no verificado como pendiente. Para resultados productivos, coordina la implementación y verificación con las skills correspondientes.
