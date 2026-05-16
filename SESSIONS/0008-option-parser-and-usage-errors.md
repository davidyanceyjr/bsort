---
id: 0008
title: option-parser-and-usage-errors
status: done
layer: L3
depends_on: ["0001", "0002", "0004"]
parallel_safe: true
conflicts_with: ["0012", "0013", "0014"]
risk: medium
allowed_paths: ["src/cli.rs", "src/lib.rs", "src/main.rs", "tests/cli_args.rs"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**"]
requires: ["0001 concrete entrypoint layout", "0002 shared types", "0004 order helpers"]
emits: ["CLI option parser", "Positional argument validation", "Usage-error results"]
test_command: "cargo test"
---

## Goal

Parse CLI options and positional arguments into one validated execution mode.

## Scope

- Accept zero or one positional file argument.
- Reject more than one positional argument.
- Parse `--desc`, `--unique`, `--count`, `--check`, `--ignore-blank`, `--help`, `--version`.
- Reject `--desc --count`.
- Reject `--desc --check`.
- Surface usage errors as exit `2`.

## Non-Scope

- No input reading.
- No sorting or parsing of integer lines.

## Contract

Inputs:
- Raw argv

Outputs:
- Validated CLI config
- Usage error for invalid combinations or too many args

Errors:
- Exit-2 class usage errors via shared error type

## Acceptance

- Tests cover zero args, one file arg, too many args, and each flag.
- Tests cover `--desc --count` and `--desc --check` as usage errors.

## Blockers

- none
