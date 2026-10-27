# Drasus-Context import manifest

- Source repository: `Drasus-Context`
- Source commit: `fb45e782bda2065fc20854470853a0e40081cc33`
- Import destination: `.agents/legacy/drasus-context/`
- Status: reference-only archive; not active runtime context

## Imported

- All versioned source files from `Drasus-Context`, including `documents/` and
  `state/`

## Deliberately excluded

- Old Git metadata, virtual environments, bytecode and runtime caches

## Promotion rule

Nothing in this snapshot is injected automatically. Promote only a useful,
source-backed contract or workflow into the active `.agents/` layer, convert
it to Codex-native front matter and interfaces, and validate it before use.
