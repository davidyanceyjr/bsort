---
id: 0014
title: help-and-version
status: ready
layer: L6
depends_on: ["0001", "0002", "0008"]
parallel_safe: false
conflicts_with: ["0001", "0012", "0013"]
risk: low
allowed_paths: ["src/main.rs", "src/help.rs", "src/lib.rs", "tests/help_version.rs"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**"]
requires: ["Concrete entrypoint from 0001", "CLI parser from 0008"]
emits: ["`--help` short-circuit", "`--version` short-circuit"]
test_command: "cargo test"
---

## Goal

Provide `--help` and `--version` behavior that exits `0` without entering normal execution flows.

## Scope

- `--help` prints usage and exits `0`.
- `--version` prints version and exits `0`.

## Non-Scope

- No sorting behavior.
- No parse or file errors unless CLI parsing fails first.

## Contract

Inputs:
- Raw argv or validated CLI config

Outputs:
- Usage text or version text
- Exit `0`

Errors:
- none beyond option-parser validation

## Acceptance

- Tests verify exit `0` for both flags and confirm help/version text is routed to stdout.

## Blockers

- none
