# STORY-076 · Dashboard conectado al Core local

| Campo | Valor |
|---|---|
| **ID** | STORY-076 |
| **Tipo** | Story |
| **Épica (Fase)** | EPIC-0 — bootstrap de escritorio |
| **Estado** | 🟡 Parcial; ledger, ruta y recuperación integrada de cursor implementados; Dashboard queda NO APTO por verificación visual; Banco/Canvas pendientes |
| **Responsable** | Tech Lead · especialistas por fase |
| **Creada** | 2026-11-08 |
| **Completada** | — |

## 0. Resumen de la Orden

La pantalla Dashboard existe, pero no prueba que el Core funcione. Conectar Tauri al Core local, mostrar readiness real y cerrar el proceso con seguridad. Los cinco widgets de Features siguen vacíos hasta sus slices propios.

**Límite vigente del propietario:** esta iteración termina en el Dashboard conectado. No iniciar Banco de Verificación ni Canvas. Sus entregables siguen siendo necesarios para cerrar TTR-001 como Feature; hasta otra instrucción, la Orden conserva ese cierre parcial.

## 1. Especificación de origen

- [SPIKE-006](./SPIKE-006-integracion-grpc-local.md): prueba y decisiones de transporte.
- [desktop-packaging-manager](../features/desktop-packaging-manager.md): TTR-001.
- [STORY-075](./STORY-075-dashboard-oficial-tauri-react.md): pantalla visual existente.
- [ADR-0142](../adr/ADR-0142.md), [ADR-0165](../adr/ADR-0165.md), [ADR-0166](../adr/ADR-0166.md): contratos y recuperación.
- [ROADMAP](../ROADMAP.md): Dashboard, Banco y Canvas, en ese orden.

## 2. Objetivo

Abrir el Dashboard con una conexión autenticada y observable al Core, sin inventar datos de Features.

## 3. Agentes y Modo de Acompañamiento

| Agente | Etapa | Depende de | Modo |
|---|---|---|---|
| Architect | Arquitectura | SPIKE-006 | Autónomo |
| General Counsel | Legal y privacidad | Architect | Autónomo |
| Quant Engineer | Dominio cuantitativo | General Counsel | Autónomo |
| UI/UX Designer | Diseño de estado y observables | Quant Engineer | Autónomo |
| Rust Engineer | Core y gRPC | UI/UX Designer | Autónomo |
| Frontend Engineer | Host Tauri y Dashboard | Rust Engineer | Autónomo |
| QA Engineer | Auditoría integrada | Frontend Engineer | Autónomo |

## 4. Instrucciones de despacho por agente

Cada fase usa el siguiente encargo exacto, más la entrega anterior registrada en §7. Cada especialista carga su propia skill y escribe evidencia en esta Orden.

### Architect

```text
Carga tu skill architect. Revisa STORY-076 contra SPIKE-006, ADR-0142/0165/0166 y TTR-001. Resuelve contradicciones de arquitectura y registra decisión, citas y bloqueos en §7. No implementes código.
```

### General Counsel

```text
Carga tu skill general-counsel. Revisa exposición legal, privacidad y manejo de credenciales del bootstrap local; registra APTO, condiciones o No aplica con motivo en §7. No implementes código.
```

### Quant Engineer

```text
Carga tu skill quant-engineer. Revisa si el bootstrap y el estado del Dashboard implican métricas, inferencias financieras o simulación; registra APTO, condiciones o No aplica con motivo en §7. No implementes código.
```

**Checklist de revisión:** contrastar entradas, salida y estados visibles; buscar métricas, precios, P&L, señales, simulaciones y fórmulas; separar salud del transporte de disponibilidad de datos de Feature.

### UI/UX Designer

```text
Carga tu skill ui-ux-designer. Define estados visibles de conexión, reconexión y error para el Dashboard, y el observable de verificación de esta infraestructura conforme a ADR-0117. Registra diseño y fuentes canónicas necesarias en §7; no implementes código.
```

### Rust Engineer

```text
Carga tus skills rust-engineer y rust-best-practices. Implementa el servidor gRPC del Core, Protobuf, autenticación local y stream recuperable según SPIKE-006 y ADR-0165/0166. Registra pruebas discriminantes y comandos en §6-7. No pases secretos en argumentos, URL, logs ni UI. Entrega transporte compilable al Frontend Engineer.
```

### Frontend Engineer

```text
Carga tus skills frontend-engineer y vercel-react-best-practices. Implementa el supervisor Tauri, cliente gRPC, readiness, cierre ordenado y estado real de conexión en el Dashboard. Conserva widgets de Features vacíos y localizados. Registra prueba nativa, teclado y estados de error en §6-7.
```

**Redespacho tras la corrección durable de Rust, 2026-11-09; mismo subagente Frontend Engineer, `gpt-6-luna` con esfuerzo `max`:**

```text
Retoma STORY-076 y carga tus skills frontend-engineer y vercel-react-best-practices. El Core ya persiste el historial en desktop-packaging-manager y acepta desktop-server --db; Tech Lead reprodujo cargo check -p app, 6/6 pruebas de la Feature y 8 unitarias + 2 de proceso del Core. Corrige solo el slice Dashboard: pasa al Core una ruta estable de drasus.db derivada de app_data_dir de Tauri, crea el directorio de forma segura y prueba que un reinicio del host reutilice esa base; sustituye el DTO TypeScript manual de apps/desktop/src/bridge/core.ts por binding generado desde el tipo Rust del host o una adaptación con prueba discriminante de deriva; preserva que Core conectado solo aparezca tras readiness autenticada actual. Repite fmt, check y test Tauri, typecheck y build React, y registra cambios, comandos y resultados en STORY-076 §6-7. Si puedes abrir la ventana en personal-env, verifica conexión, cierre desde el botón nativo, teclado y reflujo; si no, conserva el límite real. No implementes Banco ni Canvas ni cierres la Story completa. No hagas commits.
```

**Segundo redespacho por hallazgo QA, 2026-11-09; mismo subagente Frontend Engineer, `gpt-6-luna` con esfuerzo `max`:**

```text
Corrige el defecto confirmado por QA en STORY-076, sin ampliar alcance: run_supervisor relanza Core tras SessionExit::Restart, pero monitor_core recrea cursor=None y pierde la posición durable. Conserva cursor/scope entre sesiones del hijo dentro del mismo host y reanuda WatchStatus después de readiness autenticada del proceso nuevo; si el Core responde OutOfRange, pide snapshot actual y registra FreshSnapshot. Agrega una prueba discriminante del supervisor real que fuerce expiración/cierre del hijo o transición Restart, pruebe replay desde el cursor previo y distinga CursorResumed de InitialSnapshot; demuestra rojo previo y verde posterior. Repite fmt/check/test Tauri y typecheck/build React, y registra cambios y comandos en STORY-076 §6-7. Carga tus skills frontend-engineer y vercel-react-best-practices. No implementes Banco ni Canvas, no declares APTO global y no hagas commits.
```

### QA Engineer

```text
Carga tu skill qa-engineer. Audita Core, transporte, Tauri, React y recorrido integrado. Intenta refutar cada criterio con pruebas discriminantes. Registra APTO o NO APTO del conjunto y evidencia en §7.
```

**Redespacho, 2026-11-09; mismo subagente QA Engineer, `gpt-6-luna` con esfuerzo `max`:**

```text
Reaudita STORY-076 tras las correcciones Rust y Frontend; carga tu skill qa-engineer. Revisa adversarialmente historial/cursor durable y readiness viva tras reinicio, ruta --db basada en app_data_dir con permisos, y adaptación Tauri–TypeScript con prueba de deriva. Reproduce pruebas discriminantes y el Dashboard conectado con datos reales; inspecciona privacidad, cierre y estado visual/teclado si el entorno lo permite. Emite veredicto por criterio y uno separado para el slice Dashboard; conserva el veredicto global de la Story parcial o NO APTO por Banco/Canvas faltantes conforme al canon y al límite del propietario. Registra evidencia, comandos y hallazgos nuevos en STORY-076 §6-7, sin sobrescribir la auditoría histórica. No edites código, no implementes Banco/Canvas y no hagas commits.
```

