# Rust review

Integrated from BeautifulRust's `rust-review` skill under the single `rust-best-practices` entry point.

## Trigger and scope

Use this guide when the user asks to review, audit, critique, or check Rust code, a crate, a module, or a diff. Review the requested scope; for a diff, inspect changed lines and the callers, callees, tests, and contracts they affect.

Before judging API advice, identify whether the code is an application, an internal Feature port, or a library with external consumers. Read the crate's edition and MSRV from `Cargo.toml`. For Feature ports, use ADR-0137 and the current contract before recommending signature or type changes.

## Review rules

- Review without editing unless the user asks for fixes. A standalone Rust review does not replace the Tech Lead's QA phase or its `APTO/NO APTO` gate.
- Follow [AGENTS.md, «Revisión de código»](../../../../AGENTS.md) for scope, evidence, findings, and report format.
- Run commands only when the task or Order authorizes them. Use the exact scoped commands recorded there; do not launch Clippy, tests, Miri, fuzzing, or docs builds merely because upstream's original review skill listed them.
- Read the code before accepting the implementer's summary. Trace affected inputs and outputs, errors, state changes, and callers.

## Priorities

1. **Correctness and soundness:** reachable panic on external input, unchecked overflow or lossy cast, invalid `unsafe`, unsound safe API, incorrect `Eq`/`Ord`/`Hash` contracts, ignored errors, lock guards across `.await`, or destructive/blocking `Drop`.
2. **Error handling:** lost source/context, errors callers cannot classify, recoverable failures represented as panics, or public errors that violate the owning contract.
3. **Types and API:** missing domain types or representable invalid states, unnecessary clones, poor ownership choices, accidental public exposure, breaking changes, or signatures that confuse call sites. Treat rules such as avoiding public `bool` parameters as review signals; validate them against the real callers and contract.
4. **Idioms and determinism:** nondeterministic map iteration in serialized/output data, confusing clone/borrow workarounds, unclear iterator chains, or abstractions without a real consumer.
5. **Docs and tests:** comments that disagree with code, missing public contract details, or tests that fail to distinguish the required behavior when such evidence is in scope.
6. **Performance:** unbounded work, blocking async paths, allocations or copies in a measured hot path, or violated latency/resource budgets. Do not report speculative micro-optimizations.

## Finding quality

Each required finding needs an exact `file:line`, a reachable scenario, incorrect behavior or concrete cost, evidence, and the smallest viable correction. Cite the relevant Rust API guideline, source digest, or lint when it supports the finding. Group repeated instances of one defect. Separate required fixes from recommendations and optional polish using the categories in `AGENTS.md`; omit empty categories.

Do not treat every `unwrap`, `expect`, public `bool`, or Clippy warning as a defect by itself. Apply the repository's invariant, contract, and lint rules. If there are no findings, state the reviewed scope and the material checks performed or not run.

