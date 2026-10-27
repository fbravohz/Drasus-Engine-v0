# Context contract

## Goal

Deliver the smallest useful context bundle for a selected workflow in Drasus Engine. The provider must make source paths and validation commands explicit; it must not pretend that a ranked suggestion is a normative rule.

## Workflow request

A context request contains:

- `workflow`: stable workflow identifier from `context-runtime/workflows.json`;
- `repository`: `engine` for product work, or `workspace` for the context layer;
- `paths`: relevant files or directories when known;
- `risk`: `normal`, `high`, or `critical`;
- `task_summary`: short human request.

## Bundle response

The provider returns:

- `required_sources`: documents that must be read before editing or deciding;
- `recommended_sources`: references that may resolve the current question;
- `validation`: commands and expected evidence owned by the target repository;
- `constraints`: applicable safety, permission, ownership, and scope rules;
- `next_workflows`: valid follow-on workflow identifiers;
- `source_of_truth`: why each source was selected.

The response may include short excerpts for orientation, but the agent must read the cited source before treating it as authoritative.

## Selection rules

1. Product questions resolve first to this repository's canonical docs.
2. Workflow rules resolve to the selected skill and its references.
3. Cross-cutting safety and permissions resolve to this repository's `AGENTS.md`.
4. Historical notes, session reports, and memories are optional evidence, never automatic authority.
5. If sources conflict, return the conflict instead of merging it silently.

## Non-goals

- No full-corpus prompt injection.
- No semantic claim that is not backed by a cited source.
- No mutation, commit, or external action from the context provider.
- No compatibility logic for other agent platforms.
