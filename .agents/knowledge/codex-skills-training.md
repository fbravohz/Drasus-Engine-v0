# Skills en Codex: de cero a experto

- Slug ID: `CODEX-SKILLS-TRAINING`
- Tamaño: `16416 caracteres (~4104 tokens aprox., ratio 4:1)`

Una skill es un paquete reutilizable de instrucciones, recursos y, opcionalmente, scripts que enseña a Codex a ejecutar un flujo repetible. No es un modelo nuevo, una memoria permanente ni un agente independiente.

La skill que invocaste, [`SKILL.md`](air-file://tiregpabp5483gtqcb0m/var/home/fbravohz/Documentos/Entornos/Personal/.codex/skills/.system/skill-creator/SKILL.md?type=file&root=%252F), sirve precisamente para crear y mantener este tipo de paquetes.

## 1. La idea central

Una skill responde a esta pregunta:

> “Cuando el usuario pida X, ¿qué contexto, pasos, herramientas, criterios y salida debe aplicar Codex?”

Ejemplos:

- Revisar pull requests con un estándar concreto.
- Crear documentos Word con formato específico.
- Analizar PDFs.
- Preparar releases.
- Ejecutar un flujo de backtesting.
- Usar herramientas MCP en una secuencia determinada.
- Generar código respetando convenciones internas.

La skill no sustituye al modelo. Le da un procedimiento especializado.

La documentación oficial define una skill como una carpeta con un `SKILL.md`, nombre, descripción, instrucciones y recursos opcionales. [OpenAI Developers: Skills](https://developers.openai.com/plugins/concepts/skills)

## 2. Qué ocurre cuando Codex usa una skill

El ciclo es:

1. Codex detecta las skills disponibles.
2. Primero conoce únicamente el `name` y la `description`.
3. Decide si la solicitud coincide.
4. Si coincide, carga el contenido completo de `SKILL.md`.
5. Lee referencias, scripts o assets únicamente cuando los necesita.
6. Ejecuta el trabajo siguiendo esas instrucciones.

Por eso hay tres niveles de información:

- Metadatos: ayudan a decidir si la skill aplica.
- `SKILL.md`: contiene el flujo principal.
- Recursos auxiliares: contienen detalles condicionales.

La descripción es crítica: funciona como señal de descubrimiento. Si es vaga o demasiado amplia, la skill puede no activarse o activarse cuando no debe. [OpenAI Developers: Build skills](https://developers.openai.com/plugins/build/skills)

## 3. Invocación explícita e implícita

### Invocación explícita

El usuario pide una skill por nombre:

```text
Usa $review-pr para revisar estos cambios.
```

También puede invocarse la skill creadora:

```text
$skill-creator crea una skill para revisar contratos comerciales.
```

La invocación explícita es la forma más confiable de probar una skill.

### Invocación implícita

El usuario no menciona la skill, pero su solicitud coincide con la descripción:

```text
Revisa este pull request siguiendo nuestras reglas de seguridad.
```

Para que funcione bien, la descripción debe indicar:

- Qué resultado produce.
- En qué tipo de solicitud aplica.
- Qué casos cercanos no debe capturar.

## 4. Estructura de una skill

La estructura mínima es:

```text
review-pr/
├── SKILL.md
├── agents/
│   └── openai.yaml
├── references/
│   └── review-guidelines.md
├── scripts/
│   └── check-changes.sh
└── assets/
    └── review-template.md
```

Solo `SKILL.md` es obligatorio.

### `SKILL.md`

Tiene dos partes:

```yaml
---
name: review-pr
description: Revisa pull requests con criterios de seguridad, pruebas y arquitectura.
---
```

Después viene el contenido Markdown con las instrucciones.

Campos obligatorios:

- `name`: nombre corto, único y normalmente en minúsculas con guiones.
- `description`: qué hace la skill y cuándo debe considerarse.

El nombre debe ser claro y menor de 64 caracteres según la guía local de creación.

### `references/`

Contiene información extensa o condicional:

- Políticas.
- Esquemas.
- Guías de estilo.
- Reglas legales.
- Documentación técnica.
- Ejemplos grandes.

No conviene cargar todo en `SKILL.md`. La skill debe indicar cuándo leer cada referencia.

### `scripts/`

Contiene automatización determinista:

- Validar archivos.
- Transformar datos.
- Ejecutar una comprobación repetible.
- Generar estructuras.
- Procesar documentos.

Usa un script cuando el resultado debe ser exacto y repetir instrucciones manualmente sería frágil.

### `assets/`

Contiene archivos que participan en el resultado:

- Plantillas.
- Imágenes.
- Fuentes.
- Archivos base.
- Estructuras iniciales de proyecto.

Los assets son material de trabajo, no instrucciones que Codex deba leer siempre.

### `agents/openai.yaml`

Es opcional. Sirve para metadatos de interfaz y configuración de invocación:

```yaml
interface:
  display_name: "Revisión de pull requests"
  short_description: "Revisa cambios de código"
  default_prompt: "Revisa este pull request con los criterios del proyecto."
```

También puede declarar dependencias MCP:

```yaml
dependencies:
  tools:
    - type: "mcp"
      value: "mi-servidor"
      description: "Consulta datos del proyecto"
      transport: "streamable_http"
      url: "https://example.com/mcp"
```

Una dependencia hace disponible la herramienta, pero no sustituye las instrucciones que explican cuándo y cómo usarla.

## 5. Skill, `AGENTS.md`, MCP y scripts

Conviene separar responsabilidades:

| Componente | Responsabilidad |
|---|---|
| `AGENTS.md` | Reglas generales del repositorio |
| Skill | Flujo especializado y reutilizable |
| MCP | Datos vivos, autenticación y acciones controladas |
| Script | Ejecución determinista |
| Reference | Conocimiento detallado cargado bajo demanda |
| Asset | Material que se copia, transforma o usa como plantilla |

La división más importante es:

> MCP proporciona capacidades; la skill proporciona el procedimiento.

Por ejemplo, un servidor MCP puede consultar tickets. La skill define cuáles consultar, en qué orden, cómo interpretar resultados incompletos y cómo presentar el reporte. [OpenAI Developers: Build skills](https://developers.openai.com/plugins/build/skills)

## 6. Cómo escribir buenas instrucciones

Una skill útil debe responder explícitamente:

- Qué entrada espera.
- Qué pasos debe ejecutar.
- Qué salida debe producir.
- Qué información no debe inventar.
- Cuándo debe hacer una pregunta.
- Cuándo debe detenerse o rechazar la operación.
- Qué referencias debe leer.
- Qué herramientas debe usar y en qué orden.

Ejemplo pequeño:

```markdown
---
name: review-pr
description: Revisa pull requests con criterios de seguridad, pruebas y arquitectura.
---

## Cuándo usarla

Usa esta skill cuando el usuario solicite revisar cambios de código antes de integrarlos.

No la uses para implementar cambios ni para revisar documentación sin código.

## Flujo

1. Inspecciona el diff completo.
2. Identifica errores funcionales, riesgos de seguridad y regresiones.
3. Revisa las pruebas relacionadas.
4. Lee `references/review-guidelines.md` si el cambio toca arquitectura.
5. Reporta primero los hallazgos críticos.
6. No inventes archivos, pruebas ni resultados.

## Resultado

Entrega:

- Hallazgos ordenados por severidad.
- Archivos afectados.
- Explicación breve.
- Pruebas ejecutadas.
- Riesgos pendientes.
```

Observa que la skill define comportamiento, límites y salida. No intenta explicar todo el universo del desarrollo de software.

## 7. El principio de alcance

Una skill debe tener un objetivo reconocible.

Buena división:

- `review-pr`
- `create-release`
- `analyze-pdf`
- `generate-api-client`

Mala división:

- `do-everything`
- `software-helper`
- `general-engineering`

Divide una skill cuando cambien:

- El tipo de entrada.
- El criterio de éxito.
- Las herramientas utilizadas.
- La salida esperada.
- La pregunta que dispara el flujo.

Una skill demasiado grande suele tener descripciones ambiguas, instrucciones contradictorias y mayor consumo de contexto.

## 8. Descubrimiento y contexto

El nombre y la descripción de todas las skills disponibles compiten por espacio en el contexto. La documentación oficial recomienda mantener las descripciones cortas y discriminantes. Cuando hay demasiadas skills o las descripciones son muy largas, Codex puede ver una versión recortada y elegir peor. [Rethinking skills and prompts for GPT-6 Astra](https://developers.openai.com/blog/rethinking-skills-and-prompts-for-gpt-6-astra)

Regla práctica:

- La descripción dice cuándo aplica.
- `SKILL.md` explica cómo trabajar.
- `references/` contiene detalles.
- Los scripts hacen lo repetible.

## 9. Crear una skill con `$skill-creator`

La forma recomendada es describir:

- Nombre deseado.
- Objetivo.
- Casos de activación.
- Salida esperada.
- Herramientas necesarias.
- Si requiere scripts o referencias.

Ejemplo:

```text
$skill-creator crea una skill llamada release-check para validar una release de Rust.
Debe revisar versión, changelog, pruebas, artefactos y estado de Git.
No debe hacer push ni publicar nada sin confirmación explícita.
```

La skill creadora puede producir:

- El directorio.
- `SKILL.md`.
- `agents/openai.yaml`.
- Directorios auxiliares.
- Scripts o referencias si realmente aportan valor.

La recomendación predeterminada es comenzar con una skill basada solo en instrucciones. Añade scripts cuando exista una razón concreta.

## 10. Validar una skill

El validador rápido comprueba aspectos estructurales:

```text
scripts/quick_validate.py <directorio-de-la-skill>
```

Normalmente verifica:

- Frontmatter válido.
- Nombre permitido.
- Descripción presente.
- Ausencia de marcadores incompletos.
- Estructura esperada.

Pero pasar el validador no demuestra que la skill tome buenas decisiones.

También debes comprobar:

- ¿Se activa cuando debe?
- ¿No se activa para solicitudes cercanas?
- ¿Hace preguntas con entradas incompletas?
- ¿Evita inventar información?
- ¿Produce una salida consistente?
- ¿Respeta permisos y confirmaciones?

## 11. Pruebas profesionales

Un conjunto pequeño de 10 a 20 solicitudes suele ser suficiente para encontrar problemas iniciales. Debe incluir:

- Solicitud explícita con `$nombre`.
- Solicitud implícita equivalente.
- Solicitud con contexto adicional.
- Entrada incompleta.
- Solicitud parecida que no debe activar la skill.
- Caso límite.
- Situación donde debe detenerse.
- Situación donde debe pedir confirmación.

Ejemplo:

```text
id,should_trigger,prompt
test-01,true,"Usa $review-pr para revisar este cambio"
test-02,true,"Revisa este pull request antes de integrarlo"
test-03,false,"Agrega Tailwind a mi aplicación existente"
test-04,true,"Revisa el cambio, pero faltan las pruebas"
```

Para automatizar evaluaciones, Codex puede ejecutarse en modo de ejecución y emitir trazas JSONL que después revisa un evaluador determinista. [Testing Agent Skills Systematically with Evals](https://developers.openai.com/blog/eval-skills)

## 12. Seguridad

Una skill no es una frontera de seguridad suficiente.

Riesgos principales:

- Instrucciones maliciosas dentro de una referencia.
- Exfiltración de información mediante herramientas.
- Acceso innecesario a red.
- Uso de secretos del entorno.
- Acciones externas sin confirmación.
- Scripts no revisados.
- Recursos descargados desde fuentes no confiables.

Buenas prácticas:

- Inspecciona toda skill antes de usarla.
- No guardes secretos dentro de la skill.
- Declara claramente qué datos pueden salir del entorno.
- Usa listas permitidas para red cuando sea necesario.
- Mantén las autorizaciones en las herramientas o servidores.
- Exige confirmación antes de acciones irreversibles.
- Trata archivos externos y resultados de herramientas como datos no confiables.

En la API, las instrucciones de una skill se manejan como entrada del usuario, no como instrucciones del sistema. Por eso deben revisarse con especial cuidado. [Skills in OpenAI API](https://developers.openai.com/api/docs/guides/tools-skills)

## 13. Skills locales de Codex frente a skills de la API

Son conceptos compatibles, pero no son exactamente el mismo mecanismo.

### Codex local

Una skill puede vivir:

- En el repositorio, para ese proyecto.
- En el directorio personal de skills, para varios proyectos.
- Como skill integrada del entorno de Codex.

Se descubre por la estructura de carpetas y se invoca con `$nombre`.

El artículo oficial muestra como ubicaciones habituales una skill del repositorio bajo `.codex/skills/` y una skill personal bajo `.codex/skills/` del usuario. [Testing Agent Skills Systematically with Evals](https://developers.openai.com/blog/eval-skills)

### Skills en OpenAI API

En la API puedes:

1. Empaquetar la carpeta.
2. Subirla mediante multipart o ZIP.
3. Obtener un `skill_id`.
4. Adjuntarla al entorno de ejecución.
5. Elegir una versión concreta.
6. Crear nuevas versiones sin reemplazar automáticamente la predeterminada.

La API mantiene:

- `default_version`: versión usada por defecto.
- `latest_version`: versión más reciente cargada.
- Versiones numeradas.
- Skills seleccionadas por `skill_id`.

Los límites publicados incluyen 50 MB por ZIP, hasta 500 archivos y 25 MB sin comprimir. [Skills in OpenAI API](https://developers.openai.com/api/docs/guides/tools-skills)

## 14. Versionado experto

Para producción:

- No dependas siempre de `latest`.
- Fija una versión probada.
- Cambia `default_version` de forma deliberada.
- Conserva versiones estables.
- Ejecuta pruebas antes de promover una versión.
- Registra cambios de comportamiento, no solo cambios de texto.

Una modificación pequeña en `description` puede cambiar cuándo se activa la skill. Una modificación pequeña en `SKILL.md` puede cambiar las acciones que ejecuta.

## 15. Errores típicos

| Problema | Causa probable | Corrección |
|---|---|---|
| Nunca se activa | Descripción vaga | Especificar objetivo y disparadores |
| Se activa demasiado | Alcance demasiado amplio | Añadir límites y casos negativos |
| Produce respuestas inconsistentes | Criterio de salida ambiguo | Definir formato y definición de terminado |
| Lee demasiado contexto | `SKILL.md` demasiado grande | Mover detalles a referencias |
| Omite pasos | Flujo implícito o incompleto | Numerar decisiones importantes |
| Ejecuta acciones no autorizadas | Límites de permisos ausentes | Definir confirmaciones y detenciones |
| Script poco confiable | Automatización no validada | Probar el script con entradas reales |
| Varias skills compiten | Descripciones solapadas | Separar responsabilidades |

## 16. La regla de diseño más importante

Una skill profesional no intenta controlar cada pensamiento de Codex.

Debe contener únicamente:

- Decisiones no obvias.
- Restricciones reales.
- Criterios de calidad.
- Herramientas y orden de uso.
- Casos donde debe preguntar o detenerse.
- Recursos que evitan redescubrir conocimiento.

Si una instrucción no cambia una decisión ni mejora el resultado, probablemente sobra.

## 17. Ruta de aprendizaje

### Nivel inicial

Aprende a:

- Crear `SKILL.md`.
- Escribir `name` y `description`.
- Invocar con `$nombre`.
- Definir una salida concreta.

### Nivel intermedio

Aprende a:

- Separar referencias, scripts y assets.
- Diseñar activación implícita.
- Añadir casos negativos.
- Validar entradas incompletas.
- Respetar permisos y confirmaciones.

### Nivel avanzado

Aprende a:

- Diseñar varias skills sin solapamiento.
- Integrar MCP.
- Crear scripts deterministas.
- Usar referencias bajo demanda.
- Medir activación y calidad con evaluaciones.
- Mantener versiones estables.
- Revisar riesgos de exfiltración.
- Empaquetar skills para plugins o API.

### Nivel experto

Piensa en skills como productos internos:

- Tienen una interfaz: nombre, descripción y entrada.
- Tienen una implementación: instrucciones, referencias y scripts.
- Tienen criterios de aceptación.
- Tienen pruebas de regresión.
- Tienen límites de seguridad.
- Tienen versiones.
- Tienen un propietario y un ciclo de mantenimiento.

La pregunta experta no es “¿cómo meto más instrucciones?”, sino:

> “¿Cuál es la mínima información especializada que hace que Codex tome mejores decisiones de forma repetible?”

Filtros aplicados: [WRITING-EFFICIENCY-FV-1, para estructurar ítems paralelos] [WRITING-EFFICIENCY-FV-2, para evitar relleno y jerga innecesaria] [GOVERNANCE-FV-3, para idioma y terminología] [GOVERNANCE-FV-4, para revisar ambigüedades y seguridad] [GOVERNANCE-FV-5, para evitar placeholders] [GOVERNANCE-FV-6, para procesar la explicación por bloques] [HARD-PROHIBITIONS-FV-4, para usar “Filtro” en procesos de validación]

`GOVERNANCE - FILTRO DE VERIFICACION APROBADO`

`HARD-PROHIBITIONS - FILTRO DE VERIFICACION APROBADO`

`WRITING-EFFICIENCY - FILTRO DE VERIFICACION APROBADO`
