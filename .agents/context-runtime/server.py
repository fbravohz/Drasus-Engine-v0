#!/usr/bin/env python3
"""Proveedor MCP de solo lectura y CLI local del contexto Codex de Drasus."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
import uuid
from pathlib import Path
from typing import Any

SERVER_NAME = "drasus_context"
SERVER_VERSION = "0.2.0"
PROTOCOL_VERSION = "2025-03-26"
MAX_PAGE_CHARS = 9000
MAX_RECEIPTS = 64
WORKFLOWS = {"dispatch", "plan", "implement", "integrate", "verify", "review", "context-maintenance"}
ROOT = Path(os.environ.get("DRASUS_CONTEXT_ROOT", Path(__file__).resolve().parents[2])).expanduser().resolve()
CATALOG_PATH = ROOT / ".agents/context-runtime/workflows.json"
RECEIPTS: dict[str, list[str]] = {}


def rel(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def safe_path(value: str, *, source: bool = False) -> Path:
    if not isinstance(value, str) or not value or "\x00" in value:
        raise ValueError("path must be a non-empty repository-relative string")
    path = Path(value)
    if path.is_absolute() or ".." in path.parts:
        raise ValueError(f"path outside repository: {value}")
    resolved = (ROOT / path).resolve()
    if not resolved.is_relative_to(ROOT):
        raise ValueError(f"symlink outside repository: {value}")
    if source and (not resolved.is_file() or rel(resolved).startswith(".agents/legacy/")):
        raise ValueError(f"missing or inactive source: {value}")
    return resolved


def load_catalog() -> dict[str, Any]:
    with CATALOG_PATH.open(encoding="utf-8") as handle:
        value = json.load(handle)
    if not isinstance(value, dict):
        raise ValueError("catalog must be an object")
    return value


def check_skill(path: Path) -> None:
    lines = path.read_text(encoding="utf-8").splitlines()
    if not lines or lines[0] != "---" or "---" not in lines[1:]:
        raise ValueError(f"missing skill front matter: {rel(path)}")
    metadata = {}
    for line in lines[1:lines[1:].index("---") + 1]:
        if ":" in line:
            key, value = line.split(":", 1)
            metadata[key.strip()] = value.strip().strip("\"'")
    if metadata.get("name") != path.parent.name or not metadata.get("description"):
        raise ValueError(f"invalid skill metadata: {rel(path)}")


def catalog_errors(catalog: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if catalog.get("schema_version") != 3 or catalog.get("provider") != SERVER_NAME:
        errors.append("unsupported catalog schema/provider")
    workflows, domains, common = (catalog.get(key, {}) for key in ("workflows", "domains", "common"))
    if not isinstance(workflows, dict) or set(workflows) != WORKFLOWS:
        return errors + ["catalog must declare exactly seven Codex workflows"]
    if not isinstance(domains, dict) or not domains or not isinstance(common, dict):
        return errors + ["common and non-empty domains objects required"]
    entries = {"common": common, **{f"workflow:{k}": v for k, v in workflows.items()},
               **{f"domain:{k}": v for k, v in domains.items()}}
    reachable: set[str] = set()
    for owner, entry in entries.items():
        if not isinstance(entry, dict):
            errors.append(f"invalid entry: {owner}")
            continue
        for field in ("required_sources", "recommended_sources", "validation"):
            values = entry.get(field, [])
            if not isinstance(values, list) or any(not isinstance(v, str) or not v for v in values):
                errors.append(f"invalid {field}: {owner}")
                continue
            if len(values) != len(set(values)):
                errors.append(f"duplicate {field}: {owner}")
            if field.endswith("sources"):
                for value in values:
                    try:
                        safe_path(value, source=True).read_text(encoding="utf-8")
                        reachable.add(value)
                    except (OSError, ValueError, UnicodeError) as exc:
                        errors.append(f"{owner}: {exc}")
        if owner.startswith("workflow:"):
            next_steps = entry.get("next")
            if not isinstance(next_steps, list) or any(not isinstance(v, str) or v not in workflows for v in next_steps):
                errors.append(f"invalid next workflows: {owner}")
            try:
                skill = entry.get("skill")
                check_skill(safe_path(skill, source=True))
                if skill not in entry.get("required_sources", []):
                    errors.append(f"workflow skill not required: {owner}")
            except (OSError, ValueError, UnicodeError, TypeError) as exc:
                errors.append(f"{owner}: {exc}")
        if owner.startswith("domain:"):
            patterns = entry.get("patterns", [])
            if not isinstance(patterns, list) or not patterns or any(not isinstance(v, str) for v in patterns):
                errors.append(f"invalid patterns: {owner}")
            else:
                for pattern in patterns:
                    try:
                        re.compile(pattern)
                    except re.error as exc:
                        errors.append(f"{owner}: {exc}")
    for path in (ROOT / ".agents/knowledge").rglob("*.md"):
        if rel(path) not in reachable:
            errors.append(f"unrouted active knowledge: {rel(path)}")
    obligations = catalog.get("obligations")
    if not isinstance(obligations, list) or not obligations:
        return errors + ["semantic obligations required"]
    ids = set()
    for item in obligations:
        if not isinstance(item, dict) or not all(isinstance(item.get(k), str) and item[k] for k in ("id", "trigger", "owner", "source")):
            errors.append("invalid semantic obligation")
            continue
        if item["id"] in ids:
            errors.append(f"duplicate obligation: {item['id']}")
        ids.add(item["id"])
        if item["source"] not in entries.get(item["owner"], {}).get("required_sources", []):
            errors.append(f"obligation not enforced: {item['id']}")
    return errors


def checked_catalog() -> dict[str, Any]:
    catalog = load_catalog()
    errors = catalog_errors(catalog)
    if errors:
        raise ValueError("Invalid context catalog: " + "; ".join(errors))
    return catalog


def repository_path(repository: str) -> Path:
    if repository not in {"engine", "workspace"}:
        raise ValueError(f"Unknown repository: {repository}")
    return ROOT


def select_domains(catalog: dict[str, Any], summary: str, paths: list[str], domains: list[str]) -> list[str]:
    if not isinstance(paths, list) or not isinstance(domains, list) or any(not isinstance(v, str) for v in paths + domains):
        raise ValueError("paths and domains must be string arrays")
    for value in paths:
        safe_path(value)
    known = catalog["domains"]
    unknown = set(domains) - set(known)
    if unknown:
        raise ValueError(f"Unknown domains: {', '.join(sorted(unknown))}")
    text = "\n".join([summary, *paths]).casefold()
    selected = set(domains)
    selected.update(k for k, v in known.items() if any(re.search(p, text) for p in v["patterns"]))
    return sorted(selected)


def bundle(workflow: str, repository: str, task_summary: str, risk: str,
           paths: list[str] | None = None, domains: list[str] | None = None) -> dict[str, Any]:
    catalog = checked_catalog()
    repository_path(repository)
    if workflow not in catalog["workflows"]:
        raise ValueError(f"Unknown workflow: {workflow}")
    if risk not in {"normal", "high", "critical"}:
        raise ValueError("risk must be normal, high, or critical")
    if not isinstance(task_summary, str) or not task_summary.strip():
        raise ValueError("task_summary is required")
    selected = select_domains(catalog, task_summary, paths if paths is not None else [], domains if domains is not None else [])
    entries = [("common", catalog["common"]), (f"workflow:{workflow}", catalog["workflows"][workflow])]
    entries.extend((f"domain:{d}", catalog["domains"][d]) for d in selected)
    if risk in {"high", "critical"}:
        entries.append(("risk:" + risk, {"required_sources": [".agents/knowledge/critical-domain-reasoning.md"]}))
    required, recommended, commands = {}, {}, []
    for owner, entry in entries:
        for field, destination in (("required_sources", required), ("recommended_sources", recommended)):
            for value in entry.get(field, []):
                if value not in destination:
                    content = safe_path(value, source=True).read_bytes()
                    destination[value] = {"path": value, "reason": owner, "bytes": len(content),
                                          "sha256": hashlib.sha256(content).hexdigest()}
        commands.extend(entry.get("validation", []))
    for value in required:
        recommended.pop(value, None)
    return {
        "workflow": workflow, "repository": repository, "risk": risk,
        "task_summary": task_summary, "paths": paths or [], "domains": selected,
        "required_sources": list(required.values()), "recommended_sources": list(recommended.values()),
        "source_of_truth": {"product": "docs/README.md → canon vigente → código y pruebas",
                            "workflow": catalog["workflows"][workflow]["skill"],
                            "archive": "evidence only; never runtime authority"},
        "validation": list(dict.fromkeys(commands)),
        "constraints": [
            "Read required files completely; source lists and receipts do not prove reading.",
            "Preserve selected domains and paths when handing off to another workflow.",
            "The provider is read-only and grants no action, commit, publication or trading permission.",
            "Historical personas, memories and dispatch queues are not active authorization.",
            "Resolve archived guidance against the current product canon.",
        ],
        "next_workflows": catalog["workflows"][workflow]["next"],
    }


def text_result(value: Any, *, is_error: bool = False) -> dict[str, Any]:
    return {"content": [{"type": "text", "text": json.dumps(value, ensure_ascii=False, indent=2)}], "isError": is_error}


def tool_definitions() -> list[dict[str, Any]]:
    empty = {"type": "object", "properties": {}, "additionalProperties": False}
    return [
        {"name": "list_workflows", "description": "List workflows, domains and routing.", "inputSchema": empty},
        {"name": "start_workflow", "description": "Compose workflow, risk and domain sources; read every source file.", "inputSchema": {
            "type": "object", "properties": {
                "workflow": {"type": "string"}, "repository": {"type": "string", "enum": ["engine", "workspace"]},
                "task_summary": {"type": "string"}, "risk": {"type": "string", "enum": ["normal", "high", "critical"]},
                "paths": {"type": "array", "items": {"type": "string"}},
                "domains": {"type": "array", "items": {"type": "string"}}},
            "required": ["workflow", "repository", "task_summary"], "additionalProperties": False}},
        {"name": "read_context_page", "description": "Read remaining receipt pages in order.", "inputSchema": {
            "type": "object", "properties": {"receipt": {"type": "string"}, "page": {"type": "integer", "minimum": 1}},
            "required": ["receipt", "page"], "additionalProperties": False}},
        {"name": "validate_context", "description": "Validate sources, skills and semantic reachability.", "inputSchema": empty},
    ]


def call_tool(name: str, arguments: dict[str, Any]) -> dict[str, Any]:
    definitions = {item["name"]: item for item in tool_definitions()}
    if not isinstance(name, str) or name not in definitions:
        return text_result({"error": f"unknown tool: {name}"}, is_error=True)
    schema = definitions[name]["inputSchema"]
    if not isinstance(arguments, dict) or set(arguments) - set(schema["properties"]) or set(schema.get("required", [])) - set(arguments):
        raise ValueError("invalid tool arguments")
    for key, value in arguments.items():
        kind = schema["properties"][key]["type"]
        if ((kind == "string" and not isinstance(value, str)) or
            (kind == "array" and not isinstance(value, list)) or
            (kind == "integer" and (isinstance(value, bool) or not isinstance(value, int)))):
            raise ValueError(f"invalid argument type: {key}")
    if name == "validate_context":
        catalog = load_catalog()
        errors = catalog_errors(catalog)
        return text_result({"valid": not errors, "errors": errors, "workflow_count": len(catalog.get("workflows", {})),
                            "domain_count": len(catalog.get("domains", {}))}, is_error=bool(errors))
    if name == "list_workflows":
        return text_result(checked_catalog())
    if name == "start_workflow":
        payload = bundle(arguments["workflow"], arguments["repository"], arguments["task_summary"],
                         arguments.get("risk", "normal"), arguments.get("paths"), arguments.get("domains"))
        serialized = json.dumps(payload, ensure_ascii=False, indent=2)
        pages = [serialized[i:i + MAX_PAGE_CHARS] for i in range(0, len(serialized), MAX_PAGE_CHARS)]
        if len(RECEIPTS) >= MAX_RECEIPTS:
            del RECEIPTS[next(iter(RECEIPTS))]
        receipt = uuid.uuid4().hex
        RECEIPTS[receipt] = pages
        output = {"receipt": receipt, "page": 1, "page_count": len(pages), "complete": len(pages) == 1,
                  "next_action": "Read all source files." if len(pages) == 1 else "Read remaining pages, then all source files."}
        output.update({"bundle": payload} if len(pages) == 1 else {"text": pages[0]})
        return text_result(output)
    receipt, page = arguments["receipt"], arguments["page"]
    pages = RECEIPTS.get(receipt)
    if not pages or page < 1 or page > len(pages):
        raise ValueError("receipt expired or page out of range; restart workflow if needed")
    return text_result({"receipt": receipt, "page": page, "page_count": len(pages),
                        "complete": page == len(pages), "text": pages[page - 1]})


def handle(message: Any) -> dict[str, Any] | None:
    if not isinstance(message, dict) or message.get("jsonrpc") != "2.0" or not isinstance(message.get("method"), str):
        return {"jsonrpc": "2.0", "id": None, "error": {"code": -32600, "message": "invalid request"}}
    if "id" not in message:
        return None
    output: dict[str, Any] = {"jsonrpc": "2.0", "id": message["id"]}
    try:
        method = message["method"]
        if method == "initialize":
            result = {"protocolVersion": PROTOCOL_VERSION, "capabilities": {"tools": {}},
                      "serverInfo": {"name": SERVER_NAME, "version": SERVER_VERSION},
                      "instructions": "Select workflow and domains; read every required source."}
        elif method == "ping":
            result = {}
        elif method == "tools/list":
            result = {"tools": tool_definitions()}
        elif method == "tools/call":
            params = message.get("params", {})
            if not isinstance(params, dict):
                raise ValueError("params must be an object")
            result = call_tool(params.get("name", ""), params.get("arguments", {}))
        else:
            output["error"] = {"code": -32601, "message": "method not found"}
            return output
        output["result"] = result
    except (OSError, ValueError, KeyError, TypeError, UnicodeError) as exc:
        output["error"] = {"code": -32602, "message": str(exc)}
    return output


def main() -> int:
    if len(sys.argv) > 1:
        parser = argparse.ArgumentParser(description=__doc__)
        parser.add_argument("operation", choices=["validate", "start-workflow", "list-workflows"])
        parser.add_argument("--workflow", default="dispatch")
        parser.add_argument("--repository", default="engine")
        parser.add_argument("--risk", default="normal")
        parser.add_argument("--task-summary", default="Inspeccionar contexto de Drasus")
        parser.add_argument("--domain", action="append", default=[])
        parser.add_argument("--path", action="append", default=[])
        args = parser.parse_args()
        try:
            if args.operation == "start-workflow":
                payload = bundle(args.workflow, args.repository, args.task_summary, args.risk, args.path, args.domain)
            else:
                result = call_tool("validate_context" if args.operation == "validate" else "list_workflows", {})
                payload = json.loads(result["content"][0]["text"])
                if result["isError"]:
                    print(json.dumps(payload, ensure_ascii=False, indent=2))
                    return 1
            print(json.dumps(payload, ensure_ascii=False, indent=2))
            return 0
        except (OSError, ValueError, UnicodeError) as exc:
            print(json.dumps({"error": str(exc)}, ensure_ascii=False), file=sys.stderr)
            return 1
    for line in sys.stdin:
        if not line.strip():
            continue
        try:
            output = handle(json.loads(line))
        except json.JSONDecodeError as exc:
            output = {"jsonrpc": "2.0", "id": None, "error": {"code": -32700, "message": str(exc)}}
        if output is not None:
            print(json.dumps(output, ensure_ascii=False), flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
