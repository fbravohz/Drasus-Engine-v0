# Dashboard conectado — punto de reanudación

## Intención y límite vigente

El propietario pidió un Dashboard inicial Tauri/React conectado al Core, sin datos de Features todavía. Después se abordarán Banco de Verificación y Canvas, pero **esta iteración se detiene en Dashboard**. Los equivalentes Tauri/React de Banco y Canvas aún no están implementados. Más adelante se auditará cada Feature backend y se conectará su frontend.

## Estado comprobado

- [STORY-075](../../docs/execution/STORY-075-dashboard-oficial-tauri-react.md) contiene la composición React con cinco widgets vacíos y textos es/en. `corepack pnpm typecheck` y `corepack pnpm build` pasaron. Sigue parcial por inspección visual/QA propia.
- [SPIKE-006](../../docs/execution/SPIKE-006-integracion-grpc-local.md) cerró la viabilidad gRPC con experimento reproducible; el Tech Lead repitió su prueba 1/1.
- [STORY-076](../../docs/execution/STORY-076-bootstrap-escritorio-conectado.md) conecta un Core real con Tauri mediante Protobuf, sesión local, listener loopback efímero, readiness y stream. React recibe estado de conexión; los widgets siguen vacíos. La prueba de proceso encontró que un stream activo impedía apagar Core por EOF; Rust corrigió el cierre y la misma prueba pasó. QA inicial emitió **NO APTO** por historial no durable, DTO TypeScript manual, superficies de Feature ausentes y falta de inspección visual.
- Architect resolvió que el historial de salud y cursor deben persistir, mientras la readiness viva se recalcula en cada arranque. El dueño es `desktop-packaging-manager`; no se reutiliza `instance-continuity` ni `agent_operations`.
- El propietario pudo ver la ventana ejecutada en `personal-env`. CUA del agente devolvió `apps: [], browsers: []`; falta recoger resultados visuales por criterio. Hay recorrido en STORY-076 §6.
- La skill [tech-lead](../skills/tech-lead/SKILL.md) ahora usa por defecto subagentes `gpt-6-luna` con esfuerzo `max` y contexto explícito. La skill [frontend-engineer](../skills/frontend-engineer/SKILL.md) exige tipo generado o prueba de deriva para payloads Tauri–React. Ambas ediciones fueron autorizadas por el propietario; no hay commits.

## Trabajo en curso al escribir

El **mismo subagente Rust Engineer** entregó historial durable para `WatchStatus`. El Tech Lead reprodujo `cargo check --locked -p app`, `cargo test --locked -p desktop-packaging-manager` (6/6) y `cargo test --locked -p app bootstrap_` (8 unitarias + 2 de proceso), todos con código 0. Las pruebas cubren 16 escritores concurrentes, clasificación de errores, hashes, contención, retención, inmutabilidad, dos procesos Core con la misma base y readiness fresca. La migración 0024 y `docs/data-model/desktop_status_events.md` coinciden en columnas, restricciones, índices y triggers. **QA todavía no reaudita esta corrección.** El propietario pidió detener el trabajo antes del siguiente despacho Frontend; no se inició esa fase tras la entrega Rust.

## Pendientes y orden de ejecución

1. **Al reanudar, despachar Frontend Engineer** con Luna max: pasar al Core una ruta estable desde `app_data_dir` Tauri, generar o verificar el tipo del payload Tauri–React y probar deriva. Revalidar host, cierre, build y pantalla. La regla reutilizable ya está en su skill. El propietario pidió detenerse antes de iniciar este paso en la sesión actual.
2. Repetir QA independiente del Dashboard y criterios afectados, incluido el historial durable. QA anterior sigue NO APTO hasta nuevo veredicto. El criterio de superficies Banco/Canvas permanece pendiente por instrucción actual del propietario; no implementarlo ni cerrar TTR-001 como Feature completa.
3. Solicitar al propietario el resultado del recorrido visual de STORY-076 §6 o una captura: estado conectado, cinco widgets vacíos, cambio es/en, foco, teclado, reflujo a 320 px y cierre de ventana. Haber visto la ventana no equivale a verificar esos puntos.
4. Actualizar STORY-075/076, ROADMAP y PROGRESS solo con resultados verificados. El Filtro de mutación de ADR-0133 corresponde al cierre de la Story con correctitud en persistencia; no se ha ejecutado. No crear commit sin instrucción explícita del propietario.

