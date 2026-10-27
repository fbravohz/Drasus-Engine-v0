---
name: drasus-integrate
description: Coordinate a Drasus change that crosses repositories, generated interfaces, Tauri, gRPC, MCP, or shared contracts. Use when one change has multiple owners or a boundary must remain coherent.
---

# Drasus integration

1. Identify every affected repository and call `drasus_context.start_workflow` with `workflow=integrate` for the primary target.
2. Read the required source-of-truth documents in each affected repository before editing any boundary.
3. Define the contract, owner, direction of dependency, compatibility impact, and validation evidence before implementation.
4. Keep generated bindings generated from the canonical producer; do not hand-maintain duplicate DTOs or schemas.
5. Verify both sides of the boundary and record any staged rollout or deferred consumer work.
6. Escalate security, permission, live-operation, financial, or irreversible changes instead of treating them as ordinary integration work.
