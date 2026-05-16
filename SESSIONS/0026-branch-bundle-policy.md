---
id: 0026
title: branch-bundle-policy
status: ready
layer: L0
depends_on: ["0025"]
parallel_safe: false
conflicts_with: ["0027", "0028"]
risk: low
allowed_paths: ["README.md", ".github/CONTRIBUTING.md"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "src/**", "tests/**", ".github/workflows/**"]
requires: ["0025 GitHub repo wired", "Approved default policy B"]
emits: ["Documented branch-per-bundle workflow", "Shared branch naming and slice grouping rules"]
test_command: "rg -n \"branch|slice|bundle|PR\" README.md .github/CONTRIBUTING.md"
---

## Goal

Document the default branch strategy for slice delivery.

## Scope

- Define default policy: one branch per small related slice bundle.
- Define exception for single-slice branches.
- Define forbidden case: unrelated slices on one branch.
- Define branch naming shape for slice bundles.

## Non-Scope

- No PR template yet.
- No CI enforcement yet.
- No source or workflow logic changes.

## Contract

Inputs:
- Approved branch policy decision B
- Existing repo README and GitHub repo presence

Outputs:
- Contributor-facing branch workflow guidance
- Stable naming guidance for later PR workflow slices

Errors:
- fail if target docs path choice conflicts with existing repo docs structure

## Acceptance

- Docs state default policy B clearly.
- Docs state when one slice alone is still preferred.
- Docs state that unrelated slices must not share a branch.
- Docs include one branch naming example.

## Blockers

- none
