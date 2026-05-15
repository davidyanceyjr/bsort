---
id: 0018
title: unit-test-skeleton
status: done
layer: L1
depends_on: ["0017"]
parallel_safe: true
conflicts_with: []
risk: low
allowed_paths: ["tests/unit_skeleton.rs", "tests/smoke_cli.rs", "src/lib.rs"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSIONS/**", "SESSION.md", ".github/**", "Cargo.toml", "rust-toolchain.toml"]
requires: ["0017 canonical cargo test command", "Gate 2 approval"]
emits: ["Baseline library tests", "Baseline CLI smoke coverage"]
test_command: "cargo test"
---

## Goal

Add a minimal, stable unit and integration test skeleton for later repo and CI slices.

## Scope

- Add a small library-level test file.
- Keep the existing CLI smoke test as the integration entrypoint.
- Export only what tests need from `src/lib.rs`.

## Non-Scope

- No feature implementation beyond the current no-op scaffold.
- No CI workflow files.
- No broad test matrix.

## Contract

Inputs:
- Existing `run()` scaffold behavior
- Existing smoke test binary contract

Outputs:
- At least one library-level test
- Existing binary smoke test still passing

Errors:
- none

## Acceptance

- `cargo test` runs both library and CLI-level tests.
- Tests assert current scaffold behavior without guessing future behavior.
- Changes stay within test-facing files and minimal library export surface.

## Blockers

- none
