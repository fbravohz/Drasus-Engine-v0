# Sistema de diseño de Drasus Engine

> **Estado: borrador en entrevista.** Este documento conserva el contexto y la justificación legibles del sistema de diseño. [DESIGN.yaml](./DESIGN.yaml) es la fuente estructurada para automatización y estados de decisión; no se deben mantener decisiones contradictorias entre ambos. La dirección visual del Dashboard está aprobada; tokens, componentes y el sistema completo siguen en definición.

Este archivo acompaña la ficha estructurada con contexto y justificación para entrevistas por bloques. El cliente actual es de escritorio; la adaptación a tablet o móvil y el renderizado en cliente web siguen abiertos y no quedan descartados. Presentaciones y otros materiales de marca no forman parte de esta ficha de producto.

## Cómo usar este documento

- **Vigente:** una decisión ya establecida en el canon, identificada por su fuente.
- **Preferencia del propietario:** dirección expresada durante la entrevista, que se concretará y confirmará con ejemplos.
- **Por confirmar:** una elección que se discutirá y registrará antes de usarla como regla de diseño.
- **Por validar:** una propuesta que requiere prototipo, medición o investigación con usuarios.

Las referencias visuales y los ejemplos de una propuesta no aprueban por sí solos colores, componentes ni comportamientos. Mientras se construye esta guía, los ADR vigentes prevalecen sobre sus secciones pendientes. Si se acuerda una dirección que los contradiga, hay que reconciliar la decisión en su fuente antes de implementarla.

## 1. Estrategia y dirección

### Producto y contexto

Drasus ayuda a investigar, evaluar y operar estrategias de inversión mediante una aplicación ejecutada en el entorno del usuario. Las capacidades y el público descritos por el proyecto se encuentran en [PRODUCT.md](./PRODUCT.md) y [PRODUCT-SIMPLE.md](./PRODUCT-SIMPLE.md). Esa documentación describe la intención del producto; no sustituye la investigación con usuarios.

Los objetivos de negocio y las señales de éxito del primer release distribuido siguen abiertos; se definirán en el bloque `Objectives` y no se inferirán a partir de las capacidades del producto.

El Canvas es el principal atractivo de Drasus Engine: ahí se construyen y conectan capacidades. El Dashboard es una superficie oficial para monitorear y navegar dentro del producto, no la definición completa del producto.

### Visión expresada por el propietario

La aplicación debe sentirse como un centro de mando refinado, moderno, tecnológico, pulido y premium para construir, investigar, probar y operar sistemas y portafolios de trading. Debe servir a principiantes y expertos y transmitir control sobre el trabajo. Estos segmentos son dirección confirmada por el propietario, no personas validadas mediante investigación UX.

El producto cubre trabajos complementarios a lo largo del ciclo de vida:

- Construir procesos en el Canvas, investigar y generar estrategias, evaluarlas y validar su robustez.
- Incubar y operar estrategias y portafolios, con controles de capital y revisión de riesgos.
- Consultar resúmenes de estado y resultados en el Dashboard, y abrir el contexto correspondiente en el Canvas.

Esta es una aspiración de experiencia, no una promesa de rentabilidad. La entrevista con el propietario orienta el diseño; las necesidades de otros usuarios siguen siendo hipótesis hasta investigarlas.

**Por confirmar:** la ubicación visual concreta del menú global, la distribución inicial del resumen y cómo se accede al detalle.

Las capacidades expuestas como nodos de Canvas se integran mediante APIs de feature y puertos tipados. Cada nodo presenta sus entradas, salidas y la gráfica o información que aplique según su contrato; no toda feature necesita un nodo, una gráfica o una superficie propia. Los módulos ofrecen composiciones predefinidas para facilitar el inicio; los usuarios expertos pueden conectar features compatibles directamente. La autoridad de dominio está en [ADR-0137](./adr/ADR-0137.md).

### Referencias de la entrevista

El propietario eligió `top1.webp`, `top2.webp` y `top3.webp` como punto de partida. Los archivos están en `../inspiration/`, ruta relativa a la raíz del repositorio. Se encuentran fuera del repositorio y pueden no estar disponibles para otros colaboradores.

Le interesan la composición modular tipo *bento*, el equilibrio de los negros, la apariencia de botones, desplegables e inputs, la tipografía, los espacios y los colores tenues de `top2`. De las imágenes adicionales eligió el vidrio oscuro de los paneles y un halo de luz en el fondo. Busca bordes delicados, iluminados y sombreados como los de `top1`, con la sensación de un control refinado de un vehículo de lujo. La aplicación no tendrá fotografías de fondo: la luz será blanca, sin matiz azul ni gris, y se sugerirá con rayos tenues que creen una transición entre claridad y oscuridad.

