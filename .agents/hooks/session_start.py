#!/usr/bin/env python3
"""Small Codex-native SessionStart hook for the workspace."""

from __future__ import annotations

import json
import sys


def main() -> int:
    # Codex reads this JSON from stdout and adds it to the developer context.
    print(
        json.dumps(
            {
                "hookSpecificOutput": {
                    "hookEventName": "SessionStart",
                    "additionalContext": (
                        "Drasus workspace: Codex-only mode. For non-trivial work, "
                        "select a workflow skill, consult AGENTS.md and docs/README.md, "
                        "then read only the relevant canonical sources before editing."
                    ),
                }
            }
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
