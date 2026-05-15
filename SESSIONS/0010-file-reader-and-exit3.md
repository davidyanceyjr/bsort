---
id: 0010
title: file-reader-and-exit3
status: proposed
layer: L4
depends_on: ["0001", "0002"]
parallel_safe: true
conflicts_with: []
risk: low
allowed_paths: ["src/io.rs", "src/lib.rs", "tests/file_io.rs"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**"]
requires: ["0001 concrete runtime IO APIs", "0002 exit code constants"]
emits: ["File reader", "File-read error mapping to exit 3"]
test_command: "cargo test"
---

## Goal

Read input from one file path and map read/open failures to exit status `3`.

## Scope

- Open and read one file path.
- Error payload includes file path.

## Non-Scope

- No CLI arg parsing.
- No integer parsing.

## Contract

Inputs:
- File path string

Outputs:
- Raw input text
- File-read error tagged for exit `3`

Errors:
- File open/read failure with file path context

## Acceptance

- Tests cover readable file and unreadable or missing file.

## Blockers

- Needs concrete runtime IO APIs from `0001`.
