---
id: 0028
title: pr-policy-ci-check
status: approved
layer: L2
depends_on: ["0026", "0027"]
parallel_safe: false
conflicts_with: []
risk: medium
allowed_paths: [".github/workflows/pr-policy.yml"]
forbidden_paths: ["SPEC.md", "BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "src/**", "tests/**", "README.md"]
requires: ["0026 documented branch bundle policy", "0027 PR template fields defined"]
emits: ["PR CI check for branch naming and slice metadata", "Tracking-oriented workflow signal on pull requests"]
test_command: "git diff --stat .github/workflows/pr-policy.yml"
---

## Goal

Add a pull-request workflow that checks basic branch and tracking policy signals.

## Scope

- Trigger on pull request events.
- Check branch name shape against bundle policy.
- Check PR body for slice ID and test evidence markers.
- Fail clearly when required tracking fields are missing.

## Non-Scope

- No merge blocking configuration in GitHub settings.
- No deployment or release automation.
- No source or test behavior changes.

## Contract

Inputs:
- Branch policy docs
- PR template field names
- GitHub Actions support on the repo

Outputs:
- CI signal for PR tracking hygiene
- Repeatable policy check for slice bundle PRs

Errors:
- fail if workflow depends on unapproved third-party actions

## Acceptance

- Workflow runs on pull request events.
- Workflow checks branch naming shape.
- Workflow checks PR body for slice IDs and tests.
- Workflow reports a failing status when required markers are absent.

## Blockers

- none
