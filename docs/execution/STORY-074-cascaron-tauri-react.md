# STORY-074 · Cascarón de desarrollo Tauri y React

> **Orden de trabajo — Spec-Driven.** Fuente de verdad de la ejecución. Vive en git, no en el chat.

| Campo | Valor |
|---|---|
| **ID** | STORY-074 |
| **Título** | Cascarón de desarrollo Tauri y React |
| **Tipo** | Story (preparación de plataforma) |
| **Épica (Fase)** | EPIC-0 — Fundación residual Tauri/React |
| **Sprint** | Sin asignar |
| **Estado** | 🟡 En ejecución; validación nativa pendiente |
| **Responsable** | Frontend-Engineer (Luna, `gpt-6.1-luna`, razonamiento máximo) · auditan Tech-Lead + QA-Engineer independiente |
| **Creada** | 2026-10-30 |
| **Completada** | — |

---

## 0. Resumen de la Orden

Preparar una base de desarrollo Tauri/React para que Drasus pueda construir su nueva interfaz sin depender del legado Flutter.

- Crear una aplicación Tauri 2 + React/TypeScript + Vite bajo `apps/desktop`, separada del CLI Rust actual.
- Conectar la compilación Tailwind y cargar localmente Geist Sans y Geist Mono, sin agregar valores visuales que siguen pendientes en `docs/DESIGN.yaml`.
- Verificar que la ventana de desarrollo abre con el punto de montaje React vacío.

Esta preparación no implementa una superficie de producto y no cierra el bootstrap gRPC de ADR-0165. El transporte local requiere primero el Spike de ADR-0142.

---

## 1. Especificación de origen

