---
id: 0011
title: output-formatters-and-count
status: ready
layer: L4
depends_on: ["0001", "0002"]
parallel_safe: true
conflicts_with: []
risk: low
allowed_paths: ["src/output.rs", "src/lib.rs", "tests/output.rs"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**"]
requires: ["0001 concrete path layout", "0002 shared types"]
emits: ["Sorted output formatter", "Count output formatter", "stderr error formatter"]
test_command: "cargo test"
---

## Goal

Format stdout and stderr text exactly enough for the CLI contract.

## Scope

- One integer per output line.
- Trailing newline when at least one value is printed.
- Count mode output with one unsigned decimal integer and newline.
- Human-readable stderr formatting for parse and file errors.

## Non-Scope

- No actual IO writes.
- No sorting.

## Contract

Inputs:
- Parsed integers or count
- Shared error payloads

Outputs:
- stdout/stderr strings ready for write

Errors:
- none

## Acceptance

- Tests cover empty output, multi-line output, count output, and example invalid-integer stderr text shape.

## Blockers

- Needs concrete paths from `0001`.
