# Revisión estructural

Esta guía adapta el método de [`software-architecture`](https://github.com/bengous/agents-skills/tree/master/software-architecture)

## Alcance

Úsala para una decisión arquitectónica nueva o para revisar una estructura existente. Si el usuario ya aprobó la solución y pidió ejecutarla, respeta ese alcance; no vuelvas a abrir el diseño completo.

Trata los patrones como opciones que deben resolver una necesidad comprobable. El nombre de una arquitectura no justifica adoptarla por sí solo.

## Evidencia del diseño existente

1. Sigue un flujo representativo del área señalada: entrada, consumidores, puertos, dueño de datos, persistencia y efectos externos pertinentes. Lee las verificaciones cercanas y las decisiones canónicas aplicables.
2. Separa en el análisis: conducta observada en el código, intención establecida en el canon e inferencias. Cita ruta y línea. Si falta código o evidencia, limita el hallazgo y márcalo como provisional.
3. Considera señales concretas: coordinación repetida, propiedad de datos inconsistente, ciclos de dependencias, estado global oculto, cambios no relacionados que siempre viajan juntos o una garantía difícil de comprobar. El historial puede mostrar fricción repetida; la falta de actividad no prueba que un componente sea seguro.
4. No declares un defecto por tamaño de archivo, nombre, cantidad de módulos o ausencia de un patrón. No llenes una cuota de hallazgos; recomendar conservar el diseño es válido.

## Comparación de opciones

Parte del flujo del usuario y las restricciones que lo gobiernan. Distingue requisitos medidos de pronósticos. Agrupa el código según cambios y propiedad reales; usa modelos de dominio más ricos cuando las reglas interactúan y se repiten, y límites de dominio cuando cambian significados o dueños.

Compara opciones realistas, incluida conservar el diseño actual, y explica qué evidencia cambiaría la recomendación. Mantén llamadas directas para coordinación simple. Usa trabajos asíncronos o eventos si hay distribución a varios consumidores (fan-out) o ciclos de vida separados; declara los costos de orden y consistencia. Mantén modelos de lectura y escritura compartidos salvo que necesidades distintas justifiquen separarlos; CQRS (separación de comandos y consultas) no exige procesos, bases ni event sourcing separados.

Evalúa cada frontera por lo que sus consumidores deben conocer: contrato, orden, fallos, propiedad, consistencia y rendimiento. Justifica una abstracción si reduce ese conocimiento, protege una invariante o permite independencia útil. Incluye sus costos de traducción, operación, migración y mantenimiento.

En Drasus, un monolito modular puede combinar features, puertos hexagonales y núcleos funcionales. Propón separación de procesos solo cuando el despliegue independiente, aislamiento de fallos, seguridad o escalado compense la complejidad de red y operación. Si el beneficio depende de una premisa incierta, plantea una medición o un corte vertical pequeño que permita distinguir opciones; no inventes una necesidad de escala.

## Cambio en sistemas existentes

Migra por el límite útil más pequeño. Conserva comportamiento relevante y casos históricos sin congelar errores conocidos. Separa movimientos de código, cambios de comportamiento y migraciones de datos, salvo que dependan entre sí; revertir el código no revierte necesariamente los datos. Una comparación paralela no debe duplicar escrituras o notificaciones externas.

Antes de quitar una abstracción, confirma que no centralice un contrato, traducción o aislamiento que sus consumidores necesitarían duplicar. No la conserves ni la elimines solo por su tamaño o estilo.

## Estado, efectos y tipos

Cuando la decisión cruce estados o efectos externos, identifica estados, eventos, orden, reintentos, duplicados, propiedad del compromiso y fallos recuperables. Consulta ADR-0141, ADR-0165 y ADR-0166; esta guía no reemplaza sus contratos.

Usa tipos para excluir estados inválidos cuando el lenguaje lo permita y valida los datos externos en la frontera. Los tipos en memoria no garantizan integridad ante escrituras concurrentes; consulta el ADR y el modelo de datos de la tabla dueña.

Evalúa puertos y adaptadores por la independencia que compran. Un puerto puede justificarse con un solo adaptador si protege propiedad, pruebas o aislamiento de tecnología. No exijas una capa o interfaz por entidad o tabla, ni ocultes efectos y latencia bajo nombres genéricos. No agregues un lenguaje genérico de efectos solo para perseguir pureza.

Una verificación local de idempotencia no garantiza que un efecto externo ocurra exactamente una vez. La idempotencia debe protegerse donde el efecto se confirma y, si el destino no confirma, puede requerirse conciliación; aplica ADR-0166.

## Resultado

Vincula cada hallazgo con evidencia, consecuencia, responsabilidad o contrato afectado, cambio propuesto, costo y verificación mínima. Elige la verificación según la garantía: compilación para estados inválidos, pruebas de contrato para adaptadores, medición para latencia o una verificación de transacción para persistencia. Una simulación no prueba por sí sola el comportamiento del proveedor o la base reales. Prioriza riesgos importantes. No agregues herramientas, capas o pruebas universales solo para cumplir una forma.

Da una recomendación breve con motivo, trade-off principal y siguiente paso. En una decisión importante, muestra alternativas. No presentes una verificación propia como revisión independiente.

Para las reglas del proyecto, consulta `AGENTS.md`, `docs/README.md` y los documentos canónicos que esos índices señalen. En dominios especializados, aplica primero la revisión requerida por `AGENTS.md`.
