# Dirección y prototipos

Usa esta guía para definir una superficie, explorar opciones visuales o traducir una composición aprobada a un prototipo. Aplica las decisiones aprobadas de `docs/DESIGN.yaml` y distingue una decisión del propietario de una hipótesis de investigación.

## Determina cuánto está abierto

- **Ajuste localizado:** identifica el elemento solicitado, la causa del cambio y los límites. Hereda el sistema, la composición y los comportamientos existentes donde no se haya pedido cambiarlos.
- **Superficie nueva dentro del sistema:** conserva identidad y fundamentos aprobados; resuelve tarea, contenido, jerarquía, estados e interacción de esa superficie. Ofrece alternativas de estructura si el usuario las solicita o si la decisión sigue materialmente abierta.
- **Dirección o identidad nueva:** identifica qué partes ya están confirmadas y cuáles requieren decisión del propietario. Una propuesta de marca o de una pantalla no reemplaza automáticamente las decisiones globales.

Cuando el pedido ya contiene lo necesario, avanza. Si falta una decisión que cambia el resultado, plantea pocas preguntas relacionadas y usa lo ya conocido; no repitas decisiones que el propietario ya tomó.

## Prepara el encargo de diseño

Reúne la información pertinente, sin convertir cada punto en un cuestionario:

1. **Persona, situación y tarea:** quién llega a la superficie, qué intenta hacer y qué necesita entender para actuar.
2. **Resultado y evidencia:** cómo se reconoce que la tarea terminó, qué información real está disponible y qué contenido será de ejemplo.
3. **Alcance:** superficie o región, nivel de fidelidad, interacciones que debe mostrar y elementos que deben conservarse.
4. **Contenido y estados:** datos mínimos, habituales y extensos; carga, vacío, error, permisos, recuperación y estados propios de la tarea.
5. **Restricciones:** ventana, métodos de entrada, accesibilidad, idiomas, componentes existentes, contratos y decisiones abiertas que no deben inventarse.

Un encargo específico puede resolverse con unas pocas notas. Un flujo amplio necesita más detalle. Si el propietario solicita confirmar la dirección o el sistema carece de una decisión indispensable, presenta la decisión antes de tratarla como aprobada.

## Elige cómo explorar

- Para una interfaz de trabajo, la tarea, las áreas funcionales, los datos y los controles conocidos deben leerse como tales. La identidad se expresa a través de las decisiones visuales aprobadas sin disfrazar la función.
- Para una superficie de lectura, conserva un recorrido claro, una jerarquía de títulos y una medida cómoda para el texto. La navegación debe indicar dónde está la persona y cómo continuar.
- Para una superficie de marca o presentación solicitada, explica qué acción se espera y qué evidencia respalda la propuesta. Las afirmaciones comerciales y los datos reales necesitan una fuente.
- Si hay varias alternativas, varía la decisión que se intenta resolver: orden, jerarquía, distribución, densidad o tratamiento de una interacción. Mantén comparables la fidelidad, el contenido, los estados y el tamaño de ventana para que la diferencia sea visible.
- Si se solicita un ajuste puntual de una propuesta aprobada, cambia solo ese aspecto. No prepares una ronda nueva de alternativas por rutina.

Una imagen conceptual, una maqueta HTML o una especificación pueden servir para decisiones distintas. Elige el formato que permita comprobar la pregunta. Una imagen de una interfaz no demuestra comportamiento de controles, lectura asistida ni adaptación de ventana.

### Explorar variantes en una interfaz en ejecución

Cuando la decisión dependa de cómo se percibe una interfaz ya montada, reutiliza el servidor de desarrollo existente y compara variantes dentro de la misma superficie. Cambia el eje que se quiere decidir —por ejemplo, jerarquía, separación, tipografía o material— y conserva contenido, datos, tamaño de ventana y estado para que la diferencia sea legible. No abras servidores o pestañas adicionales si el entorno ya ofrece la superficie.

