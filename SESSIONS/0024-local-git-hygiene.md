---
id: 0024
title: local-git-hygiene
status: done
layer: L0
depends_on: ["0023", "0016", "0017"]
parallel_safe: false
conflicts_with: ["0023", "0025"]
risk: low
allowed_paths: [".gitignore", ".git/info/exclude"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "src/**", "tests/**", ".github/**"]
requires: ["0023 local git worktree", "Useful local ignore policy"]
emits: ["Clean git status signal for workflow use"]
test_command: "git status --short"
---

## Goal

Make local git status output useful by ignoring build and machine-local noise.

## Scope

- Audit `.gitignore` against current Rust and workflow artifacts.
- Add machine-local ignores only if still needed.
- Verify `git status --short` shows signal, not cache noise.

## Non-Scope

- No remote setup.
- No commit creation.
- No source or workflow edits.

## Contract

Inputs:
- Existing `.gitignore`
- Output from local git status after init

Outputs:
- Ignore rules aligned with local workflow use

Errors:
- none

## Acceptance

- `git status --short` excludes build/cache noise.
- Needed project files still appear as trackable changes.

## Blockers

- none
