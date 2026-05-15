---
id: 0012
title: sort-flow-main
status: proposed
layer: L5
depends_on: ["0002", "0003", "0004", "0006", "0007", "0008", "0009", "0010", "0011"]
parallel_safe: false
conflicts_with: ["0001", "0013", "0014"]
risk: high
allowed_paths: ["src/main.rs", "src/pipeline.rs", "src/lib.rs", "tests/sort_cli.rs"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**"]
requires: ["All normal-mode dependencies"]
emits: ["Default sort flow", "`--desc` flow", "`--unique` flow", "`--count` flow"]
test_command: "cargo test"
---

## Goal

Wire normal CLI execution for sort, unique, and count behavior.

## Scope

- Default ascending sort.
- Descending sort with `--desc`.
- Unique filtering after sort with `--unique`.
- Count mode prints parsed integer count only.
- Exit `0` for successful normal flows.

## Non-Scope

- No `--check` logic.
- No help/version short-circuit.

## Contract

Inputs:
- Validated CLI config
- Raw input text from stdin or file

Outputs:
- stdout text for sorted values or count
- exit `0` on success

Errors:
- Propagate parse and usage errors

## Acceptance

- CLI tests cover stdin ascending, descending, unique, count, duplicate preservation, and empty input.

## Blockers

- Depends on blocked slices `0007` and `0008`.
