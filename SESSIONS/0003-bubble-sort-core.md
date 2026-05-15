---
id: 0003
title: bubble-sort-core
status: proposed
layer: L1
depends_on: ["0001"]
parallel_safe: true
conflicts_with: []
risk: low
allowed_paths: ["src/sort.rs", "src/lib.rs", "tests/sort.rs"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**"]
requires: ["0001 concrete path layout"]
emits: ["Pure bubble sort implementation for signed integers"]
test_command: "cargo test"
---

## Goal

Implement pure bubble sort without using built-in sort as the primary algorithm.

## Scope

- Ascending bubble sort over signed integers.
- Preserve duplicates.
- Support negative integers.

## Non-Scope

- No descending selection.
- No dedupe.
- No CLI wiring.

## Contract

Inputs:
- Integer list

Outputs:
- New or mutated list sorted ascending by bubble sort

Errors:
- none

## Acceptance

- Empty, single-item, duplicate, negative, sorted, and reverse-sorted inputs are covered.
- Tests assert bubble sort behavior, not built-in sort delegation.

## Blockers

- Needs concrete paths from `0001`.
