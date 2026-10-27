---
name: instruction-file-refiner
description: Refine AGENTS.md and other instruction or context files by removing redundancy and stale detail, preserving operational invariants, and auditing the standalone file plus any available diff. Use when a user asks to compact, reorganize, audit, or clean up an instruction document.
metadata:
  short-description: Compact and audit instruction files
---

# Instruction File Refiner

Refine an instruction or context file without weakening its behavior. The goal is a smaller, clearer, more navigable document whose critical rules remain executable by both humans and agents.

## Workflow

1. **Scope the target.** Resolve the exact file, its repository, and any canonical documents it references. A diff is optional context, not the audit target. Do not edit unrelated files or create commits unless explicitly requested.
2. **Read before editing.** Read and audit the complete target file even when it has no diff. Use `rg` to locate related rules, duplicated sections, dates, counts, model names, paths, and source-of-truth references. Inspect the current `git diff` only when the file is version-controlled and already modified, as a change-review supplement.
3. **Build a preservation checklist.** Extract operational invariants, safety constraints, authority/order rules, paths, workflows, escalation rules, permissions, and domain-specific semantics. Mark each as preserve, merge, or intentionally remove. Treat dates, counts, examples, and prose as removable only when they do not change behavior.
4. **Delimit the context boundary.** For every passage, choose one: **keep locally** because it is an always-active guardrail; **replace with a pointer** because the canonical document owns the detail; **remove** because it is duplicated, stale, or non-operational. For `AGENTS.md`, consult `docs/README.md` as the complete routing index when it exists, then route architecture to ADR/SAD, product to PRODUCT, execution to modules/features, and operating policy to `.agents/knowledge/` or the relevant skill. Keep a one-line local invariant when omitting its detail would allow a serious mistake.
5. **Choose a functional structure.** Prefer this order when applicable: purpose; pre-work decisions; invariants; document/knowledge map; recovery workflow; governance, memory, and communication. Use short headings, compact bullets, and a table only when it improves lookup.
6. **Compact semantically.** Remove repetition, stale history, ornamental prose, implementation detail duplicated in canonical documents, and metadata that can drift. Replace repeated rules with one authoritative statement plus a precise link/path. Preserve exact conditions, exceptions, thresholds, ownership, security boundaries, and failure behavior.
7. **Resolve ambiguity.** Do not silently merge contradictory rules. Prefer the explicitly authoritative source, the user's current instruction, or the newest confirmed decision. If the conflict affects behavior or safety, stop and report it before editing.
8. **Edit surgically.** Use `apply_patch`. Keep unrelated user changes intact. Do not rewrite a larger scope than the request requires; a full reorganization is allowed only when the user explicitly asks for it.
9. **Audit the result twice.** First audit the final file itself for structure, contradictions, missing routes, unclear precedence, orphan references, and readability. Then compare it against the preservation checklist and, when available, the prior version/diff to detect lost invariants. Restore any omission in the smallest clear wording and repeat the audit.
10. **Validate.** Run syntax/markdown or repository validation when available and appropriate. If the file is version-controlled, run `git diff --check` and inspect the final diff/status as an additional change review. Do not require a diff to declare the standalone file coherent and complete.

## Preservation rules

- Keep source-of-truth paths, ADR/policy references, security and privacy rules, data ownership, permission boundaries, migration constraints, lifecycle states, escalation criteria, and required reporting/checkpoint behavior.
- Never compact by deletion alone: classify each passage as local guardrail, canonical detail, routing pointer, or non-operational prose. If the canonical source is not verified, preserve the passage until its owner is found.
- For `AGENTS.md`, preserve architecture invariants, repository layout, context-recovery rules, agent/model-selection rules, subagent boundaries, memory policy, governance precedence, language, and commit authorization.
- Replace volatile details such as dates, file counts, line counts, historical wording, and duplicated summaries with the canonical path or decision reference when the detail is not behaviorally required.
- Do not invent names, paths, examples, requirements, or model capabilities. If a referenced path or source is unclear, verify it before changing the instruction. If a reference is missing or orphaned, report it or repair it only when the canonical source is clear.
- Keep the final document direct and operational. Avoid marketing language, repeated warnings, decorative explanations, and rules that merely restate the surrounding heading.

## Final report

Report:

- target file and scope changed;
- what was removed, consolidated, and preserved;
- any information deliberately omitted as non-operational;
- validation performed and remaining risks or ambiguities.

If the audit finds a meaningful omission, say so explicitly and fix it before declaring completion when the user requested the edit itself.
