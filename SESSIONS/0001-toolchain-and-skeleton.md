---
id: 0001
title: toolchain-and-skeleton
status: done
layer: L0
depends_on: []
parallel_safe: false
conflicts_with: ["0012", "0013", "0014"]
risk: medium
allowed_paths: ["Cargo.toml", "src/main.rs", "src/lib.rs", "tests/smoke_cli.rs"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**"]
requires: ["Gate 1 approval", "Minimal dependency-light Rust CLI scaffold"]
emits: ["Cargo package", "Base CLI entrypoint", "Base library module", "Concrete test command"]
test_command: "cargo test"
---

## Goal

Create the minimal Rust CLI/test skeleton for later slices.

## Scope

- Create `Cargo.toml`.
- Establish `src/main.rs`, `src/lib.rs`, and `tests/` layout.
- Define version source and base entrypoint shape.

## Non-Scope

- No sorting logic.
- No parser logic.
- No CLI behavior beyond smoke-level skeleton.

## Contract

Inputs:
- Gate 1 approval
- Rust stack decision from planning

Outputs:
- Concrete manifest and entrypoint paths
- `cargo test` command for later slices

Errors:
- none

## Acceptance

- Rust CLI skeleton exists.
- Later slices can target concrete Rust paths.
- `cargo test` is documented for later slices.

## Blockers

- none
