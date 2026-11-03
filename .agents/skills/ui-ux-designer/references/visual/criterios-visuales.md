# Criterios visuales para superficies de trabajo

Usa esta guía para diagnosticar o ajustar la composición, la tipografía, el color y la intensidad de una superficie. Parte de la tarea, el contenido, las decisiones aprobadas y los límites del encargo. Los principios de Impeccable son herramientas de evaluación: sus ejemplos no sustituyen la dirección elegida en `docs/DESIGN.yaml`.

## Diagnóstico antes de cambiar

1. Identifica la tarea principal, las decisiones que toma el usuario, los datos que necesita y la frecuencia de uso. Distingue el foco actual de acciones secundarias y estados que exigen atención.
2. Inspecciona la superficie completa y su recorrido, no solo un componente aislado. Compara estados habituales, vacíos, carga, error, permisos y contenido largo cuando sean pertinentes.
3. Describe el problema observado y su consecuencia: por ejemplo, qué dato compite con la acción principal o qué separación hace parecer que dos campos están relacionados.
4. Ubica lo que el propietario pidió modificar y lo que debe conservarse. Para un ajuste localizado, no replantees identidad, navegación o componentes vecinos sin que el encargo lo requiera.
5. Separa observación visual, resultado medido e hipótesis. Una medición aporta evidencia sobre una propiedad concreta; una captura o un resultado sin hallazgos no demuestra por sí solo comprensión o accesibilidad.

## Composición, jerarquía y densidad

- Identifica el orden de lectura y de tarea: qué debe localizarse primero, qué lo explica y qué información debe permanecer disponible para decidir. Comprueba el orden con contenido representativo y con el recorrido real.
- Agrupa por significado y proximidad. Usa bordes, superficies y separadores cuando expresen una relación o una frontera útil; no añadas contenedores solo para compensar una agrupación poco clara.
- Evalúa la cadencia entre grupos y elementos internos. La repetición ayuda a reconocer patrones; la variación debe corresponder a una diferencia de función, prioridad o contenido.
- Ajusta la densidad según la tarea y el uso frecuente. Una cuadrícula de información puede ser apropiada para comparar; más espacio o menos tarjetas no son una mejora universal.
- Al probar un cambio de distribución, indica qué permanece fijo, qué se mueve y por qué. Examina tamaños de ventana, escalado, traducciones y extremos del contenido con la guía de adaptación a ventanas.
- Verifica que el orden visual, el orden de foco y la lectura asistida sigan una secuencia comprensible. Una prueba de desenfoque o de vistazo rápido ayuda a detectar competencia visual, pero no reemplaza la lectura de datos reales.

## Tipografía

- Identifica las funciones tipográficas necesarias: título de pantalla, título de panel, texto, etiqueta, dato principal, dato secundario, unidad y ayuda. Compara cada función con otras de igual importancia.
- Usa tamaño, peso, tono y espaciado de manera coordinada. La jerarquía debe seguir siendo reconocible cuando la información cambia, sin que todos los elementos compitan por el mayor peso.
- Conserva Geist Sans y Geist Mono como perfil predeterminado mientras no se apruebe otro. Geist Mono tiene un papel en las cifras; no lo conviertas en una regla para todo el texto técnico.
- Comprueba cifras de distinta longitud, signos, unidades, alineación de columnas y números tabulares cuando la comparación lo necesite. Revisa nombres extensos, texto traducido, aumento de tamaño y fuentes de respaldo.
- En texto continuo, revisa longitud de línea, interlineado y saltos en la ventana real. Las recomendaciones para párrafos extensos no son dimensiones obligatorias para etiquetas, tablas ni paneles densos.
- En superficies oscuras, revisa el resultado visual de peso, contraste e interlineado sobre el material real. No sustituyas la dirección aprobada por un mínimo genérico de tamaño.

## Color, profundidad y materiales

1. Identifica el papel de cada color: fondo, superficie, texto, borde, selección, foco, acción, estado o dato. Un acento decorativo no debe confundirse con un significado del producto.
2. Conserva las funciones protegidas de ganancias, pérdidas, alertas y puertos del Canvas. Usa texto, símbolos, forma u otra señal pertinente además del color.
3. La exploración de acentos de color está aplazada en `docs/DESIGN.yaml`. Mantén el blanco inicial salvo que el propietario reactive o solicite esa exploración.
4. Evalúa el contraste sobre el resultado compuesto, no sobre un color aislado: transparencia, halo, estados y superposiciones pueden cambiar el fondo detrás de una etiqueta o control.
5. En el Dashboard, compara los widgets con el material uniforme y el halo aprobados. No cambies opacidad, borde o dirección de luz de forma aislada cuando el encargo indique conservarlos.
6. Revisa foco, selección, hover, error y deshabilitado donde apliquen. El color de una serie o un estado necesita un significado estable entre superficies y temas.

## Aumentar o reducir intensidad

Para dar más presencia a una región, localiza la fuente de debilidad: jerarquía, escala, posición, densidad, contraste o falta de una relación clara. Amplifica primero recursos que ya pertenecen al sistema. No necesitas aumentar todos los efectos ni introducir otra paleta para hacer visible una prioridad.

Para disminuir ruido, identifica qué compite: saturación, varios pesos fuertes, líneas, superficies, adornos, movimiento o demasiadas acciones del mismo nivel. Reduce la causa concreta y conserva los anclajes que permiten leer y operar. Verifica que el resultado siga siendo reconocible como Drasus.

Para simplificar, examina si una información, acción o separación ayuda a la tarea. Agrupa o revela progresivamente lo secundario cuando sea localizable y accesible. No ocultes una alerta, una consecuencia, un dato de riesgo o una función necesaria para ahorrar espacio. Un menú, tooltip o hover no sustituye una vía con teclado ni una etiqueta accesible.

## Comprobación

- La jerarquía dirige la atención al trabajo y los datos correctos en estados y tamaños representativos.
- La densidad, los controles y el material siguen las decisiones aprobadas o identifican la decisión aún abierta.
- La tipografía soporta lectura, comparación, escalado y contenido realista.
- El color conserva su función semántica y el contraste se revisa en el fondo realmente visible.
- El cambio se limita al objetivo solicitado y no elimina información ni formas de acceso necesarias.

Para una crítica completa o una revisión final, continúa con [Revisión visual](revision.md); consulta la guía de accesibilidad para el método y criterios aplicables.
