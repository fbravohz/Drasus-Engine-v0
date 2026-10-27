---
name: drasus-review
description: Independently audit a Drasus plan, diff, or deliverable against user intent and canonical repository evidence. Use when the user asks for review, audit, risk analysis, or a second opinion.
---

# Drasus review

1. Call `drasus_context.start_workflow` with `workflow=review` and the owning repository.
2. Read the required sources independently of the author's summary.
3. Compare user intent, stated plan, actual diff, tests, and durable documentation.
4. Lead with concrete findings ordered by severity and cite exact files or commands.
5. Distinguish confirmed defects, unverified assumptions, and acceptable trade-offs. Do not silently fix the reviewed work unless asked.
