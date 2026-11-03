> Guía de ui-ux-designer. Las rutas de recursos son relativas a la raíz de la skill. Adapta los ejemplos y umbrales genéricos al canon de Drasus y al encargo.

# Propuesta visual Drasus

Consulta esta guía cuando el usuario pida explorar o presentar una superficie de Drasus Engine. Aplica las decisiones aprobadas de marca y del sistema de diseño. El resultado es una propuesta verificable, no una decisión de producto ni una implementación productiva.

## Fuentes permitidas

1. Lee `AGENTS.md` y `docs/README.md`.
2. Lee `.agents/skills/architect/SKILL.md` y solo los ADR, SAD, features y contratos que esos índices activen para resolver la superficie solicitada.
3. Identifica qué decisiones están aprobadas en el canon y qué muestra la referencia elegida por el propietario. Una captura aislada o un archivo heredado no aprueba por sí mismo una decisión. No abras material excluido explícitamente por el usuario.

## Procedimiento

1. Define quién usa la superficie, qué tarea necesita completar, qué queda fuera del alcance y qué evidencia autoriza los datos mostrados.
2. Separa los hechos del producto de la dirección creativa. Conserva la arquitectura vigente: Tauri 2 + React/TypeScript, Core Rust y contratos de transporte Protobuf/gRPC. No presentes el prototipo como conectado al Core.
3. Diseña la superficie con composición, tipografía, paleta, componentes y tono de interfaz definidos en el canon. Si la marca aún no está definida, no la inventes; registra la dependencia y presenta alternativas solo cuando el usuario las solicite.
4. Entrega el alcance solicitado como un artefacto autónomo, con navegación e interacciones que funcionen sin dependencias de red. Rotula la información inventada junto a su superficie.
5. Conserva los límites operativos: una vista de lectura no muta estado; un Canvas muestra el número de lienzos y nodos que solicite el usuario; operaciones sobre capital u órdenes reales nunca se simulan como ejecutadas.
6. Revisa el documento fuente, los estados de navegación, el comportamiento adaptable y el contraste. Valida el HTML con las herramientas disponibles y reporta cualquier verificación visual que no se pudo ejecutar.

## Salida

Entrega la ruta del prototipo y resume las identidades, componentes y validaciones. No crees otra ficha, una dependencia o una decisión canónica salvo que el usuario lo pida.

## Límites

- El canon y la instrucción vigente del usuario prevalecen sobre la propuesta.
- Marca define la identidad; movimiento, su expresión temporal; y adaptación, el comportamiento ante cambios de ventana. La propuesta visual aplica esas decisiones.
- No inventes capacidades, contratos, datos reales, permisos ni resultados.
- El prototipo no autoriza cambios al producto, al código productivo ni a sistemas externos.

## Referencias de trabajo

- Para definir una superficie, comparar alternativas o trasladar una composición aprobada a un prototipo, consulta [Dirección y prototipos](direccion-y-prototipos.md).
- Para comparar una región de una interfaz React ya ejecutándose, consulta la sección [Explorar variantes en una interfaz en ejecución](direccion-y-prototipos.md#explorar-variantes-en-una-interfaz-en-ejecución).
- Para diagnosticar y ajustar composición, densidad, tipografía, color o intensidad visual, consulta [Criterios visuales](criterios-visuales.md).
- Para una crítica o un pase final de refinamiento, consulta [Revisión visual](revision.md). La revisión de accesibilidad sigue la guía de ese dominio.

Estas referencias adaptan procedimientos de [Impeccable](https://github.com/pbakaus/impeccable) y los combinan con las guías de los demás dominios de la skill.
