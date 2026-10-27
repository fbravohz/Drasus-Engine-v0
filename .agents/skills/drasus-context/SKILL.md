---
name: drasus-context
description: Design or maintain the Codex-native context system for Drasus Engine, including AGENTS.md, workflow skills, the context MCP, and mechanical hooks. Use when changing agent guidance or context delivery.
---

# Drasus context engineering

Use this skill only for the Engine Codex layer: `AGENTS.md`, `.agents/skills/`, `.agents/knowledge/`, `.agents/context-runtime/`, `.codex/config.toml`, and `.codex/hooks.json`.

## Operating rules

- Keep `AGENTS.md` small and durable; move workflow detail into skills or focused references.
- Make each skill about one repeatable workflow, with valid YAML front matter and a discriminating description.
- Use MCP for live context selection and external capabilities. The skill defines the order and interpretation; MCP does not define policy by itself.
- Use hooks only for deterministic lifecycle checks, validation, or context-size control. Do not encode semantic role selection in a hook.
- Remove non-Codex compatibility language unless a current Codex workflow explicitly needs the same concept.
- Preserve ownership: product truth and the reusable workflow layer stay in Drasus Engine.

## Context delivery

For every new or changed workflow, update the workflow catalog and define its required sources, optional references, validation commands, and valid follow-on workflows. Test the provider with a real task summary and verify that it returns paths that exist.

## Completion

Validate front matter, catalog schema, provider responses, hook registration, and a dry-run of the selected workflow. Report any legacy source that remains intentionally unmigrated.