**Preferencia confirmada por el propietario:** la primera versión usa tema oscuro y acento blanco. No hay planes actuales para un tema claro. Los tonos exactos del catálogo de acentos siguen por revisar.

[Comparativa de luz y materiales](./design/comparativa-materiales.html): las dos primeras pantallas reconstruyen las referencias compartidas por el propietario. La tercera adapta Overview a Drasus: conserva su composición, distribución, densidad y lenguaje visual, con la navegación lateral ya elegida para el producto. De NestIQ toma el acabado y los bordes de las superficies; los widgets son más transparentes y dejan pasar un halo blanco continuo desde la cabecera hasta las separaciones. El propietario aprobó esta dirección visual; la maqueta no fija valores de tokens ni componentes de producción, y sus datos son ficticios.

**Dirección aprobada del Dashboard:** Overview define la composición, distribución inicial, densidad, separación, controles y estilo general. NestIQ aporta el acabado y los bordes; Drasus usa una transparencia uniforme entre widgets para dejar ver el halo blanco a través de las superficies y las separaciones. El campo de luz cruza toda la interfaz en diagonal, desde la esquina superior derecha hasta la inferior izquierda, también detrás de la cabecera y sin cortes entre contenedores. El propietario aprobó su intensidad actual y el material de los widgets. La legibilidad con información de Drasus sigue por validar.

[Dirección de identidad e interfaz](./design/VISUAL-DIRECTIONS.md) conserva propuestas anteriores sin seleccionar. Se consultan como exploración, no como una dirección aprobada.

**Dirección aprobada:** el propietario aprobó el acabado de los controles de la pantalla 01 de la comparativa. Esta aprobación cubre su apariencia general; los estados interactivos y los valores precisos aún deben especificarse y verificarse.

**Por validar:** la legibilidad de las superficies con más transparencia usando contenido representativo y tamaños de ventana distintos. La continuidad del halo, la composición, distribución, densidad y separación iniciales siguen la dirección aprobada; los valores precisos de sus materiales se definirán como tokens.

## 2. Identidad visual y marca

La personalidad visual y la paleta de acentos se describen en la dirección anterior y en la sección de personalización. Las familias tipográficas predeterminadas se indican en los fundamentos. **Por confirmar:** tratamiento de marca en la aplicación, logotipo, tonos concretos de la paleta, iconografía e imágenes. Las aplicaciones de marca fuera del escritorio quedan para una etapa posterior.

Las decisiones de marca deberán convivir con los significados del producto: un color que representa un estado o un tipo de dato necesita una función identificable. Los colores de los puertos del Canvas están documentados en [ADR-0137](./adr/ADR-0137.md); el diseño propuesto debe revisarlos antes de cambiarlos.

## 3. Fundamentos y tokens

Un **token de diseño** es un valor con nombre que representa una decisión reutilizable, como el color de una superficie o el espacio entre elementos.

La arquitectura a desarrollar contempla tres capas:

1. **Primitivos:** valores base, como una escala de colores o espacios.
2. **Semánticos:** significado de uso, como texto principal, superficie o advertencia.
3. **De componente:** decisiones específicas de un componente cuando las necesite.

Los patrones y las plantillas consumen tokens; no son una cuarta capa de tokens. Las categorías y los nombres concretos se definirán con la dirección visual.

### Color y materiales: decisiones aprobadas y temas por definir

El propietario confirmó que el color debe corresponder a lo que comunica la interfaz:

| Color | Significado acordado |
|---|---|
| Verde | Ganancia, celebración u otra condición positiva |
| Amarillo | Alerta que requiere atención |
| Rojo | Pérdida financiera, urgencia o error |
| Azul | Tranquilidad o calma |

Los valores, contrastes y variantes se definirán con ejemplos. Una pérdida y una urgencia comparten el rojo; las urgencias incluyen además una etiqueta y un icono que indiquen su gravedad. Como criterio para validar, el estado también debe poder distinguirse por texto, forma o símbolo. Un resultado financiero positivo no equivale por sí mismo a una estrategia validada.

Las superficies de los widgets toman de NestIQ el acabado y los bordes. Por decisión del propietario, tienen más transparencia que la reconstrucción NestIQ para dejar ver el halo a través de los paneles y en las separaciones. Todos los widgets de la cuadrícula comparten exactamente los mismos valores de opacidad, transparencia y borde en su material base; no hay variantes por tipo de widget. Su composición y el resto del lenguaje visual siguen Overview. Un solo campo de luz blanca recorre toda la interfaz en diagonal desde la esquina superior derecha hasta la esquina inferior izquierda; continúa detrás de la cabecera, las superficies y sus separaciones, sin cortes abruptos entre contenedores. El propietario aprobó la intensidad y la transparencia que muestra actualmente la tercera pantalla de la comparativa. La legibilidad de las superficies sigue por validar con contenido representativo. El acabado general aprobado de los controles se describe a continuación; sus estados interactivos aún deben definirse.

