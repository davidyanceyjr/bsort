---
id: 0009
title: stdin-reader
status: done
layer: L4
depends_on: ["0001"]
parallel_safe: true
conflicts_with: []
risk: low
allowed_paths: ["src/io.rs", "src/lib.rs", "tests/stdin_io.rs"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**"]
requires: ["0001 concrete runtime IO APIs"]
emits: ["Stdin text reader"]
test_command: "cargo test"
---

## Goal

Read complete input text from stdin when no file path is provided.

## Scope

- Stdin read success path.
- Empty stdin success path.

## Non-Scope

- No parsing.
- No file reads.

## Contract

Inputs:
- Process stdin

Outputs:
- Raw input text

Errors:
- Runtime read errors only if target stack can surface them

## Acceptance

- Tests cover empty stdin and normal stdin content.

## Blockers

- Needs concrete runtime IO APIs from `0001`.
