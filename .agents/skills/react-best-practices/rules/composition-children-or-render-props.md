---
title: Choose Children or Render Props by Data Flow
impact: MEDIUM
impactDescription: keeps flexible rendering APIs clear
tags: composition, children, render-props
---

## Choose Children or Render Props by Data Flow

Use `children` for static structure and slots that consumers arrange. Use a render prop when the parent must provide per-item data or state, such as an item, index, or selection, to custom rendering. Do not replace a useful data callback with children merely to avoid render props.
