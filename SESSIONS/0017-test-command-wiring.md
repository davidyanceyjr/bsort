---
id: 0017
title: test-command-wiring
status: done
layer: L0
depends_on: ["0001"]
parallel_safe: true
conflicts_with: []
risk: low
allowed_paths: ["Cargo.toml", "rust-toolchain.toml"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "src/**", "tests/**", ".github/**"]
requires: ["0001 concrete path layout", "Gate 2 approval"]
emits: ["Stable local Rust toolchain pin", "Canonical cargo test entrypoint"]
test_command: "cargo test"
---

## Goal

Make `cargo test` the stable local verification command for the scaffold.

## Scope

- Pin a Rust toolchain for local and CI consistency.
- Add minimal manifest metadata needed for predictable local test runs.

## Non-Scope

- No new source behavior.
- No test expansion beyond existing command contract.
- No CI workflow files.

## Contract

Inputs:
- Existing Cargo package layout

Outputs:
- Local toolchain pin
- Stable documented test command

Errors:
- none

## Acceptance

- Repo includes a Rust toolchain pin.
- `cargo test` remains the canonical verification command.

## Blockers

- none
