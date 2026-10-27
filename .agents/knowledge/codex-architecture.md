# Arquitectura nativa de Codex para Drasus

Este es el modelo operativo activo de Drasus Engine. El antiguo repositorio
`Drasus-Context` está archivado bajo `.agents/legacy/` y no es una dependencia
de ejecución.

## Orden de autoridad

1. La solicitud del usuario define el resultado y el alcance permitido.
2. El `AGENTS.md` raíz define el enrutamiento, las invariantes duraderas y la
   propiedad.
3. La habilidad del flujo de trabajo seleccionado define el orden de decisión y la
   transferencia.
4. El MCP `drasus_context` proporciona contexto acotado, de solo lectura, y
   superficies de validación para ese flujo de trabajo.
5. El `AGENTS.md`, los documentos de producto, el código y las pruebas del
   repositorio objetivo son la fuente de verdad del comportamiento del producto.
6. Los hooks de Codex realizan únicamente comprobaciones deterministas del
   ciclo de vida.

Cuando dos fuentes entren en conflicto, detente y haz visible el conflicto. No
lo resuelvas combinando silenciosamente el contexto antiguo con las reglas nuevas.

## Responsabilidades por capa

| Capa | Responsabilidad | No debe convertirse en |
|---|---|---|
| `AGENTS.md` | Constitución pequeña, enrutamiento e invariantes | Un manual de producto o un volcado de prompts |
| Habilidades | Flujo de trabajo reutilizable y orden de decisión | Personas o políticas autónomas |
| MCP | Contexto en vivo y de solo lectura, selección de fuentes y superficie de validación | Un agente oculto o una barrera de mutaciones |
| Conocimiento | Contratos y mapas enfocados | Una segunda copia de cada documento del repositorio |
| Ganchos | Comprobaciones de arranque y deterministas | Un revisor semántico o reescritor de intenciones |
| Drasus Engine | Código, documentación, pruebas e historial de Git del producto | Un lugar para el enrutamiento global fuera del repositorio |

## Taxonomía de flujos de trabajo

- `dispatch`: clasificar el repositorio, el flujo de trabajo y el riesgo.
- `plan`: establecer hechos, restricciones, superficies afectadas y validación.
- `implement`: realizar el cambio más pequeño en el repositorio propietario.
- `integrate`: coordinar contratos entre repositorios o generados.
- `verify`: recopilar evidencia de pruebas, límites, persistencia, permisos,
  errores y documentación.
- `review`: inspeccionar de forma independiente la intención, los cambios, las
  pruebas y los supuestos.
- `context-maintenance`: evolucionar esta capa de Codex sin reactivar
  comportamientos heredados específicos de una plataforma.

El material antiguo basado en roles se asigna a estos flujos en lugar de seguir
cargándose globalmente. Por ejemplo, el material de arquitectura y alianzas
informa a `plan` y `review`; el material de Rust, Flutter, puentes y
refactorización informa a `implement` o `integrate`; el material de QA y
adversario informa a `verify` y `review`. El material específico del dominio
solo se migra cuando un flujo real lo necesita.

## Contrato de inyección de contexto

La habilidad llama a `drasus_context.start_workflow` con:

- el flujo de trabajo seleccionado;
- el repositorio propietario;
- un resumen breve de la tarea; y
- un nivel de riesgo.

El MCP devuelve fuentes obligatorias, fuentes opcionales, una superficie de
validación, restricciones y los siguientes flujos de trabajo. Codex lee las
fuentes obligatorias antes de editar. El proveedor no modifica archivos ni
concede permiso para realizar acciones de alto impacto.

Esto conserva la parte útil del sistema de contexto anterior: una inyección
consciente del flujo de trabajo sin un corpus centralizado de instrucciones.

## Regla de migración

El árbol de contexto antiguo se conserva como instantánea de referencia bajo
`.agents/legacy/drasus-context/`, pero no se inyecta ni se descubre. Promueve un
contrato, flujo de trabajo o referencia útil cada vez a `.agents/`, asígnale un
propietario y una ruta de validación, y después elimina sus supuestos de
plataforma antiguos. En particular, los nombres de eventos heredados, las
variables de entorno de plataformas externas, las habilidades exclusivas de
personas y las cargas útiles de ganchos específicos de una plataforma no son
interfaces activas aquí.
