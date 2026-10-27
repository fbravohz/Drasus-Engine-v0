# Codex layer for Drasus Engine

This is the active Codex-native layer for Drasus Engine.

## Responsibilities

- `skills/`: workflow skills, not permanent personas.
- `knowledge/`: small contracts and routing references.
- `context-runtime/`: workflow catalog and the `drasus_context` provider.
- `hooks/`: only Codex-native mechanical checks once registered in `.codex/hooks.json`.

The former `Drasus-Context` repository has been imported under `legacy/` as a
reference-only snapshot. It is not loaded as a single context bundle and is
not the runtime authority for this workspace.

## Design rule

The agent receives context by workflow. A skill selects a workflow; the context provider returns the minimum required documents and validation surface; the agent reads those sources and works in this repository.
