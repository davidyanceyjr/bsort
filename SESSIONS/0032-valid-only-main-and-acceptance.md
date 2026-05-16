---
id: 0032
title: valid-only-main-and-acceptance
status: done
layer: L5
depends_on: ["0012", "0013", "0015", "0030", "0031"]
parallel_safe: false
conflicts_with: []
risk: medium
allowed_paths: ["src/lib.rs", "tests/acceptance_cli.rs"]
forbidden_paths: ["BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "SPEC.md", "src/cli.rs", "src/help.rs", "src/parse.rs", "tests/cli_args.rs", "tests/parse_cli.rs"]
requires: ["0012 sort flow", "0013 check flow", "0015 acceptance CLI coverage", "0030 valid-only CLI surface", "0031 valid-only parser filter", "Gate 2 approval"]
emits: ["End-to-end `--valid-only` behavior for sort, count, and check"]
test_command: "cargo test"
---

## Goal

Wire `--valid-only` through the app and prove dirty-file behavior end to end.

## Scope

- Pass `valid_only` from `CliConfig` into parsing.
- Cover dirty input success for sorted output.
- Cover dirty input success for `--count`.
- Cover dirty input success for `--check`.

## Non-Scope

- No new output format or warnings for skipped rows.
- No changes to release artifacts.
- No changes to non-`valid-only` behavior.

## Contract

Inputs:
- stdin or file input containing valid integers mixed with dirty rows
- `--valid-only` with sort, `--count`, or `--check`

Outputs:
- sorted/count/check results based only on valid parsed integers
- empty stderr on successful dirty-row filtering

Errors:
- without `--valid-only`, the same dirty input still fails with exit `2`

## Acceptance

- Dirty fixture succeeds under `--valid-only` in normal sort mode.
- Dirty fixture succeeds under `--valid-only --count` and counts only valid integers.
- Dirty fixture succeeds under `--valid-only --check` and evaluates only valid integers.
- Existing dirty fixture still fails without `--valid-only`.

## Blockers

- none
