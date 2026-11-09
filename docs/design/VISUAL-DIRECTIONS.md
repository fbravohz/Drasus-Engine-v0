# Drasus Engine · Dirección de identidad e interfaz

**Estado:** propuesta para iteración; ninguna dirección está aprobada ni sustituye ADRs.  
**Versión:** 0.2 · 28 de octubre de 2026.  
**Alcance:** identidad de escritorio, experiencia de uso y fundamentos visuales. No implementa producto, permisos ni lógica financiera.  
**Comparativa:** [abrir las tres propuestas](./visual-directions.html). Datos y actividad de las maquetas son ficticios.

## 1. Recomendación y formato de trabajo

Recomiendo **Observatorio**, la propuesta A: una herramienta de investigación precisa, sobria y reconocible, con acento cian y superficies oscuras. Comunica instrumentación y evidencia; funciona tanto para explorar estrategias como para monitorear operaciones. Su principal costo es la familiaridad del lenguaje oscuro en herramientas técnicas: la diferenciación debe venir de la composición, la tipografía y la trazabilidad visible.

El documento adecuado para esta etapa es una **dirección de diseño de producto**: combina un brief de marca, alternativas visuales y una especificación inicial de experiencia. Un manual de marca cubre identidad; un sistema de diseño cubre reglas y componentes reutilizables. Ninguno sustituye el descubrimiento y la validación de los flujos.

Propongo esta secuencia de entregables, sin convertirla todavía en compromiso de implementación:

1. **Dirección de diseño:** este documento y tres propuestas comparables. Elegir personalidad y prioridad de uso.
2. **Prototipo de flujos:** tareas completas con estados normales, errores y recuperación. Validar con personas representativas.
3. **Sistema de diseño v1:** tokens aprobados, biblioteca de componentes, patrones, accesibilidad y reglas de contenido.
4. **Especificación por Feature:** definir la vista/observable del Banco, el widget y el nodo Canvas según contratos reales, incluida plomería.

Este documento permanece como registro de alternativas. Una vez elegida la dirección, conviene crear una guía específica del sistema aprobado y enlazarla desde aquí. Evita convertir tres alternativas incompatibles en instrucciones activas.

## 2. Producto que debe expresar

### Hechos del canon

- Drasus abarca datos, generación, validación, incubación, portafolios, ejecución y revisión; la validación rigurosa es central en su narrativa. [Producto](../PRODUCT-SIMPLE.md).
- El Dashboard monitorea; la edición de lógica ocurre en un Canvas único con inspector lateral. El nombre comercial del Canvas sigue abierto. [ADR-0136](../adr/ADR-0136.md).
- El banco global admite una tabla masiva; una Selección temporal no equivale a un Portafolio. [ADR-0153](../adr/ADR-0153.md).
- La interfaz nueva usa Tauri/React y presenta resultados del Core por gRPC. [ADR-0165](../adr/ADR-0165.md).
- La actividad agéntica muestra identidad, estado confirmado, conflicto, recuperación y alcance autorizado. Presencia y actividad durable son conceptos distintos. [ADR-0166](../adr/ADR-0166.md).
- El Banco de Verificación distingue formato inválido de fallo operativo; sus resultados reales llegan del Core. [ADR-0152](../adr/ADR-0152.md), enmendado en transporte por ADR-0165.

### Interpretación de diseño

**Personalidad propuesta:** precisión, autonomía, claridad y prudencia. El producto debe sentirse como un instrumento que explica su evidencia. La interfaz debe dar prioridad a tres preguntas: qué estoy observando, con qué evidencia y qué puedo hacer desde aquí.

**Hipótesis de usuarios:** quien investiga necesita comparar y comprender; quien opera necesita contexto y continuidad; quien administra necesita alcance, permisos y procedencia. Son modos de trabajo derivados del producto, pendientes de investigación con usuarios.

**Idea verbal de marca:** «Investiga. Comprueba. Opera.» Es una propuesta de firma, no una promesa de resultados.

### Límites de esta exploración

