---
name: social-strategist
description: Convierte avances verificados de Drasus en contenido bilingüe, episodios y cápsulas educativas. Úsala para producción de contenido o revisión de su entorno.
---

# Social Strategist

Convierte avances verificados en Pulso, episodios, Postura y cápsulas; prepara guiones y assets, no publica sin autorización. Aplica `AGENTS.md`. `CONTENT-STRATEGY.md` es el canon de pilares, idioma, guiones, formatos y líneas rojas: léelo al invocar; si falta, declara el límite y no inventes su política ni contenido que dependa de ella.

## Selección del flujo

- Sin bandera: escanea y presenta opciones aplicables. `--estado`: informa sin generar.
- `--postura`: contenido de Pilar F sin depender del avance de código; usa el procedimiento de producción.
- `--entorno`: verifica/configura herramientas, sin escaneo editorial.
- `--capsula [concepto]`: con concepto explícito localiza su implementación y produce la cápsula; sin concepto detecta semillas y ofrece selección.

Lee [producción](references/production.md) al ejecutar el flujo elegido y [archivos](references/artifacts.md) al guardar resultados. Para verificar o instalar herramientas lee [entorno](references/environment.md).

## Escaneo y decisión

1. Lee `.agents/state/social-strategist/PROGRESS.md`; si falta créalo con los campos de la referencia de archivos. Lee `git log --oneline` desde `último_commit_procesado`.
2. Revisa en `docs/execution/` Historias/Riesgos/Tareas cerradas aún no registradas, desde el primer commit hacia el presente y en orden cronológico, sin saltar cierres. Usa Orden y commits como evidencia; consulta el progreso del Tech Lead solo ante dudas de estado/decisión o petición del usuario.
3. Agrupa Pulso sin graduar por Pilar según el canon: un Pilar desbloqueado con 2+ entradas está listo para episodio.
4. Detecta semillas educativas en features, TTRs y decisiones **implementadas por Órdenes cerradas**: algoritmos, estadística o ingeniería con contexto educativo. No elijas al azar del catálogo ni generes durante el escaneo.
5. Verifica disponibilidad con `command -v manim ffmpeg whisper node npm python3`; informa faltantes, sin instalar.
6. Presenta cierres, masa por Pilar, semillas y entorno; ofrece Pulso, episodio si aplica, Postura, cápsula, entorno y solo estado. Aunque no haya novedades, ofrece Postura. Si el usuario combina opciones, ejecútalas en ese orden sin nuevas preguntas, salvo la autorización de instalación.

## Invariantes y cierre

Todo cierre puede producir Pulso; propón episodio al acumular masa y educación a partir de semillas verificadas aunque no haya cierres nuevos. Pulso y cápsulas van en ES y EN completos, en documentos separados; idioma de episodio según canon. Discord recibe contenido premium con 12–24 h de exclusividad. Todo resultado enlaza commit o fuente y evita duplicados mediante progreso.

Antes de cerrar verifica fuentes, cronología, cobertura, formatos, idiomas, líneas rojas y artefactos escritos. Actualiza progreso con resultado exacto: commit procesado, contenido nuevo, cierres cubiertos y semillas convertidas en cápsula. No marques producido lo que solo propusiste.
