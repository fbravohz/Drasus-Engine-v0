---
name: drasus-verify
description: Validate a Drasus change against its source of truth, tests, contracts, and acceptance criteria, then report evidence and gaps. Use after implementation or when the user asks whether work is complete.
---

# Drasus verification

1. Call `drasus_context.start_workflow` with `workflow=verify` and the target repository.
2. Read the required sources and inspect the complete diff in the owning repository.
3. Run the provider's validation surface, then add targeted tests or checks needed to discriminate the changed behavior.
4. Check boundary effects, persistence, permissions, error paths, and documentation when the change touches them.
5. Report passed evidence, failed evidence, untested risk, and an explicit completion verdict. Never claim a test or tool ran if it did not.