Las paletas, radios, tipografías, tratamientos de conexiones y firmas gráficas se diseñaron desde cero. Existen valores visuales históricos en ADR-0136/0137 que podrían entrar en conflicto con estas propuestas. Se conservan los comportamientos funcionales; aprobar nuevos colores de puertos, fondos o efectos requerirá reconciliar esas decisiones en su fuente canónica antes de implementar. La propuesta no modifica esos ADRs.

No se consultó el documento de diseño eliminado, su historial ni las skills de frontend/UI/UX. No se reutilizó Flutter como referencia visual. Se revisó intención documental, no disponibilidad de capacidades en código: una pantalla propuesta no demuestra que su backend ya exista.

## 3. Tres direcciones

### A · Observatorio — instrumentación y profundidad

**Sensación:** laboratorio técnico contemporáneo. Oscuro frío, cian preciso, contraste tranquilo. El acento ocupa controles y selección; los datos conservan protagonismo.

**Identidad:** símbolo propuesto con dos planos desplazados que delimitan una D abierta y un punto de referencia. Evoca medición y profundidad. Logotipo tipográfico «drasus» en minúsculas y «ENGINE» como descriptor pequeño con espaciado amplio. El símbolo deberá dibujarse y probarse en vector antes de aprobarse.

**Composición:** navegación lateral de 208 px, cabecera contextual de 56 px, área de trabajo amplia, inspector de 320–400 px. Dashboard en cuadrícula asimétrica: evidencia principal a la izquierda, calidad y actividad a la derecha. Evitar convertir cada número en tarjeta.

**Tipografía propuesta:** IBM Plex Sans para interfaz; IBM Plex Mono para valores, identificadores y registros. Títulos 28/34 px; secciones 18/24; cuerpo 14/21; etiquetas 12/18. Empaquetar fuentes localmente y verificar licencia antes de distribuir.

| Token propuesto | Valor | Aplicación |
|---|---|---|
| `surface.base` | `#0B1118` | Fondo |
| `surface.panel` | `#121C27` | Paneles |
| `surface.raised` | `#1B2938` | Menús y selección suave |
| `text.primary` | `#E7EFF7` | Lectura principal |
| `text.secondary` | `#A6B5C5` | Etiquetas y metadatos |
| `accent.primary` | `#67D7E8` | Acción y foco |
| `accent.onPrimary` | `#0B1118` | Texto sobre acento |
| `border.subtle` | `#304356` | Separadores decorativos |
| `border.control` | `#6F879E` | Campos y límites funcionales |

**Componentes distintivos:** widgets con encabezado editorial, tablas de filas de 36 px, nodos de borde fino y radio 8 px; chips con icono y texto; actividad como secuencia de eventos. Gráficas con línea cian, comparación gris discontinua y banda de incertidumbre explícita.

**Dashboard:** prioriza evidencia de investigación y estado de datos. **Canvas:** topología clara y puertos rotulados. **Banco:** tabla y comparador lateral. **Operación:** cabecera persistente de entorno y cuenta; las alertas preceden a métricas decorativas.

**Fortaleza:** equilibrio entre profundidad técnica y lectura. **Riesgo:** parecer una terminal genérica si se abusa de monoespaciado o se llenan todos los huecos.

### B · Cuaderno — claridad analítica y criterio editorial

**Sensación:** cuaderno de investigación profesional. Grafito cálido, tinta marfil y acento lavanda. Todas sus superficies son oscuras. Explica mejor y reduce la impresión de una consola especializada.

**Identidad:** monograma D construido con dos corchetes y un eje de lectura. Wordmark en mayúsculas pequeñas o caja mixta, con espaciado moderado. Una retícula de márgenes y referencias funciona como firma visual.

**Composición:** cabecera horizontal de 64 px con navegación visible; índice contextual plegable. Área central con filas amplias, encabezados narrativos y anotaciones. Inspector lateral con fondo grafito cálido y texto marfil. El Canvas sigue siendo único, con puntos discretos; la diferencia es editorial, no una nueva arquitectura.

**Tipografía propuesta:** Source Sans 3 para controles; Source Serif 4 solo para títulos de informes; IBM Plex Mono para valores. Títulos 30/36 px; secciones 20/28; cuerpo 15/23; etiquetas 13/18. Nunca usar serif en celdas ni controles.

