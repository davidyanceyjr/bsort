---
id: 0025
title: gh-remote-wiring
status: done
layer: L6
depends_on: ["0023", "0024"]
parallel_safe: false
conflicts_with: ["0023", "0024"]
risk: medium
allowed_paths: [".git/**"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "src/**", "tests/**", ".github/**"]
requires: ["0023 local git worktree", "0024 clean local status", "Authenticated gh CLI or explicit remote URL"]
emits: ["GitHub repo wired to origin remote", "Working gh-aware local repo state"]
test_command: "git remote -v"
---

## Goal

Wire the local git worktree to a GitHub repository for `git` and `gh` workflow use.

## Scope

- Create the GitHub repo with `gh` or attach an existing remote.
- Configure `origin`.
- Verify local repo can query remote metadata.

## Non-Scope

- No push unless explicitly approved during implementation.
- No CI or release workflow edits.
- No source or test edits.

## Contract

Inputs:
- Local git worktree
- GitHub CLI auth or remote URL

Outputs:
- `origin` remote configured
- `gh repo view` or equivalent remote verification works

Errors:
- fail if `gh` is unauthenticated and no remote URL is provided

## Acceptance

- `git remote -v` shows `origin`.
- `gh repo view` or equivalent confirms repo linkage.

## Blockers

- May need user choice between creating a new GitHub repo and attaching an existing one.
