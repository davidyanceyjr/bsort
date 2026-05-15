---
id: 0016
title: github-repo-scaffold
status: done
layer: L0
depends_on: ["0001"]
parallel_safe: true
conflicts_with: []
risk: low
allowed_paths: ["README.md", ".gitignore", ".editorconfig", "LICENSE"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "src/**", "tests/**"]
requires: ["0001 concrete path layout", "Gate 2 approval"]
emits: ["Repo-facing README", "Rust-appropriate ignore rules", "Editor defaults", "Optional license placeholder"]
test_command: "cargo test"
---

## Goal

Make the scaffold look like a publishable GitHub repository.

## Scope

- Replace placeholder README content with project-specific repo docs.
- Add Rust-appropriate `.gitignore` entries while preserving useful local ignores.
- Add a compact `.editorconfig`.

## Non-Scope

- No source code changes.
- No CI workflows.
- No release workflow.

## Contract

Inputs:
- Existing Rust package name and version
- CLI behavior from `SPEC.md`

Outputs:
- Repo-facing scaffold files aligned with the CLI contract

Errors:
- none

## Acceptance

- README describes purpose, usage, and test command.
- `.gitignore` covers Rust build outputs.
- `.editorconfig` exists with basic text defaults.

## Blockers

- none