| Token propuesto | Valor | Aplicación |
|---|---|---|
| `surface.base` | `#19171E` | Fondo grafito cálido |
| `surface.panel` | `#24212B` | Paneles |
| `surface.raised` | `#332D40` | Selección suave |
| `text.primary` | `#EEE9E2` | Texto marfil |
| `text.secondary` | `#BAB1C5` | Metadatos |
| `accent.primary` | `#C0A5F5` | Acciones y foco |
| `accent.onPrimary` | `#19171E` | Texto sobre acento |
| `border.subtle` | `#51465F` | Separadores decorativos |
| `border.control` | `#93849F` | Campos y límites funcionales |

**Componentes distintivos:** bloques de hallazgos con fuente y fecha, tablas de 44 px, radio 4 px, botones sólidos con etiqueta verbal, comparaciones alineadas y leyendas directas. Sombras reservadas para superposición. Los informes se sienten parte de la aplicación.

**Dashboard:** resumen narrativo y lista de hallazgos. **Canvas:** nodos como fichas con encabezado de tinta. **Banco:** protagonismo de tablas y notas de metodología. **Operación:** separador y rótulo de entorno persistentes; nunca depender de un fondo oscuro para reconocer dinero real.

**Fortaleza:** lectura prolongada y explicación de resultados. **Riesgo:** la composición amplia reduce la cantidad de datos visibles; hay que validar que la lectura editorial compense ese espacio.

### C · Taller — control industrial y eficiencia

**Sensación:** mesa de instrumentos industrial. Grafito neutro, acento ámbar, estructura marcada y mayor densidad. Favorece supervisar procesos concurrentes.

**Identidad:** D modular construida con segmentos rectangulares y una separación de ensamblaje. Descriptor ENGINE alineado a una pequeña escala de divisiones. Geometría consistente, sin engranes, velas financieras ni efectos metálicos.

**Composición:** riel lateral de 64 px con etiquetas al expandir; barra superior de 48 px, herramientas contextuales de 40 px, paneles divisibles. Dashboard organizado por franjas operativas. El inspector mantiene el lugar de la edición y la actividad ocupa una franja inferior plegable.

**Tipografía propuesta:** Barlow Semi Condensed para títulos y navegación; Barlow para lectura; IBM Plex Mono para datos. Títulos 26/32 px; secciones 18/24; cuerpo 14/20; etiquetas 12/18. Evitar condensada en párrafos y datos.

| Token propuesto | Valor | Aplicación |
|---|---|---|
| `surface.base` | `#151617` | Fondo |
| `surface.panel` | `#202224` | Paneles |
| `surface.raised` | `#2C2F32` | Selección suave |
| `text.primary` | `#F1F0EA` | Lectura principal |
| `text.secondary` | `#B7B9BC` | Metadatos |
| `accent.primary` | `#F2BE62` | Acción y foco |
| `accent.onPrimary` | `#151617` | Texto sobre acento |
| `border.subtle` | `#454B50` | Separadores decorativos |
| `border.control` | `#89939B` | Campos y límites funcionales |

**Componentes distintivos:** radio 2 px, tablas compactas de 32 px, encabezados firmes, medidores con escala y unidad, canales de proceso y paneles ajustables. Evitar marcos alrededor de cada cifra.

**Dashboard:** procesos, colas y calidad de conexión. **Canvas:** nodos compactos y modo mixer del módulo, conservando la equivalencia con su grafo. **Banco:** columnas fijadas y operaciones por selección. **Operación:** alertas explícitas con destino y consecuencia.

**Fortaleza:** eficiencia para usuarios frecuentes. **Riesgo:** curva inicial mayor; el ámbar de marca puede confundirse con advertencias. Reservar la advertencia a un icono triangular y texto visible; validar esa distinción o cambiar el acento.

### Comparación para decidir

| Criterio | Observatorio | Cuaderno | Taller |
|---|---|---|---|
| Centro de gravedad | Evidencia y exploración | Comprensión y lectura | Supervisión y procesos |
| Organización | Lateral + cuadrícula asimétrica | Horizontal + documento | Riel + franjas instrumentales |
| Densidad inicial | Media | Cómoda | Compacta |
| Identidad | Técnica y serena | Editorial oscura y accesible | Industrial y directa |
| Principal costo | Diferenciarse de otras herramientas oscuras | Equilibrar espacio y densidad | Enseñar navegación y jerarquía |

