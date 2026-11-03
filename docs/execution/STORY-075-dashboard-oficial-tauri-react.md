# STORY-075 · Dashboard oficial en Tauri y React

> **Orden de trabajo — Spec-Driven.** Implementa la superficie oficial de Dashboard. Sus widgets siguen incompletos hasta recibir datos reales del Core.

| Campo | Valor |
|---|---|
| **ID** | STORY-075 |
| **Título** | Dashboard oficial en Tauri y React |
| **Tipo** | Story (superficie visual) |
| **Épica (Fase)** | EPIC-8 — entrega parcial, no criterio de salida |
| **Sprint** | Sin asignar |
| **Estado** | 🟡 Parcial; faltan datos gRPC reales, persistencia de locale, revisión visual y QA independiente |
| **Responsable** | Frontend-Engineer (Codex) · revisión independiente de QA-Engineer pendiente |
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

## 5. Criterio de aceptación (cada criterio ↔ su verificación)

| # | Criterio verificable | Verificación que lo demuestra |
|---|---|---|
| 1 | TypeScript valida los componentes, traducciones y props del Dashboard. | `dashboard_typecheck` |
| 2 | Vite genera el bundle con Tailwind CSS 4 y recursos Geist locales. | `dashboard_build` |
| 3 | Tauri presenta los cinco widgets y un aviso visible de datos del Core no disponibles, sin métricas ni actividad simuladas. | `dashboard_native_empty_states` (manual) |
| 4 | El selector cambia todas las etiquetas entre español e inglés durante la sesión; no se afirma que la preferencia sobreviva al reinicio. | `dashboard_locale_switch` (manual) |
| 5 | La ventana estrecha refluye los widgets; las acciones disponibles conservan foco visible y pueden activarse por teclado. | `dashboard_responsive_keyboard` (manual) |
| 6 | No hay mock data, personalización o acciones ficticias, cliente gRPC, DTO manuales ni lógica de dominio en el frontend. | `dashboard_scope_audit` (revisión de código y manifiestos) |

## 6. Comandos de validación

```bash
cd apps/desktop && corepack pnpm typecheck
cd apps/desktop && corepack pnpm build
distrobox enter --name personal-env -- bash -ic 'export TERM=xterm-256color; cd "$PWD/apps/desktop" && pnpm tauri dev'
```

Si el último comando informa que el puerto 1420 está ocupado, cierra primero la instancia anterior de `pnpm tauri dev`.

**Recorrido manual:** abre la ventana Tauri, revisa los cinco estados vacíos, cambia entre español e inglés, recorre el control de fijar navegación con teclado y reduce la ventana hasta 320 px. Confirma que Canvas, Estrategias y Datos no se activan y que ningún texto sugiere que hay datos reales. La preferencia de idioma vuelve a español al reiniciar mientras no exista el almacenamiento aprobado.

## 7. Registro de ejecución

| Fecha | Etapa y responsable | Encargo | Resultado y estado | Evidencia, bloqueo o siguiente paso |
|---|---|---|---|---|
| 2026-11-02 | Codex · Frontend-Engineer | Convertir la maqueta en el Dashboard oficial y retirar contenido de demostración | Implementación del cascarón realizada | Componentes separados, cinco estados vacíos y recursos ES/EN. Sin cliente gRPC, datos inventados ni textos de maqueta en `apps/desktop/src`. |
| 2026-11-02 | Codex · typecheck y build | Validar tipos y bundle final | APTO en ambas verificaciones | El primer intento encontró `baseCatalogs` sin uso; se conectó al registro de locales, se endureció la validación de claves y se repitieron ambos comandos en verde. Vite transformó 27 módulos y generó el bundle. No se ejecutaron pruebas automatizadas. |
| 2026-11-02 | Codex · arranque nativo y revisión visual | Ejecutar Tauri y observar la pantalla final | El servidor previo sigue activo; inspección visual no disponible | El nuevo `pnpm tauri dev` compiló/inició el binario Tauri, pero Vite falló al enlazar porque el puerto 1420 ya lo ocupa una instancia anterior. El servidor existente respondió HTTP 200 en `[::1]:1420`. CUA devolvió `apps: []`, `browsers: []`; `listWindows` y el navegador IAB no están disponibles. No se afirma revisión visual. |
| 2026-11-02 | QA-Engineer independiente | Auditar criterios y conjunto final | Pendiente de invocación | La Story no puede cerrarse como completa sin veredicto `APTO`. |

## 8. Pendientes derivados / decisiones

- Datos reales para los widgets: dependen de SPIKE-006 y de los contratos gRPC; su falta impide considerar los widgets funcionales según ADR-0117. Siguiente paso: resolver el Spike y despachar la integración por las Features dueñas. Destino: ROADMAP y Story de integración posterior.
- Persistencia del locale: TTR-001 de `localization` requiere una preferencia persistida; ADR-0165 reserva persistencia al Core y no hay contrato disponible. Siguiente paso: resolverla al implementar el contrato de preferencias. Destino: Feature `localization` y su futura Orden.
- Revisión visual y QA independiente: pendientes; sin ambas la Story conserva estado parcial. Siguiente paso: revisar la ventana final y auditar los seis criterios. Destino: esta Orden.
- Tokens visuales de producción: colores, medidas y materiales de la implementación local no son tokens aprobados. Siguiente paso: formalizarlos desde `docs/DESIGN.yaml`. Destino: `docs/DESIGN.yaml`.

## 9. Cierre ejecutivo

```text
ESTADO: 🟡 PARCIAL O REQUIERE DECISIÓN

PROGRESO MACRO:
- El Dashboard oficial está compuesto en React/Tailwind con cinco widgets vacíos y textos localizados; no muestra datos inventados.
- `corepack pnpm typecheck` y `corepack pnpm build` pasan; Vite responde HTTP 200 en el servidor previo, pero no se pudo observar la ventana final.
- Sin invocaciones de subagentes. QA independiente pendiente. Siguiente paso: revisar la ventana final y pedir auditoría independiente.

FRICCIONES Y DEUDA:
- Los widgets no reciben datos reales porque SPIKE-006 debe resolver la integración gRPC; por ADR-0117 la superficie sigue parcial.
- La preferencia de idioma no persiste porque falta el contrato de ajustes del Core.
- La inspección visual y QA independiente siguen pendientes; los tokens visuales canónicos también faltan.

INPUT REQUERIDO DEL CEO:
- Ninguna.
```
