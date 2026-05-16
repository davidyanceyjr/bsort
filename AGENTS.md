# AGENTS.md

## Mode

Token cheap. No essays. No broad context loading. No guessing.

Default output style: caveman.

Good:

```text
Done.
Changed:
- src/foo.ts
- tests/foo.test.ts

Tests:
- npm test: pass

Blocked:
- none
```

Bad:

```text
I carefully reviewed the repository and considered several approaches...
```

## Context Budget

Hard limits:

- `SESSION.md` max: 2k tokens.
- Atomic slices must fit in one prompt chunk.
- Session bundles are 3 slices maximum.
- Current bundle file and referenced slice rows must fit in one prompt chunk.
- Implementation stage may use max 20% of full LLM context.
- Prefer many narrow slices over broad slices.
- Load only files needed for the current slice bundle.
- Do not load whole repo unless explicitly told.

## Repo Truth Files

Use only these workflow files:

```text
SPEC.md
PLAN.md
SESSION.md
BOOTSTRAP.md
SESSIONS/*.md
AGENTS.md
WORKFLOW.md
.agents/skills/*/SKILL.md
```

No extra planning docs unless human approves.

## Bootstrap Rule

For first run or major spec rewrite, use `BOOTSTRAP.md`.

Bootstrap may create or update:

- `PLAN.md`
- `SESSION.md`
- `SESSIONS/*.md`

Bootstrap must not:

- edit source files
- write implementation code
- install dependencies
- mark bundles ready without Gate 1 approval

Bootstrap asks at most one compact batch of planning-level blocker questions.

After bootstrap, stop at Gate 1.

## Session Flow

1. Read `AGENTS.md`.
2. Read `WORKFLOW.md`.
3. Read `SESSION.md`.
4. Read current bundle from `SESSIONS/`.
5. Read only needed slice rows from `PLAN.md`.
6. Read only required repo skills from `.agents/skills/*/SKILL.md`.
7. Read only source/test files listed by current bundle.
8. Act only inside allowed paths.
9. Run required checks.
10. Make `SESSION.md` factually correct.
   Bundle truth includes current branch, completed bundle id, and completed slice list.
11. Update `PLAN.md` if the `SESSION.md` change affects plan truth.
12. Update the completed bundle file in `SESSIONS/` so its status matches reality.
13. Run `session-update` cleanup only if needed.
14. Report compact result.

## Branch Rule

- Only workflow-scoped changes may be done on `main`.
- Any implementation session that changes source files, tests, or other non-workflow files must use a session branch.
- Create or switch to the session branch before editing implementation files.
- Keep `SESSION.md` branch truth aligned with the actual branch name.

## Plan Sync Rule

- Make `SESSION.md` factually correct before any cleanup or plan sync.
- Update `PLAN.md` after `SESSION.md` when the session change affects plan truth.
- Update the completed bundle file after `PLAN.md` when implementation changes slice status.
- Keep `SESSION.md`, `PLAN.md`, and the completed `SESSIONS/*.md` bundle file factually aligned.
- Do not leave `SESSION.md` without a valid next-session handoff.
- In bundle workflow, prefer `last_completed_bundle` and `last_completed_slices` over single-slice `last_completed`.
- Keep `SESSION.md` branch truth aligned with `git branch --show-current`.
- Run session cleanup under the `session-update` skill only if needed.
- Plan truth means slice status, ready queue, blocked state, dependencies, conflicts, slice inventory, or bundle inventory.
- Do not update `PLAN.md` for session-only notes that do not change plan truth.

## Human Gates

There are two approval gates.

### Gate 1: Plan Approval

Required after atomic slice planning and bundle selection.
Required before implementation bundles are marked `ready`.

Agent may:

- inspect `SPEC.md`
- inspect `PLAN.md`
- propose slice list
- propose dependencies
- propose bundle grouping after slice planning
- propose parallel-safe slices inside a bundle
- mark blockers

Agent must not:

- write code
- create bundles larger than 3 slices
- mark bundles `ready` unless approved

Output format:

```text
GATE 1 REQUIRED

Generated:
- PLAN.md
- SESSION.md
- SESSIONS/*.md

Atomic slices:
- count: N

Proposed bundles:
- bundle-id: slice-a, slice-b
- bundle-id: slice-c

Parallel:
- bundle-id: yes|no

Blocked:
- none | N items

Approve bundle plan?
```

### Gate 2: Implementation Approval

Required before implementing an approved bundle or a single approved slice.

Agent may:

- inspect current slice or bundle
- inspect allowed files
- inspect tests
- identify ambiguity

Agent must not:

- edit source
- edit tests
- install dependencies
- run destructive commands

Output format:

```text
GATE 2 REQUIRED

Bundle:
- bundle-id

Slices:
- 0001-name
- 0002-name

Will change:
- path/a
- path/b

Will test:
- command

Risk:
- low|medium|high

Approve?
```

After approval, implement only that bundle.

## Post-Implementation Git Flow

After any completed implementation slice or approved bundle:
- run required local tests
- update `SESSION.md`
- update `PLAN.md` if plan truth changed
- update completed `SESSIONS/*.md` files
- stage only current slice or bundle files
- ask only for commit summary approval
- commit on session branch
- push branch
- wait for required remote tests / CI / CD checks to complete
- switch to `main`
- `git pull --ff-only`
- `git merge --ff-only <session-branch>`
- push `main`
- clean merged branch local and remote
- end on clean `main`

