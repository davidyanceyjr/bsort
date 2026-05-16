---
id: 0005
title: sortedness-check
status: done
layer: L1
depends_on: ["0003", "0004"]
parallel_safe: true
conflicts_with: []
risk: low
allowed_paths: ["src/check.rs", "src/lib.rs", "tests/check.rs"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**"]
requires: ["0003 bubble sort core", "0004 order helpers"]
emits: ["Pure already-sorted predicate for asc/desc input"]
test_command: "cargo test"
---

## Goal

Detect whether parsed integers are already sorted for the requested order.

## Scope

- Ascending sortedness check.
- Descending sortedness check.
- Duplicate-preserving sortedness behavior.

## Non-Scope

- No exit code handling.
- No input parsing.

## Contract

Inputs:
- Integer list
- Order mode

Outputs:
- Boolean or equivalent sortedness result

Errors:
- none

## Acceptance

- Tests cover empty, single-item, sorted, unsorted, duplicate, and descending cases.

## Blockers

- none
