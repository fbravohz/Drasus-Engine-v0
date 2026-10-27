# Drasus Engine Codex configuration

This directory contains Drasus Engine's native Codex configuration.

- `config.toml` contains trusted project settings and Codex-native feature flags.
- `hooks.json` will contain only Codex-native mechanical hooks.
- Hook commands must resolve paths from this repository's Git root.

Keep this layer Codex-only. Test each hook against Codex event payloads.

The product's `agentic-mcp-gateway` is a separate runtime surface and remains
governed by its ADRs.
