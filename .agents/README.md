# Drasus Context

Repositorio canónico del sistema de agentes y del contexto operativo que usan los proyectos Drasus.

## Alcance

- `knowledge/`: gobernanza y conocimiento bajo demanda.
- `skills/`: procedimientos especializados para agentes.
- `hooks/`: comprobaciones deterministas del ciclo de vida.
- `context-runtime/`: catálogo, runtime y pruebas del resolver de contexto.
- `templates/`: formatos reutilizables para documentos de agentes.
- `plans/`, `sessions/`, `state/` y `memory/`: artefactos operativos del sistema de agentes.

El repositorio no contiene código de producto, migraciones, UI ni decisiones de dominio de `Drasus-Engine`.

## Integración

`Drasus-Engine` consume una revisión fijada de este repositorio como submódulo en `.agents`. Los adaptadores específicos de cada plataforma se generan desde esta fuente y no se editan manualmente en el repositorio de código.

## Estado de la extracción

La historia relevante de `.agents` se conservó en una rama independiente. La publicación del repositorio remoto y la conexión del submódulo quedan como pasos de integración posteriores.
