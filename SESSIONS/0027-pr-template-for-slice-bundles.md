---
id: 0027
title: pr-template-for-slice-bundles
status: approved
layer: L1
depends_on: ["0026"]
parallel_safe: false
conflicts_with: ["0028"]
risk: low
allowed_paths: [".github/PULL_REQUEST_TEMPLATE.md"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "src/**", "tests/**", ".github/workflows/**"]
requires: ["0026 documented branch bundle policy"]
emits: ["Pull request template for slice tracking", "Required fields for CI/CD review context"]
test_command: "rg -n \"Slice|Branch|Tests|CI|Risk\" .github/PULL_REQUEST_TEMPLATE.md"
---

## Goal

Add a pull request template that makes slice bundles reviewable and trackable.

## Scope

- Require slice IDs included in the PR.
- Require branch intent and bundle summary.
- Require test evidence and CI/CD notes.
- Require risk and rollback notes.

## Non-Scope

- No workflow enforcement yet.
- No issue templates.
- No source or test changes.

## Contract

Inputs:
- Documented branch-bundle policy
- Existing GitHub repo structure

Outputs:
- Stable PR template for slice bundle review
- Structured fields for project tracking and CI evidence

Errors:
- none

## Acceptance

- Template has a section for slice IDs.
- Template has a section for bundle rationale.
- Template has a section for checks run.
- Template has a section for deployment or rollback notes.

## Blockers

- none
