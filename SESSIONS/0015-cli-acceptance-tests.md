---
id: 0015
title: cli-acceptance-tests
status: proposed
layer: L5
depends_on: ["0012", "0013", "0014"]
parallel_safe: false
conflicts_with: []
risk: medium
allowed_paths: ["tests/acceptance_cli.rs", "tests/fixtures/**"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**"]
requires: ["Normal flow", "Check flow", "Help/version flow"]
emits: ["Spec-level CLI coverage for acceptance list"]
test_command: "cargo test"
---

## Goal

Cover the acceptance list from `SPEC.md` with end-to-end CLI tests.

## Scope

- stdin and file input flows
- ascending and descending sort
- duplicate preservation and `--unique`
- count mode
- check mode exit codes
- blank-line handling with and without `--ignore-blank`
- invalid integer, too many args, file read error, help, and version cases

## Non-Scope

- No source behavior beyond test coverage.
- No benchmark or performance testing.

## Contract

Inputs:
- CLI executable under test
- Fixture files where needed

Outputs:
- Passing end-to-end coverage for acceptance criteria

Errors:
- Test failures only

## Acceptance

- Every acceptance bullet in `SPEC.md` maps to at least one CLI test.
- Example commands from `SPEC.md` are represented directly or by equivalent tests.

## Blockers

- Depends on slices `0012`, `0013`, and `0014`.
