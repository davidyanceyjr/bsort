---
id: 0004
title: order-helpers
status: proposed
layer: L1
depends_on: ["0001"]
parallel_safe: true
conflicts_with: []
risk: low
allowed_paths: ["src/order.rs", "src/lib.rs", "tests/order.rs"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**"]
requires: ["0001 concrete path layout"]
emits: ["Order-aware compare helpers", "Ascending/descending selection helpers"]
test_command: "cargo test"
---

## Goal

Provide small pure helpers that express ascending versus descending behavior.

## Scope

- Comparison helper for asc/desc.
- Shared adapter that lets later slices reuse one core sort/check path.

## Non-Scope

- No CLI parsing.
- No output formatting.

## Contract

Inputs:
- Order enum or equivalent shared type
- Integer pairs or integer lists

Outputs:
- Deterministic comparison decisions for asc/desc behavior

Errors:
- none

## Acceptance

- Helpers produce opposite ordering results for asc versus desc.
- Tests cover equal values and negative values.

## Blockers

- Needs concrete paths from `0001`.
