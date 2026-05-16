---
id: 0030
title: valid-only-cli-surface
status: done
layer: L3
depends_on: ["0008", "0014"]
parallel_safe: true
conflicts_with: ["0032"]
risk: low
allowed_paths: ["src/cli.rs", "src/help.rs", "tests/cli_args.rs"]
forbidden_paths: ["BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "SPEC.md", "src/lib.rs", "src/parse.rs", "tests/acceptance_cli.rs", "tests/parse_cli.rs"]
requires: ["0008 option parser", "0014 help text", "Gate 2 approval"]
emits: ["CLI flag surface for `--valid-only`"]
test_command: "cargo test"
---

## Goal

Add the `--valid-only` flag to CLI parsing and help output.

## Scope

- Add `valid_only` to `CliConfig`.
- Parse `--valid-only` from argv.
- Document the flag in help text.
- Add CLI argument tests for the new flag.

## Non-Scope

- No parser behavior changes.
- No runtime wiring into sort, count, or check flows.
- No acceptance coverage for dirty files yet.

## Contract

Inputs:
- `bsort --valid-only`
- `bsort --help`

Outputs:
- `CliConfig.valid_only == true` when the flag is present
- help text lists `--valid-only`

Errors:
- unknown-option behavior remains unchanged for other flags

## Acceptance

- `parse_args()` accepts `--valid-only`.
- Existing flag behavior remains intact.
- Help output includes one-line `--valid-only` description.

## Blockers

- none
