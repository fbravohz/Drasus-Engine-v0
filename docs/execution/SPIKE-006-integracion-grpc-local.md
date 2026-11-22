# SPIKE-006 · Integración gRPC local para escritorio

| Campo | Valor |
|---|---|
| **ID** | SPIKE-006 |
| **Tipo** | Spike |
| **Épica (Fase)** | EPIC-0 — bootstrap de escritorio |
| **Estado** | Completado: viabilidad de transporte demostrada |
| **Responsable** | Architect · Tech Lead |
| **Creada** | 2026-11-08 |
| **Completada** | 2026-11-08 |

## 0. Resumen de la Orden

Resolver las decisiones de integración que bloquean la conexión segura entre Tauri y el Core. Entregar una prueba mínima reproducible y una especificación ejecutable para el slice vertical.

## 1. Especificación de origen

- [ADR-0142](../adr/ADR-0142.md): exige Spike previo al servidor.
- [ADR-0165](../adr/ADR-0165.md): Protobuf, gRPC autenticado en loopback efímero y host Tauri.
- [ADR-0166](../adr/ADR-0166.md): vista inicial y recuperación del stream.
- [desktop-packaging-manager](../features/desktop-packaging-manager.md): TTR-001.
- [STORY-075](./STORY-075-dashboard-oficial-tauri-react.md): Dashboard visual parcial.

## 2. Objetivo

Dejar cerradas y probadas las decisiones que permiten implementar TTR-001 sin improvisar el contrato ni la seguridad local.

## 3. Agentes y Modo de Acompañamiento

| Agente | Etapa | Depende de | Modo |
|---|---|---|---|
| Architect | Investigación y decisión técnica | Canon y código actual | Autónomo |
| Tech Lead | Auditoría y cierre | Evidencia del Architect | Autónomo |

## 4. Instrucciones de despacho por agente

### Architect

```text
Carga tu skill architect. Resuelve SPIKE-006 con una prueba mínima reproducible. Define ubicación de esquemas y bindings Protobuf, generación compatible con el workspace Cargo raíz y el host Tauri aislado, mapeo de errores, autenticación de sesión en loopback efímero y pruebas de unary y stream recuperable. Respeta ADR-0142/0165/0166. Registra decisiones, alternativas descartadas, evidencia y comandos en esta Orden. No implementes el producto; entrega el contrato para la Story siguiente.
```

## 5. Criterios de aceptación

| # | Criterio | Verificación |
|---|---|---|
| 1 | El generador compila un esquema en un workspace Cargo aislado; la Orden siguiente fija la integración en Core y Tauri. | `spike_codegen` |
| 2 | Los errores probados conservan códigos canónicos y no filtran la credencial. | `spike_errors` |
| 3 | Una prueba de loopback efímero rechaza credenciales ausentes o inválidas y admite la vigente. | `spike_auth` |
| 4 | Una operación puntual y un stream prueban vista inicial y recuperación conforme ADR-0166. | `spike_transport` |
| 5 | TTR-001 recibe decisiones técnicas y una lista explícita de garantías aún sin probar. | `spike_handoff_review` |

## 6. Comandos de validación

```bash
distrobox enter --name personal-env -- bash -ic 'cargo test --locked --manifest-path "$PWD/docs/execution/spikes/SPIKE-006/Cargo.toml"'
git diff --check
```

El primer comando compiló el esquema con `tonic-prost-build` y `protoc-bin-vendored` y pasó `local_auth_unary_and_recoverable_stream` (1/1). El manifiesto del experimento tiene `[workspace]`: Cargo lo compila aislado del workspace raíz, igual que el host Tauri. No se compiló aún el transporte en `crates/app` ni en `apps/desktop/src-tauri`; eso pertenece a TTR-001.

### Decisiones de integración para TTR-001

