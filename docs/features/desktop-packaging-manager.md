# Desktop Packaging Manager

**Carpeta:** `./features/desktop-packaging-manager.md`
**Estado:** 🟡 En curso (2026-11-09). TTR-001 parcial; TTR-002 pendiente; TTR-003 parcial. Evidencia de TTR-001: [STORY-076](../execution/STORY-076-bootstrap-escritorio-conectado.md). Evidencia de TTR-003: [STORY-074](../execution/STORY-074-cascaron-tauri-react.md).
**Decisión Arquitectónica Asociada:** ADR-0165

## ¿Qué es esta feature?

Empaqueta Drasus Engine como una aplicación de escritorio Tauri 2 + React/TypeScript y supervisa el Core Rust mediante gRPC autenticado local. El usuario instala una aplicación por sistema operativo; no instala Rust, compiladores ni dependencias de desarrollo.

## Narrativa de Producto

**Capacidad:** `CAP-09` — Una interfaz donde se ve el razonamiento.

La feature entrega el contenedor de escritorio, el ciclo de vida del Core y los instaladores. No modifica reglas de trading ni expone servicios fuera de la máquina del usuario.

## Comportamientos Observables

- Al abrir la aplicación, Tauri inicia o conecta al Core Rust local y muestra la UI React solo cuando el health check gRPC es correcto.
- El Core escucha únicamente en loopback, con puerto efímero y credencial de sesión de vida corta; no acepta tráfico de red externa.
- Al cerrar la ventana, el supervisor solicita cierre ordenado, espera el límite definido y registra cualquier terminación anómala para recuperación.
- El paquete se instala y abre en una máquina limpia sin compilador ni runtime de desarrollo.
- Los logs de arranque y salud no incluyen secretos, credenciales de bróker ni payloads de trading.

## Restricciones

- El Core mantiene la autoridad de dominio, persistencia y ejecución; React nunca contiene lógica de trading.
- Los contratos Protobuf son la fuente tipada compartida; no se duplican DTOs manualmente en TypeScript.
- Tauri administra ciclo de vida y permisos del escritorio. La interfaz no abre listeners ni recibe conexiones remotas directamente.
- Cada plataforma se empaqueta y firma según su mecanismo nativo antes de distribuirse.

## Estructura Interna (FCIS)

- **Core:** proceso Rust con puertos públicos gRPC, autenticación de sesión local y checkpoints recuperables.
- **Host:** Tauri crea, supervisa y detiene el Core; aplica permisos de sistema y entrega eventos seguros a la UI.
- **Shell:** React/TypeScript consume contratos generados o la fachada estrecha del host; renderiza estados y solicita operaciones autorizadas.

## Ciclo de Vida de la Feature — Desktop Packaging Manager

1. El host crea una sesión local y arranca el Core.
2. El Core publica readiness en loopback y valida la credencial de sesión.
3. El host habilita la UI y transmite solo los eventos y comandos permitidos por contrato.
4. Ante cierre o fallo, el host registra el resultado, intenta cierre ordenado y deja al Core aplicar su recuperación normal al siguiente arranque.

## Tareas (TTRs)

### TTR-001: Bootstrap Tauri + Core

Construir el supervisor de proceso, el health check y la credencial de sesión local. Está terminado cuando una máquina limpia abre y cierra la aplicación sin procesos huérfanos ni listener externo.

- **Estado:** 🟡 Parcial; Core/gRPC, sesión local y Dashboard conectado implementados. QA integrado mantiene NO APTO; faltan reauditoría del cursor durable, Ventana de Verificación, Banco, Canvas/inspector y revisión visual. Evidencia: [STORY-076](../execution/STORY-076-bootstrap-escritorio-conectado.md).
- **Precondición:** SPIKE-006 resolvió el transporte gRPC; el cascarón ejecutable y el Dashboard están en STORY-076. La revisión visual y QA de distribución siguen pendientes.

### TTR-002: Bundling y actualización

Producir instaladores firmados por plataforma y cablearlos al actualizador seguro. La primera instalación debe abrir en una máquina limpia sin que el usuario instale Rust, Node, compiladores ni bibliotecas de desarrollo. Está terminado cuando cada paquete verifica firma, instala, abre, actualiza y conserva la integridad de la instancia local.