- **Feature:** [`desktop-packaging-manager`](../features/desktop-packaging-manager.md)
- **TTR:** TTR-003 — Cascarón de desarrollo Tauri/React
- **ADR(s):** [ADR-0165](../adr/ADR-0165.md) (Tauri/React, Tailwind y gRPC), [ADR-0142](../adr/ADR-0142.md) (Spike antes del servidor gRPC), [ADR-0135](../adr/ADR-0135.md) (Cáscara Visual), [ADR-0117](../adr/ADR-0117.md) (entrega progresiva), [ADR-0160](../adr/ADR-0160.md) (localización)
- **Diseño:** [`docs/DESIGN.yaml`](../DESIGN.yaml), familias predeterminadas Geist Sans y Geist Mono; valores y componentes visuales pendientes.
- **Roadmap:** [`ROADMAP.md`](../ROADMAP.md) — preparación residual en EPIC-0; empaquetado y distribución en EPIC-8.
- **Referencias primarias:** [Tauri — crear proyecto](https://v2.tauri.app/start/create-project/), [Tauri — configuración de frontend](https://v2.tauri.app/start/frontend/), [Tauri — versiones de WebView](https://v2.tauri.app/reference/webview-versions/), [Tailwind — compatibilidad](https://tailwindcss.com/docs/compatibility), [Vite — requisitos de Node](https://vite.dev/guide/), [Geist — descarga y licencia](https://vercel.com/font).

### Preguntas resueltas antes de implementar

| Pregunta | Respuesta y fuente | Clasificación |
|---|---|---|
| ¿La Story puede implementar gRPC antes del Spike? | No. ADR-0142 fija que la integración del servidor, bindings, mapeo de errores y pruebas autenticadas esperan el Spike. Esta Story solo prepara el shell local. | Resuelta por canon |
| ¿Dónde vive la aplicación React y qué necesita para compilar? | En `apps/desktop`, separada de `crates/app`, que hoy es el binario CLI Rust. La raíz Cargo mantiene miembros explícitos Rust; la aplicación Tauri conserva su proyecto nativo dentro de su propio paquete. Vite requiere Node 20.19+ o 22.12+; se documenta Node 22.12+ como piso ([Vite](https://vite.dev/guide/)). Se usa pnpm, recomendado por el generador oficial de Tauri ([Tauri](https://v2.tauri.app/start/create-project/)). | Decisión técnica reversible, acotada a esta Story; requisito de Vite |
| ¿Qué versión de Tailwind se configura? | Tailwind CSS 4, con `@tailwindcss/vite`. La compatibilidad mínima de navegadores se revisará al definir la matriz de sistemas operativos, porque Tauri usa WebViews del sistema en macOS y Linux. | Decisión del propietario; revisar compatibilidad antes de empaquetar |
| ¿Qué valores visuales implementa? | Solo las familias Geist Sans/Geist Mono ya confirmadas en `docs/DESIGN.yaml`. Colores, tamaños, pesos, espaciado, bordes, opacidad y recetas esperan la definición canónica. Los archivos locales y licencia OFL se obtienen de la [descarga oficial Geist](https://vercel.com/font). | Resuelta por decisiones vigentes y alcance aprobado |
| ¿Hace falta una Cáscara Visual de producto? | No para esta Story: la ventana comprueba el montaje de desarrollo y no presenta controles, navegación ni contenido de producto. Las superficies reales deben cumplir ADR-0135. | No aplica por alcance |
| ¿Qué pasa con el ID distribuible y el piso mínimo de OS? | No se generan instaladores en esta Story. El ID de bundle será solo de desarrollo y no fija el ID de distribución; la matriz de OS se resuelve antes del TTR-002. | Diferido a empaquetado; no bloquea el shell local |

---

## 2. Objetivo

Dejar un shell de desarrollo Tauri/React reproducible y aislado del Core, listo para recibir después las decisiones visuales aprobadas y el slice gRPC.

---

## 3. Agentes y Modo de Acompañamiento (ADR-0120)

| Agente | Etapa del pipeline | Depende de | Modo |
|---|---|---|---|
| Frontend-Engineer | Etapa 4 — cascarón y configuración | Especificación de esta Orden | Autónomo |
| **QA-Engineer independiente** | **Etapa 5 — obligatoria en toda Story** | **Entrega de Frontend-Engineer** | **Autónomo** |

El Tech-Lead reproduce los comandos de §6 y audita el alcance. Etapa 0.3: no aplica Quant ni revisión legal, porque no hay fórmulas, decisiones financieras, datos persistidos o conducta regulada; la viabilidad del shell se respalda en las plantillas oficiales de Tauri y Vite citadas en §1. El barrido Architect de Etapa 0.4 se documenta en §1.

---

## 4. Instrucciones de despacho por agente

### 4.1 Frontend-Engineer

```text
Implementa exclusivamente STORY-074 y TTR-003 en apps/desktop.

Construye una aplicación Tauri 2 con React/TypeScript y Vite, usando pnpm y Tailwind CSS 4 integrado mediante `@tailwindcss/vite`. Activa el modo estricto de TypeScript y ofrece un comando de typecheck. Declara Node.js 22.12 o posterior como requisito y registra el package manager con su lockfile. Mantén el proyecto fuera del binario crates/app y del workspace Cargo raíz; modifica la exclusión raíz solo si Cargo lo requiere para preservar ese límite.

Configura las familias predeterminadas Geist Sans y Geist Mono con archivos WOFF2 locales del paquete oficial de Geist e incluye su licencia OFL. No dependas de una descarga de fuentes durante el uso de la aplicación. La escala, pesos, colores, espaciado, bordes, opacidad, recetas de clases y demás tokens permanecen pendientes de docs/DESIGN.yaml.

La ventana debe abrir con el punto de montaje React vacío. No agregues navegación, controles, texto de producto, pantallas, componentes, estilos de marca ni datos de demostración. No instales HeroUI, shadcn/ui, Radix, TanStack, React Flow ni paquetes de caché.

No implementes Core sidecar/supervisor, health check, credencial local, servidor o cliente gRPC, Protobuf, streams, persistencia ni empaquetado distribuible. ADR-0142 exige el Spike antes del trabajo del servidor. Documenta solo configuración de desarrollo necesaria para repetir la instalación y el arranque.

Entrega el resumen de implementación y la evidencia por criterio. No cierres la Story: requiere auditoría del Tech-Lead y APTO independiente de QA.
```

**Plan de implementación:** Autónomo; la carpeta y límites están definidos en TTR-003. Si la plantilla oficial actual de Tauri no admite el scope sin reintroducir lógica de producto o dependencias gRPC, detén solo esa parte y registra el hallazgo aquí.

### 4.2 QA-Engineer independiente

```text
Audita STORY-074 contra cada criterio y los archivos reales. Repite los comandos de §6, comprueba que la ventana de desarrollo abre vacía, que las fuentes se sirven desde recursos locales y que el alcance no incluye transporte, Core ni componentes de producto. No aceptes el resumen del implementador como evidencia. Emite APTO o NO APTO por criterio y para el conjunto.
```

---

## 5. Criterio de aceptación (cada criterio ↔ su verificación)

| # | Criterio verificable | Prueba que lo demuestra |
|---|---|---|
| 1 | `apps/desktop` contiene una aplicación Tauri 2 + React/TypeScript estricto + Vite con Node.js 22.12+ declarado, `pnpm-lock.yaml` y scripts documentados para instalar, typecheck, compilar y abrir desarrollo. | `desktop_shell_package_build`: `corepack pnpm install --frozen-lockfile`, `corepack pnpm typecheck` y `corepack pnpm build` |
| 2 | La aplicación abre una ventana Tauri y monta React sin mostrar contenido, controles ni navegación de producto. | `desktop_shell_empty_window`: iniciar Tauri dentro de `personal-env`; revisar que la ventana nativa abra vacía |
| 3 | Tailwind CSS 4 está conectado mediante `@tailwindcss/vite` y participa en el build sin definir tokens visuales aún no especificados. | `tailwind_vite_pipeline_builds`: `corepack pnpm build` más revisión de la hoja CSS importada |
| 4 | Geist Sans y Geist Mono se cargan desde archivos locales incluidos en el paquete y su licencia OFL acompaña los recursos. | `geist_fonts_are_local`: inspección de fuentes, licencia y referencias de red en los archivos de estilo |
| 5 | El host Tauri compila separado del workspace Rust raíz; ejecutar Cargo en la raíz no incorpora el host nativo como miembro. | `tauri_host_workspace_isolated`: `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` y `cargo metadata --manifest-path Cargo.toml --no-deps --format-version 1` dentro de `personal-env` |
| 6 | La Story no agrega servidor/cliente gRPC, Protobuf, supervisión del Core, health check, credenciales, UI kit, React Flow, TanStack ni caché. | `desktop_shell_scope_audit`: revisión de manifiestos, fuentes y cambios de la Story |

---

## 6. Comandos de validación

```bash
cd apps/desktop && corepack pnpm install --frozen-lockfile
cd apps/desktop && corepack pnpm typecheck
cd apps/desktop && corepack pnpm build
distrobox enter --name personal-env -- bash -ic 'cd "$PWD/apps/desktop"; pnpm tauri dev'
distrobox enter --name personal-env -- bash -ic 'cargo check --manifest-path "$PWD/apps/desktop/src-tauri/Cargo.toml"'
distrobox enter --name personal-env -- bash -ic 'cargo metadata --manifest-path "$PWD/Cargo.toml" --no-deps --format-version 1'
distrobox enter --name personal-env -- bash -ic 'cargo test --workspace'
```

**Recorrido manual de `desktop_shell_empty_window`:** inicia el comando `distrobox enter` de desarrollo de Tauri; confirma que abre la ventana nativa; verifica que no aparece contenido de plantilla, texto, navegación o controles; cierra la ventana y confirma que el proceso de desarrollo termina. Esta prueba no inicia el Core ni demuestra el cierre ordenado del proceso Core.

**Resultado esperado:** frontend y host compilan; la app de desarrollo abre una ventana vacía; los fuentes de Geist están dentro de los recursos locales; el `cargo metadata` raíz lista solo los miembros del workspace Rust existente.

---

## 7. Registro de ejecución

| Fecha | Etapa y responsable o subagente/modelo | Encargo | Resultado y estado | Evidencia, bloqueo o siguiente paso |
|---|---|---|---|---|
| 2026-10-30 | Tech-Lead · preparación | Separar el cascarón de desarrollo del transporte gRPC y redactar el alcance verificable | Orden preparada; implementación no despachada | TTR-003, ADR-0165 y ADR-0142; permanece pendiente el Spike para el servidor gRPC |
| 2026-11-02 | Codex · implementación directa solicitada | Crear base Tauri/React con Tailwind 4, fuentes locales y ventana React vacía | Parcial: instalación congelada, typecheck, build, servidor Vite, compilación/arranque Tauri y aislamiento del workspace APTO; falta revisión visual independiente | `personal-env` aporta Rust 1.96, WebKitGTK 2.52.3 y GTK 3.24.41. El shell interactivo carga Cargo y el compilador del contenedor; el script `build-personal-env.sh` ahora habilita el shim pnpm con Corepack. Tauri corre desde ese shell. GTK emite el aviso no bloqueante `Failed to load module "canberra-gtk-module"`. |

---

## 8. Pendientes derivados / decisiones

- **SPIKE-006 sigue pendiente:** definir la integración de generación Protobuf con el workspace, ubicación de esquemas y bindings, convenciones de errores/mapeo y pruebas autenticadas de loopback efímero, conforme a ADR-0142. No se implementa en esta Story.
- **TTR-001 sigue pendiente del Spike:** supervisor del Core, readiness/health check y sesión local autenticada pertenecen al siguiente slice de `desktop-packaging-manager`.
- **Para TTR-002:** definir versiones mínimas y arquitecturas por SO, formatos de paquete, instalación WebView/dependencias, firma/notarización y recorrido en máquinas limpias. La Story debe validar Tailwind CSS 4 contra esos WebViews; los criterios y fuentes están en [`desktop-packaging-manager` TTR-002](../features/desktop-packaging-manager.md#ttr-002-bundling-y-actualización).
- **Después de la base:** definir tokens y recetas en `docs/DESIGN.yaml`; luego aplicar esa fuente en Tailwind y ejecutar el piloto visual HeroUI vs shadcn/ui + Radix. Esta Story no selecciona una biblioteca.
- **Caché de datos:** sigue aplazada según `docs/DESIGN.yaml` y el disparador del Roadmap; esta Story no instala TanStack Query.

---

## 9. Cierre ejecutivo

```text
ESTADO: 🟡 PARCIAL O REQUIERE DECISIÓN

PROGRESO MACRO:
- `apps/desktop` ya contiene el shell Tauri/React con Tailwind 4 y fuentes Geist locales; instalación congelada, typecheck, build y servidor Vite verificados.
- Tauri compiló y lanzó su proceso nativo desde `personal-env`; falta revisión visual independiente de la ventana vacía.
- Siguiente paso: completar revisión visual/QA; resolver SPIKE-006 antes de TTR-001 y del slice gRPC.

FRICCIONES Y DEUDA:
- El alcance mínimo de sistemas operativos y el ID distribuible siguen pendientes del empaquetado.

INPUT REQUERIDO DEL CEO:
- Ninguna; queda pendiente la revisión independiente de QA.
```
