---
name: drasus-dispatch
description: Route a non-trivial Drasus Engine request to the correct workflow, context bundle, and validation surface before editing. Use when the risk or next step is unclear.
---

# Drasus dispatcher

Use this skill as the Engine entry point when a request does not clearly identify its workflow.

1. Confirm that the target is Drasus Engine from `.agents/knowledge/repository-map.md`.
2. Classify the work as `plan`, `implement`, `integrate`, `verify`, `review`, or `context-maintenance`.
3. Classify risk as `normal`, `high`, or `critical`; raise the level for security, money, live operations, legal decisions, data contracts, or cross-repository changes.
4. Call the `drasus_context` provider's workflow-context operation when it is connected. Request only the selected workflow and `engine` as the repository.
5. Read every `required_sources` item before editing or sealing a decision. Treat `recommended_sources` as targeted lookup, not mandatory corpus loading.
6. Continue with the selected workflow skill or state the smallest missing decision. Do not modify files while routing is unresolved.

The dispatcher routes work; it does not implement product code, grant permissions, or replace the target repository's canonical documentation.