Son hipótesis de diseño, no resultados de pruebas. Recomiendo A como sistema completo; las anotaciones de B pueden incorporarse como patrón de informes. Mezclar paletas y estructuras de las tres diluiría identidad y consistencia.

## 4. Arquitectura de información compartida

Navegación propuesta: **Dashboard, Canvas, Banco de estrategias, Actividad, Verificación y Configuración**. Son accesos a superficies y utilidades, no nuevos módulos del dominio. Banco y Verificación conservan sus responsabilidades canónicas.

- **Cabecera:** proyecto o contexto, entorno explícito, cuenta cuando corresponda, conexión y acceso a comandos.
- **Dashboard:** lectura y navegación contextual. Personalizar disposición de widgets no modifica lógica ni órdenes.
- **Canvas:** breadcrumb, filtros de visibilidad, cámara única, puertos tipados, estado del objeto e inspector derecho. Navegar entre Workspaces no crea nuevos lienzos.
- **Banco:** búsqueda, filtros, columnas, orden, selección y comparación. Selección temporal y Portafolio tienen etiquetas distintas. Seleccionar filas no promueve ni ejecuta nada.
- **Actividad:** actor, objeto, acción, estado, tiempo, detalle y referencia de operación. Separar «agente conectado» de «operación confirmada».
- **Verificación:** catálogo izquierdo; formulario, validación y resultado a la derecha. Distinguir input inválido, permiso denegado, fallo del Core y resultado recibido.

A 1440 × 900, mostrar navegación, contenido e inspector cuando exista espacio. A 1280 × 720, plegar navegación y permitir paneles ajustables. En ventanas más estrechas, usar inspector superpuesto cerrable y tabla con desplazamiento propio. No escalar toda la pantalla como una imagen. La aplicación objetivo es de escritorio; una experiencia móvil queda fuera del alcance.

## 5. Flujos prioritarios a prototipar

### Preparar datos y comprobar calidad

Elegir fuente propia; mostrar capacidades y requisitos; validar parámetros; iniciar; consultar progreso; revisar calidad y procedencia; abrir el objeto en Canvas. Si falla, conservar configuración y explicar qué se puede corregir. Drasus provee conectores, no inventa disponibilidad de datos.

### Investigar y comparar candidatas

Abrir un Pipeline del Workspace en el mismo Canvas; inspeccionar una feature; configurar y guardar cambios; solicitar corrida; seguir su actividad; abrir resultados en Banco; filtrar y comparar; conservar procedencia y contexto de evaluación. Una curva atractiva nunca reemplaza el estado de validación.

### Revisar antes de operar

Desde monitoreo, navegar al objeto en Canvas; revisar entorno, cuenta, alcance y evidencia; solicitar la acción permitida; mostrar confirmación con objeto y consecuencias; esperar confirmación real; consultar actividad. El Dashboard conserva su carácter de lectura. La maqueta no simula ejecución ni da por aprobada una autorización.

### Recuperar continuidad

Al perder conexión, conservar la última vista como desactualizada y mostrar cuándo se recibió. Al reconectar, reconstruir desde el Core antes de declarar éxito. Si una acción externa no se confirmó, mostrar «Requiere conciliación»; no ofrecer un reintento que aparente ser inocuo.

## 6. Fundamentos del sistema propuesto

### Color semántico

Separar color de marca, estado, serie de gráfica y tipo de puerto. El acento no significa éxito. Un tipo de puerto no cambia de significado al seleccionar el nodo. Los valores de puertos se ratificarán contra ADR-0137 antes de implementarlos.

| Rol | A y C, sobre panel oscuro | B, sobre panel oscuro |
|---|---|---|
| Éxito | `#78D9AB` | `#78D9AB` |
| Advertencia | `#FFD080` | `#FFD080` |
| Error | `#FF949D` | `#FF949D` |
| Información | `#8EC5FF` | `#8EC5FF` |

Usar estos colores como texto o marca sobre su panel; los fondos de badges se definirán aparte. Un estado incluye icono y etiqueta. Positivo/negativo financiero debe incluir signo, unidad y período; un resultado positivo no significa que una estrategia esté validada.

