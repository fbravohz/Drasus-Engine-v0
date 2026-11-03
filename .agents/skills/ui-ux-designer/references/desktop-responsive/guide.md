> Guía de ui-ux-designer. Las rutas de recursos son relativas a la raíz de la skill. Adapta los ejemplos y umbrales genéricos al canon de Drasus y al encargo.

# Adaptación a ventanas de escritorio

Define cómo conserva su utilidad la interfaz cuando cambia el espacio disponible. Diseña para el tamaño real de la ventana de la aplicación, no para la resolución física del monitor.

## Entradas

- Las tareas, prioridades y composición aprobada en `docs/DESIGN.yaml`.
- El contenido representativo, componentes y superficies que deben adaptarse.
- Tamaños de ventana, escalas y restricciones que el propietario haya definido. Si faltan, registra la decisión pendiente.

## Procedimiento

1. Identifica qué contenido y controles son esenciales en cada tarea.
2. Agrupa ventanas de prueba según el espacio de la aplicación; no inventes breakpoints antes de observar dónde falla el contenido.
3. Para cada rango, define si la interfaz reposiciona, redimensiona, refluye, oculta contenido secundario o cambia de patrón.
4. Conserva navegación, contexto, foco, acceso a datos y controles esenciales. No ocultes funciones críticas para resolver overflow.
5. Define desplazamiento, recorte y límites de paneles cuando el contenido necesite espacio bidimensional, como tablas densas o Canvas.
6. Revisa texto ampliado, escalado de pantalla, distribución densa y estados de navegación como parte de la revisión de accesibilidad.

## Entregable

Entrega una matriz de tamaños de ventana con prioridades, transformaciones de layout, overflow, navegación y verificaciones visuales pendientes.

## Revisión de contenido y entrada

- Prueba cada rango con contenido corto, largo y ausente; listas de distintos tamaños; cifras y unidades extensas; errores; y texto en los idiomas soportados. El contenido determina dónde es necesario cambiar la distribución.
- Cuando un panel comparte espacio con navegación, barras o un detalle, revisa también el ancho disponible para ese panel. El tamaño de la pantalla no determina por sí solo el tamaño del componente.
- Conserva el orden de lectura y foco cuando el contenido se reordena. Una acción que aparece con hover necesita una vía accesible con teclado y no debe depender exclusivamente del cursor.
- Para tablas y Canvas, documenta qué se mantiene visible, qué se desplaza y cómo se conserva el contexto. Diferencia un desplazamiento bidimensional intencional de contenido recortado por accidente.
- Para controles de arrastre o redimensionamiento, considera inicio, final, cancelación, pérdida de foco y liberación fuera del control. Después de una interrupción debe ser posible volver a operar. Define una alternativa sin arrastre cuando corresponda.
- Identifica la evidencia: ventana redimensionada, captura, teclado, entrada simulada o dispositivo real. Una captura muestra distribución; no demuestra por sí sola que un gesto o lector de pantalla funciona.

## Límites

- Esta guía define la adaptación del layout; la arquitectura de información define la jerarquía y el diseño de interacción define los cambios de comportamiento.
- El diseño de visualización de datos decide cómo conservar la lectura de datos en cada superficie.
- No asumas patrones móviles, no fijes dimensiones sin evidencia y no escribas CSS ni configuración Tailwind.

## Referencias

- [Microsoft Learn: Responsive design techniques](https://learn.microsoft.com/en-us/windows/apps/design/layout/responsive-design)
- [Microsoft Learn: Screen sizes and breakpoints](https://learn.microsoft.com/en-us/windows/apps/design/layout/screen-sizes-and-breakpoints-for-responsive-design)
- [W3C: Reflow](https://www.w3.org/WAI/WCAG21/Understanding/reflow.html)
