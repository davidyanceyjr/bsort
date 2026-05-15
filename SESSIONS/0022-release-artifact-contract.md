---
id: 0022
title: release-artifact-contract
status: done
layer: L6
depends_on: ["0021"]
parallel_safe: false
conflicts_with: []
risk: medium
allowed_paths: [".github/workflows/release.yml"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "src/**", "tests/**", ".github/workflows/ci.yml"]
requires: ["0021 release workflow draft", "Gate 2 approval"]
emits: ["Release workflow with explicit artifact naming and validation"]
test_command: "cargo test"
---

## Goal

Define a stable release artifact shape and validate it before upload.

## Scope

- Keep the manual release trigger.
- Derive artifact naming from package metadata and target.
- Validate staged release files before upload.

## Non-Scope

- No GitHub release publishing.
- No extra release platforms.
- No source or test edits.

## Contract

Inputs:
- Existing release workflow draft
- Cargo package metadata

Outputs:
- Release artifact with predictable name and staged file layout
- Validation step that fails if required files are missing

Errors:
- workflow fails when staged artifact files are missing

## Acceptance

- Release workflow still uses `workflow_dispatch`.
- Uploaded artifact name is deterministic from app/version/target.
- Workflow validates required files before upload.

## Blockers

- none
