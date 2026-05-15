---
id: 0002
title: exit-codes-and-shared-types
status: done
layer: L0
depends_on: ["0001"]
parallel_safe: true
conflicts_with: []
risk: low
allowed_paths: ["src/exit_codes.rs", "src/types.rs", "src/lib.rs", "tests/shared_types.rs"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**"]
requires: ["0001 concrete path layout", "Stable exit code contract from SPEC"]
emits: ["Exit code constants", "Order/mode type definitions", "Shared option/result types"]
test_command: "cargo test"
---

## Goal

Define shared constants and types used by the CLI pipeline.

## Scope

- Exit statuses `0`, `1`, `2`, `3`.
- Shared order and mode types.
- Common result/error type used by later slices.

## Non-Scope

- No parsing.
- No sorting.
- No IO wiring.

## Contract

Inputs:
- Exit status table from `SPEC.md`
- Option names from `SPEC.md`

Outputs:
- Stable constants/types for downstream slices

Errors:
- none

## Acceptance

- Shared types cover ascending, descending, sort, count, and check modes.
- Exit codes match `SPEC.md`.
- Unit tests cover exported constants/types where applicable.

## Blockers

- Needs concrete paths from `0001`.