**Decisión aprobada por el propietario:** mantener el halo blanco de la tercera pantalla de la comparativa como luz ambiental continua desde la cabecera hasta los widgets y sus separaciones, en diagonal desde la esquina superior derecha hasta la esquina inferior izquierda. Su intensidad actual está aprobada. Se conserva la composición, distribución, densidad y separación de Overview, junto con la transparencia uniforme de los widgets. La legibilidad se validará con contenido y tamaños de ventana representativos.

### Acabado de controles aprobado como dirección

La pantalla 01 de la comparativa muestra el acabado que el propietario quiere conservar:

- Controles oscuros con forma de cápsula o círculo, según su función.
- Un borde superior delicado y luminoso, con sombreado que aporta relieve.
- Fondo oscuro con transición suave y sombra discreta alrededor del control.
- Selección blanca con texto oscuro, conservando la forma del control.

Este lenguaje se aplicará al especificar los componentes compartidos. Los colores y medidas del prototipo no son todavía tokens aprobados. Faltan los estados de foco, hover, pulsación, deshabilitado y carga, además de su verificación de contraste y uso con teclado.

### Tipografía predeterminada: decisión del propietario

La interfaz usará [Geist Sans y Geist Mono](https://vercel.com/font) como perfil tipográfico predeterminado: Geist Sans para los textos y Geist Mono para las cifras. **Decisión del propietario:** cada opción tipográfica será un perfil completo que combina ambas familias y se selecciona como una unidad. [La ficha de fuentes](./design/fonts/README.md) conserva JetBrains Mono como referencia, no como perfil aprobado. Los perfiles alternativos siguen por definir; no hay una comparativa tipográfica HTML en el repositorio.

**Jerarquía tipográfica aprobada en la maqueta:** las métricas principales destacarán por tamaño y peso con intensidad media. El tamaño llevará el énfasis principal y el peso lo reforzará sin dominar el widget. Los títulos tendrán más presencia que las cifras de apoyo y permanecerán debajo de la métrica principal. Las cifras secundarias serán apenas menores que la principal para facilitar la comparación.

La muestra aprobada usa estos valores de referencia: métrica principal de 34–36 px con peso 600; cifras de apoyo de 33 px con peso 400; títulos de widget de 17–18 px con peso 700. Estos valores aún no son tokens de producción.

**Texto de apoyo aprobado:** el propietario eligió la variante «Más legible» para etiquetas, unidades, subtítulos y detalles. La muestra usa 14 px para etiquetas y subtítulos, y 12–13 px para detalles, con tonos neutros más claros. Algunos textos pueden ocupar líneas adicionales; validar la legibilidad y la densidad con contenido representativo.

Los proyectos de [Geist](https://github.com/vercel/geist-font#license) y [JetBrains Mono](https://www.jetbrains.com/lp/mono/) publican sus fuentes bajo SIL Open Font License 1.1. [La ficha de fuentes](./design/fonts/README.md) identifica los archivos de referencia y sus licencias; la aplicación incluye sus propias copias locales de Geist. Antes de distribuir otra fuente, se verificará la licencia de la versión incorporada.

### Personalización visual: dirección aprobada

**Decisión aprobada por el propietario:** el usuario podrá personalizar la interfaz dentro de un catálogo de opciones predefinidas. La configuración inicial conserva la identidad visual de Drasus: Geist Sans + Geist Mono y acento blanco.

La personalización comprende:

- **Fuentes:** el usuario podrá elegir perfiles tipográficos completos y predefinidos. Cada perfil coordina una familia para textos y otra para cifras; las alternativas a Geist se seleccionarán y probarán como pareja antes de ofrecerlas.
- **Acentos (aplazados por el propietario):** la exploración de temas cromáticos y de sus puntos de aplicación se retomará en otra etapa. El blanco permanece como acento inicial y se prefiere un color común por tema. Opciones pendientes de revisión: rojo, azul, verde, violeta, morado, cian, plata y dorado; dorado, no amarillo. Los colores semánticos conservarán su función.
- **Significados protegidos:** las ganancias, pérdidas, alertas y tipos de puerto del Canvas conservan su función. Los estados deben distinguirse también por texto, símbolos o formas.
- **Restauración:** el usuario tendrá una opción para volver al aspecto original de Drasus.

Al retomar esta exploración, cada acento deberá revisarse en contexto para evitar confusiones con ganancias, pérdidas, advertencias o estados críticos. Los colores de una serie financiera o de una alerta mantienen su significado aunque el usuario cambie el acento común.

Como referencias de alcance, [Notion](https://www.notion.com/help/customize-and-style-your-content) ofrece estilos tipográficos para el contenido de cada página, mientras que [VS Code](https://code.visualstudio.com/api/extension-guides/color-theme) permite modificar colores individuales y crear temas. Una selección acotada reduce las combinaciones por revisar; una edición detallada da más control y facilita temas personales, pero amplía la validación y el soporte. Su valor como ventaja competitiva para Drasus sigue siendo una hipótesis por investigar.

Los estados financieros y operativos necesitan conservar su significado y distinguirse también por texto, forma o símbolo. Los colores de los tipos de puerto y conexiones del Canvas siguen el catálogo de [ADR-0137](./adr/ADR-0137.md). Una futura alternativa de accesibilidad que modifique esos colores requiere validar el significado y conciliar el cambio con ese ADR. El tratamiento de gráficas y series aún está por diseñar.

Antes de ofrecer una combinación, habrá que revisarla con tablas, gráficas, controles, Canvas y estados representativos. La revisión debe incluir contraste sobre los fondos reales, foco, uso de teclado, escala de texto, alineación de cifras y legibilidad de signos y separadores. Los criterios [WCAG 1.4.1](https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html), [1.4.3](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html) y [1.4.11](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html) sirven para orientar esa validación; una propuesta o una maqueta no demuestra conformidad.

**Por confirmar:** perfiles tipográficos alternativos; almacenamiento, sincronización y momento de entrega. La selección y aplicación de temas de acento quedan aplazadas para otra etapa. Un archivo JSON o parámetros de entrada son posibilidades mencionadas, no una decisión de implementación. La edición libre de colores o de temas no forma parte del alcance aprobado.

**Vigente:** [ADR-0165](./adr/ADR-0165.md) establece Tailwind CSS como capa de estilos de React. Este documento conserva la definición canónica de tokens y componentes; el tema de Tailwind los proyecta a la interfaz. Los valores predeterminados del framework no fijan la identidad de Drasus. [ADR-0139](./adr/ADR-0139.md) establece la fuente única y la extracción de componentes reutilizables al aparecer un segundo consumidor real.

### Límite entre diseño e implementación

Antes de comparar HeroUI con shadcn/ui + Radix, este documento definirá los tokens, su significado y los comportamientos visuales compartidos que necesite el piloto. No contendrá nombres de clases Tailwind, recetas de clases ni código CSS. Cuando exista la superficie React/Tailwind, su tema y archivos CSS proyectarán los tokens a utilidades y recetas reutilizables; ambas bibliotecas consumirán la misma capa Drasus. Así se comparan los mismos controles sin duplicar valores ni estilos. Los nombres y valores de los tokens siguen pendientes de definición en este documento.

**Por confirmar:** valores del tema oscuro y contraste, escala tipográfica, espaciado, cuadrícula, tamaños, radios, elevación y niveles de densidad. La maqueta conserva el acabado y los bordes de NestIQ, aumenta la transparencia y extiende el halo por toda la superficie; falta validar su legibilidad y expresar los valores compartidos como tokens. La composición y densidad iniciales del Dashboard siguen Overview; falta convertirlas en medidas y reglas responsivas. También se decidirá si conviene un formato de intercambio compatible con [Design Tokens Community Group 2025.10](https://www.designtokens.org/technical-reports/); esa referencia no instala herramientas ni selecciona un formato para el proyecto.

Los nombres, estados y referencias estructuradas de tokens se conservan en [DESIGN.yaml](./DESIGN.yaml). Este documento explica sus decisiones y contexto; no contiene clases Tailwind ni sustituye los valores estructurados.

**Vigente:** [ADR-0136](./adr/ADR-0136.md) se enmienda para usar blanco como acento general inicial. El color de los tipos de puerto de [ADR-0137](./adr/ADR-0137.md) conserva su significado y su definición.

## 4. Catálogo de componentes

**Estado: sistema de componentes en formación.** La interfaz anterior Flutter y su catálogo de widgets a medida quedaron retirados con [ADR-0165](./adr/ADR-0165.md). La aplicación actual ya tiene componentes React para Dashboard, pero aún no una biblioteca compartida aprobada para todas las features. [ADR-0135](./adr/ADR-0135.md) ya no exige IDs para completar una Cáscara Visual: si existe una ficha en `components.specifications`, se referencia su clave YAML; si no, se especifica el rol y se implementa localmente para su primer consumidor.

`components.registry` organiza categorías como `button`, `icon_button` y `text_input`; no identifica implementaciones compartidas. Los componentes se extraen cuando exista un segundo consumidor real, según [ADR-0139](./adr/ADR-0139.md). El Banco de Verificación es un harness funcional para probar features por separado, no un catálogo visual; su contrato permanece en [ADR-0117](./adr/ADR-0117.md) y [ADR-0152](./adr/ADR-0152.md).

**Bibliotecas:** se hará un piloto visual con [HeroUI](https://heroui.com/en/docs/react/getting-started) y [shadcn/ui](https://ui.shadcn.com/docs) + [Radix](https://www.radix-ui.com/primitives/docs/overview/introduction), después de definir los tokens y comportamientos visuales que se compararán. La implementación de clases y componentes vive en CSS/Tailwind, no en este documento. La biblioteca general sigue por decidir hasta revisar el resultado. Se adoptan TanStack Table para tablas y TanStack Virtual para listas o tablas cuyo volumen justifique virtualización. La [comparación de bibliotecas](./design/bibliotecas-ui.md) registra alcance y criterios.

**Iconos:** el Dashboard usa SVG locales, pero no se ha elegido una biblioteca o sistema de iconos para el producto. Se explorará junto con el piloto de componentes, revisando cobertura, accesibilidad, consistencia visual, licencia y mantenimiento.

**Caché de datos de interfaz — aplazada:** Local-First y la persistencia del Core no bastan para decidir si React necesita una caché propia. No se selecciona TanStack Query ni una alternativa, y no se define política de persistencia. El tema se reabrirá cuando una superficie real de Tauri/React demuestre una necesidad concreta. Por ahora no corresponde registrarlo como deuda, SPIKE o ADR; el disparador queda en el [Roadmap](./ROADMAP.md).

Cada entrada deberá explicar su propósito, anatomía, variantes, estados, comportamiento, contenido, accesibilidad, adaptación a la ventana y tokens aplicables. Los componentes se describirán según su función y se diseñarán junto con los flujos que los necesiten.

### Inventario inicial por especificar

La siguiente lista organiza las primeras fichas; no asigna IDs ni afirma que los componentes estén implementados:

| Familia | Necesidad que cubre | Punto de partida |
|---|---|---|
| Navegación lateral | Cambiar de superficie y mantener el contexto | Preferencia del propietario por Overview |
| Selector segmentado | Elegir una opción dentro de un conjunto, como un período | Acabado aprobado de la pantalla 01 |
| Botón circular con icono | Activar una acción identificable | Acabado aprobado de la pantalla 01 |
| Búsqueda y menú de perfil | Encontrar contenido y acceder a opciones del usuario | Acabado aprobado de la pantalla 01 |
| Widget de Dashboard | Resumir información y permitir ajustar su densidad | Dashboard personalizable de ADR-0136 |
| Alerta urgente | Comunicar gravedad mediante color, etiqueta e icono | Preferencia del propietario |
| Tabla, gráfica y nodo | Presentar datos o relaciones para inspección | Forma y comportamiento por definir |

Cada ficha deberá concretar sus variantes, estados y reglas de uso antes de que una feature la cite como componente aprobado. Los elementos visibles en una maqueta no demuestran que exista una implementación.

## 5. Experiencia, estructura e interacción

El Canvas es la superficie central de composición del producto y su principal atractivo; el Dashboard es una superficie oficial complementaria para monitoreo y navegación. Cada feature arrastrable como nodo expone sus puertos de entrada y salida tipados; la información, las gráficas y el contenido del nodo dependen de su contrato y de lo que aplique. No toda feature requiere un nodo, una gráfica o una superficie propia. Los módulos sirven como presets para facilitar el inicio y los usuarios expertos pueden conectar features compatibles directamente, conforme a [ADR-0136](./adr/ADR-0136.md) y [ADR-0137](./adr/ADR-0137.md).

**Vigente:** [ADR-0136](./adr/ADR-0136.md) define el Dashboard como pantalla de inicio y centro de monitoreo. El usuario organiza widgets de features arrastrándolos en una cuadrícula *bento*. Los datos del Dashboard son de solo lectura y sus elementos navegables llevan al Canvas en el contexto correspondiente. El Canvas es único para las entidades y los procesos; las features se inspeccionan en un panel lateral. [ADR-0165](./adr/ADR-0165.md) fija los atajos de la aplicación y distingue la escala tipográfica global del zoom del Canvas.

**Canvas:** [ADR-0136](./adr/ADR-0136.md) selecciona React Flow como base de la superficie nodal. La elección no significa que la integración, el zoom anidado o los puertos ya estén implementados o verificados.

**Vigente:** [ADR-0166](./adr/ADR-0166.md) distingue presencia de actividad confirmada y define operaciones observables y recuperables para las mutaciones agénticas. La interfaz debe representar su estado real y su procedencia según ese contrato.

**Preferencia del propietario:** Drasus ofrece una distribución inicial del Dashboard que cada usuario puede adaptar con los widgets que necesita. Ganancias y pérdidas, desempeño del portafolio, avisos y riesgos, y resúmenes de generación y evaluación de estrategias tienen un peso visual similar en la disposición inicial. Las urgencias conservan su tratamiento de gravedad. Cuando aún no existan datos, los widgets aparecen y orientan al usuario para comenzar.

**Decisión del propietario:** el Dashboard de Drasus adopta la composición, distribución inicial, densidad y lenguaje visual de Overview: un widget principal con un panel lateral y tres widgets inferiores. Las superficies toman el acabado y los bordes de NestIQ, con más transparencia para que el halo también se perciba a través de ellas. Se conserva la navegación lateral y cada widget permite ajustar su densidad de forma independiente, tanto mediante un control visible como desde su menú. Las medidas, el orden del contenido específico y la adaptación a distintos tamaños de ventana siguen por validar.

**Preferencia del propietario:** reducir el texto visible mediante información contextual, *tooltips* y desplegables. Algunos paneles mostrarán una síntesis directa y otros revelarán más información al interactuar. La información relevante no debe depender exclusivamente del hover; las ayudas y acciones necesitan acceso mediante teclado. Al diseñar contenido que aparece con hover o foco, se revisará [WCAG 1.4.13](https://www.w3.org/WAI/WCAG22/Understanding/content-on-hover-or-focus.html). La jerarquía concreta de cada widget sigue por definir.

**Vigente:** [ADR-0167](./adr/ADR-0167.md) establece el selector global simple/avanzado en un menú accesible desde cualquier pantalla. Cambia qué controles de configuración se muestran, mantiene una sola configuración efectiva y conserva los valores ocultos. El modo inicial es simple. La densidad de cada widget es independiente de este selector.

**Preferencia del propietario:** la navegación principal será lateral, tomando como referencia la pantalla Overview. Se presenta como un riel de iconos que se expande para mostrar etiquetas al pasar el cursor y se contrae al retirarlo si no está fijada. Un clic permite fijarla abierta; también será accesible mediante teclado. **Por confirmar:** el manejo concreto del foco y la acción de teclado para fijarla, sus destinos y etiquetas, los flujos prioritarios, la ayuda contextual y el tratamiento de carga, error, desconexión y recuperación. Cada interacción se especificará con su activación, comportamiento y respuesta visible.

## 6. Visualización de datos

Drasus presenta datos y resultados que influyen en decisiones de inversión. Su diseño necesita distinguir el dato, su contexto y su estado de validación. Los cálculos y la lógica de dominio corresponden al Core según [ADR-0165](./adr/ADR-0165.md).

El propietario priorizó cuatro grupos para la disposición inicial: resultados de ganancias y pérdidas; desempeño del portafolio; avisos y riesgos; y generación y evaluación de estrategias. Son familias de información, no una lista cerrada de métricas ni evidencia de implementación.

**Propuesta por validar:** acompañar los resultados con su período y unidad; distinguir si la operación es real o simulada y si la información es histórica o en vivo; y diferenciar un dato ausente de uno desactualizado. Una simulación puede utilizar datos de mercado reales, por lo que el entorno de ejecución y la procedencia del dato se presentan por separado. El contexto de entorno y la actividad confirmada siguen [ADR-0123](./adr/ADR-0123.md) y [ADR-0166](./adr/ADR-0166.md). La definición y el cálculo de cada métrica pertenecen a su fuente de dominio.

**Por confirmar y validar:** familias de gráficas prioritarias, unidades, períodos, escalas, comparación de series, leyendas, datos ausentes, procedencia y alternativas accesibles. Las reglas se probarán con datos y escenarios representativos; una gráfica atractiva por sí sola no demuestra que una estrategia esté validada.

La [anatomía de gráficas de Carbon](https://www.carbondesignsystem.com/building-blocks/data-visualization/chart-anatomy) sirve como referencia para discutir ejes, leyendas y contexto; no implica adoptar sus componentes ni su identidad.

## 7. Composición y plantillas

El Dashboard usa la cuadrícula modular tipo *bento* y widgets reorganizables de [ADR-0136](./adr/ADR-0136.md). La composición y la densidad inicial siguen Overview: un widget principal, un panel lateral y tres widgets inferiores. NestIQ aporta el acabado y los bordes de las superficies; Drasus aumenta su transparencia para que el halo también se vea a través de los paneles. Todos los widgets comparten exactamente la misma opacidad, transparencia y borde en su material base. El usuario puede reorganizarlos y ajustar su densidad. **Por confirmar:** medidas responsivas y orden del contenido específico. La separación inicial sigue Overview y se formalizará en la escala de espaciado.

La primera versión se verificará con superficies representativas del producto, especialmente Dashboard, Canvas, inspector y tablas. Las plantillas son patrones de composición; consumen los fundamentos y componentes aprobados.

## 8. Movimiento y comportamiento

**Por confirmar:** cuándo el movimiento ayuda a comprender una transición o un estado, cuánto protagonismo visual debe tener y cómo se comporta con movimiento reducido. Las duraciones, curvas y efectos se definirán y verificarán con la dirección visual.

El Canvas mantiene los comportamientos de navegación vigentes en [ADR-0136](./adr/ADR-0136.md). Ese ADR también contiene tratamientos visuales específicos; cualquier cambio acordado requiere reconciliarlo con el canon.

## 9. Accesibilidad, contenido y localización

**Vigente:** la interfaz ofrece escala tipográfica global y el contenido se reacomoda sin quedar recortado, conforme a [ADR-0165](./adr/ADR-0165.md). El español y el inglés se mantienen a la par mediante recursos de localización, conforme a [ADR-0160](./adr/ADR-0160.md).

**Propuesta por confirmar:** usar [WCAG 2.2 nivel AA](https://www.w3.org/TR/WCAG22/) como objetivo para las superficies aplicables y verificar teclado, foco, contraste, lectores de pantalla, ampliación y alternativas a interacciones por arrastre. Una meta de diseño no demuestra conformidad: se necesita evaluar la interfaz implementada.

**Por confirmar:** voz de marca, tono para contextos de riesgo o error, terminología, formatos de datos y ayuda. Se definirán ejemplos en español e inglés para acciones, mensajes, estados vacíos y recuperación.

## 10. Guía para agentes de IA

Al generar una interfaz, un agente consulta el canon de producto y este archivo. Para la dirección ya acordada:

- Usa blanco como acento inicial y limita la personalización al catálogo predefinido de la sección de personalización visual. Reserva los colores de estado para los significados definidos; una urgencia roja incluye etiqueta e icono de gravedad.
- Usa Geist Sans para textos y Geist Mono para cifras como combinación predeterminada. La personalización ofrece fuentes y acentos predefinidos y revisados, y una opción para restaurar el aspecto original; las alternativas concretas siguen abiertas.
- Sigue Overview para la composición, distribución inicial, densidad, separación, controles y estilo general del Dashboard. Todos los widgets comparten exactamente la misma opacidad, transparencia y borde en su material base; no crees variantes por tipo. Toma de NestIQ el acabado y los bordes; aplica mayor transparencia y un campo de luz continuo que cruce cabecera, widgets y separaciones. Conserva la navegación lateral de Drasus y valida la legibilidad con información representativa. Las medidas responsivas y los tokens compartidos siguen por definir.
- Usa los tokens y componentes aprobados cuando estén definidos. Si falta una decisión necesaria, la señala y propone alternativas para revisión.
- Conserva el Dashboard de monitoreo con widgets personalizables y densidad independiente; el modo global simple/avanzado controla la visibilidad de configuración según [ADR-0167](./adr/ADR-0167.md).
- No presenta datos ficticios como resultados del Core ni una maqueta como una capacidad implementada. Las acciones sensibles y los estados de actividad siguen [ADR-0165](./adr/ADR-0165.md) y [ADR-0166](./adr/ADR-0166.md).

**Por confirmar:** reglas adicionales para seleccionar componentes, distribuir contenido, incorporar referencias y evaluar la calidad de una propuesta generada.

## 11. Ingeniería y mantenimiento

**Por confirmar:** responsables, proceso de propuesta y aprobación, versiones, catálogo interactivo, herramientas de diseño, intercambio de tokens y verificaciones visuales. Storybook, Figma y los generadores de tokens son opciones para evaluar; no son decisiones adoptadas por este documento.

El sistema crecerá junto con superficies y necesidades reales. Una modificación deberá identificar los componentes afectados, sus consumidores y cómo se verificará su aplicación.

## 12. Temas de la entrevista

La entrevista avanzará por bloques, con síntesis y confirmación antes de incorporar decisiones:

1. Visión del producto, usuarios y alcance.
2. Lectura de las imágenes y dirección visual.
3. Marca, color, tipografía y densidad.
4. Navegación, Dashboard, Canvas y flujos.
5. Gráficas, estados y decisiones de riesgo.
6. Interacción, movimiento, accesibilidad y contenido.
7. Componentes, tokens, herramientas y mantenimiento.

Las respuestas del propietario se distinguirán de las hipótesis y de los resultados de investigación con usuarios. Los detalles sin confirmar permanecerán abiertos.

## 13. Continuidad del trabajo

Esta sección permite retomar la construcción del sistema de diseño con otro agente. Las instrucciones vigentes del entorno y [AGENTS.md](../AGENTS.md) gobiernan la selección de modelos, herramientas y delegación.

### Contexto que debe consultar

1. Lee [AGENTS.md](../AGENTS.md), el índice [README.md](./README.md), esta explicación y la ficha estructurada [DESIGN.yaml](./DESIGN.yaml). Consulta [PRODUCT.md](./PRODUCT.md) o [PRODUCT-SIMPLE.md](./PRODUCT-SIMPLE.md) para entender el producto y su público.
2. Para arquitectura y contratos de interfaz, consulta [ADR-0135](./adr/ADR-0135.md), [ADR-0136](./adr/ADR-0136.md), [ADR-0137](./adr/ADR-0137.md), [ADR-0139](./adr/ADR-0139.md), [ADR-0160](./adr/ADR-0160.md), [ADR-0165](./adr/ADR-0165.md), [ADR-0166](./adr/ADR-0166.md) y [ADR-0167](./adr/ADR-0167.md), según el tema. Los ADR vigentes prevalecen sobre propuestas visuales.
3. Abre la [comparativa de referencias](./design/comparativa-materiales.html) para revisar las referencias Overview y NestIQ y la propuesta Drasus, que adopta la composición de Overview y solo toma de NestIQ el acabado de las superficies de widgets. Es una maqueta con datos ficticios y controles ilustrativos, no una aplicación conectada al Core.
4. Las imágenes `top1.webp` a `top3.webp` están en `../inspiration/` respecto de la raíz del repositorio y no están versionadas aquí. Las imágenes adicionales se compartieron en la conversación y pueden no estar disponibles en otra sesión. La comparativa y las decisiones de este documento permiten continuar; si hace falta comprobar un detalle del original ausente, solicítalo al propietario.

### Flujo de skills

| Momento | Skill | Uso y resultado |
|---|---|---|
| Diseño e investigación | [$ui-ux-designer](../.agents/skills/ui-ux-designer/SKILL.md) | Integra investigación UX, arquitectura de información, interacción, diseño visual y prototipos, sistema de diseño, contenido, accesibilidad, visualización de datos, marca, movimiento y adaptación a ventanas. Consulta las guías pertinentes. |
| Conciliación del canon | [$architect](../.agents/skills/architect/SKILL.md) | Identifica conflictos con ADR o SAD y actualiza sus fuentes cuando una decisión confirmada lo requiera. No es especialista de diseño. |
| Implementación y verificación | [$frontend-engineer](../.agents/skills/frontend-engineer/SKILL.md) y [$qa-engineer](../.agents/skills/qa-engineer/SKILL.md) | Implementan y verifican la UI contra el sistema aprobado y los contratos reales. No son categorías de diseño. |

Sigue la skill [$caveman](../.agents/skills/caveman/SKILL.md) para la comunicación conforme a AGENTS.md. Lee cada skill y sus recursos relevantes antes de aplicarla; el texto de esta tabla no sustituye sus instrucciones.

### Próximo paso y forma de retomar

1. Completar la validación de legibilidad y densidad con tablas, gráficas, contenido representativo y tamaños de ventana distintos. La composición, la distribución, la densidad, la separación, el material de los widgets y el halo ya tienen dirección aprobada.
2. Acordar el foco, la activación y la expansión de la barra lateral con teclado.
3. Evaluar perfiles tipográficos completos alternativos después de validar la escala inicial.
4. Definir valores y usos de los tokens del tema oscuro, los estados de los controles y las primeras fichas del catálogo. Separar decisiones confirmadas de propuestas y comprobaciones pendientes.
5. Retomar la selección y aplicación de temas de acento cuando el propietario reactive ese bloque.
6. Continuar con Dashboard, Canvas, visualización de datos, estados, accesibilidad, contenido y gobernanza. Actualizar este documento al confirmar cada bloque.

Prompt para retomar:

> Continúa el sistema de diseño desde `docs/DESIGN.yaml` (propiedades estructuradas) y `docs/DESIGN.md` (contexto y justificación). Consulta `AGENTS.md`, `docs/README.md` y los ADR pertinentes. Usa `$ui-ux-designer` y consulta las guías de los dominios que requiera el entregable. Entrevístame por bloques, resume lo entendido y confirma cada decisión. Conserva las preferencias registradas y señala lo que sigue abierto. Para comparar una propuesta visual, identifica los datos ficticios y sincroniza ambos documentos solo con las decisiones que confirme.
