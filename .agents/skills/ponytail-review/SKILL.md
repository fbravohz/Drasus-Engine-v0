---
name: ponytail-review
description: Review code changes for correctness, safety, load, tests, speed and unnecessary complexity. Use for code review, PR review, last-commit review or /ponytail-review. Reports without editing.
---

# Code review

Review the named files, staged/uncommitted change, branch or PR. With no target use uncommitted changes, otherwise the last commit. Prioritize correctness, safety, expected load, tests, performance, then simplicity. Report fully; do not edit code.

## Evidence

Read the diff and connected callers, callees, tests and README. Trace input, storage and output. Search every caller when signatures, return values or behavior change. Establish expected users/processes and load from repository/deployment evidence; state the assumed load.

Check wrong results, crashes and boundaries —empty, zero, last item, rounding, time zones—; security and missing input checks; swallowed errors, transaction/write ordering and loss; races, duplicated per-process work, unbounded memory, per-item queries and quadratic growth. A local fix that leaves a shared function incorrect remains a defect.

Require a discriminating test for risky branches, parsers, money, security, writes and bug fixes. Significant slowdowns are defects; small hot-loop improvements are suggestions, weighted by actual latency needs.

For simplicity, identify unused/speculative code, duplicate helpers —cite the existing path—, unnecessary dependencies where native/standard capabilities suffice, one-implementation abstractions, unused configuration and copies that must evolve together. Split unrelated jobs, not by line count or merely to shorten functions.

Before reporting, confirm the exact lines and a reachable scenario: input/situation and wrong outcome. A `shortcut:`/`ponytail:` with explicit limit is a decision unless expected load exceeds it. Recommend the smallest correct fix, preferably removing code; do not add layers or configuration without need. Exclude taste, vague concerns and unsupported "consider" suggestions.

## Report

Use the user's language and explain technical terms once. Start with what the change does in 2–3 sentences. Group findings by required fixes —bugs/security/data loss/load failures—, recommended fixes —missing risky tests, real slowness, duplication or mixed jobs— and optional improvements. Skip empty groups; number findings across groups.

Each finding cites file/line and has four short parts: what the code does, concrete failure, smallest fix and consequence if deferred. End with verdict —ready or finding numbers to fix first—, possible line reduction when simplification findings exist and one line of material checks not performed.

If clean, state what changed, that it is ready and what you checked; do not invent findings.
