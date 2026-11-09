---
title: Apply React 19 Ref and Context APIs Where They Fit
impact: MEDIUM
impactDescription: uses current React APIs without mechanical migrations
tags: composition, react19, refs, context
---

## Apply React 19 Ref and Context APIs Where They Fit

Apply this rule only when the project uses React 19 or later.

- New components that expose a ref can accept `ref` as a regular prop; `forwardRef` is no longer necessary. React's reference marks `forwardRef` deprecated for a future release.
- `useContext` remains suitable for top-level context reads. Use `use(context)` when a context must be read conditionally or in a loop; it is an alternative, not a blanket replacement.
- React 19 allows `<Context value={...}>` as provider syntax. Existing `.Provider` code does not need a mechanical migration.

References: [forwardRef](https://react.dev/reference/react/forwardRef), [use](https://react.dev/reference/react/use), [useContext](https://react.dev/reference/react/useContext), [createContext](https://react.dev/reference/react/createContext).
