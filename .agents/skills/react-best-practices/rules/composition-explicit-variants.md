---
title: Use Explicit Variants for Interacting Component Modes
impact: MEDIUM
impactDescription: prevents invalid combinations in complex component APIs
tags: composition, props, variants, maintainability
---

## Use Explicit Variants for Interacting Component Modes

Avoid a growing set of boolean props when they select different structures or actions. Their combinations create hidden modes and can allow invalid states. Use named variants or composed parts when they make real behavior differences clearer, and share only the internals that have real consumers.

A single boolean for independent state remains appropriate. Do not replace a simple prop with a compound API or extract a reusable component before a second real consumer exists. Drasus follows the extraction criterion in [ADR-0139](../../../../docs/adr/ADR-0139.md).
