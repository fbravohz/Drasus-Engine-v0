from __future__ import annotations

import json
import subprocess
import sys
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SERVER = ROOT / ".agents" / "context-runtime" / "server.py"


class ContextServerTests(unittest.TestCase):
    def test_catalog_and_workflow_bundle(self) -> None:
        sys.path.insert(0, str(SERVER.parent))
        import server

        catalog = server.load_catalog()
        self.assertIn("context-maintenance", catalog["workflows"])
        payload = server.bundle(
            "context-maintenance",
            "engine",
            "test the Codex context layer",
            "normal",
        )
        self.assertTrue(payload["required_sources"])
        self.assertIn(".agents/knowledge/context-contract.md", {item["path"] for item in payload["required_sources"]})
        self.assertIn("docs/README.md", {item["path"] for item in payload["required_sources"]})

    def test_stdio_initialize_and_tool_list(self) -> None:
        messages = "\n".join(
            [
                json.dumps(
                    {
                        "jsonrpc": "2.0",
                        "id": 1,
                        "method": "initialize",
                        "params": {"protocolVersion": "2025-03-26", "capabilities": {}, "clientInfo": {"name": "test", "version": "0"}},
                    }
                ),
                json.dumps({"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}),
            ]
        )
        result = subprocess.run(
            [sys.executable, str(SERVER)],
            input=messages,
            text=True,
            capture_output=True,
            check=True,
        )
        output = [json.loads(line) for line in result.stdout.splitlines() if line]
        self.assertEqual(output[0]["result"]["serverInfo"]["name"], "drasus_context")
        self.assertIn("start_workflow", {item["name"] for item in output[1]["result"]["tools"]})


if __name__ == "__main__":
    unittest.main()
