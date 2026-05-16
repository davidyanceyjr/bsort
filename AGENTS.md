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
- Slices are implemented in budles of 3 maximum.
- Current slice bundle files must fit in one prompt chunk.
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
SKILLS/*.md
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
- mark slices ready without Gate 1 approval

Bootstrap asks at most one compact batch of planning-level blocker questions.

After bootstrap, stop at Gate 1.

## Session Flow

1. Read `AGENTS.md`.
2. Read `WORKFLOW.md`.
3. Read `SESSION.md`.
4. Build current slice bundle from `SESSIONS/`.
5. Read only required `SKILLS/`.
6. Read only source/test files listed by slice bundle.
7. Act only inside allowed paths.
8. Run required checks.
9. Make `SESSION.md` factually correct.
   Bundle truth includes current branch, completed bundle id, and completed slice list.
10. Update `PLAN.md` if the `SESSION.md` change affects plan truth.
11. Update the completed slice files in `SESSIONS/` so its status matches reality.
12. Run `session-update` cleanup only if needed.
13. Report compact result.

## Branch Rule

- Only workflow-scoped changes may be done on `main`.
- Any implementation session that changes source files, tests, or other non-workflow files must use a session branch.
- Create or switch to the session branch before editing implementation files.
- Keep `SESSION.md` branch truth aligned with the actual branch name.

## Plan Sync Rule

- Make `SESSION.md` factually correct before any cleanup or plan sync.
- Update `PLAN.md` after `SESSION.md` when the session change affects plan truth.
- Update the completed slice file after `PLAN.md` when implementation changes slice status.
- Keep `SESSION.md`, `PLAN.md`, and the completed `SESSIONS/*.md` file factually aligned.
- Do not leave `SESSION.md` without a valid next-session handoff.
- In bundle workflow, prefer `last_completed_bundle` and `last_completed_slices` over single-slice `last_completed`.
- Keep `SESSION.md` branch truth aligned with `git branch --show-current`.
- Run session cleanup under the `session-update` skill only if needed.
- Plan truth means slice status, ready queue, blocked state, dependencies, conflicts, or slice inventory.
- Do not update `PLAN.md` for session-only notes that do not change plan truth.

## Human Gates

There are two approval gates.

### Gate 1: Plan Approval

Required before implementation slices are marked `ready`.
Required before any implementation bundle is approved.

Agent may:

- inspect `SPEC.md`
- inspect `PLAN.md`
- propose slice list
- propose dependencies
- propose bundle grouping by scope
- propose parallel-safe slices inside a bundle
- mark blockers

Agent must not:

- write code
- create bundles larger than 3 slices
- mark slices `ready` unless approved

Output format:

```text
GATE 1 REQUIRED

Generated:
- PLAN.md
- SESSION.md
- SESSIONS/*.md

Proposed slices:
- count: N

Proposed bundles:
- bundle-id: slice-a, slice-b
- bundle-id: slice-c

Parallel:
- bundle-id: yes|no

Blocked:
- none | N items

Approve slice plan?
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

## Bundle Rules

Each bundle must stay small.

A bundle should:

- contain 1 to 3 slices
- group slices by scope
- include only slices with clear dependency order
- allow downstream slices in the same bundle when the bundle also includes their upstream dependency slices
- allow parallel implementation only for slices that do not depend on each other
- stay small enough for one reviewable session bundle

Reject broad bundles.

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

Bundle related slices. Do not merge unrelated scopes.

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

Slice:
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

Do not commit unless human asks.

## Definition of Done

Slice is done only when:

- acceptance criteria met
- allowed paths respected
- forbidden paths untouched
- required tests run or reason documented
- `SESSION.md` updated
- compact result reported
