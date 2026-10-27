# Desktop Packaging Manager

**Carpeta:** `./features/desktop-packaging-manager.md`
**Estado:** En Diseño
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

### TTR-002: Bundling y actualización

Producir instaladores firmados por plataforma y cablearlos al actualizador seguro. Está terminado cuando el paquete verifica firma, instala, actualiza y conserva la integridad de la instancia local.

## Persistencia

Registrar únicamente identidad de instancia, marca temporal, resultado de salud, versión del paquete, sesión y hash de auditoría. Secretos, credenciales y contenido de operaciones quedan excluidos.

## Gobernanza y Estándares

- Local-First y Zero-Docker permanecen fijos.
- La interfaz se verifica contra el Core real mediante gRPC; no sustituye operaciones con mocks para cerrar una SVF.
- La distribución por plataforma no cambia el contrato de dominio ni la soberanía de los datos.
