---
id: 0007
title: line-parser-and-parse-errors
status: proposed
layer: L2
depends_on: ["0001", "0002"]
parallel_safe: true
conflicts_with: []
risk: medium
allowed_paths: ["src/parse.rs", "src/lib.rs", "tests/parse_cli.rs"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**"]
requires: ["0001 Rust skeleton", "0002 shared error/result types", "Signed 64-bit integer contract"]
emits: ["Line-based i64 parser", "Blank-line handling", "Line-numbered parse errors"]
test_command: "cargo test"
---

## Goal

Parse newline-separated signed 64-bit integers with whitespace trimming and line-numbered errors.

## Scope

- One signed integer per line.
- Value range must fit signed 64-bit integer limits.
- Leading and trailing whitespace allowed.
- Empty input returns empty parsed list.
- Blank line rejection unless ignore-blank mode is enabled.
- Invalid integer errors include line number and original value.

## Non-Scope

- No CLI flag parsing.
- No file IO.

## Contract

Inputs:
- Raw input text
- Ignore-blank setting

Outputs:
- Parsed `i64` list
- Parse error for invalid integer or forbidden blank line

Errors:
- Exit-2 class parse errors via shared error type

## Acceptance

- Tests cover whitespace, blank lines, invalid first line, invalid later line, and empty input.
- Tests cover out-of-range integer rejection.
- Error payload can support stderr text with line number.

## Blockers

- none