- **Estado:** Pendiente; la matriz de soporte y la elección de formatos aún no están definidas.
- **Matriz de soporte:** fijar versiones mínimas de Windows/Linux/macOS y arquitecturas. Validar cada piso contra el WebView del sistema: WebView2 en Windows, WKWebView ligado a la versión de macOS y WebKitGTK 4.1 en Linux.
- **Dependencias del instalador:** asegurar WebView2 en Windows. En Linux, comparar paquetes `.deb`/`.rpm`, AppImage y Flatpak según las distribuciones objetivo; AppImage exige construir desde una base antigua compatible con WebKitGTK 4.1 y comprobar las dependencias de runtime porque Tauri no incluye el WebView. Flatpak aporta un runtime controlado, pero requiere declarar permisos de acceso a datos locales. No pedir al usuario paquetes `-dev` ni toolchains.
- **Confianza de instalación:** conservar un identificador de aplicación estable y firmar los artefactos. La distribución directa de macOS requiere Developer ID y notarización; Windows debe evitar binarios sin firma que disparen bloqueos o avisos de SmartScreen.
- **Verificación de primera ejecución:** instalar y arrancar en imágenes limpias de cada SO soportado; comprobar WebView, permisos, readiness del Core, mensaje accionable ante fallo, actualización firmada y conservación de datos al actualizar o desinstalar.
- **Referencias oficiales:** [WebViews de Tauri](https://v2.tauri.app/reference/webview-versions/), [instalación de Windows](https://v2.tauri.app/distribute/windows-installer/), [AppImage](https://v2.tauri.app/distribute/appimage/), [Flatpak](https://v2.tauri.app/distribute/flatpak/) y [firma de macOS](https://v2.tauri.app/distribute/sign/macos/).

### TTR-003: Cascarón de desarrollo Tauri/React

Preparar la aplicación React/TypeScript con Tauri 2 y Vite en `apps/desktop`, con TypeScript estricto, la compilación de Tailwind, Node.js 22.12 o posterior y las familias tipográficas Geist Sans/Geist Mono aprobadas. El paquete usa pnpm, es independiente del binario CLI `crates/app` y del workspace Rust raíz. Solo entrega una ventana de desarrollo vacía; no agrega una superficie de producto ni tokens visuales pendientes de `docs/DESIGN.yaml`.

- **Estado:** 🟡 Parcial (2026-11-02); typecheck, build y arranque nativo verificados, revisión visual/QA pendiente. Evidencia: [STORY-074](../execution/STORY-074-cascaron-tauri-react.md).
- **Consumidor:** TTR-001 y las futuras superficies React/TypeScript.
- **Observable de verificación:** la aplicación de desarrollo abre una ventana Tauri con el punto de montaje React vacío y compila la hoja Tailwind.
- **Precondición:** ADR-0165 y las decisiones vigentes de `docs/DESIGN.yaml`.
- **Postcondición:** el cascarón puede iniciarse localmente sin iniciar el Core; el workspace Cargo raíz conserva solo sus miembros Rust actuales.
- **Límite:** no implementa el servidor ni el cliente gRPC, generación Protobuf, supervisor del Core, sesión local, vistas de producto, biblioteca de componentes ni caché. La instalación del Core y el transporte siguen en TTR-001 y requieren el Spike previo de ADR-0142.

## Persistencia

Registrar únicamente identidad de instancia, marca temporal, resultado de salud, versión del paquete, sesión y hash de auditoría. Secretos, credenciales y contenido de operaciones quedan excluidos.

El Core conserva en `desktop_status_events` las transiciones históricas STARTING, READY y STOPPED, con cursor estable por base local. Un READY histórico no acredita presencia viva: CheckReadiness consulta solo el proceso actual. El historial se reproduce desde el cursor dentro de una ventana lógica de 64 eventos; sus filas no se eliminan.

## Gobernanza y Estándares

- Local-First y Zero-Docker permanecen fijos.
- La interfaz se verifica contra el Core real mediante gRPC; no sustituye operaciones con mocks para cerrar una SVF.
- La distribución por plataforma no cambia el contrato de dominio ni la soberanía de los datos.
