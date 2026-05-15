---
id: 0019
title: ci-test-workflow
status: done
layer: L6
depends_on: ["0017", "0018"]
parallel_safe: true
conflicts_with: []
risk: medium
allowed_paths: [".github/workflows/ci.yml"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "src/**", "tests/**", "Cargo.toml", "rust-toolchain.toml"]
requires: ["0017 canonical cargo test command", "0018 baseline tests", "Gate 2 approval"]
emits: ["GitHub Actions CI workflow for test verification"]
test_command: "cargo test"
---

## Goal

Add a minimal CI workflow that runs the canonical Rust test command on push and pull request.

## Scope

- Create a GitHub Actions workflow for test verification.
- Use the repo's stable Rust toolchain contract.
- Keep checks limited to test execution.

## Non-Scope

- No fmt or lint gates.
- No release automation.
- No source or test behavior changes.

## Contract

Inputs:
- Rust package manifest and toolchain pin
- Baseline passing `cargo test`

Outputs:
- Workflow file that runs `cargo test` in CI

Errors:
- none

## Acceptance

- Workflow triggers on push and pull request.
- Workflow installs stable Rust and runs `cargo test`.
- No non-test quality gates are added yet.

## Blockers

- none
