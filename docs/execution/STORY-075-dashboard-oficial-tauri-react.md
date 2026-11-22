# STORY-075 · Dashboard oficial en Tauri y React

> **Orden de trabajo — Spec-Driven.** Implementa la superficie oficial de Dashboard. Sus widgets siguen incompletos hasta recibir datos reales del Core.

| Campo | Valor |
|---|---|
| **ID** | STORY-075 |
| **Título** | Dashboard oficial en Tauri y React |
| **Tipo** | Story (superficie visual) |
| **Épica (Fase)** | EPIC-8 — entrega parcial, no criterio de salida |
| **Sprint** | Sin asignar |
| **Estado** | 🟡 Parcial; faltan datos de Features, persistencia de locale y verificación manual completa de interfaz |
| **Responsable** | Frontend-Engineer (Codex) · reauditoría independiente de QA registrada en §7 |
| **Creada** | 2026-11-02 |
| **Completada** | — |

## 0. Resumen de la Orden

La aplicación de escritorio necesita mostrar su Dashboard oficial en lugar de una maqueta aislada.

- Trasladar la composición aprobada de `drasus-dashboard.html` a componentes React/TypeScript y Tailwind CSS 4.
- Presentar cinco widgets de lectura con estados vacíos localizados; no inventar resultados, portafolios ni actividad.
- Adaptar la cuadrícula a ventanas estrechas y conservar controles de teclado y foco visible.

La pantalla queda establecida como el Dashboard del producto. Los datos reales por gRPC se integrarán cuando SPIKE-006 cierre las decisiones de transporte; hasta entonces la Story permanece parcial.

## 1. Especificación de origen

