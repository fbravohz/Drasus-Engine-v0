> Guía de ui-ux-designer. Las rutas de recursos son relativas a la raíz de la skill. Adapta los ejemplos y umbrales genéricos al canon de Drasus y al encargo.

# Diseño de visualización de datos

Diseña la representación para que el usuario pueda responder una pregunta analítica con precisión. La forma de la gráfica debe seguir la relación entre los datos y la tarea.

## Entradas

- La pregunta que el usuario necesita resolver y el conjunto de datos disponible.
- Definiciones de métricas, unidades, período, precisión y estados de datos.
- `docs/DESIGN.yaml` y las decisiones de color protegidas por el producto.

## Procedimiento

1. Identifica la relación que se debe leer: cambio en el tiempo, magnitud, comparación, distribución, rango, proporción o correlación.
2. Elige una gráfica conocida o una tabla si comunica mejor la tarea. No elijas por decoración.
3. Especifica ejes, escala, línea base, series, unidades, etiquetas y el papel de leyenda o tooltip.
4. Conserva el significado de los colores financieros, alertas y tipos de puerto. No dependas solo del color: usa etiquetas, forma, posición o símbolos cuando corresponda.
5. Define los estados de carga, vacío, desactualización y error; añade un resumen textual o alternativa tabular cuando la gráfica no baste.
6. Revisa la accesibilidad de la propuesta. Usa la skill `quant-engineer` para validar unidades, cálculos e interpretación financiera cuando estén en juego.

## Entregable

Entrega una ficha con pregunta analítica, tipo de datos, codificación visual, escalas, etiquetas, estados, alternativa accesible y supuestos pendientes.

## Actualizaciones y lectura en contexto

- Cuando los datos cambien, indica si la vista muestra información actual, parcial, desactualizada o de ejemplo cuando esa distinción afecte la decisión. La hora de actualización y el período deben ser interpretables junto con los valores.
- Revisa cifras positivas, negativas, cero, valores ausentes, magnitudes largas y formatos de moneda, fecha y separador en los idiomas soportados. Conserva signo, unidad y precisión definidos por la fuente de la métrica.
- Si una transición ayuda a seguir un cambio, mantén legible el valor final y ofrece la alternativa de movimiento reducido. Una animación no debe retrasar una alerta ni hacer que datos estimados parezcan datos observados.
- Evalúa si la frecuencia de actualización, el número de series y la densidad dificultan seguir los cambios. Señala cualquier límite de rendimiento observado o que no se haya podido medir.

## Límites

- Esta guía decide cómo presentar datos; no inventa, calcula ni corrige métricas.
- No modifica significados de color protegidos ni reglas financieras.
- No implementa gráficas ni selecciona una biblioteca de código.

## Referencias

- [Office for National Statistics: Data visualisation](https://service-manual.ons.gov.uk/data-visualisation)
- [Government Analysis Function: Data visualisation colours](https://gss.civilservice.gov.uk/policy-store/data-visualisation-colours-in-charts/)
- [W3C: Non-text Contrast](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast)
