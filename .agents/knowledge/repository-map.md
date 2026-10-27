# Repository map

## Repository

Drasus Engine is the repository root. It contains the product and its active Codex context layer.

## Product source

| Repository | Canonical content | Typical workflows |
|---|---|---|
| Drasus Engine | Rust core, Tauri/React shell, migrations, product docs, ADRs, SADs, features and execution orders | plan, implementation, integration, verification and review |

## Context sources

The active Codex layer is `.agents/`. The former `Drasus-Context` repository
is archived at `.agents/legacy/drasus-context/`; its local working copy is
preserved alongside it and neither is used by the provider.

## Git boundary

Use this repository's Git root for product and context changes. The archived
context stays versioned here but is not an active instruction source.
