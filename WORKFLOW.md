# WORKFLOW

## Scope

This file is the workflow policy source of truth for human and agent contributors.

It covers:
- branch creation
- branch naming
- slice gates
- session/work-bundle grouping
- pull request expectations

## Gate Boundaries

Gate approval is per slice.

Rules:
- Keep approval and implementation scope at the slice level.
- Do not treat every approved slice as a mandatory branch boundary.
- A slice may be implemented alone or as part of a small related session bundle.

## Branch Boundaries

Branch policy is per session/work bundle, not per slice.

Rules:
- Small slice implementations do not require a new branch per slice.
- Use one branch for a small related bundle of slices completed in the same work session.
- Do not mix unrelated slices on one branch.
- If work is expected to be committed, pushed, or reviewed in a PR, create a branch before that happens.
- If work expands beyond a trivial isolated slice, create a branch.

## Main Branch Exception

Trivial local-only work may remain on `main` for a while.

Rules:
- A very small isolated slice may stay on `main` when it is local-only and not yet intended for PR review.
- Once local-only work becomes a reviewable bundle, move to a branch.
- If multiple related slices accumulate into one reviewable unit, use a branch.

## Branch Naming

When a branch is created, use:

`slice/<slice-ids>-<short-topic>`

Examples:
- `slice/0003-bubble-sort-core`
- `slice/0003-0004-0009-core-helpers-stdin`

## Session Bundles

A session bundle is a small related set of slices that can be reviewed together.

Rules:
- Group only closely related slices.
- Keep the bundle small enough for one clear PR.
- Dependent slices in the same topic may share a branch.
- Unrelated behavior changes must be split onto separate branches.

## Pull Requests

When opening a PR for a branch:
- list all slice IDs
- state why the branch is a single slice or a small related bundle
- record tests run
- record CI-relevant context
- record risk and rollback notes