Usa las herramientas de navegador disponibles para seleccionar o inspeccionar la región, mostrar alternativas y recibir la elección. Mantén las variantes en un prototipo o entorno de desarrollo aislado; la exploración no modifica por sí misma los componentes productivos ni `docs/DESIGN.yaml`. Cuando el propietario elija una variante, entrega la decisión y su evidencia a `frontend-engineer` para llevarla al componente real.

Drasus usa Tauri con React y Vite. Una vista React puede explorarse en un navegador si sus componentes no dependen de APIs exclusivas de Tauri; marca como simuladas las integraciones que deban representarse temporalmente. Comprueba el resultado final en la ventana Tauri cuando estén disponibles el cascarón y las APIs reales. No inyectes un helper en la aplicación ni cambies su política CSP para habilitar una vista previa: este método reutiliza las herramientas existentes y no requiere el ejecutable ni el hook de Impeccable.

## De una referencia aprobada al prototipo

1. Identifica en la referencia los elementos que el propietario aprobó: estructura, navegación, orden, densidad, tipografía, controles y materiales pertinentes. Distingue esos elementos de los datos de ejemplo.
2. Compara el resultado con la referencia a tamaños y estados equivalentes. Si un cambio es necesario por contenido, accesibilidad o tamaño de ventana, identifícalo y explica su motivo.
3. Mantén texto, controles y datos como elementos que puedan leerse e interactuar cuando corresponda; una captura integrada como imagen no sustituye una interfaz funcional.
4. Si el diseño necesita imágenes o ilustraciones, identifica sus regiones, posición, proporción y qué partes cambian de forma independiente. Separa el contenido variable de marcos o elementos que se muevan, cuando esa separación aporte al resultado.
5. Para un activo derivado de una referencia, conserva los rasgos aprobados y verifica su calidad y transparencia cuando aplique. No sustituyas silenciosamente una región o una textura que el usuario eligió. La herramienta de imágenes se usa cuando el entregable requiere un recurso raster.
6. Rotula los datos inventados junto a la superficie. No presentes cálculos simulados, operaciones, claims o resultados como hechos verificados.

### Mapa de regiones para activos

Cuando una composición aprobada incluya recursos gráficos que deban producirse por separado, describe cada región con su función, posición, proporción y relación con las demás. Una región contenedora no sustituye los textos o controles que contiene: identifícalos por separado para que sigan siendo editables y accesibles.

Separa las partes que cambian de forma independiente. Por ejemplo, si un marco contiene una imagen variable y un control superpuesto, registra el marco, la imagen y el control como elementos distintos. Revisa solapamientos, recortes y áreas transparentes frente a la referencia; un borde de recorte correcto no garantiza que la imagen conserve todo el contenido necesario.

Si se genera un recorte con transparencia, comprueba el canal alfa sobre fondos claros y oscuros, incluidas aberturas interiores, bordes finos y sombras. Conserva el origen o la instrucción de generación cuando el flujo de activos lo requiera. Una captura de referencia puede ayudar a guiar el trabajo, pero su recorte puede no tener calidad suficiente para ser un activo final.

## Comparación con el sistema documentado

Al revisar un diseño existente, distingue lo que muestra el artefacto de lo que el canon aprueba. Una implementación o maqueta puede revelar una discrepancia, pero no aprueba por sí misma un cambio de producto. Señala la discrepancia y su fuente; registra una decisión en `docs/DESIGN.yaml` cuando el encargo y la confirmación correspondiente lo autoricen.

Para un inventario de patrones, tokens y componentes, usa la ficha canónica existente. Distingue una convención repetida de un valor aislado y una intención documentada de un comportamiento observado. No crees otra ficha de diseño, una copia paralela ni un esquema de tokens separado solo porque el método de origen los utilice.

## Entrega

Muestra el artefacto solicitado o su ruta, la decisión que permite revisar, lo que se conservó, los datos de ejemplo y cualquier límite de la comprobación. Si una propuesta necesita aprobación del propietario, indica exactamente qué permanece abierto; no presentes una captura como decisión confirmada.
