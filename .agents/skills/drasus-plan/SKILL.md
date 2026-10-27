---
name: drasus-plan
description: Create a bounded, source-backed plan for a Drasus product or technical change before implementation. Use when the request needs architecture, scope, dependencies, or cross-repository decisions.
---

# Drasus planning

1. Identify the target repository and call `drasus_context.start_workflow` with `workflow=plan`.
2. Read every returned `required_sources` item and inspect the target code or docs named by the request.
3. Separate confirmed facts, open decisions, constraints, affected files, validation, and deferred work.
4. Resolve product truth from the target repository. Do not promote a memory, historical session, or ranked context suggestion into a requirement.
5. Produce the smallest executable plan and list the next workflow (`implement`, `integrate`, `verify`, or `review`).

This skill plans; it does not edit product code unless the user explicitly asks for implementation in the same turn.
