> Guía de ui-ux-designer. Las rutas de recursos son relativas a la raíz de la skill. Adapta los ejemplos y umbrales genéricos al canon de Drasus y al encargo.

# Diseño de movimiento

Define movimiento solo cuando ayude a entender una acción, un cambio de estado, una relación espacial o una actualización. Prioriza una interfaz densa que responda de inmediato.

## Entradas

- El flujo y los estados definidos por el diseño de interacción.
- La superficie y las reglas vigentes de `docs/DESIGN.yaml`.
- Las preferencias del sistema operativo y los requisitos de accesibilidad aplicables.

## Procedimiento

1. Indica qué información o relación comunica el movimiento. Si no mejora la comprensión, omítelo.
2. Describe el evento, el estado inicial y final, el origen, el desplazamiento y qué puede interrumpirlo.
3. Recomienda ritmo y curva según frecuencia, urgencia, distancia y densidad de la superficie. No impongas duraciones universales.
4. Define una alternativa clara cuando el usuario solicite movimiento reducido o el sistema lo indique. Conserva la información y la respuesta funcional.
5. Evita movimiento decorativo continuo, parallax y animaciones que bloqueen entrada o lectura.
6. Valida la propuesta frente al diseño de interacción y la revisión de accesibilidad antes de convertirla en tokens o comportamiento implementado.

## Entregable

Entrega una especificación por transición: propósito, activador, inicio y fin, comportamiento espacial, ritmo, interrupción y variante de movimiento reducido.

## Criterios para una propuesta

- Distingue respuesta inmediata al control, continuidad de una transición y un momento singular que merezca énfasis. No añadas una secuencia de entrada a cada sección por defecto.
- Elige el cambio que explique la relación: desplazamiento para continuidad espacial; cambio de opacidad o forma para aparición; y luz, profundidad o recorte solo cuando ayuden a entender el material o el foco. Un efecto no reemplaza una etiqueta de estado.
- Ajusta la duración a la distancia, la consecuencia y la frecuencia de uso. Como referencias de exploración, una respuesta puede durar 100–150 ms; un cambio rutinario 150–300 ms; y una transición de panel o vista 300–500 ms. Son ejemplos, no tokens aprobados ni límites universales.
- Comprueba cómo se interrumpe, repite o revierte una transición. La salida, la entrada y el cambio de datos no tienen que usar la misma duración.
- Para movimiento reducido, evita desplazamientos amplios y conserva la información que confirma la acción. Una alternativa puede usar una transición más simple o un cambio inmediato, según el caso.
- Si hay bucles no esenciales, define cómo pausarlos y qué ocurre cuando la superficie deja de estar visible. El sonido requiere una decisión explícita del usuario y controles apropiados.
- Al evaluar un prototipo, distingue fluidez observada de rendimiento medido. Revisa si sombras, transparencias, desenfoques o superficies numerosas degradan la experiencia en el entorno de destino; comunica lo que no se pudo medir.

## Límites

- El diseño de interacción define activaciones, reglas y estados; esta guía define su expresión temporal y espacial.
- El sistema de diseño registra tokens de movimiento aprobados; esta guía no los configura en CSS.
- No escribas CSS, JavaScript, GSAP ni código de otra biblioteca.

## Referencias

- [W3C: Animation from Interactions](https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions)
