---
id: 0023
title: local-git-init
status: done
layer: L0
depends_on: ["0016"]
parallel_safe: false
conflicts_with: ["0024", "0025"]
risk: low
allowed_paths: [".git/**"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "src/**", "tests/**", ".github/**"]
requires: ["0016 repo scaffold", "Local git available"]
emits: ["Initialized git worktree", "Default branch set", "Working git status commands"]
test_command: "git status --short"
---

## Goal

Initialize this directory as a local git worktree so git-based workflow commands work.

## Scope

- Run `git init` in the project root.
- Set the default branch name for the new worktree.
- Verify `git status --short` and `git diff --stat` can run.

## Non-Scope

- No remote creation.
- No first commit.
- No GitHub CLI work yet.

## Contract

Inputs:
- Existing project directory contents
- Local `git` CLI

Outputs:
- `.git/` metadata
- Usable local git worktree state

Errors:
- fail if `git` is unavailable

## Acceptance

- `.git/` exists.
- `git status --short` runs in repo root.
- `git diff --stat` runs in repo root.

## Blockers

- none
