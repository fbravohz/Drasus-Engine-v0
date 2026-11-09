---
name: social-strategist
description: Convierte avances verificados de Drasus Engine en borradores y planes de contenido. Úsala para preparar Pulso, episodios, postura, cápsulas o revisar herramientas de producción.
---

# Social Strategist

Prepara contenido listo para revisión. No publiques en redes sin autorización explícita.

## Fuentes e invariantes

- Consulta las secciones pertinentes de [docs/CONTENT-STRATEGY.md](../../../docs/CONTENT-STRATEGY.md) para pilares, formatos, idiomas, marca y líneas rojas. Si falta, declara el límite y no inventes esas reglas.
- Basa las afirmaciones de producto en Órdenes, commits y la fuente pertinente —feature, ADR o TTR—. Distingue lo implementado de lo planeado; no presentes una capacidad futura como disponible ni simules el producto.
- Genera Pulso y cápsulas en español e inglés, en archivos separados. Sigue el canon para el idioma de episodios.
- En contenido del Pilar F, no presentes una tasa fiscal fija. Somete cada cifra fiscal a revisión del General Counsel por jurisdicción antes de publicarla.
- Lee la referencia del flujo elegido. Las instalaciones requieren confirmación explícita; para acciones con sudo o instalaciones manuales, entrega instrucciones y deja la ejecución al usuario.

## Entrada y selección del flujo

- Sin modo: escanea el estado y presenta un menú de 2–4 opciones aplicables. Genera contenido solo después de que el usuario elija.
- `--estado`: informa el estado sin generar contenido ni modificar archivos.
- `--postura`: inicia el flujo de postura del Pilar F, sin depender de cierres recientes.
- `--entorno`: verifica herramientas; no instala nada automáticamente.
- `--capsula [concepto]`: produce el concepto indicado desde su fuente canónica. Sin concepto, detecta semillas y ofrece opciones.

Al combinar opciones, ejecútalas en el orden indicado. La confirmación para instalar herramientas sigue siendo obligatoria.

## Escaneo

1. Compara todas las Órdenes cerradas con `.agents/state/social-strategist/PROGRESS.md` y `git log`. Procesa cada cierre no registrado en orden cronológico, aunque su commit sea anterior al último procesado. Si falta PROGRESS, cubre desde el primer commit.
2. Calcula la masa narrativa según el canon: 2 o más entradas sin cubrir en un Pilar desbloqueado habilitan una propuesta de episodio.
3. Detecta semillas educativas en fuentes pertinentes de `docs/features/`, `docs/adr/` y TTRs. Prioriza conceptos conectados con trabajo cerrado. No generes durante el escaneo.
4. Verifica disponibilidad de las herramientas requeridas, sin instalar. Consulta el progreso del Tech Lead solo ante dudas de estado o arquitectura, o si el usuario lo pide.
5. Presenta solo opciones aplicables; si no hay cierres ni episodio disponible, ofrece postura, cápsula o estado.

El escaneo no escribe archivos. Si falta PROGRESS, inicialízalo solo cuando el usuario elija un flujo que produzca contenido.

## Producción y cierre

- Lee [production.md](references/production.md) para el flujo elegido, [artifacts.md](references/artifacts.md) para rutas y seguimiento, y [environment.md](references/environment.md) al verificar herramientas.
- Para una cápsula, verifica el concepto en una fuente canónica. Si la implementación no existe, declara ese estado y menciona planes solo cuando la fuente los confirme.
- Antes de cerrar, verifica fuentes, cronología, idioma, líneas rojas y archivos generados. Registra solo contenido efectivamente producido; no marques propuestas como publicadas o terminadas.
