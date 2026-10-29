# Casos de comportamiento del refinador

Al modificar el método de auditoría de `context-refiner`, aplica su inventario y sus dos pasadas a cada par. Compara la decisión con el resultado esperado y anota la diferencia concreta. Estos casos prueban el juicio del revisor; no sustituyen una validación automática ni demuestran equivalencia para otros documentos.

## Condición y excepción

- Original: «Antes de publicar, revisa la licencia si el recurso es externo; omite la revisión si el propietario ya la verificó».
- Resultado: «Antes de publicar, revisa la licencia».
- Esperado: **Rechazar**. El resultado elimina el disparador y la excepción; exige la revisión también en los casos excluidos.

## Umbral y exclusión

- Original: «Asigna el máximo identificador local más uno, sin contar el identificador 999».
- Resultado: «Verifica la numeración local».
- Esperado: **Rechazar**. El resultado no determina el siguiente identificador ni excluye 999.

## Conservación del comportamiento

- Original: «Si hay tres intentos fallidos, detén el proceso y devuelve un error; no reintentes errores permanentes».
- Resultado: «No reintentes errores permanentes. Tras tres intentos fallidos, detén el proceso y devuelve un error».
- Esperado: **Aceptar**. Conserva el umbral, el efecto y la exclusión.

## Referencia canónica

- Original: «El propietario del dato autoriza el acceso; los demás componentes usan su puerto público».
- Resultado: «Para acceso a datos, consulta la sección verificada del canon que exige autorización del dueño y uso de su puerto público».
- Esperado: **Aceptar solo si** la ruta y sección existen y contienen ambos requisitos. Si falta uno o la fuente no está verificada, **rechazar** la sustitución y conservar la regla.