### Espaciado, tamaño y elevación

Escala de espacio: 4, 8, 12, 16, 24, 32 y 48 px. Controles cómodos de 40 px; compactos de 32 px con área accionable suficiente. Nodos y filas pueden crecer con la escala tipográfica. Proponer modos cómodo y compacto después de validar la dirección, conservando jerarquía y accesibilidad.

Tres capas: superficie base, panel y superposición. Distinguir paneles por borde y tono; reservar sombras para menús y diálogos superpuestos. El foco requiere un indicador explícito; una sombra decorativa no lo sustituye.

### Iconografía, gráficas y movimiento

Iconos lineales de una sola familia, tamaño nominal 18–20 px, con etiquetas en acciones esenciales. La elección final de librería y su licencia queda pendiente. Probar símbolo de marca a 16, 24 y 128 px, en monocromo y junto al nombre antes de aprobarlo.

Gráficas con ejes, unidades, intervalo, zona horaria y procedencia. Comparaciones distinguidas por trazo y etiqueta además de color. Señalar muestras faltantes; no convertirlas en ceros. Presentar rangos de incertidumbre como rangos, sin inventar precisión ni probabilidades.

Movimiento propuesto: 120–180 ms en controles; 180–240 ms en paneles. El movimiento de cámara debe conservar orientación y admitir reducción de movimiento. Evitar parpadeo continuo, contadores animados y partículas. Esto difiere de ciertos efectos históricos de ADR-0136 y requiere reconciliación antes de implementación.

### Contenido y localización

Escribir acciones concretas: «Comparar selección», «Abrir en Canvas», «Revisar conflicto». Mensajes: qué pasó, qué se conservó y qué puede hacer el usuario. Nunca exponer secretos ni copiar texto no confiable del agente en actividad.

Números con cifras tabulares; precisión por magnitud; moneda explícita; guion largo o «Sin dato» para ausencia. Conservar zona horaria y distinguir hora de mercado de hora de recepción. Preparar traducción y expansión de etiquetas; las maquetas están en español y no fijan política de idioma.

## 7. Inventario inicial de componentes

| Componente | Anatomía y comportamiento propuestos | Estados por diseñar |
|---|---|---|
| Shell | Navegación, contexto, entorno y conexión | Conectado, reconectando, sin permisos |
| Widget | Título, dato, unidad, fuente, antigüedad, navegación | Cargando, vacío, desactualizado, error |
| Tabla | Encabezado, orden, filtro, selección y resumen | Vacía, sin coincidencias, selección parcial |
| Nodo | Nombre, tipo, estado, hasta 4–5 valores, puertos | Reposo, seleccionado, procesando, inválido |
| Inspector | Contexto, campos, cambios y acción explícita | Limpio, modificado, inválido, conflicto |
| Campo | Etiqueta persistente, unidad, ayuda y error | Foco, válido, inválido, solo lectura |
| Botón | Verbo y objeto cuando haga falta | Reposo, hover, foco, ocupado, deshabilitado |
| Evento | Actor, objeto, estado y referencia | Solicitado, confirmado, rechazado, desconocido |
| Diálogo sensible | Entorno, cuenta, objeto, alcance y consecuencias | Revisión, envío, rechazo, conciliación |
| Resultado | Resumen, evidencia y detalle expandible | Input inválido, fallo, recibido, no disponible |
| Gráfica | Título, ejes, unidades, series y fuente | Sin datos, parcial, actualizado, desactualizado |
| Notificación | Hecho breve y acceso al contexto | Informativa, advertencia, error |

Un aviso temporal nunca es la única evidencia de una operación. Al deshabilitar una acción, explicar el motivo sin depender de hover. Guardar configuración y ejecutar un trabajo deben verse como acciones diferentes.

## 8. Accesibilidad y verificación

