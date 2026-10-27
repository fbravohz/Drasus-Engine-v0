---
name: drasus-implement
description: Implement an approved Drasus change in the correct child repository with workflow-scoped context and repository-owned validation. Use when the design or acceptance criteria are clear and code or documentation must change.
---

# Drasus implementation

1. Resolve the target child repository and call `drasus_context.start_workflow` with `workflow=implement`.
2. Read the required bundle sources, then read the target repository's relevant module, feature, contract, and tests.
3. Preserve the target repository's architecture and ownership boundaries. Do not move product truth into `.agents/`.
4. Make a narrow change. Keep unrelated working-tree changes intact and do not commit unless explicitly authorized.
5. Run the validation commands returned by the context provider plus targeted tests for the changed surface.
6. Hand off to `drasus-verify` or `drasus-review` with exact files, commands, and remaining risks.
