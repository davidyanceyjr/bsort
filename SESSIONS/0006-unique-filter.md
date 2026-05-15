---
id: 0006
title: unique-filter
status: proposed
layer: L1
depends_on: ["0003"]
parallel_safe: true
conflicts_with: []
risk: low
allowed_paths: ["src/unique.rs", "src/lib.rs", "tests/unique.rs"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**"]
requires: ["0003 sorted integer output"]
emits: ["Post-sort dedupe helper"]
test_command: "cargo test"
---

## Goal

Remove duplicate values after sorting while preserving sorted order.

## Scope

- Dedupe already-sorted ascending or descending values.
- Keep first instance of each sorted run.

## Non-Scope

- No raw unsorted dedupe contract.
- No output formatting.

## Contract

Inputs:
- Sorted integer list

Outputs:
- Sorted integer list with duplicates removed

Errors:
- none

## Acceptance

- Tests cover empty, single-item, no-duplicate, all-duplicate, and mixed duplicate cases.

## Blockers

- none
