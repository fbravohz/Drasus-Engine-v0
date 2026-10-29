---
name: ponytail-debt
description: List deliberate shortcut comments as a debt ledger. Use for "ponytail debt", "list the shortcuts" or /ponytail-debt. Reports without edits.
---

# Shortcut ledger

Scan comment markers `shortcut:` and older `ponytail:`, or the marker the user explicitly supplies. Skip `.git` and everything ignored by `.gitignore`, including dependencies, builds and caches. Use the existing command, adding comment prefixes for the stack when needed:

```bash
git grep --no-index --exclude-standard -nE '(#|//|/[*]) ?(shortcut|ponytail):'
```

Keep real deferrals only; a keyboard-shortcut note or prose mentioning the convention is not debt. Each marker is one row, grouped by file:

`<file>:<line>, <what was simplified>. ceiling: <named limit>. upgrade: <revisit trigger>.`

Extract ceiling and upgrade path from the comment; do not invent missing values. Tag any missing trigger/path `no-trigger`. If the user wants owners, add `git blame -L<line>,<line>`.

End with `<N> markers, <M> with no trigger.` If empty: `No shortcut debt. Clean ledger.` Check counts against actual hits. One-shot, read-only; persist the ledger only if the user authorizes a file.
