# Primera experiencia y condiciones límite

Usa esta guía para diseñar la llegada a una tarea, el aprendizaje contextual y las situaciones que interrumpen un flujo. Complementa la guía de interacción, la plantilla de onboarding y las guías de contenido y accesibilidad: no reemplaza sus patrones de estados, mensajes o investigación con usuarios.

Consulta la [guía de interacción](guide.md), la [plantilla de onboarding](../../assets/ux-writing/templates/onboarding-flow-template.md), la [guía de contenido](../ux-writing/guide.md) y la [guía de accesibilidad](../accessibility/guide.md) según el encargo.

## Diseña el primer avance

1. Identifica qué tarea aporta valor a la persona y qué necesita conocer para realizarla con seguridad. Distingue una hipótesis del diseñador de evidencia observada con usuarios.
2. Enumera los conceptos, permisos y decisiones realmente necesarios antes de esa tarea. Enseña el resto cuando aparezca en el contexto de uso.
3. Decide si conviene un ejemplo, una guía breve, ayuda contextual o una tarea practicable. Identifica qué experiencia se puede omitir y retomar, sin eliminar pasos exigidos por seguridad o por el contrato de la operación.
4. Facilita que personas con experiencia continúen sin instrucción repetida. Quien empieza debe poder volver a consultar la ayuda sin perder el contexto.
5. Define cómo reconocer el avance: una acción terminada, comprensión demostrada en una investigación o una medida autorizada. No inventes métricas ni presupongas telemetría.

## Patrones según el momento

- **Primera visita:** explica qué puede hacer la persona y cuál es un paso inicial útil. Presenta solo los datos y permisos que la tarea requiere, con una razón cuando no sea evidente.
- **Estado vacío:** distingue primer uso, contenido eliminado, filtro sin resultados, falta de permisos y fallo de carga. Ofrece el siguiente paso pertinente; no conviertas un error en una aparente ausencia de datos.
- **Ayuda contextual:** asocia la explicación con el control o decisión. La ayuda debe poder descubrirse con teclado y permanecer accesible sin depender solo de hover. Evita repetir consejos ya descartados cuando el producto permita recordarlo.
- **Recorrido guiado:** úsalo si explica una secuencia que no se entiende en contexto. Haz claro el avance, la salida y la posibilidad de retomarlo; no bloquees la aplicación por enseñar una función ordinaria.
- **Práctica:** si el flujo requiere entrenamiento, diferencia los datos y efectos de ejemplo de los reales. Para decisiones financieras y órdenes, una simulación debe identificarse como tal y nunca prometer una ejecución real.
- **Persona que regresa:** conserva contexto y preferencias cuando el producto lo soporte. No afirmes que existe persistencia o recuperación si no está verificada en el contrato o la implementación.

## Diseña condiciones límite

Usa contenido representativo y situaciones plausibles para la tarea:

- Datos ausentes, desactualizados, parciales o numerosos; textos y cifras cortos y largos; signos, unidades y caracteres de los idiomas soportados.
- Espera inicial, actualización y operaciones interrumpidas. Muestra progreso determinado solo si existe; si no, explica el estado sin inventar porcentajes ni tiempos.
- Error de validación, falta de acceso, desconexión local o remota cuando corresponda, conflicto e imposibilidad de terminar. Conserva las entradas del usuario cuando el comportamiento real lo permita y describe una vía de recuperación verificable.
- Doble activación, cambios de ventana o interrupción de un arrastre. Especifica la respuesta visible y una forma de continuar sin inferir garantías de concurrencia o persistencia.
- Ampliación de texto, teclado, lector de pantalla y nombres accesibles. Revisa la guía de accesibilidad; no afirmes conformidad a partir de una sola inspección.

## Carácter y respuesta emocional

Una respuesta distintiva puede reconocer un logro importante, orientar durante una espera o hacer más humana una recuperación. Debe corresponder a la voz de Drasus, al esfuerzo y a la consecuencia. Una acción rutinaria necesita certeza; no una celebración repetida.

No inventes progreso ni hagas esperar a la persona para mostrar una animación. En pérdidas, riesgo, privacidad o trabajo bloqueado, prioriza una explicación clara y una acción posible. El carácter no debe ocultar información, producir sonido sin autorización ni dificultar la repetición o el acceso.

## Verificación y entrega

Entrega el recorrido, las condiciones y alternativas consideradas, las decisiones pendientes y la evidencia disponible. Si se realizaron sesiones con usuarios, registra sus hallazgos conforme a la guía de investigación. Si solo se inspeccionó la propuesta, identifícalo como revisión experta.