- **Maqueta:** [`drasus-dashboard.html`](../design/drasus-dashboard.html)
- **Roadmap:** [`EPIC-8`](../ROADMAP.md#epic-8-canvas-forgereactor-tbd-y-pulido-adr-0136-adr-0117)
- **Base de escritorio:** [`STORY-074`](./STORY-074-cascaron-tauri-react.md)
- **Feature transversal:** [`localization`](../features/localization.md)
- **ADR(s):** [ADR-0117](../adr/ADR-0117.md) (cáscaras con datos reales), [ADR-0136](../adr/ADR-0136.md) (Dashboard read-only), [ADR-0142](../adr/ADR-0142.md) (Spike requerido antes del servidor gRPC), [ADR-0160](../adr/ADR-0160.md) (localización), [ADR-0165](../adr/ADR-0165.md) (Tauri/React y frontera gRPC)
- **Diseño:** [`DESIGN.yaml`](../DESIGN.yaml); la composición del Dashboard está aprobada, pero los valores exactos de tokens y componentes siguen pendientes.

### Decisiones y límites

| Tema | Decisión | Fuente |
|---|---|---|
| Estado del Dashboard | Esta es la superficie oficial del producto. El HTML queda como referencia de composición y no se presenta como otra aplicación de muestra. | ADR-0136, `docs/DESIGN.yaml`, aclaración del propietario |
| Widgets y datos | Mostrar resultados, período, investigación, portafolios y riesgo. Cada widget presenta un estado vacío hasta recibir su información real del Core; no hay cifras ni actividad ficticias. | ADR-0117, ADR-0136 |
| Transporte | No implementar cliente ni servidor gRPC en esta Story. SPIKE-006 debe resolver la integración antes del slice vertical. La vista no se considera funcionalmente completa sin datos reales. | ADR-0142, ADR-0165, ROADMAP |
| Estilos | Tailwind CSS 4, recursos Geist locales y reglas compartidas en la hoja de la aplicación. Los valores CSS de esta pantalla son implementación local, no tokens aprobados. | ADR-0165, `docs/DESIGN.yaml` |
| Localización | Recursos JSON paralelos en español e inglés y selector funcional en la sesión. La preferencia no se persiste: ADR-0165 reserva la persistencia al Core y aún no hay contrato para ella. | ADR-0160, ADR-0165, Feature `localization` |
| Interacción | Navegación lateral con Dashboard activo, otras rutas deshabilitadas y control local para fijar la barra. No se simulan operaciones de producto. | ADR-0136 |

## 2. Objetivo

Establecer en Tauri la composición oficial del Dashboard con cinco estados vacíos localizados, preparada para integrar datos reales por la frontera aprobada.

## 3. Agentes y Modo de Acompañamiento

| Agente | Etapa del pipeline | Depende de | Modo |
|---|---|---|---|
| Frontend-Engineer | Etapa 4 — superficie visual | STORY-074, maqueta y ADR de origen | Autónomo |
| **QA-Engineer independiente** | **Etapa 5 — obligatoria** | **Entrega de Frontend-Engineer** | **Pendiente de invocación** |

## 4. Instrucciones de despacho por agente

### 4.1 Frontend-Engineer

```text
Implementa STORY-075 en apps/desktop a partir de docs/design/drasus-dashboard.html como Dashboard oficial de Drasus.

Compón la pantalla con componentes React/TypeScript pequeños y props tipadas. Separa la definición estática de widgets de su presentación. Usa Tailwind CSS 4, las fuentes Geist locales y la capa de estilos de la aplicación. Respeta la modularidad del AGENTS.md y no declares los valores CSS locales como tokens de producción.

Incluye cinco widgets —resultados, período, investigación, portafolios y riesgo— en la composición de la maqueta. No presentes cifras, actividad, fechas, gráficas, errores ni estados de Core inventados. Mientras no exista integración real, cada widget muestra un estado vacío localizado y la cabecera indica que los datos del Core no están disponibles.

Localiza todas las cadenas visibles en español e inglés con recursos JSON en paridad. Conserva el selector de idioma solo como estado de sesión; no añadas persistencia UI porque ADR-0165 reserva la persistencia al Core y el contrato sigue pendiente. No añadas cliente/servidor gRPC, DTO manuales ni lógica de dominio: ADR-0142 exige SPIKE-006 antes de implementar el servidor.

Conserva controles de navegación que existan en la composición y deja deshabilitadas las rutas todavía ausentes. Adapta la distribución a ventanas estrechas, foco visible y teclado. No implementes edición de widgets, filtros, densidad, drag-and-drop, personalización visual ni acciones de trading.

Compila y ejecuta Tauri. No cierres la Story: deja la verificación visual y QA independiente pendientes si no puedes observar la ventana final.
```

**Resumen de implementación:** la definición estática de cinco widgets queda en `data/dashboard.ts`; `Dashboard`, `WidgetCard`, `WidgetEmptyState`, `DashboardHeader`, `Sidebar`, `IconButton` e `Icon` separan composición, contenido y controles. `LocaleProvider` carga archivos JSON por locale, permite cambiar idioma en sesión y verifica que estén las traducciones base.

### 4.2 QA-Engineer independiente

```text
Audita el código y la aplicación contra STORY-075, ADR-0117, ADR-0136, ADR-0160 y ADR-0165. Repite typecheck y build. En Tauri, confirma los cinco widgets sin cifras ficticias, el aviso de datos no disponibles, el cambio entre español e inglés, la navegación deshabilitada y la adaptación responsive con teclado y foco visible. Revisa que el selector no se presente como persistente, que no haya cliente gRPC/DTO manual ni lógica de dominio en React, y que los valores CSS no se declaren tokens canónicos. Emite APTO o NO APTO por criterio y para el conjunto; no des por satisfecha la integración de datos reales.
```

**Redespacho de QA, 2026-11-09; mismo subagente QA Engineer, `gpt-6-luna` con esfuerzo `max`:**

```text
Carga tu skill qa-engineer y audita STORY-075 sobre el Dashboard actual después de STORY-076. Revisa composición, cinco widgets vacíos sin datos inventados, textos es/en, selector de idioma de sesión, navegación deshabilitada, CSS/tokens, semántica accesible y reflujo por código. Repite typecheck/build y prueba visual nativa si CUA expone la ventana; si no, documenta el límite y emite veredicto por criterio sin afirmar observación visual. STORY-076 incorporó gRPC y el contrato Tauri–React después de STORY-075: no trates su presencia como violación retrospectiva del alcance de esta Story; revisa que React no compute dominio ni duplique DTOs libres de prueba. Registra evidencia y veredicto en STORY-075 §6-7, sin editar código ni iniciar Banco/Canvas. No hagas commits.
```

## 5. Criterio de aceptación (cada criterio ↔ su verificación)

| # | Criterio verificable | Verificación que lo demuestra |
|---|---|---|
| 1 | TypeScript valida los componentes, traducciones y props del Dashboard. | `dashboard_typecheck` |
| 2 | Vite genera el bundle con Tailwind CSS 4 y recursos Geist locales. | `dashboard_build` |
| 3 | Tauri presenta los cinco widgets y un aviso visible de datos del Core no disponibles, sin métricas ni actividad simuladas. | `dashboard_native_empty_states` (manual) |
| 4 | El selector cambia todas las etiquetas entre español e inglés durante la sesión; no se afirma que la preferencia sobreviva al reinicio. | `dashboard_locale_switch` (manual) |
| 5 | La ventana estrecha refluye los widgets; las acciones disponibles conservan foco visible y pueden activarse por teclado. | `dashboard_responsive_keyboard` (manual) |
| 6 | No hay mock data, personalización o acciones ficticias, cliente gRPC, DTO manuales ni lógica de dominio en el frontend. | `dashboard_scope_audit` (revisión de código y manifiestos) |
| 7 | La barra superior muestra minimizar, maximizar/restaurar y cerrar juntos; sus controles tienen nombre accesible, foco y funcionan sin dejar al Core huérfano. | `dashboard_window_titlebar` (configuración y recorrido nativo manual) |

## 6. Comandos de validación

```bash
cd apps/desktop && corepack pnpm typecheck
cd apps/desktop && corepack pnpm build
distrobox enter --name personal-env -- bash -ic 'export TERM=xterm-256color; cd "$PWD/apps/desktop" && pnpm tauri dev'
```

Si el último comando informa que el puerto 1420 está ocupado, cierra primero la instancia anterior de `pnpm tauri dev`.

### Reauditoría de QA, 2026-11-09

```bash
distrobox enter --name personal-env -- bash -ic 'cd apps/desktop && corepack pnpm typecheck'
distrobox enter --name personal-env -- bash -ic 'cd apps/desktop && corepack pnpm build'
distrobox enter --name personal-env -- bash -ic 'cd apps/desktop && pnpm tauri dev'
ps -eo pid,args | rg 'drasus-desktop|vite --host|vite$'
ss -ltnp | rg ':1420'
```

- Typecheck terminó con `tsc --noEmit`; build terminó con `tsc && vite build`, 33 módulos transformados y bundle con Geist local. Ambos comandos salieron con código 0.
- La comparación independiente de `locales/es.json` y `locales/en.json` encontró 38 claves iguales. Para repetirla, ejecuta desde la raíz del repo en el contenedor: `node -e 'const fs=require("node:fs");const b="apps/desktop/src/localization/locales/";const k=x=>Object.keys(JSON.parse(fs.readFileSync(b+x+".json","utf8"))).sort().join("\n");if(k("es")!==k("en"))process.exit(1);console.log(k("es").split("\n").length+" claves iguales")'`.
- `pnpm tauri dev` compiló y arrancó `target/debug/drasus-desktop`; Vite escuchó en `[::1]:1420`. La consulta `await cua.getState()` devolvió `{"apps":[],"browsers":[]}`. La revisión no tuvo acceso directo a la ventana por CUA. Al interrumpir el comando con Ctrl+C, `ps` y `ss` confirmaron que el proceso y el puerto 1420 ya no seguían activos.
- El propietario aportó la captura nativa `/tmp/codex-clipboard-o6j6so.png`, inspeccionada con `view_image`: muestra la parte superior, `Core conectado`, el selector en Español y el estado vacío de Ganancias y pérdidas. Solo acredita ese widget; no acredita los otros cuatro ni una interacción.

**Recorrido manual:** abre la ventana Tauri, revisa los cinco estados vacíos, cambia entre español e inglés, recorre el control de fijar navegación con teclado y reduce la ventana hasta 320 px. Confirma que Canvas, Estrategias y Datos no se activan y que ningún texto sugiere que hay datos reales. La preferencia de idioma vuelve a español al reiniciar mientras no exista el almacenamiento aprobado.

**Barra superior:** confirma que minimizar, maximizar/restaurar y cerrar aparecen juntos a la derecha del título, no junto a Idioma. Activa minimizar y recupera la ventana desde el sistema; activa maximizar y después restaurar; recorre los tres botones con Tab y verifica foco visible. Cierra con la X de esa barra y comprueba que `drasus-desktop` y `drasus desktop-server` terminan. Los comandos para repetir tipos, build y arranque son los de esta sección; `distrobox enter --name personal-env -- bash -ic 'cargo check --locked --manifest-path apps/desktop/src-tauri/Cargo.toml'` comprueba también la capacidad Tauri.

## 7. Registro de ejecución

| Fecha | Etapa y responsable | Encargo | Resultado y estado | Evidencia, bloqueo o siguiente paso |
|---|---|---|---|---|
| 2026-11-02 | Codex · Frontend-Engineer | Convertir la maqueta en el Dashboard oficial y retirar contenido de demostración | Implementación del cascarón realizada | Componentes separados, cinco estados vacíos y recursos ES/EN. Sin cliente gRPC, datos inventados ni textos de maqueta en `apps/desktop/src`. |
| 2026-11-02 | Codex · typecheck y build | Validar tipos y bundle final | APTO en ambas verificaciones | El primer intento encontró `baseCatalogs` sin uso; se conectó al registro de locales, se endureció la validación de claves y se repitieron ambos comandos en verde. Vite transformó 27 módulos y generó el bundle. No se ejecutaron pruebas automatizadas. |
| 2026-11-02 | Codex · arranque nativo y revisión visual | Ejecutar Tauri y observar la pantalla final | El servidor previo sigue activo; inspección visual no disponible | El nuevo `pnpm tauri dev` compiló/inició el binario Tauri, pero Vite falló al enlazar porque el puerto 1420 ya lo ocupa una instancia anterior. El servidor existente respondió HTTP 200 en `[::1]:1420`. CUA devolvió `apps: []`, `browsers: []`; `listWindows` y el navegador IAB no están disponibles. No se afirma revisión visual. |
| 2026-11-02 | QA-Engineer independiente | Auditar criterios y conjunto final | Pendiente de invocación | La Story no puede cerrarse como completa sin veredicto `APTO`. |
| 2026-11-09 | QA-Engineer independiente | Reauditar STORY-075 con el Dashboard integrado de STORY-076 | `NO APTO` para el cierre integral; código sin defecto funcional bloqueante observado | Typecheck/build y paridad es/en pasan. La captura del propietario acredita solo Core conectado y el primer widget vacío; la aplicación no está expuesta por CUA. El detalle por criterio y las verificaciones manuales pendientes siguen abajo. |
| 2026-11-09 | Codex · implementación directa | Agregar controles junto a cerrar por instrucción del propietario | Barra superior propia implementada; verificación nativa de clics pendiente | La primera versión puso minimizar/maximizar junto a Idioma; se corrigió. `decorations=false` sustituye la barra del gestor por título arrastrable y tres botones contiguos. Tauri habilita solo cerrar, minimizar, alternar maximizado y arrastre; React usa `getCurrentWindow()` y conserva el cierre ordenado existente. Typecheck, build, `cargo check` Tauri y `git diff --check` pasan. Falta recorrer visualmente los tres botones y el cierre por X en `personal-env`. No se delegó ni se hizo commit. |
| 2026-11-10 | Propietario y Codex | Verificar controles y cierre ordenado de la barra propia | Tres botones operativos por prueba manual; procesos cerrados tras X | El propietario confirmó que minimizar, maximizar/restaurar y cerrar funcionan juntos en la barra. Tras pulsar X en la versión actual, `ps -C drasus-desktop,drasus -o pid,ppid,comm` no encontró ninguno de los dos procesos y la sesión `pnpm tauri dev` terminó con código 0. Se cambió el cierre de la X a `close_desktop_window`, que usa la misma secuencia de apagado ordenado del Core que `CloseRequested`. `cargo fmt --check`, `cargo check --locked` y 9/9 pruebas Tauri, typecheck/build React y `git diff --check` pasaron. El foco de los controles con Tab y el resto de la pantalla aún requieren recorrido visual. |

### Reauditoría independiente, 2026-11-09

| # | Veredicto | Evidencia y límite |
|---|---|---|
| 1 | **APTO** | `corepack pnpm typecheck` terminó con `tsc --noEmit` y código 0. |
| 2 | **APTO** | `corepack pnpm build` terminó con código 0; Vite transformó 33 módulos y empaquetó los dos archivos Geist locales. |
| 3 | **PENDIENTE de verificación visual completa** | `apps/desktop/src/data/dashboard.ts:15,58` define cinco widgets y su orden; `apps/desktop/src/features/dashboard/Dashboard.tsx:18-35` los recorre y los muestra con `apps/desktop/src/components/DashboardWidgets.tsx:8-16` en estado vacío. La captura acredita solo el primero y su aviso de que aparecerán resultados al recibir datos del Core; no certifica visualmente los otros cuatro ni la ventana completa. No se encontraron métricas ni actividad simuladas. |
| 4 | **PENDIENTE de interacción manual** | `apps/desktop/src/localization/LocaleProvider.tsx:14-18` inicia en `es` y solo mantiene la selección en estado React; ambos catálogos tienen 38 claves iguales. La captura muestra Español, pero no un cambio a inglés ni la actualización de todas las etiquetas. No hay persistencia en `localStorage` o `sessionStorage`. |
| 5 | **PENDIENTE de interacción manual** | `apps/desktop/src/styles.css:33,48-52,451,474,555,597` implementa ancho mínimo declarado de 320 px, foco visible para `button`/`select`, reflujo y reducción de movimiento. Falta comprobar visualmente 320 px y activar controles con teclado; CUA no expuso ventana y el propietario confirmó que aún no hizo ese recorrido. |
| 6 | **APTO** | `apps/desktop/src/data/dashboard.ts:15-58` contiene solo claves, orden y layout, sin datos de dominio. `apps/desktop/src/components/DashboardChrome.tsx:13-18,48-59` deshabilita Canvas/Estrategias/Datos. La integración posterior de STORY-076 solo adapta estado del host: React no calcula dominio (`apps/desktop/src/bridge/core.ts:16-42`). El descriptor IPC genera sus tipos TypeScript (`apps/desktop/src/bridge/coreStatusContract.ts:2-35`) y `apps/desktop/src-tauri/src/desktop_core.rs:892` incluye una prueba que contrasta campos, tipos y variantes serializadas; la integración no se trata como incumplimiento retrospectivo de STORY-075. |

**Dictamen:** `NO APTO` para cierre integral de STORY-075 porque siguen sin verificarse tres recorridos manuales obligatorios (cinco widgets en la ventana completa, cambio es/en y teclado/reflujo). Son pendientes de evidencia, no fallos observados. El slice Dashboard delgado —cinco estados vacíos, localización de sesión, navegación futura deshabilitada y estado real del host— queda **APTO por inspección de código**, con verificación visual solo parcial.

**Accesibilidad y tokens:** navegación y sidebar tienen nombres accesibles; el control de fijar expone `aria-pressed`, el selector tiene etiqueta asociada, el estado del Core usa `role="status"`/`aria-live`, y cada tarjeta es una `section` nombrada por su encabezado. **Observación, prioridad baja:** `apps/desktop/src/features/dashboard/Dashboard.tsx:18` asigna `aria-label` al `div` genérico del grid, nombre que puede no exponerse a tecnologías de asistencia. El impacto queda acotado porque las tarjetas se nombran como secciones con sus encabezados (`apps/desktop/src/components/WidgetCard.tsx:14-17`), así que la navegación por regiones/títulos sigue disponible. No bloquea el slice. Las variables CSS locales se proyectan al tema Tailwind, pero `docs/DESIGN.yaml:1685-1688` y `docs/DESIGN.md:66-76,104` identifican los valores de color/medida como provisionales, no tokens aprobados. No se comprobó contraste con una herramienta ni acceso con lector de pantalla.

## 8. Pendientes derivados / decisiones

**Actualización posterior, 2026-11-09:** la revisión de arquitectura siguiente describe el corte de STORY-075 previo a STORY-076. STORY-076 añadió el supervisor Tauri, el Core local por gRPC y el estado real de conexión en el Dashboard. Los widgets siguen vacíos hasta recibir datos de sus Features; la conexión del Core no implica que esos datos estén disponibles. Véase [STORY-076](./STORY-076-bootstrap-escritorio-conectado.md) para el estado actual del slice.

### Revisión de arquitectura para conectar el Dashboard

El Dashboard ya es la superficie React oficial, pero el host Tauri solo monta la ventana y `crates/app` solo ofrece CLI: no hay servidor gRPC, esquemas Protobuf, supervisor ni health check (`apps/desktop/src-tauri/src/lib.rs`, `crates/app/src/main.rs`, `crates/app/Cargo.toml`). Por tanto, una pantalla visible no demuestra conexión con el Core. La separación de procesos y contratos sigue siendo viable según [ADR-0165](../adr/ADR-0165.md), [SAD-01](../sad/SAD-01.md) y la ficha de [desktop-packaging-manager](../features/desktop-packaging-manager.md).

| Pregunta | Respuesta y fuente | Clasificación |
|---|---|---|
| ¿Se integra el transporte en esta Orden? | No: esta Orden excluye cliente y servidor gRPC. [ADR-0142](../adr/ADR-0142.md) exige resolver SPIKE-006 antes de abrir la Story del servidor; `desktop-packaging-manager` asigna supervisor, sesión y health check a TTR-001. | Resuelta por canon; abrir Orden distinta después del Spike |
| ¿Qué significa «Dashboard conectado» sin widgets de Features? | El host arranca el Core local, verifica readiness con credencial de sesión y muestra un estado real de conexión; los cinco widgets permanecen vacíos hasta que cada Feature aporte su contrato y datos. [ADR-0165](../adr/ADR-0165.md) y `desktop-packaging-manager` fijan esa frontera. | Resuelta por canon; el observable de estado exacto se define en la Orden de TTR-001 |
| ¿Qué resuelve SPIKE-006? | Ubicación de `.proto` y bindings, integración de generación con ambos proyectos Cargo, mapeo de errores, autenticación en puerto efímero y prueba del canal puntual más stream recuperable. [ADR-0142](../adr/ADR-0142.md), [ADR-0165](../adr/ADR-0165.md) y [ADR-0166](../adr/ADR-0166.md). | Investigación técnica pendiente; no inventar implementación antes del veredicto |
| ¿Qué garantiza el stream? | Vista inicial y cursor recuperable dentro de un alcance explícito; detección de huecos, duplicados y versiones incompatibles. [ADR-0166](../adr/ADR-0166.md). El estado de conexión del shell no sustituye la actividad durable. | Resuelta por canon; contrato concreto pendiente del Spike |

**Secuencia:** cerrar el alcance visual y QA de STORY-075; resolver SPIKE-006 con una prueba mínima de generación y loopback autenticado; ejecutar TTR-001 como slice de transporte y supervisor; conectar al Dashboard solo el estado real del Core. El Banco y Canvas siguen después, conforme al orden indicado por el propietario. No canonizar un estado «conectado» a partir de HTTP de Vite ni de que abra Tauri.

- Datos reales para los widgets: SPIKE-006 cerró y STORY-076 conectó el Core, pero cada widget depende todavía del contrato y la UI de su Feature dueña; su falta impide considerarlo funcional según ADR-0117. Siguiente paso: auditar cada Feature backend y conectar su slice frontend después de Banco y Canvas, conforme al ROADMAP.
- Persistencia del locale: TTR-001 de `localization` requiere una preferencia persistida; ADR-0165 reserva persistencia al Core y no hay contrato disponible. Siguiente paso: resolverla al implementar el contrato de preferencias. Destino: Feature `localization` y su futura Orden.
- Validación visual e interacción manual: pendientes para los cinco widgets completos, cambio es/en, teclado/foco y reflujo a 320 px. QA independiente ya está registrado en §7; conserva la Story parcial hasta completar esos recorridos. Destino: esta Orden.
- Tokens visuales de producción: colores, medidas y materiales de la implementación local no son tokens aprobados. Siguiente paso: formalizarlos desde `docs/DESIGN.yaml`. Destino: `docs/DESIGN.yaml`.

## 9. Cierre ejecutivo

```text
ESTADO: 🟡 PARCIAL

PROGRESO MACRO:
- El Dashboard oficial está compuesto en React/Tailwind con cinco widgets vacíos y textos localizados; no muestra datos inventados.
- `corepack pnpm typecheck` y `corepack pnpm build` pasan. La captura compartida muestra conexión real al Core y un widget vacío; CUA no expuso ventana y no se verificó la pantalla completa.
- QA independiente registrado en §7: `NO APTO` para cierre integral por recorridos manuales pendientes; el código no presenta defecto funcional bloqueante observado.
- La barra superior propia reúne minimizar, maximizar/restaurar y cerrar. El propietario probó los tres botones; tras la X, Tauri y el Core terminaron sin procesos remanentes. Falta comprobar el foco con Tab.

FRICCIONES Y DEUDA:
- Los widgets no reciben datos reales porque sus Features todavía no tienen el slice frontend conectado; por ADR-0117 la superficie sigue parcial.
- La preferencia de idioma no persiste porque falta el contrato de ajustes del Core.
- La revisión visual completa, el cambio de idioma, teclado y reflujo a 320 px siguen pendientes; los tokens visuales canónicos también faltan.
- Los clics de los tres controles y el cierre de procesos están comprobados; falta verificar su foco con Tab porque CUA no expuso la ventana nativa.

INPUT REQUERIDO DEL CEO:
- Completar el recorrido visual pendiente de §6 en `personal-env`: cinco widgets, cambio es/en, Tab/foco —incluidos los controles de la barra— y ventana de 320 px; indicar qué paso falla.
```
