---
id: 0021
title: release-workflow-draft
status: done
layer: L6
depends_on: ["0016", "0017"]
parallel_safe: true
conflicts_with: []
risk: medium
allowed_paths: [".github/workflows/release.yml"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "src/**", "tests/**", "Cargo.toml", "rust-toolchain.toml", "README.md"]
requires: ["0016 repo scaffold", "0017 canonical cargo test command", "Gate 2 approval"]
emits: ["Manual GitHub Actions release workflow skeleton"]
test_command: "cargo test"
---

## Goal

Add a manual release workflow skeleton that builds the CLI and uploads a draft artifact.

## Scope

- Create a GitHub Actions workflow triggered manually.
- Build the release binary.
- Upload a named artifact for later release-contract work.

## Non-Scope

- No automatic tag publishing.
- No GitHub release creation.
- No artifact contract validation beyond a stable draft shape.

## Contract

Inputs:
- Rust package manifest and toolchain pin

Outputs:
- Manual workflow that builds release output and uploads an artifact

Errors:
- none

## Acceptance

- Workflow uses `workflow_dispatch`.
- Workflow builds `cargo build --release`.
- Workflow uploads a predictable artifact for later slices to refine.

## Blockers

- none
