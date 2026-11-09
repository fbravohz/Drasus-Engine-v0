---
title: Use Compound Components for Shared UI State
impact: MEDIUM
impactDescription: supports flexible subparts when consumers need to compose one widget
tags: composition, compound-components, context, state
---

## Use Compound Components for Shared UI State

Use compound components when consumers need to arrange related subparts and those parts share state. Put state-source and synchronization details behind a provider; expose only the stable interface the UI needs. Lift state to the nearest common owner when sibling UI outside the visual frame must read or update it.

If multiple real providers must drive the same UI, define their shared context contract; `state`, `actions`, and `meta` can help organize it. Do not impose that shape, add a provider, or move local state when a simple component and props are enough.
