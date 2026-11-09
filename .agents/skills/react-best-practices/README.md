# React Best Practices

Performance rules for React and Next.js plus conditional component API guidance.

## Contents

- `SKILL.md`: entrypoint and conditional rule routing.
- `rules/`: detailed rules. `_sections.md` maps prefixes; `_template.md` is the authoring template.
- `AGENTS.md`: full guide and table of contents.
- `metadata.json`: skill metadata and references.

The eight performance categories remain impact-ranked. The `composition-` rules cover maintainability and apply only when component modes, slots, or shared state justify them. Follow [ADR-0139](../../../docs/adr/ADR-0139.md) before extracting a shared UI component. Apply the React 19 API rule only to React 19 or later.

## Rule Maintenance

1. Copy `rules/_template.md` to a filename using a prefix in `rules/_sections.md`.
2. Keep composition guidance conditional; do not ban simple boolean props or require providers by default.
3. Include examples only when they materially clarify the decision.
4. Keep the relevant `AGENTS.md` section and table of contents synchronized with the rule files. Add links from `SKILL.md` when a rule needs conditional discovery.
5. This directory contains no build or validation scripts; update the rule and compiled guide together.

## Impact

Impact levels rank performance rules. `MEDIUM` on a `composition-` rule describes maintainability impact, not performance.

## Acknowledgments

The performance guidance originated with [Vercel Engineering](https://vercel.com/).
