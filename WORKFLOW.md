# WORKFLOW

## Scope

This file is the workflow policy source of truth for human and agent contributors.

It covers:
- branch creation
- branch naming
- slice gates
- session/work-bundle grouping
- workflow file sync
- pull request expectations

## Gate Boundaries

Gate 1 approval is per planned bundle.
Gate 2 approval is per implementation bundle.

Rules:
- Keep bundle scope small and coherent.
- A bundle may contain 1 to 3 slices.
- Group slices by scope during planning.
- Only place slices in the same bundle when they are closely related.
- Slices with no dependency on each other may be implemented in parallel inside one approved bundle.
- A bundle may include dependency-ordered slices when the same bundle includes the upstream slices they depend on.
- Do not put dependency-blocked future work into the current bundle unless the bundle also includes the needed upstream slice work.
- Do not treat every approved slice as a mandatory branch boundary.
- A slice may be implemented alone or as part of a small related session bundle.

## Branch Boundaries

Branch policy is per session/work bundle, not per slice.

Rules:
- Only workflow-scoped changes may be done on `main`.
- Small slice implementations do not require a new branch per slice.
- Use one branch for a small related bundle of slices completed in the same work session.
- Do not mix unrelated slices on one branch.
- Create or switch to the session branch before changing source files, tests, or other non-workflow files.
- If work is expected to be committed, pushed, or reviewed in a PR, it must already be on a branch.
- There is no local-only implementation exception on `main`.

## Branch Naming

When a branch is created, use:

`slice/<slice-ids>-<short-topic>`

Examples:
- `slice/0003-bubble-sort-core`
- `slice/0003-0004-0009-core-helpers-stdin`

## Session Bundles

A session bundle is a small related set of slices that can be reviewed together.

Rules:
- Group only related slices.
- Bundle size max: 3 slices.
- Keep the bundle small enough for one clear PR.
- Dependent slices in the same topic may share a branch.
- Independent slices in the same scope may be implemented in parallel.
- Unrelated behavior changes must be split onto separate branches.

## Workflow File Sync

Workflow truth must stay aligned across session, plan, and slice files.

Rules:
- After completing a slice, update `SESSION.md` first.
- When work was done as a bundle, record bundle truth in `SESSION.md` first.
- Update `PLAN.md` next if the completed work changes plan truth.
- Update the completed slice file in `SESSIONS/` so its header status matches reality.
- For completed slice work, keep `SESSION.md`, `PLAN.md`, and the completed `SESSIONS/<id>-*.md` file in sync in the same change.
- Do not leave a slice marked done in one workflow file and proposed, approved, or ready in another.
- Before closing a session, produce a valid next-session handoff.
- Minimum handoff truth:
- `SESSION.md` branch matches `git branch --show-current`.
- `SESSION.md` records `last_completed_bundle` when the work was done as a bundle.
- `SESSION.md` records `last_completed_slices` and each listed slice is `done` in `PLAN.md`.
- `SESSION.md` names the next ready slice, or says blocked, or says project complete.
- `PLAN.md` marks all newly unblocked slices `ready`.
- The completed slice file status is `done`.
- Do not end a session with an empty ready queue unless the project is blocked or complete.

## Pull Requests

When opening a PR for a branch:
- list all slice IDs
- state why the branch is a single slice or a small related bundle
- record tests run
- record CI-relevant context
- record risk and rollback notes
