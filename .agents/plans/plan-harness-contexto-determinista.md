# Plan: Harness de contexto determinista y auditable

**Estado:** propuesta de arquitectura y recuperación. No autoriza una migración ni modifica el staging actual.

**Decisión recomendada:** construir primero un *harness* local, versionado y verificable que resuelva contexto por contrato; exponerlo por MCP; y dejar una UI de inspección para una fase posterior. No construir un «Agent OS» genérico ni confundirlo con `agentic-mcp-gateway`, que pertenece al producto y atiende otro problema.

## 1. Veredicto ejecutivo

Sí hay pain points claros y repetidos. No son un problema de que falten más reglas Markdown ni de que el modelo sea incapaz de seguirlas. El sistema actual carece de una fuente de verdad ejecutable que responda de forma determinista estas preguntas:

1. ¿Qué piezas de contexto exactas aplican a este rol, flujo y acción?
2. ¿Cuál es su versión y contenido verificables?
3. ¿Se entregaron y reconocieron antes de una acción que depende de ellas?
4. ¿Qué adaptador de plataforma lo hizo cumplir?
5. ¿Qué ocurre si la resolución, un documento o el adaptador fallan?

Hoy esas respuestas viven repartidas entre instrucciones, punteros, scripts y estado efímero. Por ello el sistema puede parecer correcto durante una sesión y, a la vez, no ser reproducible, no ser portable a Codex y no dejar evidencia suficiente para confiar en él.

El objetivo no es forzar que el agente «piense de cierta manera». Es hacer determinista la selección, entrega, versión, comprobación y trazabilidad del contexto que el proyecto ya decidió que es obligatorio.

## 2. Evidencia de la auditoría

### Alcance revisado

- Inventario del historial desde el 15 de junio de 2026: **251 commits**; 114 tocaron `.agents`, `.claude`, `AGENTS.md` o `.codex`.
- Lectura de los diffs y auditorías de los cambios que alteran el sistema de contexto, sus roles, conocimientos, hooks y catálogos; y clasificación de los demás commits por impacto contextual.
- Revisión completa del staging actual: **115 archivos, +6,146 / -2,068**, sin cambios sin stage y con un directorio de runtime sin rastrear.

### Hallazgos que explican la pérdida de confianza

| Pain point | Evidencia | Consecuencia práctica |
|---|---|---|
| Cambios amplios, heterogéneos y difíciles de verificar | `972bab5` mezcló refactor de habilidades, conocimiento, gobernanza y mecánica; el staging actual vuelve a mezclar hooks, documentos, assets, PDFs y dependencias. | No se puede aislar una hipótesis, revertir con seguridad ni saber qué causó una regresión. |
| La política prometida no coincide con su ejecución | Los hooks dependen de `.claude/settings.json`; no existe adaptador equivalente activo para Codex. | «Siempre» significa algo distinto según el agente que se use. |
| La verificación llega tarde o permite continuar | El validador opera después de editar y varios hooks permiten continuar cuando faltan archivos, eventos o reglas resolubles. | El primer cambio no conforme ya llegó al disco; una configuración dañada puede degradarse en silencio. |
| El cumplimiento es una marca, no una prueba | El estado de lectura es un archivo vacío por sesión, sin hash del documento ni invalidez al editar la fuente. | Un agente puede aparecer como enterado de una versión que ya cambió. |
| El catálogo no es una fuente ejecutable sana | `catalog.json` apunta a 15 `ROLE.md` inexistentes y a `agent-platform-mechanisms.md`, también inexistente. | Resolver contexto desde el catálogo no es reproducible en un checkout limpio. |
| El runtime MCP no está recuperable desde Git | Sólo está versionado el catálogo; el árbol contiene metadatos/bytecode de Python, no las fuentes, manifiesto ni pruebas del servidor. | No hay forma fiable de reconstruir ni validar el servidor que se declara canónico. |
| Esquemas y validadores ya discrepan | La auditoría actual informa 4 violaciones duras y 33 avisos; incluso hay whitespace fallando `git diff --cached --check`. | El propio sistema no puede afirmar que su base sea consistente. |
| La recuperación usa búsqueda como si fuera autoridad | La inyección FTS/BM25 actual devuelve sugerencias y declara que el agente decide si leerlas. | Una búsqueda lexical puede ayudar a descubrir, pero no puede determinar obligaciones. |