1. **Contrato:** ubicar los `.proto` versionados en `proto/drasus/<dominio>/v1/` en la raíz. Generar Rust en `OUT_DIR` desde los `build.rs` de ambos consumidores Rust, con una única lista de esquemas compartida; no versionar bindings generados. `tonic-prost-build` 0.14 y `protoc-bin-vendored` 3 compilaron en el experimento. Cada `build.rs` resuelve la ruta desde `CARGO_MANIFEST_DIR`. TTR-001 comprobará que ambos generan el mismo descriptor y reconstruyen ante un cambio del esquema; el experimento valida el generador y el aislamiento, no esa comparación final.
2. **Frontera:** el Core implementa servidor y puertos de dominio; Tauri consume el cliente gRPC y ofrece a React una fachada estrecha de estado y eventos. React no recibe la credencial ni define DTOs de dominio a mano. Generar TypeScript cuando una Feature necesite sus mensajes.
3. **Autenticación:** Tauri crea una credencial aleatoria de sesión de vida corta, arranca el Core por un canal de bootstrap protegido y recibe el puerto efímero de `127.0.0.1`. El Core valida cada llamada y suscripción. La prueba rechaza credenciales ausentes e incorrectas y admite la correcta. El literal fijo solo pertenece a la muestra; TTR-001 probará generación, entrega y rotación sin filtrar secretos en argumentos, logs, errores o UI.
4. **Errores:** devolver `tonic::Status` con código canónico y mensaje estable de máquina, sin datos sensibles. Autenticación → `Unauthenticated`; cursor fuera de retención → `OutOfRange`; entrada inválida → `InvalidArgument`; conflicto → `Aborted`; error interno → `Internal` con identificador diagnóstico separado. El host localiza el mensaje para React. La prueba comprueba código y `AUTH_REQUIRED` sin credencial en el rechazo.
5. **Stream:** cada suscripción pide alcance y cursor; el Core envía vista inicial con posición y después eventos comprometidos. Cursor válido reanuda sin repetir efectos; uno fuera de retención responde `OutOfRange` y exige nueva vista. La prueba comprueba vista inicial, reconexión desde 1 y cursor 3 fuera de rango. TTR-001 implementará registro durable, alcance, duplicados, retención y reinicio según ADR-0166; la muestra usa una secuencia fija.

**Alternativas descartadas:** credencial por URL o argumento de proceso, DTOs TypeScript manuales, WebSocket paralelo y HTTP de Vite como señal de Core listo. El mecanismo exacto del canal protegido de bootstrap se elige y prueba en TTR-001; no asumir que argumentos o variables de entorno son secretos.

## 7. Registro de ejecución

| Fecha | Etapa y responsable | Encargo | Resultado | Evidencia y siguiente paso |
|---|---|---|---|---|
| 2026-11-08 | Tech Lead | Abrir el Spike requerido por ADR-0142 | Orden abierta; criterios pendientes | El código actual no contiene servidor gRPC ni supervisor Tauri. Despachar investigación. |
| 2026-11-08 | Architect | Probar generación, autenticación y transporte local; definir entrega a TTR-001 | Prueba mínima apta: 1/1; decisiones registradas | `docs/execution/spikes/SPIKE-006/` contiene manifiesto bloqueado, esquema, generador y prueba. `cargo test --locked` terminó con código 0. No equivale a Core/Tauri conectado ni a stream durable. |
| 2026-11-08 | Tech Lead | Reproducir y auditar el experimento | APTO para viabilidad del Spike | Repetí `cargo test --locked`: 1/1. Inspeccioné `src/lib.rs`: prueba loopback efímero, rechazo sin token y con token incorrecto, aceptación, unary, cursor inicial/reanudado y fuera de rango. El secreto fijo y la secuencia en memoria son solo del experimento. |

## 8. Pendientes derivados / decisiones

- TTR-001 implementará y verificará generación en ambos proyectos, canal seguro de bootstrap, secreto aleatorio y vida corta, supervisor, readiness y stream durable. La muestra valida viabilidad de transporte, no las garantías de producción.

## 9. Cierre ejecutivo

```text
ESTADO: 🟢 COMPLETADO

PROGRESO MACRO:
- SPIKE-006 demostró la viabilidad del transporte: la prueba independiente pasó 1/1.
- El bootstrap de producto queda para TTR-001; el Dashboard aún no está conectado.

FRICCIONES Y DEUDA:
- El experimento usa secreto fijo y secuencia en memoria; TTR-001 debe probar las garantías de producción.

INPUT REQUERIDO DEL CEO:
- Ninguna.
```
