# Drasus Engine Desktop

Base de desarrollo Tauri 2 + React + TypeScript + Vite.

## Requisitos

- Node.js 22.12 o posterior
- pnpm 12.16.1
- Rust estable
- Dependencias nativas de Tauri para el sistema operativo

## Desarrollo

```bash
corepack pnpm install --frozen-lockfile
corepack pnpm typecheck
corepack pnpm build
```

La ventana monta React vacía. Las fuentes Geist se sirven desde archivos locales.

La app compila el binario Core antes de iniciar Tauri. El host entrega una sesión corta por un pipe local, valida readiness gRPC y cierra el proceso al salir. Los widgets siguen vacíos hasta que sus Features publiquen datos reales.

Ejecuta Tauri dentro de `personal-env` para usar Rust y las bibliotecas nativas:

```bash
distrobox enter --name personal-env -- bash -ic 'cd "$PWD/apps/desktop" && pnpm tauri dev'
```

`pnpm tauri dev` ejecuta `cargo build --manifest-path ../../Cargo.toml --bin drasus` desde `beforeDevCommand`. `pnpm tauri build` compila la variante `--release` antes de construir la interfaz.