### Patrón histórico

Los commits de junio a octubre muestran la misma secuencia: concentrar reglas, fragmentarlas para ahorrar contexto, reescribir roles, restaurar contenido perdido, reparar punteros y añadir más controles. La auditoría histórica de `972bab5` a `3007f92` documenta pérdida de comportamiento, referencias desactualizadas y deudas que se declararon pagadas antes de una prueba estable. El problema se ha tratado como edición documental; requiere tratarse como infraestructura con contrato, pruebas y compatibilidad entre plataformas.

## 3. Principios de diseño

1. **Markdown y JSON siguen siendo la fuente humana y versionada.** El sistema no reemplaza la documentación ni la encierra en una base opaca.
2. **El catálogo se compila y valida antes de servir.** Rutas libres, punteros inexistentes, ciclos, IDs duplicados y versiones ambiguas son errores, no recomendaciones.
3. **Las obligaciones se expresan con datos, no con una frase dentro de una skill.** Rol, flujo, acción, plataforma, dependencia, nivel de cumplimiento y hash deben ser parte del contrato.
4. **Búsqueda es descubrimiento; resolución es autoridad.** FTS, etiquetas o ranking pueden sugerir lectura adicional, nunca decidir qué es obligatorio.
5. **Las obligaciones críticas fallan cerradas.** Si no puede resolverse o comprobarse una precondición requerida, la acción protegida se bloquea con una razón accionable. Las sugerencias pueden fallar abiertas.
6. **La evidencia es por contenido.** Una confirmación vincula `document_id + version/hash + sesión + plataforma + momento`. Editar el contenido invalida la confirmación relevante.
7. **MCP define capacidades; las skills definen procedimiento.** El servidor no sustituye los flujos especializados y una skill no es una frontera de cumplimiento.
8. **Paridad antes de declararlo universal.** Codex y Claude sólo pueden llevar el mismo sello cuando ambos adaptadores superan los mismos contratos de integración.
9. **Local primero.** El primer runtime es local por `stdio`, sin secreto, telemetría ni envío de documentación de trabajo. Un HTTP remoto sólo se evalúa si existe una necesidad real de varios equipos o máquinas.

## 4. Arquitectura objetivo

```text
Markdown / JSON versionados
          │
          ▼
 Compilador y validador de catálogo ──► ContextManifest con hash
          │                                      │
          ▼                                      ▼
    MCP drasus_context                    bitácora de evidencia
  resolver / lector / evaluador                 │
          │                                      ▼
          ├──────────────► Adaptador Codex (plugin local)
          └──────────────► Adaptador Claude (hooks delgados)
                                      │
                                      ▼
                         Context Studio, sólo lectura al inicio
```

### Contrato mínimo

El compilador debe producir entidades tipadas y estables:

- `Document` y `Section`: ID, ruta canónica, hash, categoría y contenido o rango exacto.
- `Policy`: requisito, severidad (`informational`, `required`, `blocking`), dependencias y acciones que cubre.
- `Role` y `Workflow`: IDs explícitos, fuentes existentes y sus políticas aplicables.
- `ContextManifest`: resolución ordenada para `(rol, flujo, clase de tarea, acción, plataforma, revisión Git)`.
- `ReadAcknowledgement`: manifiesto y hashes reconocidos, sesión, plataforma y caducidad.
- `Decision`: permitir, bloquear o advertir, con las piezas faltantes y su explicación.

La API MCP propuesta es intencionalmente pequeña:

| Operación | Resultado |
|---|---|
| `resolve_context` | Manifiesto determinista: elementos obligatorios, sugeridos, razones, hashes y orden. |
| `get_context_item` | Contenido exacto de un ID/rango del manifiesto, más su hash. |
| `acknowledge_read` | Registra que una sesión recibió un hash concreto. No acepta una ruta arbitraria. |
| `evaluate_action` | Decisión previa a una acción protegida y las precondiciones pendientes. |
| `validate_catalog` | Referencias, hashes, esquema, ciclos, duplicados y compatibilidad de plataforma. |
| `search_context` | Resultados auxiliares claramente no autoritativos. |

El servidor se implementa como fuente versionada, pruebas incluidas. El spike técnico decidirá si el resolver se entrega como binario Rust local con envoltura MCP o como servidor incluido en un plugin; no se aceptará bytecode o un entorno local como única fuente de verdad. Dado el stack del repositorio, Rust es la opción base para el núcleo del resolver; scripts de plataforma pueden mantenerse delgados donde la plataforma los exija.

## 5. Fases y criterios de salida

### Fase 0 — Recuperación verificable

1. Congelar el alcance del staging actual para este trabajo: no mezclarlo con el harness.
2. Separar conceptualmente hooks, documentos, assets generados y dependencias; no se aprueba un commit conjunto.
3. Corregir la línea base: `git diff --cached --check` limpio y cero violaciones duras del validador.
4. Recuperar desde una fuente confiable o reconstruir las fuentes de `drasus_context`; añadir manifiesto de build, tests y una ejecución en checkout limpio.
5. Corregir el catálogo para que cada rol, documento y flujo referenciado exista o se elimine explícitamente.

**Salida:** una fotografía de base que compila, valida y puede ser reproducida sin archivos residuales de una máquina.

### Fase 1 — Contrato y compatibilidad

1. Escribir el esquema versionado del catálogo y `ContextManifest`.
2. Declarar para cada política: disparador, acción cubierta, severidad, dependencia y plataforma soportada.
3. Diseñar una migración compatible de los punteros existentes; no convertir los ~45 documentos hasta que el compilador valide ambos formatos.
4. Añadir fixtures de regresión provenientes de fallos reales: referencias huérfanas, documentos omitidos, reglas reubicadas, rol inexistente, contenido modificado tras lectura y reanudación de sesión.

**Salida:** el contrato puede resolver los casos históricos sin depender de interpretación del modelo.

### Fase 2 — Resolver, MCP y evidencia

1. Implementar el compilador, validador, resolver y almacenamiento local de reconocimientos por hash.
2. Publicar el MCP local por `stdio` con las seis operaciones anteriores.
3. Proteger las operaciones que modifican estado mediante `evaluate_action` antes de ejecutar, no después.
4. Registrar decisiones sin almacenar secretos ni contenido de usuario ajeno al contexto de proyecto.

**Salida:** pruebas deterministas para resolución, fallos cerrados y reproducción de una decisión desde un manifiesto.

### Fase 3 — Adaptadores de plataforma

1. Convertir los hooks de Claude en adaptadores del contrato; no volver a implementar reglas en expresiones regulares independientes.
2. Empaquetar para Codex un plugin local con skill, MCP y hooks de ciclo de vida cuando corresponda. Las capacidades MCP y el flujo de la skill se mantienen separados, conforme al modelo oficial de plugins de Codex.
3. Ejecutar la misma batería de integración en Claude y Codex: inicio, selección de rol, lectura, edición, cambio de contenido, compactación/reanudación y configuración rota.

**Salida:** paridad demostrada. Mientras una plataforma no supere el contrato, el sistema se etiqueta explícitamente como parcial, no universal.

### Fase 4 — Migración controlada y evaluación

