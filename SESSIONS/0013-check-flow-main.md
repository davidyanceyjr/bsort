---
id: 0013
title: check-flow-main
status: proposed
layer: L5
depends_on: ["0002", "0004", "0005", "0007", "0008", "0009", "0010"]
parallel_safe: false
conflicts_with: ["0001", "0012", "0014"]
risk: medium
allowed_paths: ["src/main.rs", "src/pipeline.rs", "src/lib.rs", "tests/check_cli.rs"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**"]
requires: ["Check-mode dependencies", "Blocked CLI and parser slices resolved"]
emits: ["`--check` execution path", "Exit 0/1 behavior with no stdout on success/failure"]
test_command: "cargo test"
---

## Goal

Wire `--check` mode so the CLI reports sortedness without printing sorted output.

## Scope

- Exit `0` when input is already sorted.
- Exit `1` when input is not sorted.
- Keep stdout empty for both check outcomes.
- Reject `--desc --check` as usage error per planning decision.

## Non-Scope

- No normal sorted output mode.
- No help/version behavior.

## Contract

Inputs:
- Validated CLI config in check mode
- Parsed integer list

Outputs:
- Exit `0` or `1`
- No stdout and no stderr unless error

Errors:
- Propagate usage, file, and parse errors

## Acceptance

- CLI tests cover sorted and unsorted check examples in default order, plus `--desc --check` usage error.

## Blockers

- Depends on blocked slices `0007` and `0008`.