## Comandos de referencia

Los comandos completos viven en [STORY-076 §6](../../docs/execution/STORY-076-bootstrap-escritorio-conectado.md#6-comandos-de-validación). Para reanudar las pruebas principales:

```bash
distrobox enter --name personal-env -- bash -ic 'cargo check --locked -p app'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked -p desktop-packaging-manager'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked -p app bootstrap_'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml'
cd apps/desktop && corepack pnpm typecheck
cd apps/desktop && corepack pnpm build
```

## Resultado de la reanudación

- El mismo subagente Frontend conectó `desktop-server --db` a `app_data_dir/drasus.db` de Tauri, endureció el directorio a `0700` en POSIX y reemplazó el DTO TypeScript libre por una adaptación cuyo descriptor se contrasta con la serialización Rust. Una prueba real reutiliza la base tras reiniciar el host; la prueba de deriva falló al cambiar `connected` por `online` y pasó al restaurarlo.
- QA encontró que el supervisor Tauri descartaba el cursor al relanzar el Core. Frontend lo corrigió y añadió prueba del supervisor real con TTL de 2 s: un `InitialSnapshot`, luego `CursorResumed`, con STOPPED/STARTING históricos consumidos sin ciclo. La prueba falló con dos snapshots al restaurar temporalmente el defecto y pasó tras corregirlo. `OutOfRange` produce `FreshSnapshot`.
- Tech Lead repitió 9/9 pruebas Tauri, typecheck, build y `git diff --check`. QA repitió tres veces el caso de TTL y aprobó la corrección funcional del cursor. En [STORY-075](../../docs/execution/STORY-075-dashboard-oficial-tauri-react.md), QA no encontró defecto funcional bloqueante por código; dejó NO APTO el cierre visual completo.
- El propietario aportó una captura de la ventana nativa: muestra `Core conectado`, selector Español y el primer widget vacío. Confirmó que aún no revisa los otros cuatro widgets, cambio es/en, Tab, 320 px ni cierre con X. El Dashboard sigue **NO APTO para cierre** por esos pasos. Banco y Canvas siguen fuera de esta iteración. TTR-001 continúa parcial por sus superficies obligatorias; no hubo commits.
- Después, el propietario pidió minimizar y maximizar junto al botón de cerrar, sin delegación. Una primera colocación junto al selector de idioma fue corregida: la app usa barra superior propia con los tres botones contiguos y región de arrastre; `decorations=false` retira la barra nativa. `corepack pnpm typecheck`, `corepack pnpm build`, `cargo check --locked --manifest-path apps/desktop/src-tauri/Cargo.toml` y `git diff --check` pasan. `pnpm tauri dev` abrió host y Core en `personal-env`; CUA siguió sin exponer la ventana. Falta confirmación manual de los tres clics. El proceso de prueba quedó abierto para que el propietario lo observe.
- El propietario confirmó los tres clics. La X se conectó a `close_desktop_window`, que espera el apagado del Core; el propietario confirmó que la ventana desapareció y Tech Lead observó que no quedaron procesos `drasus-desktop` ni `drasus`. La sesión nativa terminó con código 0. Tras ese ajuste pasaron formato, check y 9/9 pruebas Tauri, typecheck/build React y `git diff --check`. Quedan por verificar teclado/foco de esos botones y el recorrido visual general del Dashboard; el proceso de prueba ya terminó.
