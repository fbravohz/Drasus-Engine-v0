# Artefactos y seguimiento

Guarda los borradores bajo `.agents/documents/social-strategist/`. Ignora `.gitkeep`; solo conserva el directorio. Un archivo generado no significa que el contenido se haya publicado.

## Progreso

Al iniciar un flujo de producción, crea `.agents/state/social-strategist/PROGRESS.md` si falta. No lo modifiques en una consulta de estado. Conserva estos datos:

- Último commit procesado: hash.
- Pulso pendiente: Historia/Riesgo/Tarea y hash.
- Masa narrativa: Pilar, entradas acumuladas y disponibilidad de episodio.
- Episodios: slug, Pilar, cierres cubiertos e idiomas.
- Semillas educativas pendientes: concepto, fuente feature/ADR y categoría.
- Cápsulas producidas: slug, concepto, fuente, categoría y fecha.
- Entorno: fecha de verificación, herramienta y estado.

Actualiza solo hechos verificados al completar cada flujo.

## Pulso y postura

Usa `social-strategy-[YYYY-MM-DD]-es.md` y `social-strategy-[YYYY-MM-DD]-en.md`. Incluye fecha, idioma, Story Core, formatos aplicables del canon y commits relacionados con hash y mensaje. Conserva traducciones completas.

## Episodios

Guarda cada episodio en `episodios/[slug]/`. Incluye `guion.md` con la plantilla del canon, `assets/` con materiales de producción y `checklist.md` con las grabaciones manuales requeridas.

## Cápsulas

Usa `capsulas/[concept-slug]-[YYYY-MM-DD]-es.md` y `capsulas/[concept-slug]-[YYYY-MM-DD]-en.md`. Incluye el concepto, los formatos definidos en `production.md` y la fuente. En Fuente identifica la feature, ADR o TTR y la categoría: algoritmo, método estadístico o patrón de ingeniería. Conserva la distinción entre implementación actual y futura.
