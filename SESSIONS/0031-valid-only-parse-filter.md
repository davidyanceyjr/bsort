---
id: 0031
title: valid-only-parse-filter
status: done
layer: L2
depends_on: ["0002", "0007"]
parallel_safe: true
conflicts_with: ["0032"]
risk: medium
allowed_paths: ["src/parse.rs", "tests/parse_cli.rs"]
forbidden_paths: ["BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "SPEC.md", "src/cli.rs", "src/help.rs", "src/lib.rs", "tests/acceptance_cli.rs"]
requires: ["0007 line parser", "Gate 2 approval"]
emits: ["Parser behavior that filters dirty rows when `valid_only` is enabled"]
test_command: "cargo test"
---

## Goal

Let parsing continue over dirty rows when `valid_only` is enabled.

## Scope

- Extend parser inputs so `valid_only` can be applied during line parsing.
- Skip blank, non-integer, and out-of-range rows when `valid_only` is enabled.
- Preserve current line-numbered parse errors when `valid_only` is disabled.
- Add parser-level tests for mixed valid and invalid input.

## Non-Scope

- No CLI flag parsing.
- No top-level `run()` wiring.
- No README or release changes.

## Contract

Inputs:
- input text with valid integers mixed with dirty rows
- parser mode with `valid_only` on or off

Outputs:
- `Vec<i64>` containing only valid parsed integers when `valid_only` is on
- existing parse errors when `valid_only` is off

Errors:
- invalid rows still fail with line-numbered exit-2 errors when `valid_only` is off

## Acceptance

- Parser skips dirty rows under `valid_only`.
- Parser still trims whitespace around valid integers.
- Parser still returns the existing first-invalid-row error when `valid_only` is disabled.

## Blockers

- none
