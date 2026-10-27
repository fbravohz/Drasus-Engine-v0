# Drasus Engine Codex configuration

This directory contains Drasus Engine's native Codex configuration.

- `config.toml` will contain trusted project settings and the `drasus_context` MCP connection.
- `hooks.json` will contain only Codex-native mechanical hooks.
- MCP and hook commands must resolve paths from this repository's Git root.

Keep this layer Codex-only. Test each hook against Codex event payloads.