Rules:
- Do not ask for a separate approval to stage, push, merge, or clean.
- The only required human gate in this flow is approval of the prepared commit summary.
- Commit summary must include commit message, included slice or bundle IDs, staged paths, test result, and any excluded unrelated dirty files.
- Do not stage unrelated dirty worktree changes.
- If any required local test, remote test, or CI/CD check fails, stop and ask.
- A failed local test, failed remote test, or failed CI/CD check is a human-gated boundary.
- Do not merge to `main`, mark `DONE`, or clean the session branch while required remote checks are failing or incomplete.
- If `git pull --ff-only`, `git merge --ff-only`, or push fails, stop and ask.

Blocked format for check failure:

```text
BLOCKED

Bundle:
- bundle-id

Reason:
- required local or remote checks failed

Need:
- human decision before merge, rerun, or further changes
```

## Slice Rules

Each slice must be small.

A slice should:

- touch 1 to 5 files
- have one clear behavior change
- have explicit allowed paths
- have explicit forbidden paths
- define tests/checks
- fit in one prompt chunk

Reject broad slices.

Planning phase makes atomic slices only.

## Bundle Rules

Each bundle must stay small.

A bundle should:

- contain 1 to 3 slices
- group slices by scope
- include only slices with clear dependency order
- allow downstream slices in the same bundle when the bundle also includes their upstream dependency slices
- allow parallel implementation only for slices that do not depend on each other
- use only slices whose combined file surface stays reviewable
- keep non-dependent slices subagent-safe only when allowed paths do not overlap
- stay small enough for one reviewable session bundle

Reject broad bundles.

Bundle phase happens after atomic planning.

Bundle phase should:

- default to one slice when bundling is unclear
- preserve atomic slice truth from `PLAN.md`
- write one executable session bundle file under `SESSIONS/`
- keep workflow-file ownership with the parent agent when subagents are used

Bad:

```text
Add auth system.
```

Good:

```text
0004-password-hash-interface
0005-password-hash-implementation
0006-user-login-contract
0007-login-route
```

## Scale Rule

When project grows, add slices. Do not widen slices.

Bundle related slices later. Do not merge unrelated scopes.

Prefer:

- more pure-function slices
- more independent tests
- explicit dependencies
- explicit conflicts
- small approved bundles
- parallel work only for independent slices

Avoid:

- broad feature slices
- broad bundle batches
- long session notes
- loading unrelated files

## Drift Control

Agent must stop if:

- requirements conflict
- needed file is outside allowed paths
- acceptance criteria are unclear
- dependency slice is incomplete
- implementation requires new dependency
- tests require unavailable service
- slice exceeds context budget

Use:

```text
BLOCKED

Slice:
- slice-id

Reason:
- short reason

Need:
- exact human decision
```

## Slice Blocker Questions

If a slice cannot be implemented safely, stop and ask targeted blocker questions.

Rules:

- Ask only questions required to unblock the slice.
- Prefer one question.
- Max three questions per blocked slice.
- Include options.
- Include recommendation.
- Include affected slices.
- Do not edit code while blocked.
- Do not expand scope.
- Do not regenerate unrelated slices.

Output format:

```text
BLOCKED

Slice:
- 0012-name

Reason:
- missing or conflicting decision

Question:
- exact question

Options:
A. option
B. option
C. option

Recommend:
- A|B|C and short reason

Affects:
- 0012-name
- dependent-slice-id
```

## Modification Rules

Allowed:

- files listed in current slice `allowed_paths`
- `SESSION.md`
- `PLAN.md` status fields for the current slice only

Forbidden unless human approves:

- unrelated refactors
- broad formatting changes
- new dependencies
- public API changes
- deleting tests
- weakening tests
- changing CI
- editing completed slice history

## Output Rules

Use compact reports.

After implementation:

```text
DONE

Bundle:
- bundle-id

Slices:
- 0001-name

Changed:
- file
- file

Tests:
- command: pass|fail|not run

SESSION.md:
- updated

Notes:
- none
```

If failed:

```text
FAILED

Slice:
- 0001-name

Changed:
- file

Failure:
- test command failed

Cause:
- short cause

Next:
- suggested fix
```

If blocked:

```text
BLOCKED

Slice:
- 0001-name

Reason:
- short reason

Need:
- exact human decision
```

## Command Safety

Do not run destructive commands unless slice explicitly allows.

Forbidden by default:

```text
rm -rf
git reset --hard
git clean -fd
drop database
force push
```

## Git Rules

Before work:

```text
git status --short
```

Before any non-workflow implementation change:

```text
git branch --show-current
```

If current branch is `main`, stop and create or switch to a session branch first.

After work:

```text
git diff --stat
```

After implementation sessions, run the post-implementation git flow.

## Definition of Done

Bundle is done only when:

- acceptance criteria met
- allowed paths respected
- forbidden paths untouched
- required tests run or reason documented
- `SESSION.md` updated
- `PLAN.md` slice status updated
- current `SESSIONS/*.md` bundle file updated
- compact result reported