**Segunda reauditoría, 2026-11-09; mismo subagente QA Engineer, `gpt-6-luna` con esfuerzo `max`:**

```text
Revisa de forma independiente la corrección del hallazgo del cursor en STORY-076; carga tu skill qa-engineer. Comprueba que run_supervisor conserva cursor al relanzar el Core, que CheckReadiness del proceso nuevo precede a Connected, que el replay de STOPPED/STARTING históricos no causa un ciclo de reinicios y que OutOfRange produce FreshSnapshot. Intenta refutar las pruebas nuevas, especialmente el caso de TTL corto con MockRuntime y dos Core reales; repite Tauri tests y validaciones necesarias. Emite un veredicto actualizado del slice Dashboard y del conjunto global, con límites visuales reales. Registra evidencia en STORY-076 §6-7 sin borrar auditorías previas. No edites código, no inicies Banco/Canvas y no hagas commits.
```

## 5. Criterios de aceptación

| # | Criterio | Verificación |
|---|---|---|
| 1 | Core y Tauri compilan el mismo Protobuf versionado, sin DTOs TypeScript manuales. | `bootstrap_codegen` |
| 2 | Core escucha solo en loopback efímero; llamadas sin sesión o inválidas fallan. | `bootstrap_auth` |
| 3 | La sesión es aleatoria, breve y se entrega sin exposición en argumentos, URL, logs ni React. | `bootstrap_session_security` |
| 4 | Tauri inicia Core, espera readiness real y lo cierra sin huérfanos. | `bootstrap_lifecycle` |
| 5 | Unary y stream entregan vista inicial, cursor y recuperación tras desconexión. | `bootstrap_recovery` |
| 6 | Dashboard muestra estado real de conexión, reconexión y error, accesible y localizado; los cinco widgets siguen vacíos. | `dashboard_core_status` |
| 7 | La infraestructura tiene observable real en Banco, nodo Canvas conectable con inspector y widget Dashboard desactivado en producción cuando corresponda a plomería. | `bootstrap_feature_surfaces` |
| 8 | QA emite APTO del conjunto integrado. | `bootstrap_integrated_qa` |
| 9 | El bootstrap no abre conexiones externas ni registra secretos, payloads de trading o detalles sensibles en logs y estado visible. | `bootstrap_privacy_boundary` |

## 6. Comandos de validación

