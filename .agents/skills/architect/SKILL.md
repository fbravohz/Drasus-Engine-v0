---
name: architect
description: "Diseña, revisa y refina arquitectura, SAD, ADR y Features de Drasus con evidencia del canon y el código; documenta decisiones autorizadas y prepara barridos previos a una Orden."
---

# Arquitectura, evidencia y canon

Revisa propuestas, código y documentos; mantiene en `docs/` la trazabilidad de decisiones arquitectónicas autorizadas. No implementes código ni despaches trabajo. Aplica la gobernanza de `AGENTS.md` y el canon vigente.

## Selección de flujo

- Para tomar una decisión nueva, revisar una estructura existente o aconsejar sobre una solución aprobada, aplica el flujo «Revisión estructural» y consulta `docs/README.md` y solo las fuentes que active el área.
- Antes de una Orden, sigue el barrido de `AGENTS.md`, sección «Órdenes de trabajo». Entrega «Preguntas resueltas antes de implementar» con pregunta, respuesta, cita y clasificación de preguntas abiertas según esa sección.

En una recomendación, da conclusión, motivo, trade-off principal y siguiente paso; muestra alternativas cuando la decisión sea importante.

## Revisión estructural

Este flujo adapta el método de [software-architecture](https://github.com/bengous/agents-skills/tree/master/software-architecture).

Usa este flujo para decidir límites, dependencias, coordinación, persistencia o efectos externos, o para investigar señales concretas en una estructura existente. También úsalo cuando el usuario pida consejo sobre una solución aprobada. Evalúa supuestos, riesgos y alternativas; separa la recomendación de la decisión vigente y del trabajo autorizado.

Trata los patrones como opciones que deben resolver una necesidad comprobable. El nombre de una arquitectura no justifica adoptarla por sí solo.

### Evidencia

1. Sigue un flujo representativo del área: entrada, consumidores, puertos, dueño de datos, persistencia y efectos externos pertinentes. Lee verificaciones cercanas y decisiones canónicas aplicables.
2. Separa la conducta observada en el código, la intención del canon y las inferencias. Cita ruta y línea. Si falta evidencia, limita el hallazgo y márcalo como provisional.
3. Busca señales concretas: coordinación repetida, propiedad de datos inconsistente, ciclos de dependencias, estado global oculto, cambios no relacionados que siempre viajan juntos o garantías difíciles de comprobar. El historial puede mostrar fricción repetida; la falta de actividad no prueba que un componente sea seguro.
4. No declares un defecto por tamaño de archivo, nombre, cantidad de módulos o ausencia de un patrón. No llenes una cuota de hallazgos; recomendar conservar el diseño es válido.

### Comparación de opciones

Parte del flujo del usuario y sus restricciones. Distingue requisitos medidos de pronósticos. Evalúa valor para el usuario, complejidad, latencia y mantenimiento; compara con StrategyQuant X cuando sea pertinente. Agrupa código según cambios y propiedad reales; usa modelos de dominio más ricos cuando las reglas interactúan y se repiten, y límites de dominio cuando cambian significados o dueños. Documenta investigación no prioritaria en `docs/moonshots/` sin convertirla en compromiso de producto.

Compara opciones realistas, incluida conservar el diseño, y explica qué evidencia cambiaría la recomendación. Mantén llamadas directas para coordinación simple. Usa trabajos asíncronos o eventos si hay varios consumidores o ciclos de vida separados; declara costos de orden y consistencia. Mantén modelos de lectura y escritura compartidos salvo que necesidades distintas justifiquen separarlos. CQRS (separación de comandos y consultas) no exige procesos, bases ni event sourcing separados.

Evalúa cada frontera por el conocimiento que exige a sus consumidores: contrato, orden, fallos, propiedad, consistencia y rendimiento. Justifica una abstracción si reduce ese conocimiento, protege una invariante o permite independencia útil. Incluye costos de traducción, operación, migración y mantenimiento.

En Drasus, un monolito modular puede combinar Features, puertos hexagonales y núcleos funcionales. Propón separar procesos solo si el despliegue independiente, el aislamiento de fallos, la seguridad o el escalado compensan la complejidad de red y operación. Si el beneficio depende de una premisa incierta, plantea una medición o un corte vertical pequeño; no inventes una necesidad de escala.

### Cambios en sistemas existentes

Al recomendar una migración, delimítala por el límite útil más pequeño. Preserva el comportamiento relevante y los casos históricos sin perpetuar errores conocidos. En el plan, separa movimientos de código, cambios de comportamiento y migraciones de datos, salvo que dependan entre sí; revertir el código no revierte necesariamente los datos. Una comparación paralela no debe duplicar escrituras ni notificaciones externas.

Antes de recomendar quitar una abstracción, confirma que no centralice un contrato, traducción o aislamiento que sus consumidores necesitarían duplicar. No la conserves ni la elimines solo por su tamaño o estilo.

### Estados, efectos y tipos

Cuando el flujo implique estados o efectos externos, identifica estados, eventos, orden, reintentos, duplicados, quién confirma el cambio y fallos recuperables. Consulta solo los contratos que active el flujo: ADR-0137 para límites y puertos de Features; ADR-0141 para persistencia y atomicidad estado-auditoría; ADR-0165 si cruza la frontera gRPC de escritorio; ADR-0166 para el ciclo observable y recuperable de mutaciones agénticas, su idempotencia y sus efectos fuera del Core. Considera actividad humana compartida solo si la propuesta adopta ese contrato. Esta guía no sustituye esos contratos.

Usa tipos para excluir estados inválidos cuando el lenguaje lo permita y valida datos externos en la frontera. Los tipos en memoria no garantizan integridad ante escrituras concurrentes; consulta el ADR y el modelo de datos de la tabla dueña.

Evalúa puertos y adaptadores por la independencia que compran. Un puerto puede justificarse con un solo adaptador si protege propiedad, pruebas o aislamiento de tecnología. No exijas una capa o interfaz por entidad o tabla, ni ocultes efectos y latencia bajo nombres genéricos. No agregues un lenguaje genérico de efectos solo para perseguir pureza.

Una verificación local de idempotencia no garantiza que un efecto externo ocurra exactamente una vez. Protege la idempotencia donde se confirma el efecto y, si el destino no confirma, considera conciliación; aplica ADR-0166.

### Resultado

Vincula cada hallazgo con evidencia, consecuencia, responsabilidad o contrato afectado, cambio propuesto, costo y verificación mínima. Elige la verificación según la garantía: compilación para estados inválidos, pruebas de contrato para adaptadores, medición para latencia o verificación de transacción para persistencia. Una simulación no prueba por sí sola el comportamiento del proveedor o la base reales. Prioriza riesgos importantes; no agregues herramientas, capas o pruebas universales solo para cumplir una forma.

Usa el formato de recomendación definido arriba. En decisiones importantes, muestra alternativas. No presentes una verificación propia como revisión independiente.

## Redacción y refinamiento del SAD

Usa este flujo al crear, refactorizar o mejorar `docs/SAD.md` o una sección bajo `docs/sad/`.

- El SAD explica el mapa de la arquitectura objetivo: propósito del sistema, componentes y responsabilidades, límites, flujos principales y propiedades que condicionan el diseño. Separa la arquitectura objetivo del estado entregado; el Roadmap conserva fases y avances.
- Escribe para lectura autónoma. Explica los términos internos en su primer uso y conserva el motivo de cada frontera. Los ADR respaldan y amplían decisiones; no sustituyas la explicación del sistema por una cadena de enlaces.
- Consulta `docs/README.md`, `docs/SAD.md` y `docs/templates/SAD.md`; lee solo los ADR, features, secciones SAD y partes del Roadmap que afecten el texto. Mantén las promesas y límites de producto en `docs/PRODUCT.md` y las decisiones detalladas en sus ADR.
- Al refinar, usa de `context-refiner` la compresión semántica, la verificación del canon y la revisión de omisiones. No apliques mecánicamente su regla de quitar todo contenido que también aparezca en `AGENTS.md`: el SAD debe explicar la arquitectura a sus lectores. En una reestructuración amplia, conserva un mapa breve de afirmación y fuente; no lo exijas para un cambio local.
- Conserva hechos, límites, valores, excepciones y conflictos verificados. Distingue hechos e inferencias; no reconcilies conflictos por intuición.
- Revisa la sección de forma independiente y junto a sus referencias. Confirma que el lector entienda el propósito, las partes, sus responsabilidades y los objetivos. Comprueba los enlaces; actualiza `docs/SAD.md` solo si cambia la estructura y el Roadmap solo si cambia el estado de entrega.

## Redacción y refinamiento de ADRs

Usa este flujo al crear, revisar o mejorar un documento bajo `docs/adr/`.

- Un ADR explica una decisión arquitectónica duradera: el problema que atiende, la opción elegida, sus restricciones, efectos y costos. No lo conviertas en especificación completa de una Feature ni en plan de implementación.
- Consulta `docs/README.md`, `docs/ADR.md`, `docs/templates/ADR.md` y el ADR objetivo. Busca con `rg` decisiones cercanas, banners de supersesión y referencias; abre solo las fuentes que puedan sostener, contradecir o consumir esa decisión.
- En un ADR existente, conserva el sentido de la decisión, su estado, el contexto histórico útil y sus relaciones de supersesión. La plantilla orienta el contenido, pero no justifica reescribir mecánicamente un documento histórico para que coincida con su formato actual.
- Aplica de `context-refiner` la claridad, compresión semántica y revisión de omisiones. Explica términos internos al primer uso y conserva la evidencia que permite entender por qué se eligió la opción. No elimines contexto necesario solo porque también aparezca en otro documento.
- Distingue una mejora editorial de un cambio de alcance, restricciones, estado o decisión. Si la evidencia revela que hace falta cambiar la decisión, trátalo como cambio arquitectónico según esta skill; no lo introduzcas como corrección de redacción.
- Revisa el ADR por sí solo y frente a sus fuentes y consumidores. Comprueba enlaces, índice, banners de supersesión y consistencia con el SAD, las Features y la implementación que materializan la decisión; sincroniza solo los documentos afectados.

## Redacción y refinamiento de Features

Usa este flujo al crear, revisar o mejorar `docs/features/<feature>.md`.

- La Feature describe una capacidad funcional completa: problema y resultado para el usuario, comportamiento observable, límites, datos y puertos públicos, ciclo de vida y relaciones de integración que apliquen. La arquitectura se apoya en sus ADR; el módulo compone Features y los TTRs detallan trabajo concreto.
- Consulta `docs/README.md`, `docs/templates/FEATURE.md` y la Feature objetivo. Revisa solo los ADR, el SAD, la capacidad de producto, los TTR, el módulo consumidor, el modelo de datos, diseño o código que sean necesarios para verificar sus afirmaciones y contratos.
- Usa de `context-refiner` la claridad, compresión semántica y revisión de omisiones. Escribe para lectura autónoma, explica términos internos al primer uso y conserva ejemplos o contexto que ayuden a entender el comportamiento. Respeta las obligaciones vigentes de la plantilla; evalúa las secciones condicionales según el tipo de Feature, declara «No aplica» cuando la plantilla lo exige y registra lo pendiente en vez de borrarlo.
- Distingue el comportamiento definido del ya implementado y de lo pendiente. Contrasta con el código solo las afirmaciones de implementación; no conviertas una refactorización editorial en un cambio de producto, propiedad de datos, puertos o contrato. Si la evidencia exige uno de esos cambios, clasifícalo y resuélvelo conforme a las reglas de arquitectura y producto antes de canonizarlo.
- Revisa la Feature de forma independiente y junto a sus consumidores. Comprueba que comportamiento, restricciones, parámetros, puertos, persistencia, dependencias, TTRs y superficie visual sean coherentes cuando apliquen; verifica enlaces e índice. Sincroniza el módulo, el SAD, producto o Roadmap solo cuando el cambio afecte su contenido, según las reglas de esta skill.

## Registro y sincronización de arquitectura

Aplica estas reglas cuando una decisión autorizada requiera registrar o sincronizar cambios en el canon.

1. Registra una decisión aprobada en el SAD y el ADR pertinentes; no dupliques decisiones vigentes. Usa la plantilla aplicable y sincroniza índices y consumidores afectados.
2. Cuando un ADR modifique, extienda o reemplace otro, añade al original un banner con la relación y la decisión vigente. Enlaza ambos, corrige afirmaciones obsoletas sin inventar restricciones y conserva los ADR históricos. Revisa la materialización y sincroniza las features, especificaciones y TTR consumidores afectados.
3. Para persistencia, aplica en `docs/adr/ADR-0020.md` la tabla «Resto por Filtro de Relevancia por Perfil»: grupo I universal y campos pertinentes por perfil, sin copiar el catálogo completo. Documenta las combinaciones de perfiles que correspondan.
4. Al mover una feature, sincroniza el SAD. Para una feature nueva o refactorizada, actualiza el TTR de orquestación del módulo o crea uno si falta; incluye un bloque explícito y enlace a la feature. Asigna el máximo TTR local más uno, sin contar TTR-999, y confirma que el identificador no exista ni sea TTR-999. Añádelo a «TTRs Etiquetados por Fase», ordenado por EPIC ascendente y con TTR-999 al final de su grupo.
5. Actualiza solo las secciones afectadas de `docs/ROADMAP.md` si cambian entregas, fases, módulos o TTR. Sincroniza `docs/README.md` si cambia el mapa. Revisa referencias huérfanas y que cada feature tenga al menos un módulo orquestador. Si cambian reglas o formato documental, evalúa las plantillas pertinentes.

## Definición de producto

Cuando cambie una feature, su fase o una promesa de producto, aplica las reglas de sincronización de `docs/PRODUCT.md` y `docs/PRODUCT-SIMPLE.md`:

- Asigna cada feature nueva a un `CAP-NN` y añade su narrativa en la feature. Al eliminar, fusionar o reubicar, actualiza capacidad y conteo; al promover o degradar un moonshot, cambia su clasificación.
- Sincroniza ambos documentos al cambiar capacidades, promesas, fases u orden del roadmap. Los identificadores `CAP-NN` son estables, no se reutilizan y una capacidad nueva recibe el siguiente libre.
- Mantén una versión técnica y otra llana, sin variantes por profesión. Describe a las personas por lo que hacen, mantén una analogía por sección y deriva el material de difusión de estas fuentes. No prometas rentabilidad ni cifras de rendimiento o resultados esperados, ni incluyas detalles de implementación.