Objetivo propuesto: WCAG 2.2 AA como referencia de interfaz, sin afirmar conformidad hasta auditar la aplicación. W3C establece contraste de 4.5:1 para texto normal y 3:1 para texto grande, con excepciones. También exige alternativas al color y define tamaño mínimo de objetivos. [WCAG 2.2](https://www.w3.org/TR/WCAG22/).

Validar combinaciones reales de texto, fondo, foco y estado; no basta aprobar la paleta aislada. Proponer controles de al menos 32 px, foco visible de 2 px y zonas más amplias para operaciones sensibles. Estos valores son decisiones de diseño propuestas, no una descripción literal de todos los mínimos AA.

- Navegación completa por teclado; orden de foco coherente y restauración al cerrar paneles.
- Alternativa de lista e inspector para comprender y editar relaciones sin arrastrar exclusivamente.
- Lectura con aumento de texto al 200%, sin perder acciones ni contexto. Respetar los atajos de ADR-0165 y distinguir escala tipográfica de zoom del Canvas.
- Etiquetas de lector de pantalla para estado, progreso, tablas y gráficos. No anunciar cada tick ni saturar la lectura con actividad.
- Verificar daltonismo, movimiento reducido, filas seleccionadas, errores y texto secundario.

## 9. Cómo elegir y mejorar

Comparar las tres direcciones con el mismo contenido. La maqueta ofrece Dashboard, Canvas y Banco con datos ficticios constantes; las diferencias muestran organización, densidad y personalidad. No cubre todos los comportamientos del producto.

Prueba propuesta con cinco personas representativas, útil para detectar problemas iniciales, sin tratar ese número como garantía estadística:

1. Encontrar el entorno y detectar información desactualizada.
2. Explicar por qué una candidata requiere revisión, sin interpretar rentabilidad como aprobación.
3. Abrir el contexto correcto desde Dashboard y localizar el inspector.
4. Seleccionar y comparar candidatas sin confundir Selección con Portafolio.
5. Identificar actor y resultado confirmado de una operación.

Registrar errores, dudas, tiempo por tarea y comprensión del resultado. Pedir preferencia estética después de las tareas. Elegir la dirección por claridad de uso y personalidad, conservando comentarios textuales y cambios observables entre iteraciones.

### Decisiones pendientes del propietario

- Dirección base: recomiendo Observatorio.
- Balance inicial entre investigación y supervisión operativa.
- Nombre visible del Canvas: conservar «Canvas» mientras siga abierto.
- Todas las direcciones usan fondos oscuros por indicación del propietario; validar el matiz y contraste preferidos.
- Aprobación futura del símbolo y de la firma verbal, tras pruebas de legibilidad y revisión de disponibilidad de marca.

No se requiere resolverlas para revisar estas alternativas. Sí se requiere una selección antes de producir tokens y componentes de producción.

## 10. Estado de la entrega

Entregados: tres direcciones, paletas, composición, tipografía, conceptos de marca, patrones de interacción, inventario de componentes, flujos y plan de validación; comparativa local sin servicios externos.

Pendientes deliberados de esta fase: dibujo final de logotipo, investigación con usuarios, estados interactivos completos, validación de lectores de pantalla y reconciliación de reglas visuales históricas. Son trabajo de diseño posterior; no se declaran como deuda de producto construido.

La maqueta no prueba gRPC, rendimiento, persistencia, permisos ni disponibilidad de features. La recomendación es criterio de diseño del autor, sin auditoría independiente ni selección final del propietario.

### Verificación de esta revisión

Se comprobaron referencias locales, sintaxis JavaScript con `node --check` y ausencia de errores de espacio con `git diff --check`. Cálculo de luminancia relativa sRGB para las combinaciones principales, en orden texto/panel, texto secundario/panel y texto sobre acento:

| Propuesta | Texto principal | Texto secundario | Botón de acento |
|---|---|---|---|
| Observatorio | 14.81:1 | 8.22:1 | 11.25:1 |
| Cuaderno | 13.10:1 | 7.67:1 | 8.41:1 |
| Taller | 13.98:1 | 8.12:1 | 10.63:1 |

Esto no verifica toda combinación ni declara conformidad WCAG. No se pudo inspeccionar render e interacciones en navegador: la herramienta devolvió `Browser is not available: iab` y el inventario de navegadores conectados estaba vacío. La revisión visual a distintos tamaños y la prueba manual de controles quedan pendientes. La maqueta usa fuentes locales de sistema para abrir sin descargas; las familias de cada dirección siguen siendo propuestas del documento.