Los comandos de Core, Frontend y la integración quedan aquí. La evidencia inicial de viabilidad está en [SPIKE-006 §6](./SPIKE-006-integracion-grpc-local.md#6-comandos-de-validación).

```bash
distrobox enter --name personal-env -- bash -ic 'cargo fmt -p app -p desktop-packaging-manager -- --check'
distrobox enter --name personal-env -- bash -ic 'cargo check --locked -p app'
distrobox enter --name personal-env -- bash -ic 'cargo build --locked -p app'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked -p desktop-packaging-manager'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked -p app bootstrap_'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked -p app --test desktop_server_pipe -- --nocapture'
distrobox enter --name personal-env -- bash -ic 'cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml -- --check'
distrobox enter --name personal-env -- bash -ic 'cargo check --locked --manifest-path apps/desktop/src-tauri/Cargo.toml'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml bootstrap_typescript_adapter_matches_rust_serialized_contract'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml bootstrap_database_path_creates_private_nested_directory_and_is_stable'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml bootstrap_host_restart_reuses_durable_database_and_live_readiness'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml bootstrap_cursor_consumes_historical_not_ready_events_before_resume'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml bootstrap_supervisor_resumes_cursor_across_expired_child_and_historical_stopped'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml bootstrap_monitor_requests_fresh_snapshot_after_out_of_range_cursor'
distrobox enter --name personal-env -- bash -ic 'cd apps/desktop && corepack pnpm typecheck'
distrobox enter --name personal-env -- bash -ic 'cd apps/desktop && corepack pnpm build'
distrobox enter --name personal-env -- bash -ic 'cd apps/desktop && pnpm tauri dev'
git diff --check
```

**Reproducción rojo/verde de esta iteración:** en `coreStatusContract.ts`, cambia temporalmente `connected` por `online` y ejecuta la prueba `bootstrap_typescript_adapter_matches_rust_serialized_contract`; debe fallar comparando los estados TS con `ConnectionPhase` Rust (salida 101). Restaura `connected` y la misma prueba pasa. Para la ruta, cambia temporalmente el argumento del hijo de `database_path.as_os_str()` a `drasus-frontend-red.db` y ejecuta `bootstrap_host_restart_reuses_durable_database_and_live_readiness`; debe fallar con `el Core creó la DB en la ruta del host` (salida 101). Esa mutación crea un archivo de base de prueba en el directorio del crate Tauri; bórralo al restaurar el argumento. Ambas mutaciones se revirtieron y el artefacto de prueba se retiró antes de la validación final.

Para la reanudación, la prueba `bootstrap_supervisor_resumes_cursor_across_expired_child_and_historical_stopped` dio rojo al mover temporalmente `let mut cursor = None` dentro del ciclo por hijo: falló con dos `InitialSnapshot` donde esperaba uno (salida 101). Restaurar el cursor al ciclo supervisor-wide produjo verde. La prueba `bootstrap_cursor_consumes_historical_not_ready_events_before_resume` también dio rojo al restaurar el retorno temprano para `ready=false`: el cursor quedó en 2 y debía avanzar a 3 (salida 101); con el avance de STOPPED/STARTING históricos, la prueba pasa. `bootstrap_monitor_requests_fresh_snapshot_after_out_of_range_cursor` confirma que `OutOfRange` solicita el snapshot actual y publica `FreshSnapshot`.

**Recorrido visual en `personal-env` para el propietario:** ejecuta el comando `pnpm tauri dev` anterior y observa la ventana nativa. Tras el arranque debe aparecer `Core conectado`, sin números ni actividad inventada, con cinco widgets vacíos. Cambia el idioma a inglés y de vuelta a español; revisa que estado, títulos y acción `Reintentar` cuando aparezca cambien de idioma. Recorre con `Tab` el selector de idioma y el control de fijar navegación; cada foco debe verse. Reduce la ventana a 320 px y confirma que no se corten los controles. Cierra la ventana con su botón nativo y comprueba que terminen `drasus-desktop` y su hijo `drasus desktop-server`. Este recorrido visual no valida la recuperación durable ni Banco/Canvas; la recuperación durable se cubre con las pruebas Core de esta sección. El propietario informó que pudo ver la ventana, pero no ha registrado resultados por cada paso.

**Evidencia visual aportada por el propietario, 2026-11-09:** una captura de la ventana nativa muestra el encabezado `Dashboard`, el indicador `Core conectado`, el selector en Español y el primer widget `Ganancias y pérdidas` en estado `Sin resultados disponibles`, sin cifras simuladas. La imagen solo cubre la parte superior; no muestra los otros cuatro widgets ni prueba interacción. El propietario confirmó que todavía no hizo las comprobaciones de cambio de idioma, Tab, 320 px, recorrido completo ni cierre con la X. Esos puntos siguen pendientes, no fallidos.

**Actualización de ventana, 2026-11-10:** la barra propia sustituyó los controles del gestor. El propietario confirmó minimizar, maximizar/restaurar y cerrar contiguos; tras la X, `ps -C drasus-desktop,drasus -o pid,ppid,comm` no mostró procesos remanentes y `pnpm tauri dev` terminó con código 0. El cierre de la barra invoca `close_desktop_window`, que comparte la secuencia de apagado ordenado con `CloseRequested`. Faltan idioma, cinco widgets completos, foco con Tab y reflujo a 320 px. Los resultados anteriores de QA describen la versión previa a esta barra.

En la repetición Frontend, los formatos, compilaciones, pruebas Core/Tauri, typecheck y build pasaron. Tauri inició `drasus-desktop` y su hijo `drasus desktop-server`; ambos terminaron al cerrar el host. La corrida nativa emitió el aviso no fatal `Failed to load module "canberra-gtk-module"`. CUA devolvió `{"apps":[],"browsers":[]}`, por lo que no hubo captura de ventana ni verificación visual/teclado.

En la reanudación del 2026-11-09, Tauri pasó a Core la ruta `app_data_dir/drasus.db`; el directorio de aplicación quedó con modo `0700` en POSIX incluso si ya existía. La corrida integrada mostró solo sockets Tauri↔Core en loopback y WebKit↔Vite local; el cierre con SIGINT terminó el host y el Core sin dejar procesos ni sockets. CUA volvió a devolver `{"apps":[],"browsers":[]}`: el estado visible, teclado, reflujo y cierre con botón nativo siguen sin verificación visual por el agente.

### Reauditoría QA independiente, 2026-11-09

Se repitieron estos filtros sobre el código integrado actual:

```bash
distrobox enter --name personal-env -- bash -ic 'cargo fmt -p app -p desktop-packaging-manager -- --check'
distrobox enter --name personal-env -- bash -ic 'cargo check --locked -p app'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked -p desktop-packaging-manager'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked -p app bootstrap_'
distrobox enter --name personal-env -- bash -ic 'cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml -- --check'
distrobox enter --name personal-env -- bash -ic 'cargo check --locked --manifest-path apps/desktop/src-tauri/Cargo.toml'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml'
distrobox enter --name personal-env -- bash -ic 'cd apps/desktop && corepack pnpm typecheck'
distrobox enter --name personal-env -- bash -ic 'cd apps/desktop && corepack pnpm build'
git diff --check
```

Para probar que el `--db` directo admite una ruta absoluta con espacios se ejecutó este smoke test aislado; la entrada aleatoria son 32 bytes temporales de sesión y la carpeta se elimina al salir:

```bash
distrobox enter --name personal-env -- bash -ic 'qa_tmp_dir=$(mktemp -d "/tmp/story076 path.XXXXXX")
mkdir -p "$qa_tmp_dir/db subdir"
head -c 32 /dev/urandom | target/debug/drasus desktop-server --db "$qa_tmp_dir/db subdir/drasus.db"
qa_status=$?
rm -rf "$qa_tmp_dir"
exit "$qa_status"'
```

La corrida nativa se inició con `distrobox enter --name personal-env -- bash -ic 'cd apps/desktop && pnpm tauri dev'` en una TTY. En ejecución, `ps -C drasus-desktop -o pid,ppid,args`, `ps -C drasus -o pid,ppid,args` y `ss -tnp 2>/dev/null | rg 'drasus|vite|tauri|:1420'` mostraron host↔Core por `127.0.0.1` y WebKit↔Vite por `::1`; la ruta efectiva fue `app_data_dir/drasus.db`. `stat -c '%a %n' /var/home/fbravohz/Documentos/Entornos/Personal/.local/share/com.drasus.engine.desktop.dev /var/home/fbravohz/Documentos/Entornos/Personal/.local/share/com.drasus.engine.desktop.dev/drasus.db` devolvió directorio `700` y archivo `644`; el modo del archivo queda protegido por el directorio `700`. SIGINT cerró la corrida; `ps -C drasus-desktop,drasus -o pid,ppid,comm,args` y la misma consulta `ss` no encontraron procesos ni sockets remanentes. CUA devolvió `{"apps":[],"browsers":[]}` durante la ejecución, así que no se pudo inspeccionar la ventana ni accionar su botón nativo.

### Reauditoría QA del cursor, 2026-11-09

```bash
distrobox enter --name personal-env -- bash -ic 'cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml -- --check'
distrobox enter --name personal-env -- bash -ic 'cargo check --locked --manifest-path apps/desktop/src-tauri/Cargo.toml'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml desktop_core::tests::bootstrap_supervisor_resumes_cursor_across_expired_child_and_historical_stopped -- --exact --nocapture'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml bootstrap_monitor_requests_fresh_snapshot_after_out_of_range_cursor -- --nocapture'
distrobox enter --name personal-env -- bash -ic 'cargo test --locked -p app bootstrap_'
distrobox enter --name personal-env -- bash -ic 'cd apps/desktop && corepack pnpm typecheck'
distrobox enter --name personal-env -- bash -ic 'cd apps/desktop && corepack pnpm build'
git diff --check
```

El filtro exacto del supervisor se repitió tres veces y cada corrida ejecutó 1/1 prueba en 2.29–2.33 s. La prueba FreshSnapshot pasó 1/1; Tauri completo, 9/9; Core, 8 unitarias + 2 de proceso. Typecheck, build, fmt, check y `git diff --check` pasaron. La corrida Tauri nativa volvió a mostrar únicamente sockets loopback/locales; CUA devolvió `{"apps":[],"browsers":[]}` mientras el host seguía activo. SIGINT terminó host y Core sin procesos ni sockets remanentes.

## 7. Registro de ejecución

| Fecha | Etapa y responsable | Encargo | Resultado | Evidencia y siguiente paso |
|---|---|---|---|---|
| 2026-11-08 | Tech Lead | Abrir TTR-001 tras SPIKE-006 | Orden preparada; fases sin despachar | SPIKE-006 prueba viabilidad, no conexión de producto. |
| 2026-11-08 | Architect | Revisar SPIKE-006, ADR-0142/0165/0166 y TTR-001 | APTO para implementar el slice autenticado, con cierre de Story necesariamente parcial hasta entregar las tres superficies | [SPIKE-006](./SPIKE-006-integracion-grpc-local.md#decisiones-de-integración-para-ttr-001) fija esquema, generación, errores y semántica de transporte; [ADR-0165](../adr/ADR-0165.md) mantiene Core dueño y Tauri como host. El experimento prueba viabilidad, no canal de bootstrap seguro ni stream durable: criterios 3 y 5 exigen evidencia de producto. [ADR-0117](../adr/ADR-0117.md) obliga Banco, Canvas y widget también para plomería. La secuencia Dashboard → Banco → Canvas indicada por el propietario permite entregar primero la conexión, pero no cerrar TTR-001 como completo antes de las otras dos superficies. No hay contradicción arquitectónica que impida iniciar Core y transporte. |
| 2026-11-08 | General Counsel | Revisar privacidad, credenciales y exposición legal del bootstrap local | APTO-CON-CONDICIONES para diseño e implementación; sin clasificación regulatoria nueva por este alcance | Hechos: canal local Core–Tauri, estado de salud y sesión corta; esta Orden no incorpora órdenes, asesoría, datos de mercado, cuenta central ni envío externo. [ADR-0165](../adr/ADR-0165.md) y [desktop-packaging-manager](../features/desktop-packaging-manager.md) prohíben listener externo y secretos en UI/logs; [ADR-0143](../adr/ADR-0143.md) mantiene la política de telemetría por tier y excluye secretos de sesión en todos los tiers. Condiciones: probar ausencia de tráfico externo introducido por el bootstrap; credencial fuera de URL, argumentos, logs y React; no registrar payloads de trading; verificar que el estado visible no revele información sensible. Los criterios 2, 3 y 6 cubren parte; agregar prueba explícita de egreso y logs antes de QA. No hay jurisdicción de comercialización ni flujo de datos personales nuevo documentado para aplicar una ley concreta; revisión legal de distribución queda en su fase. Fuentes primarias consultadas 2026-11-08: [ley mexicana vigente](https://www.diputados.gob.mx/LeyesBiblio/pdf/LFPDPPP.pdf) y [RGPD](https://eur-lex.europa.eu/eli/reg/2016/679/oj/eng/); se citan para vigencia, no como normas aplicables asumidas. No se adopta LDR nuevo. |
| 2026-11-08 | Quant Engineer | Revisar alcance de métricas, inferencias y simulación | No aplica al cálculo cuantitativo; APTO para pasar a diseño | STORY-076 §0, §2 y criterios 1–9 solo cubren ciclo de vida, autenticación, transporte y estado de conexión. [ADR-0165](../adr/ADR-0165.md) reserva cálculo y dominio al Core. [STORY-075](./STORY-075-dashboard-oficial-tauri-react.md) mantiene cinco widgets sin cifras ni actividad simuladas. No hay fórmula, precio, retorno, P&L, sizing, decisión financiera, datos de mercado ni experimento cuantitativo que auditar en este slice. La secuencia/cursor del stream sí requiere pruebas de orden, huecos y reanudación, pero es correctitud de transporte conforme [ADR-0166](../adr/ADR-0166.md), no una métrica financiera. Condición de presentación: estado “Core conectado” solo acredita transporte y readiness; no debe presentarse como datos de Features disponibles ni como resultado validado. La implementación de cada Feature activa después sus controles cuantitativos propios. |

| 2026-11-08 | UI/UX Designer | Definir estados de conexión y observable de verificación para TTR-001 | APTO para despachar Rust Engineer con semántica de estados fijada | El Dashboard inicia en “Core conectado” solo tras health gRPC autenticado. Si una sesión ya establecida pierde transporte, muestra “Reconectando al Core”; el host reanuda desde cursor durable o pide una vista inicial nueva si el cursor venció. Ante fallo terminal de recuperación, muestra “No se pudo recuperar la conexión al Core” y “Reintentar”. El botón solicita la recuperación al host y queda deshabilitado mientras está activa. Texto y símbolo acompañan cualquier color. El estado solo acredita readiness del Core; los cinco widgets continúan vacíos. Consumidora: Dashboard. Ventana de Verificación integrada: estado actual informado por Tauri, resultado del health check autenticado y resultado de recuperación del stream (cursor reanudado o vista inicial nueva), con marca temporal real y sin credenciales, payloads ni diagnóstico sensible. Fuentes: [desktop-packaging-manager](../features/desktop-packaging-manager.md), [SPIKE-006](./SPIKE-006-integracion-grpc-local.md#decisiones-de-integración-para-ttr-001), [ADR-0117](../adr/ADR-0117.md), [ADR-0165](../adr/ADR-0165.md), [ADR-0166](../adr/ADR-0166.md), [DESIGN.yaml](../DESIGN.yaml) y [DESIGN.md](../DESIGN.md). Esta etapa habilita la implementación del Dashboard; no completa las superficies del Banco ni Canvas y mantiene la Story parcial hasta entregar las tres y QA. |

## Análisis del Quant-Engineer

**Veredicto:** No aplica al dominio cuantitativo; APTO para continuar a UI/UX. Causa: el alcance actual no produce ni interpreta valores financieros, estadísticos o de mercado. Búsqueda de cobertura en `docs/README.md`, STORY-075/076, ADR-0165/0166 y `apps/desktop/src/features/dashboard/`: no hay contratos de métricas en esta Orden. El límite de confianza es explícito: readiness del Core no acredita calidad ni existencia de datos de Feature.

**Hallazgos:** el alcance está resuelto por canon; ningún vacío cuantitativo confirmado. La prueba de cursor pertenece a transporte. No se proponen fórmulas ni políticas nuevas.

**Pregunta resuelta por Architect:** se conserva el registro durable; el estado vivo se recalcula tras reinicio. Ver el dictamen de recuperación abajo.

### Entrega UI/UX — estados y ventana de verificación

**Veredicto:** APTO para despacho al Rust Engineer. El contrato de presentación queda definido; la implementación y la verificación visual siguen pendientes.

- El Dashboard se muestra tras readiness gRPC autenticada, conforme a `desktop-packaging-manager`; su estado visible inicial es **Core conectado**. Este estado confirma solo el transporte y readiness del Core. No afirma que haya datos o Features disponibles.
- Tras perder una conexión ya establecida, muestra **Reconectando al Core…** hasta que el host recupere el stream desde su cursor durable. Si el cursor venció, el host obtiene una vista inicial nueva; el Dashboard vuelve a **Core conectado** solo después de recibirla.
- Si el host informa un fallo terminal de recuperación, muestra **No se pudo recuperar la conexión al Core** y el botón **Reintentar**. El botón solicita recuperación por el host y queda deshabilitado mientras opera. La UI no muestra códigos crudos, credenciales, payloads ni diagnósticos sensibles.
- El estado permanece en el encabezado existente (`DashboardHeader.core_status`), con texto y símbolo; el color nunca basta. Se anuncia cada transición con la región `role=status`. La acción de reintento usa un botón nativo y conserva foco visible.
- La Feature de plomería es `desktop-packaging-manager`; su consumidora inmediata es el Dashboard. La Ventana de Verificación integrada muestra el estado actual informado por Tauri, el último resultado real de readiness autenticada y el resultado de recuperación (cursor reanudado o vista inicial nueva), con marca temporal. No expone identidad de sesión ni contenido de operaciones.
- El estado de la cabecera no sustituye la vista propia en Banco, el nodo Canvas con inspector ni el widget de plomería desactivado en producción. La secuencia de entrega sigue Dashboard → Banco → Canvas; hasta completar las tres superficies y QA, la Story queda parcial.

### Ejecución Rust Engineer

| Fecha | Responsable | Encargo | Resultado | Evidencia y límite |
|---|---|---|---|---|
| 2026-11-08 | Rust Engineer | Ajustar bootstrap Core, Protobuf, autenticación y cierre por pipe | Slice Rust compilable | [bootstrap.proto](../../proto/drasus/desktop/v1/bootstrap.proto) define `CheckReadiness` y `WatchStatus`; [`crates/app/build.rs`](../../crates/app/build.rs) genera bindings desde el archivo que compila Tauri. `drasus desktop-server` recibe 32 bytes de sesión por stdin, anuncia solo `127.0.0.1` y puerto efímero por una línea JSON, y valida unary y streams con `Authorization: Bearer <hex>`; TTL predeterminado 300 segundos, configurable con `--session-ttl-seconds`. El host mantiene abierto stdin; EOF solicita cierre ordenado en cualquier OS. Ctrl+C y SIGTERM se conservan como alternativas cuando aplican. **Reproducción:** la prueba Tauri real falló a los 3 s con `EOF debe cerrar el Core de forma ordenada`; un `WatchStatus` activo impedía que tonic terminara graceful shutdown. **Corrección:** al pasar readiness a falso, Core publica el evento terminal y cierra los streams activos; una prueba de subprocess verifica EOF, evento final, cierre del stream, join del proceso y salida exitosa. Tauri repitió su misma prueba y pasó. **Límite vigente:** el historial de readiness conserva hasta 64 eventos en memoria y no recupera tras reiniciar el Core. Si debe ser durable sigue pendiente de resolver la contradicción de alcance entre [ADR-0165](../adr/ADR-0165.md) y [ADR-0166](../adr/ADR-0166.md); no se añadió persistencia. No se implementó el host Tauri/React, supervisor, cierre del pipe desde Tauri, superficies de Banco/Canvas ni QA integrado. La verificación de egreso/logs y esos criterios siguen abiertos; Story permanece parcial. |
| 2026-11-08 | Tech Lead | Verificar fase Rust y reproducción Tauri | APTO para fase Rust inicial | `cargo check --locked -p app`, `cargo build --locked -p app`, seis pruebas unitarias `bootstrap_`, la prueba integrada de proceso Core y `bootstrap_supervisor_authenticates_real_core_and_closes_by_eof` pasan. `cargo fmt --check` y `git diff --check` pasan. La semántica del cursor tras reinicio quedó pendiente de dictamen arquitectónico; no se implementó un ledger durable de readiness. |
| 2026-11-09 | Frontend Engineer | Conectar Tauri al Core y mostrar readiness real | Implementación lista para QA; Story parcial | Tauri genera sesión aleatoria, arranca `drasus desktop-server` por pipe, autentica readiness/stream y cierra por EOF; React solo recibe estado localizado es/en. Los cinco widgets siguen vacíos. `cargo check` y 3 pruebas Tauri, 6 pruebas Core + subprocess, typecheck, build y Tauri dev pasan; al cerrar el host no quedan procesos. CUA reporta `apps: [], browsers: []`, sin captura ni revisión visual/teclado. Pendientes: definir en canon si la presencia/readiness exige cursor durable, Ventana de Verificación, Banco, Canvas/inspector, widget de plomería, verificación de egreso/logs y QA independiente. |
| 2026-11-09 | QA Engineer | Auditar Core, transporte, Tauri, React y recorrido integrado; refutar [criterios de aceptación](#5-criterios-de-aceptación) | **NO APTO** | Se reprodujeron [comandos de validación](#6-comandos-de-validación), pruebas Core/Tauri, typecheck/build y arranque real de Tauri. Ver hallazgos, evidencia por criterio y limitación visual en “Auditoría QA Engineer”. |
| 2026-11-09 | Propietario | Informar disponibilidad de la ventana | La ventana nativa fue visible para el propietario al ejecutar en `personal-env`; sin veredicto visual por criterio | CUA del agente devolvió `apps: [], browsers: []`. Preparar recorrido exacto y recoger observaciones de estados, idioma, foco y teclado antes de afirmar revisión visual. |
| 2026-11-09 | Tech Lead | Corregir regla reutilizable de frontera Tauri/React, con autorización del propietario | Skill `frontend-engineer` actualizada | La regla de implementación exige tipo generado o adaptación con prueba de deriva para cada payload de comando/evento; `git diff --check` pasó. El defecto de código detectado por QA sigue pendiente de corrección. |
| 2026-11-09 | Rust Engineer | Implementar historial durable y reanudación de `WatchStatus` según dictamen Architect | Slice Core durable implementado; pendiente reauditoría de QA | `cargo check --locked -p app` pasa; `cargo fmt -p app -p desktop-packaging-manager -- --check` y `git diff --check` pasan. `cargo test --locked -p app bootstrap_` pasa 8 pruebas unitarias y 2 de proceso; `cargo test --locked -p desktop-packaging-manager` pasa 6 pruebas. Incluye dos procesos reales con la misma DB, replay desde cursor previo, readiness nueva por health check del proceso actual y error gRPC por cursor vencido. |
| 2026-11-09 | Tech Lead | Reproducir entrega Rust antes de detener esta iteración | APTO técnico para fase Rust; QA global no repetido | Repetí en `personal-env`: `cargo check --locked -p app`, `cargo test --locked -p desktop-packaging-manager` (6/6) y `cargo test --locked -p app bootstrap_` (8 unitarias + 2 procesos), todos con código 0. Contrasté las 9 columnas, restricciones, índices y triggers de migración 0024 contra `docs/data-model/desktop_status_events.md`; coinciden. `git diff --check` pasó. Pendientes: ruta estable Tauri, DTO TypeScript, revisión visual y nueva QA. |
| 2026-11-09 | Frontend Engineer | Pasar `app_data_dir/drasus.db`, probar reinicio del host y eliminar deriva manual del DTO IPC | Slice Dashboard actualizado; Story continúa parcial y pendiente de QA | `launch_core` resuelve `app_data_dir`, crea/canonicaliza la ruta y aplica permisos POSIX `0700`; el Core recibe `desktop-server --db <ruta>`. Tauri testea directorio anidado, reutilización de ruta, reinicio con la misma DB, replay `STOPPED` y readiness autenticada viva del proceso nuevo. `coreStatusContract.ts` deriva tipos TS de un descriptor JSON; Rust contrasta campos, nombres, tipos y estados con la serialización del host. Rojo: `connected→online` falla la comparación; usar una ruta de hijo distinta falla con `el Core creó la DB en la ruta del host`. Verde final: fmt/check/test Tauri y 6/6 pruebas; typecheck/build React; Core 8 unitarias + 2 procesos y Feature 6/6. `pnpm tauri dev` confirmó `--db` bajo `app_data_dir`, directorio 0700 y sockets de loopback; SIGINT terminó host/Core. CUA no expuso ventana; no se verifican visual, teclado, reflujo ni cierre con botón nativo. Banco, Canvas y QA global siguen pendientes; no se declara APTO global. |
| 2026-11-09 | QA Engineer | Reauditar recuperación durable, ruta Tauri, contrato IPC, Dashboard y privacidad | **NO APTO para el slice Dashboard; NO APTO global de QA. Story permanece 🟡 Parcial por el límite del propietario.** | Repetí los [comandos de reauditoría](#reauditoría-qa-independiente-2026-11-09): ledger 6/6; Core 8 unitarias + 2 de proceso; Tauri 6/6; `cargo check`/`fmt`, `pnpm typecheck`/`build` y `git diff --check` pasan. La prueba Rust de deriva y la de reinicio durable pasan, pero el supervisor no conserva el cursor al relanzar el hijo: evidencia y responsable Frontend en la reauditoría siguiente. Ruta real, directorio POSIX `0700`, ruta con espacios, sockets solo locales y cierre por SIGINT confirmados. CUA no expuso la ventana; estado visual, teclado y cierre con botón quedan sin verificar. Banco/Canvas siguen pendientes y no se implementan en esta iteración. |
| 2026-11-09 | Frontend Engineer | Conservar cursor durable entre hijos Tauri y diferenciar replay no listo de la readiness actual | Corrección implementada; STORY-076 sigue parcial y QA global NO APTO hasta reauditoría | `run_supervisor_with_config` conserva scope/sequence fuera del ciclo de hijos y pasa el mismo cursor a `monitor_core`. La prueba de supervisor usa Tauri MockRuntime, Core real con TTL de 2 s y DB durable: observa un único `InitialSnapshot`, expiración de sesión autenticada, cierre EOF, replay de STOPPED y STARTING (`ready=false`), READY del nuevo proceso y luego `CursorResumed`. Los eventos históricos no disparan relanzamientos; el `CheckReadiness` actual se valida antes del stream. `OutOfRange` limpia el cursor y publica `FreshSnapshot` tras pedir una vista inicial nueva. Rojo reproducido al resetear cursor por hijo (dos snapshots, salida 101) y con retorno temprano de `ready=false` (cursor 2 en vez de 3, salida 101); restaurado el código, Tauri pasa 9/9. `cargo fmt --check`, `cargo check --locked --manifest-path apps/desktop/src-tauri/Cargo.toml`, `cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml`, `corepack pnpm typecheck`, `corepack pnpm build` y `git diff --check` pasan. No cambia la readiness visible antes de health gRPC autenticado; Banco, Canvas y reauditoría independiente siguen pendientes. |
| 2026-11-09 | QA Engineer | Reauditar cursor de supervisor, readiness viva, estados históricos y recuperación OutOfRange | **Corrección de cursor APTO; Dashboard y Story NO APTO para cierre QA. Story 🟡 Parcial.** | `run_supervisor_with_config` mantiene cursor entre hijos; `CheckReadiness` se consulta antes de cualquier `Connected`; el replay de STOPPED/STARTING avanza secuencia sin activar el estado conectado ni un nuevo ciclo. Tauri pasa 9/9, Core 8 unitarias + 2 procesos; el test TTL con MockRuntime y Core real pasó 3/3 corridas (2.29–2.33 s), OutOfRange/FreshSnapshot 1/1, fmt/check/typecheck/build/diff check pasan. Durante Tauri dev, CUA no expuso ventana; no se verifican visual, teclado, reflujo ni cierre con botón nativo. No se encontraron defectos funcionales nuevos en el cursor. |

### Auditoría QA Engineer

**Dictamen:** **NO APTO**. El transporte autenticado y el estado básico del Dashboard compilan y pasan sus pruebas; quedan criterios obligatorios incumplidos o sin verificación independiente.

**Alcance revisado:** diff integrado de Core Rust, Protobuf, supervisor Tauri, bridge React/TypeScript, Dashboard, configuración y pruebas. Fuentes de contraste: [ADR-0165](../adr/ADR-0165.md), [ADR-0166](../adr/ADR-0166.md), [ADR-0117](../adr/ADR-0117.md), [desktop-packaging-manager](../features/desktop-packaging-manager.md) y [DESIGN](../DESIGN.md).

**Criterios y evidencia:**

- **1 — BLOQUEANTE.** Core y Tauri compilan bindings Rust desde el mismo `proto/drasus/desktop/v1/bootstrap.proto`; typecheck y build pasan. Sin embargo, el contrato IPC está duplicado a mano: `apps/desktop/src/bridge/core.ts:4,6,12,20,28` declara los tipos y los aplica a `invoke`/eventos, mientras Rust serializa `CoreStatusSnapshot` en `apps/desktop/src-tauri/src/desktop_core.rs:64`. No hay generación ni verificación de deriva para este DTO de la frontera Tauri/React. ADR-0165 y el criterio prohíben DTOs TypeScript manuales. **Responsable:** Frontend Engineer; entregar binding generado o adaptado con contrato verificable y una prueba que detecte deriva.
- **2 — APTO.** `cargo test --locked -p app bootstrap_` pasa seis pruebas: listener loopback/puerto efímero, sesión ausente/incorrecta/vencida, lectura del pipe y cursor. `cargo test --locked -p app --test desktop_server_pipe -- --nocapture` pasa el proceso Core real con stream activo. El interceptor cubre unary y stream; el host valida que la dirección recibida sea loopback.
- **3 — APTO por inspección y corrida real.** Tauri genera 32 bytes con `OsRng` y los entrega por stdin (`desktop_core.rs:301,304,310,376`); Core aplica TTL predeterminado de 300 s (`crates/app/src/main.rs:112`, `desktop_transport.rs:178,188`) y no emite la credencial (`desktop_transport.rs:118,121,127`). El proceso observado ejecutó `drasus desktop-server` sin secreto en argumentos; los logs de arranque no lo mostraron. El bridge React solo expone fase, recuperación y timestamp (`bridge/core.ts:12`).
- **4 — BLOQUEANTE por verificación parcial.** `cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml` pasa tres pruebas, incluida conexión al Core real y cierre por EOF. `pnpm tauri dev` inició el host y su hijo Core; al enviar SIGINT desde TTY ambos terminaron y no quedaron sus sockets. Esta corrida verifica cierre del proceso host, pero no activa `on_window_event` ni demuestra el cierre por botón nativo. CUA no expone la ventana. **Responsable:** Frontend Engineer; repetir el cierre desde la ventana Tauri y comprobar que el proceso Core y su listener terminan.
- **5 — BLOQUEANTE.** La recuperación solo existe en memoria y retiene hasta 64 eventos (`crates/app/src/desktop_transport.rs:34,256,264,277,294,311`). Cada inicio crea un `stream_scope` aleatorio nuevo (`desktop_transport.rs:265,358`); tras proceso nuevo el cursor previo no puede reproducirse. El host descarta el cursor al recibir `OutOfRange` y pide snapshot nuevo (`desktop_core.rs:528,608`). La prueba `bootstrap_recovery_replays_from_cursor_and_rejects_wrong_scope` cubre replay dentro del mismo proceso, no reinicio. ADR-0166 requiere registro durable y recuperación tras reiniciar. **Responsable:** Rust Engineer; persistir alcance/eventos/cursor y probar reinicio real con datos en almacenamiento durable.
- **6 — BLOQUEANTE por verificación incompleta.** Código incluye estados localizados, texto y símbolo, `role="status"`, botón nativo de reintento y cinco widgets vacíos (`DashboardChrome.tsx:89,95`, `Dashboard.tsx:18,29`). Typecheck y build pasan. No se pudo revisar representación visual, foco ni teclado: CUA devolvió `{"apps":[],"browsers":[]}` mientras Tauri estaba iniciado. **Responsable:** Frontend Engineer; permitir la revisión de la ventana y repetir conexión, reconexión, error, reintento y teclado en español e inglés.
- **7 — BLOQUEANTE.** La aplicación solo monta Dashboard (`apps/desktop/src/App.tsx:8`) y sus cinco widgets de producto (`data/dashboard.ts:15,58`). No existe observable propio en Banco, nodo Canvas conectable con inspector ni widget de plomería desactivado en producción. ADR-0117 exige las tres superficies en esta Orden; su falta mantiene la Story parcial. **Responsable:** Frontend Engineer, con apoyo de Tech Lead para habilitar infraestructura compartida dentro de la Orden; QA debe validar las tres superficies con datos reales.
- **8 — NO CUMPLIDO.** El dictamen integrado de QA es NO APTO por los bloqueantes anteriores.
- **9 — APTO para el bootstrap observado.** Durante `pnpm tauri dev`, `ss -tnp 2>/dev/null | rg 'drasus|vite|tauri|:1420'` mostró solo Tauri↔Core en `127.0.0.1` y WebKit↔Vite en `::1`; no observé conexiones externas. La revisión de fuentes confirma que el listener se ata a `127.0.0.1`, el stderr del hijo está cerrado y el estado React no incluye endpoint, credencial, payload o diagnóstico. La muestra de sockets corresponde a la corrida de QA; el código del bootstrap no declara otro destino de red.

**Comandos reproducidos:** se ejecutaron los comandos de [validación](#6-comandos-de-validación), además del filtro de sockets anterior. Las pruebas, formatos, compilaciones, typecheck, build y `git diff --check` pasaron. La corrida `pnpm tauri dev` inició correctamente y terminó con SIGINT desde TTY; la prueba visual/teclado no estuvo disponible.

**Límite de verificación:** la aplicación Tauri estuvo ejecutándose y el Core aceptó la conexión loopback, pero la herramienta CUA no expuso ventanas. No se certifican diseño visual, foco, teclado ni anuncios del lector de pantalla.

**Copia de referencia:** `Core conectado` / `Core connected`; `Reconectando al Core…` / `Reconnecting to Core…`; `No se pudo recuperar la conexión al Core` / `Could not restore the connection to Core`; `Reintentar` / `Retry`. La implementación usa recursos es/en y localiza el error según códigos estables del host, sin presentar el código técnico.

**Fuentes de diseño:** [DESIGN.yaml](../DESIGN.yaml), fichas `components.specifications.dashboard_header`, `states.connectivity`, `states.error` y `screens.screen_contract`; [DESIGN.md](../DESIGN.md), secciones 5 y 9. Las fuentes de comportamiento son [desktop-packaging-manager](../features/desktop-packaging-manager.md), [ADR-0117](../adr/ADR-0117.md), [ADR-0165](../adr/ADR-0165.md), [ADR-0166](../adr/ADR-0166.md) y [SPIKE-006](./SPIKE-006-integracion-grpc-local.md#decisiones-de-integración-para-ttr-001).

### Dictamen Architect: durabilidad de readiness

**Decisión:** `WatchStatus` necesita historial de readiness y posición de recuperación durables tras reiniciar el Core. La presencia viva sigue siendo efímera: cada proceso debe ejecutar un health check autenticado nuevo y ningún resultado histórico `ready=true` puede presentarse como estado actual. No se requiere una decisión adicional del propietario; [ADR-0165](../adr/ADR-0165.md) exige recuperación desde registro durable, [ADR-0166](../adr/ADR-0166.md) distingue presencia efímera de historial durable y alcance del cursor, [SPIKE-006](./SPIKE-006-integracion-grpc-local.md#decisiones-de-integración-para-ttr-001) fija recuperación durable para TTR-001, y [la persistencia de `desktop-packaging-manager`](../features/desktop-packaging-manager.md#persistencia) ya incluye el resultado de salud.

- **Dueño y almacenamiento:** `desktop-packaging-manager` es dueño del historial. El Core lo conserva en su persistencia local; la UI y Tauri no rehidratan readiness desde el historial. El puerto y la tabla concretos no están definidos en el canon consultado. Rust debe resolverlos dentro de la Feature; si requiere tabla, incluir migración y actualizar `docs/data-model/` conforme a [ADR-0137](../adr/ADR-0137.md), [ADR-0141](../adr/ADR-0141.md) y [AGENTS.md](../../AGENTS.md). No reutilizar `agent_operations`.
- **Recuperación:** el cursor y el alcance durable deben sobrevivir reinicios del proceso para el mismo dueño, nodo y réplica. La recuperación reproduce el historial retenido y evalúa readiness actual desde la instancia activa. Si el cursor venció o quedó fuera de retención, el Core devuelve `OutOfRange` y entrega una vista inicial nueva basada en el estado actual.
- **Verificación requerida:** probar con dos procesos reales la reanudación después del reinicio, que el estado actual no se derive de un `ready=true` histórico y que un cursor vencido fuerce una vista inicial nueva. La retención y el comportamiento ante pérdida de historial deben declararse.

**Estado de implementación:** el criterio 5 sigue **NO APTO**. QA confirmó hasta 64 eventos solo en memoria, alcance aleatorio por proceso y ausencia de prueba de reinicio; la decisión resuelve el contrato, no cubre esos incumplimientos.

**Despacho:** Rust debe implementar la persistencia y la verificación de reinicio. Frontend conserva “Core conectado” solo tras readiness autenticada del proceso actual y no cambia su criterio visual. QA mantiene su dictamen **NO APTO** hasta reproducir cursor durable, estado vivo y recuperación tras reinicio, además de resolver los demás bloqueantes originales.

### Respuesta técnica Rust Engineer: historial durable del Dashboard

La decisión del dictamen anterior quedó implementada en el Core. La feature **desktop-packaging-manager** posee el puerto de historial; la migración [0024_desktop_status_events.sql](../../migrations/0024_desktop_status_events.sql), su [ficha de datos](../data-model/desktop_status_events.md) y el [índice del modelo](../DATA-MODEL.md) describen el ledger append-only. Se agregó [crates/shared/build.rs](../../crates/shared/build.rs) para recompilar shared si cambian las migraciones.

Cada proceso registra **STARTING** y luego **READY**; el cierre ordenado agrega **STOPPED**. Tras muerte abrupta, READY puede quedar como último evento histórico. Un proceso nuevo inicia con readiness viva falsa y solo la marca verdadera después de su propio bootstrap; CheckReadiness lee memoria del proceso actual. El cursor conserva alcance y secuencia a través de reinicios. Replay lógico retiene 64 eventos y devuelve OutOfRange/CURSOR_GAP si el cursor venció; las filas históricas no se borran.

**Evidencia reproducida:** cargo test --locked -p desktop-packaging-manager pasa 6/6; cargo test --locked -p app bootstrap_ pasa 8/8 pruebas unitarias y 2/2 pruebas de proceso. La prueba integrada termina abruptamente el primer Core, abre el segundo con la misma DB y reproduce READY histórico, STARTING y READY actuales con secuencias 2, 3 y 4. Otra prueba confirma que READY histórico no marca vivo al proceso nuevo antes de CheckReadiness; prueba gRPC aparte confirma el gap por retención. La primera corrida encontró no such table: desktop_status_events porque SQLx no había recompilado shared; el build script nuevo declara rerun-if-changed=../../migrations, compila la migración 0024 y las repeticiones pasan.

**Límite de almacenamiento:** desktop-server --db usa por defecto drasus.db, el mismo default del subcomando start; las pruebas pasan una ruta temporal explícita. Tauri aún no pasa una ruta propia y el canon no fija un directorio portable de datos por sistema operativo. El Core hereda el directorio actual del host. Frontend debe resolver el path del host antes del empaquetado distribuido; no agregué configuración de ruta ad hoc ni edité Tauri.

**Estado:** el criterio 5 tiene implementación y pruebas Rust reproducibles; falta reauditoría independiente. El dictamen NO APTO de QA permanece intacto. Los criterios 1, 4, 6 y 7 siguen pendientes; el 8 mantiene NO APTO global y la auditoría anterior calificó el 9 APTO.

### Reauditoría QA Engineer — veredicto por criterio

**Alcance:** revisión independiente del Core, ledger, migración 0024, Protobuf, Tauri, bridge TypeScript, Dashboard y corrida nativa. No se modificó código ni se iniciaron Banco o Canvas.

| Criterio | Veredicto | Evidencia actual |
|---|---|---|
| 1 — Protobuf y contrato IPC | **APTO** | Core y Tauri leen el mismo `proto/drasus/desktop/v1/bootstrap.proto` desde sus build scripts. `bootstrap_typescript_adapter_matches_rust_serialized_contract` pasó 1/1; compara los campos y variantes Rust con el descriptor TypeScript del que se derivan los tipos. Typecheck/build también pasan. |
| 2 — Loopback y autenticación | **APTO** | Las pruebas `bootstrap_` pasan 8 unitarias y 2 de proceso, incluyendo loopback efímero y rechazo de sesión ausente, inválida o vencida. La corrida nativa mostró solo el socket Tauri↔Core en `127.0.0.1`. |
| 3 — Sesión efímera y privada | **APTO** | Inspección de `desktop_core.rs:298-314,364-384,500-509` confirma 32 bytes de `OsRng`, entrega por stdin, borrado al liberar y bearer en metadata gRPC; argumentos contienen solo binario/subcomando/ruta. El endpoint stdout solo anuncia loopback/puerto. La corrida no mostró secretos en argumentos ni salida. |
| 4 — Inicio, readiness y cierre sin huérfanos | **APTO con límite** | Tauri inició el Core real y mantuvo conexión local; las pruebas de proceso pasan. SIGINT desde TTY terminó host/Core y después no había PIDs ni sockets. `desktop_core.rs:160-172` conecta el cierre nativo con cierre por EOF, pero CUA no permitió accionar el botón de ventana; ese recorrido no queda observado. |
| 5 — Vista inicial, cursor y recuperación | **NO APTO integrado; Core APTO aislado** | El ledger y la reanudación entre dos procesos pasan en pruebas reales: stream scope/sequence permanecen en SQLite, un Core nuevo responde readiness viva y acepta el cursor anterior. En Tauri, `monitor_core` crea `cursor = None` por invocación (`apps/desktop/src-tauri/src/desktop_core.rs:512-519`); ante `Unauthenticated` o salida del hijo devuelve `Restart` (`:567-569,605-608,647-649`), y `run_supervisor` vuelve a invocar `monitor_core` (`:207-216`). La nueva solicitud lleva alcance vacío y secuencia cero (`:554-560`); el Core responde solo el snapshot actual en esa rama (`crates/app/src/desktop_transport.rs:330-338`). El historial durable no se reproduce desde Tauri al reiniciar el hijo. |
| 6 — Dashboard localizado y accesible | **NO APTO por verificación incompleta** | Código presenta región `role="status"`, texto/símbolo, botón nativo y cinco widgets vacíos (`DashboardChrome.tsx:89-112`, `Dashboard.tsx:18-35`); typecheck/build pasan y el Core real se conectó en loopback. CUA devolvió `{"apps":[],"browsers":[]}` mientras corría Tauri, así que estado visual, traducciones, teclado, foco, reflujo y anuncio accesible no se pudieron observar. |
| 7 — Banco, Canvas e widget de plomería | **NO APTO** | No existe observable en Banco, nodo Canvas conectable con inspector ni widget de plomería desactivado en producción. El límite del propietario mantiene esta iteración en Dashboard; las superficies no se implementaron ni se presentan como completas. |
| 8 — QA integrado | **NO APTO** | Los criterios 5–7 impiden el cierre integrado. |
| 9 — Privacidad y egreso | **APTO para la corrida observada** | `ss` mostró únicamente loopback Tauri↔Core y WebKit↔Vite local; la inspección no encontró destino externo del bootstrap ni datos sensibles en el payload del Dashboard. En la ruta de Tauri el directorio de app data midió `0700`; el archivo SQLite `0644` está dentro de ese directorio privado. El proceso Core mantiene `stderr` cerrado y la UI solo recibe estado/fase, recuperación y marca temporal. La observación de red corresponde a esta corrida, no a telemetría de todo el producto. |

**BLOQUEANTE — recuperación durable integrada — responsable: Frontend Engineer.** La base Core sí conserva el cursor y el historial, pero el supervisor descarta el cursor en cada reinicio del hijo. El disparador es alcanzable por caída del Core y por expiración de la sesión gRPC (TTL predeterminado de 300 s); la siguiente conexión solicita snapshot en vez de replay. La prueba `bootstrap_host_restart_reuses_durable_database_and_live_readiness` crea clientes gRPC y pasa manualmente el cursor viejo al segundo proceso (`desktop_core.rs:1030-1129`), por lo que prueba la capacidad de Core y la ruta de DB, no la integración `run_supervisor`→`monitor_core`. **Responsable exacto:** Frontend Engineer, en el ciclo del cursor de `desktop_core.rs:182-234,512-668`. **Prueba requerida:** reiniciar el hijo detrás del supervisor con la misma DB y verificar que el siguiente `WatchStatus` envíe el alcance/secuencia anterior, recupere eventos posteriores y mantenga `CheckReadiness` del proceso nuevo como única fuente de estado vivo.

**Veredicto del slice Dashboard:** **NO APTO**. La ruta `app_data_dir/drasus.db`, permisos POSIX, contrato IPC, autenticación y ledger Core pasan; la ruta integrada del cursor falla ante reinicio del hijo y no se pudo validar visual/teclado. El Core subcomponente de durabilidad queda APTO de forma aislada.

**Veredicto global de la Story:** **NO APTO para QA; 🟡 Parcial por alcance del propietario**. Además del defecto integrado, continúan pendientes la Ventana de Verificación, Banco y Canvas/inspector, así como la prueba del botón nativo y el recorrido visual/teclado. No se inicia ese trabajo en esta iteración.

### Reauditoría QA Engineer — corrección del cursor

**Veredicto actualizado del criterio 5:** **APTO para la recuperación integrada del hijo Core**. `run_supervisor_with_config` declara el cursor fuera del ciclo de hijos y pasa la misma referencia mutable a cada `monitor_core` (`apps/desktop/src-tauri/src/desktop_core.rs:202-231`). En cada sesión, el monitor primero hace `CheckReadiness`; solo después puede publicar `Connected` si la respuesta actual dice `ready=true` o al consumir un evento READY posterior (`:586-615,655-697`). Los eventos históricos STOPPED/STARTING incrementan la secuencia y producen `NotReady`; el monitor continúa leyendo sin publicar `Connected` ni reiniciar el supervisor (`:706-709,745-779`). Tras `OutOfRange`, limpia cursor, cambia el resultado a `FreshSnapshot` y solicita de nuevo una vista inicial (`:636-640,722-726`).

**Pruebas independientes:** el test `bootstrap_supervisor_resumes_cursor_across_expired_child_and_historical_stopped` usa `tauri::test::mock_app()`, una DB SQLite temporal, `--session-ttl-seconds 2` y Core subprocess real. Repetido tres veces, cada corrida observó una sola `InitialSnapshot` antes de `CursorResumed`; un límite estable posterior no detectó otro snapshot/ciclo. La secuencia pasa porque el cursor sobrevive al vencimiento, y el replay real incluye STOPPED del hijo cerrado y STARTING/READY del Core nuevo. El test unitario `bootstrap_cursor_consumes_historical_not_ready_events_before_resume` confirma los avances 2→3→4→5 con STOPPED/STARTING no listos y READY final. El test `bootstrap_monitor_requests_fresh_snapshot_after_out_of_range_cursor` pasa 1/1 y publica `FreshSnapshot`.

**Límite de cobertura:** la prueba integrada de `FreshSnapshot` usa un `stream_scope` vencido/diferente, que el Core traduce a `Code::OutOfRange`; no llena la ventana de 64 eventos para generar allí mismo `CURSOR_GAP`. La prueba Core `bootstrap_recovery_rejects_cursor_outside_durable_retention` sí cubre el vencimiento real y devuelve OutOfRange; Tauri maneja ambos motivos por el mismo código gRPC. No encontré una divergencia funcional en esa frontera.

**Resultado del slice Dashboard:** **NO APTO para cierre**. La corrección del cursor pasa y no queda el defecto funcional anterior. CUA volvió a devolver `{"apps":[],"browsers":[]}` durante `pnpm tauri dev`; por eso no se pudieron observar apariencia, idioma, foco, teclado, reflujo ni el cierre por botón nativo. El código y la corrida validan conexión local real y cierre por SIGINT, pero no sustituyen ese recorrido visual.

**Resultado global de QA:** **NO APTO; Story 🟡 Parcial**. Criterio 7 sigue pendiente por la Ventana de Verificación y las superficies de Banco/Canvas/inspector/widget exigidas por ADR-0117; el propietario mantiene fuera de alcance iniciar Banco y Canvas. No hay hallazgos funcionales nuevos sobre la corrección del cursor.

## 8. Pendientes derivados / decisiones

- QA reprodujo la conservación del cursor, replay STOPPED/STARTING/READY y FreshSnapshot tras OutOfRange. El recorrido visual continúa pendiente por el límite de CUA.
- La ruta estable `app_data_dir/drasus.db` y la adaptación IPC Rust/TypeScript con prueba de deriva pasan sus pruebas dirigidas; no sustituyen la reauditoría visual ni el cierre integrado.
- Entregar la Ventana de Verificación y las superficies restantes de plomería: observable propio en Banco, nodo Canvas conectable con inspector y widget Dashboard desactivado en producción.
- La ejecución Tauri fue real, pero CUA no expuso ventanas; QA debe observar visualmente conexión, error, reintento, traducciones, foco y teclado cuando haya una superficie nativa disponible.
- La muestra de egreso y la revisión de logs/código no detectaron destinos externos ni secretos o payloads; falta inspeccionar el estado visible cuando CUA exponga la ventana.
- STORY-075 conserva revisión visual y QA de su composición.
- Banco de Verificación y Canvas generales siguen después de este Dashboard, según ROADMAP.
- El propietario detuvo esta iteración antes de Banco y Canvas. No presentarlos como implementados ni iniciar sus trabajos ahora; mantener el criterio de superficies y TTR-001 parciales.
- [ADR-0117](../adr/ADR-0117.md) afirma que SPIKE-006 ya dejó un Panel Operativo Fundacional con vistas reales de `clock`, `async-job-executor` y `audit-log`; el código y el propio [SPIKE-006](./SPIKE-006-integracion-grpc-local.md) solo muestran una prueba de transporte. Corregir esa afirmación de estado antes de usarla como evidencia de superficie entregada.

## 9. Cierre ejecutivo

```text
ESTADO: 🟡 PARCIAL

PROGRESO MACRO:
- Tauri inicia el Core autenticado con una base `drasus.db` estable en `app_data_dir`, valida readiness y alimenta el estado localizado del Dashboard; los cinco widgets siguen vacíos.
- La adaptación IPC Rust/TypeScript tiene prueba de deriva; Tauri reutiliza `drasus.db` y conserva el cursor al relanzar hijos. Formatos, compilaciones, pruebas Core/Tauri y build/typecheck React pasan. La reauditoría independiente confirma APTO para la corrección del cursor; una captura muestra la ventana conectada y el primer widget vacío. El Dashboard sigue NO APTO por el recorrido visual/teclado pendiente.

FRICCIONES Y DEUDA:
- El ledger y la readiness viva tras reinicio pasan en Core; el supervisor reanuda desde cursor y consume eventos históricos no listos. QA confirmó `OutOfRange`→`FreshSnapshot`. La X de la barra propia cerró ventana, Tauri y Core; faltan la pantalla completa, idioma, foco y reflujo.
- Faltan la Ventana de Verificación, Banco, Canvas/inspector, widget de plomería y revisión visual/teclado.
- La muestra de sockets y revisión de logs/código no detectaron egreso ni secretos; falta revisar visualmente el estado por el límite de CUA.

INPUT REQUERIDO DEL CEO:
- Confirmar el recorrido visual restante en `personal-env` de §6: cinco widgets vacíos, idioma es/en, foco con Tab y reflujo a 320 px; indicar qué punto falla o enviar captura.
```