1. Migrar un rol y un flujo piloto, con comparación de manifiestos antes/después.
2. Medir: contexto entregado, falsas inclusiones, precondiciones omitidas, bloqueos incorrectos, tiempo de resolución y fallos de adaptador.
3. Incorporar evaluaciones de activación de skills y solicitudes negativas; un validador estructural no sustituye esas pruebas.
4. Migrar por lotes pequeños y trazables, con cada cambio de esquema, runtime, adaptador y documentación en cambios separados.

**Salida:** migración reversible, con métricas y pruebas que justifican cada lote.

### Fase 5 — Context Studio

La UI es valiosa, pero sólo después de que exista el contrato. Su primer alcance debe ser de lectura y diagnóstico:

1. **Fuente:** navegar Markdown/JSON, hash, historial Git y referencias entrantes/salientes.
2. **Efectivo:** seleccionar rol, flujo y acción para ver el manifiesto exacto que recibirá el agente.
3. **Prueba:** ver decisiones, lecturas reconocidas, bloqueos y por qué ocurrió cada uno.
4. **Edición asistida:** sólo cuando el compilador pueda validar la propuesta antes de aplicarla y generar un diff normal de Git.

No debe ser una segunda base de conocimiento ni un editor que oculte cambios fuera de Git. Su función es hacer visible el contrato y reducir el costo de mantenimiento documental.

## 6. No negociables de aceptación

- Un checkout limpio debe validar catálogo, resolver contexto y ejecutar pruebas sin bytecode, rutas locales o estado residual.
- Ningún ID del catálogo puede apuntar a un archivo ausente.
- Toda acción `blocking` debe tener decisión previa, motivo legible y prueba de integración para cada plataforma declarada.
- Cambiar una fuente invalida automáticamente reconocimientos basados en su hash anterior.
- El mismo input versionado produce el mismo manifiesto ordenado, independientemente de la plataforma.
- La búsqueda no puede elevar una sugerencia a obligación.
- Ninguna migración documental puede borrar contenido o comportamiento sin prueba de equivalencia explícita.
- Los commits mantienen una sola intención: contrato/runtime/pruebas/adaptador/documentación/artefactos por separado.
- `agentic-mcp-gateway` de producto queda fuera de este alcance.

## 7. Decisiones que requiere el agente que continúe

1. Confirmar el nombre y dueño del paquete: recomiendo `drasus_context` como infraestructura de desarrollo local, no como capacidad comercial del producto.
2. Resolver o reconstruir primero las fuentes ausentes del runtime antes de ampliar sus hooks.
3. Elegir el contrato como autoridad y los documentos como entrada compilada; no conservar dos fuentes canónicas.
4. Aceptar que la UI se construye al final: antes de ella, el valor llega por MCP, validación y adaptadores confiables.

## 8. Primer bloque de trabajo recomendado

No empezar por la interfaz ni por reescribir todas las skills. El primer bloque debe producir, en un cambio aislado:

1. un ADR específico para este harness de desarrollo;
2. el esquema mínimo del catálogo/manifiesto;
3. una matriz de compatibilidad Codex/Claude;
4. fixtures de los incidentes históricos;
5. un validador que falle por los 15 roles/documentos ausentes actuales;
6. una decisión documentada sobre recuperar o reconstruir el runtime.

Con eso el agente más pesado tendrá una base factual para diseñar la implementación, sin reutilizar como fundamento el conjunto de cambios actualmente staged.

Filtros aplicados: [CONTEXT-ENGINEER-5, para separar contexto determinista de redacción de prompts] [CONTEXT-INJECTION-SYSTEM-2, para distinguir selección de autoridad] [DOC-FIRST-1, para proponer decisión arquitectónica antes de implementación] [CODEX-SKILLS-TRAINING-5, para separar MCP de procedimiento de skill] [GOVERNANCE-2.1, para declarar alcance y evidencia] [HARD-PROHIBITIONS-4, para nombrar filtros de validación de forma consistente]

`GOVERNANCE - FILTRO DE VERIFICACION APROBADO`

`HARD-PROHIBITIONS - FILTRO DE VERIFICACION APROBADO`
