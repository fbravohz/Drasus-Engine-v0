# Sistema de Diseño

Fuente de verdad visual de Drasus Engine. Aplica a la futura aplicación de escritorio Tauri 2 + React/TypeScript; la implementación usa tokens CSS y componentes React. La arquitectura y el transporte viven en ADR-0165; este documento no los duplica.

## Principios

1. **El token es la única fuente de valor visual.** No hay colores, tamaños, radios ni duraciones literales fuera del sistema de tokens.
2. **Componente = contrato funcional.** Un componente recibe datos ya resueltos, no calcula dominio ni hornea tema. Se extrae solo al segundo consumidor real (ADR-0139).
3. **Sobriedad por defecto.** Glow, gradiente, blur y animación solo comunican estado, foco o jerarquía.
4. **Semántica antes que dibujo.** Usa HTML semántico, foco, selección, teclado y scroll nativos antes de crear un canvas o interacción propia.
5. **Terminado se demuestra.** Una pantalla requiere evidencia ejecutable y visual de sus estados, no solo código.

## Tokens

Los tokens se publican como variables CSS y se consumen mediante nombres semánticos; nunca se copian sus valores a una feature.

### Superficies y texto

| Token | Valor | Uso |
|---|---|---|
| `canvas` | `#0B0D12` | Fondo raíz. |
| `surface` | `#12141B` | Panel, tabla, formulario y tarjeta. |
| `surface-raised` | `#181B24` | Hover, celda activa y elemento elevado. |
| `border` | `#242832` | División estructural. |
| `border-strong` | `#343A47` | Foco y selección. |
| `overlay` | `#12141BE6` + blur | Menú, modal, tooltip y popover; no contenido denso. |
| `text-primary` | `#EDEFF3` | Títulos y valores destacados. |
| `text-secondary` | `#A6ACBA` | Cuerpo y cabeceras. |
| `text-muted` | `#6B7180` | Metadatos y placeholder. |

Un único acento configurable tiñe foco, selección y navegación; no sustituye colores semánticos. Default: `#4C8DFF`.

| Estado | Token | Valor | Significado |
|---|---|---|---|
| Óptimo | `status-ok` | `#3DD68C` | Correcto o completado. |
| En curso | `status-pending` | `#7C8CFF` | Calculando o incubando. |
| Alerta | `status-warn` | `#E8A93D` | Atención requerida. |
| Crítico | `status-error` | `#E8543D` | Fallo o riesgo. |

Un chip de estado usa fondo al 12 %, borde al 35 % y texto del token; radio de 6 px. `999px` solo para un estado vivo con pulso.

### Tipografía, espaciado y radios

- `mono`, 13 px: números, fechas, IDs, símbolos y porcentajes; alineados a la derecha cuando formen columnas.
- `body`, 14 px: controles, texto y navegación.
- `heading`, 18–24 px, peso 600: títulos.
- La interfaz usa una escala tipográfica global. `Ctrl++` (o `Ctrl+=` cuando el teclado requiera `Shift`) aumenta el tamaño, `Ctrl+-` lo reduce y `Ctrl+0` restaura el valor base.
- La escala se aplica a todas las superficies de texto y reacomoda el contenido sin recortes. Es independiente del zoom del Canvas, que solo cambia la cámara del lienzo.
- Espaciado: `4 · 8 · 12 · 16 · 24 · 32 · 48 · 64` px.
- Radios: panel 10 px; overlay 12 px; botón/input 8 px; chip 6 px.

### Atajos de teclado

La interfaz conserva convenciones de escritorio y respeta el foco activo:

| Contexto | Atajo | Acción |
|---|---|---|
| Global | `Ctrl+Shift+P` | Abrir la paleta de comandos. |
| Global | `Ctrl+P` | Abrir rápidamente una vista o entidad. |
| Global | `Ctrl+F` / `Ctrl+Shift+F` | Buscar en la vista actual / en toda la aplicación. |
| Global | `Ctrl+S` | Guardar cambios editables. |
| Global | `Ctrl+Z` / `Ctrl+Y` | Deshacer / rehacer cambios reversibles. |
| Global | `Esc` | Cerrar, cancelar o quitar el foco de una superficie. |
| Global | `F1` / `Ctrl+Shift+?` | Abrir ayuda y listado de atajos. |
| Global | `Ctrl+Tab` | Cambiar entre vistas recientes. |
| Canvas con foco | `0` / `Home` | Ajustar el lienzo a la ventana / centrar la vista. |
| Canvas con foco | `+` / `-` | Acercar / alejar el zoom del lienzo. |
| Canvas con foco | `Espacio` + arrastre | Desplazar la cámara. |
| Canvas con foco | `Ctrl+C` / `Ctrl+V` | Copiar / pegar nodos seleccionados. |
| Canvas con foco | `Delete` | Eliminar la selección; exige confirmación si hay conexiones. |

Las acciones sensibles —pausar ejecuciones, cancelar operaciones o actuar sobre capital— aparecen en la paleta de comandos con validación de permisos, confirmación explícita y registro de auditoría. No tienen un atajo directo de una sola tecla.

## Layout y comportamiento

- Flexbox y Grid son la base. Posicionamiento absoluto solo para overlays, capas de una visualización o decoración aprobada.
- Formularios y lectura tienen ancho máximo; dashboards y tablas pueden ocupar el ancho disponible.
- Un panel mantiene un eje, alineación y escala de espacio coherentes.
- Texto de datos es seleccionable; los controles tienen nombre accesible, foco visible, orden de tabulación, `Enter`/`Esc` donde aplique y área interactiva mínima de 48 × 48 px.
- Overflow con scrollbar visible en escritorio. Tabla, nodo y campo de texto ofrecen menú contextual cuando exista una acción aplicable.
- Todo componente interactivo define, cuando aplique: default, hover, focus, pressed, disabled, loading, error y empty.
- Contraste mínimo: 4.5:1 para texto normal; 3:1 para texto grande, iconos y bordes interactivos.

## Movimiento y visualización de datos

- La animación comunica continuidad, feedback o estado. Overlay: ~200 ms; movimiento espacial: ~300–460 ms; acción de botón: ~160 ms.
- Respeta `prefers-reduced-motion`: conserva cambios de color/opacidad y elimina desplazamiento o escala no esencial.
- Los cálculos, agregación, downsampling, throttling y backpressure viven en Rust antes del contrato gRPC. React visualiza resultados, no recalcula métricas de trading.
- Canvas, WebGL o librerías de gráfica se permiten solo si la semántica HTML/SVG no alcanza el volumen o interacción requeridos; cada caso declara presupuesto de puntos, accesibilidad y degradación.

## Verificación y gobernanza

Antes de cerrar una superficie: prueba datos vacíos, error, carga, textos largos, números grandes, tamaño mínimo de ventana, navegación por teclado, la escala tipográfica y cada atajo aplicable al contexto; valida la jerarquía con una captura real. El catálogo de componentes nacerá con la nueva UI: hasta entonces, una Cáscara Visual marca cada componente como existente o a construir, sin inventar rutas ni APIs.

El UI Designer mantiene este documento. Cambiar tokens fundamentales o la arquitectura visual requiere ADR; el detalle de una feature vive en su `## Cáscara Visual`.
