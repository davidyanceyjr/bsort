# WORKFLOW

## Scope

This file is the workflow policy source of truth for human and agent contributors.

It covers:
- branch creation
- branch naming
- atomic slice planning
- bundle selection
- slice gates
- session/work-bundle grouping
- workflow file sync
- git publish and clean
- pull request expectations

## Gate Boundaries

Gate 1 approval is per bundled execution plan.
Gate 2 approval is per implementation bundle.

Rules:
- Planning first creates atomic slices.
- Bundling happens after atomic planning.
- Keep bundle scope small and coherent.
- A bundle may contain 1 to 3 slices.
- Only place slices in the same bundle when they are closely related.
- Slices with no dependency on each other may be implemented in parallel inside one approved bundle.
- A bundle may include dependency-ordered slices when the same bundle includes the upstream slices they depend on.
- Do not put dependency-blocked future work into the current bundle unless the bundle also includes the needed upstream slice work.
- Do not treat every approved slice as a mandatory branch boundary.
- A slice may be implemented alone or as part of a small related session bundle.

## Planning Phases

### PLAN

Purpose:
- create atomic prompt-chunk slices
- record dependency truth
- avoid premature execution bundling

Rules:
- each slice covers one clear behavior change
- each slice stays small enough for one prompt chunk
- record deps, conflicts, allowed paths, forbidden paths, tests, and risk
- keep slice truth in `PLAN.md`

### BUNDLE

Purpose:
- select executable session bundles from the atomic slice graph

Rules:
- create bundles of 1 to 3 slices
- default to one slice when unclear
- bundle only same-scope or dependency-linked work
- allow independent sibling slices together only when review size stays small
- allow upstream plus downstream slices together only when both fit one session
- reject overlapping risky edits
- reject unrelated behavior changes
- preserve atomic slice truth from `PLAN.md`
- write bundle/session truth to `SESSIONS/*.md`

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
- Independent parallel slices in one bundle should not overlap allowed paths.
- Unrelated behavior changes must be split onto separate branches.

## Workflow File Sync

Workflow truth must stay aligned across session, plan, and slice files.

Rules:
- After completing a bundle, update `SESSION.md` first.
- When work was done as a bundle, record bundle truth in `SESSION.md` first.
- Update `PLAN.md` next if the completed work changes plan truth.
- Update the completed bundle file in `SESSIONS/` so its header status matches reality.
- For completed bundle work, keep `SESSION.md`, `PLAN.md`, and the completed `SESSIONS/<id>-*.md` file in sync in the same change.
- Do not leave a slice marked done in one workflow file and proposed, approved, or ready in another.
- Before closing a session, produce a valid next-session handoff.
- Minimum handoff truth:
- `SESSION.md` branch matches `git branch --show-current`.
- `SESSION.md` records `last_completed_bundle` when the work was done as a bundle.
- `SESSION.md` records `last_completed_slices` and each listed slice is `done` in `PLAN.md`.
- `SESSION.md` names the next ready bundle, or says blocked, or says project complete.
- `PLAN.md` marks all newly unblocked slices `ready`.
- The completed bundle file status is `done`.
- Do not end a session with an empty ready queue unless the project is blocked or complete.

## Post-Implementation Git Flow

After any implementation slice or approved bundle is complete:
1. Run required tests and confirm pass or document why not run.
2. Sync `SESSION.md`.
3. Sync `PLAN.md` if plan truth changed.
4. Sync the completed `SESSIONS/<id>-*.md` files.
5. Stage only files that belong to the completed slice or bundle.
6. Ask for approval only on the prepared commit summary.
7. Commit on the session branch.
8. Push the session branch to `origin`.
9. Switch to `main`.
10. Update local `main` from `origin/main` with `git pull --ff-only`.
11. Merge the session branch into `main` with `git merge --ff-only`.
12. Push `main`.
13. Delete the merged session branch locally.
14. Delete the merged session branch on `origin`.
15. End on clean `main`.

Rules:
- Do not ask for an extra approval gate for staging, pushing, merging, or cleanup.
- The only human gate in this flow is approval of the prepared commit summary.
- Commit summary must include commit message, included slice or bundle IDs, staged paths, test result, and any excluded unrelated dirty files.
- Do not include unrelated uncommitted changes in the staged set.
- Prefer direct command flow over ad hoc manual steps.
- Do not create merge commits for this workflow.

Stop and ask if:
- unrelated dirty files are mixed into the worktree and cannot be safely excluded
- required tests fail
- `git pull --ff-only` fails on `main`
- `git merge --ff-only` fails
- push is rejected
- branch protection or remote policy blocks the merge flow
- workflow truth files disagree about completed status or next ready work

Command pattern:
- `git add <scoped-paths>`
- `git commit -m "<approved-message>"`
- `git push -u origin <session-branch>`
- `git checkout main`
- `git pull --ff-only`
- `git merge --ff-only <session-branch>`
- `git push origin main`
- `git branch -d <session-branch>`
- `git push origin --delete <session-branch>`
- `git status --short`

## Pull Requests

When opening a PR for a branch:
- list all slice IDs
- state why the branch is a single slice or a small related bundle
- state whether any slices were parallel-safe siblings
- record tests run
- record CI-relevant context
- record risk and rollback notes
