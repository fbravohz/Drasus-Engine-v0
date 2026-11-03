> Guía de ui-ux-designer. Las rutas de recursos son relativas a la raíz de la skill. Adapta los ejemplos y umbrales genéricos al canon de Drasus y al encargo.

# Arquitectura de información

Organiza las funciones y la información para que el usuario encuentre el destino correcto con poco esfuerzo. Consulta esta guía para navegación, jerarquías, nombres, agrupaciones y rutas entre áreas.

## Entradas

- `docs/README.md` y las secciones pertinentes de `docs/DESIGN.yaml`.
- Inventario de features, conceptos del producto y tareas que el propietario haya definido.
- Evidencia de investigación disponible. Si falta, marca la hipótesis; no la presentes como hecho.

## Procedimiento

1. Identifica objetos, funciones y tareas que necesitan una ubicación.
2. Agrupa por el modelo mental del usuario y la relación entre tareas; no copies la estructura interna del código.
3. Define jerarquía, destinos, etiquetas y reglas de navegación. Conserva términos del dominio definidos por el canon.
4. Comprueba si cada tarea tiene un destino único y predecible. Señala duplicados, callejones sin salida y destinos ambiguos.
5. Recomienda card sorting o tree testing cuando una incertidumbre de agrupación pueda cambiar la estructura.

## Entregable

Presenta un mapa de navegación, una matriz tarea-destino, las etiquetas clave y las dudas abiertas. Vincula cada propuesta con evidencia o identifícala como hipótesis.

## Descubrimiento y contexto

- Mantén identificable la ubicación actual y la relación con el destino anterior cuando el recorrido lo necesite.
- Distingue las acciones frecuentes de las opciones secundarias. La revelación progresiva puede reducir ruido, pero el usuario debe poder encontrar la función y acceder a ella con teclado.
- Cuando una decisión dependa de información de otra zona, considera mantenerla visible o recuperable en el mismo contexto. Señala los cambios entre vistas que obliguen a recordar o reconstruir información.
- Evalúa los nombres y agrupaciones con tareas concretas. Una revisión experta identifica hipótesis; la investigación con usuarios comprueba si las personas encuentran los destinos.

## Límites

- La investigación UX obtiene y valida evidencia; esta guía organiza la arquitectura resultante.
- El diseño de interacción define qué ocurre dentro de cada destino y cómo cambian los estados.
- No diseñes apariencia visual ni escribas código.

## Referencias

- [NN/g: UX Disciplines Glossary](https://www.nngroup.com/articles/ux-disciplines-glossary/)
- [GOV.UK Design System: Patterns](https://design-system.service.gov.uk/patterns/)
