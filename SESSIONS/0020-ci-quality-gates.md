---
id: 0020
title: ci-quality-gates
status: done
layer: L6
depends_on: ["0019"]
parallel_safe: false
conflicts_with: []
risk: medium
allowed_paths: [".github/workflows/ci.yml"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "src/**", "tests/**", ".github/workflows/release.yml"]
requires: ["0019 CI test workflow", "Gate 2 approval"]
emits: ["CI workflow with explicit quality gates"]
test_command: "cargo test"
---

## Goal

Expand the CI workflow with explicit formatting and lint gates alongside tests.

## Scope

- Keep the existing test workflow trigger shape.
- Add fmt and clippy checks to CI.
- Keep all checks inside the existing CI workflow file.

## Non-Scope

- No source or test edits.
- No release workflow edits.
- No new helper scripts.

## Contract

Inputs:
- Existing CI workflow
- Stable Rust toolchain contract

Outputs:
- CI workflow that runs `cargo fmt --check`, `cargo clippy`, and `cargo test`

Errors:
- none

## Acceptance

- CI still triggers on push and pull request.
- CI runs formatting, lint, and test checks.
- Clippy runs with warnings denied.

## Blockers

- none
