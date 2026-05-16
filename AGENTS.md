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
- Current slice file must fit in one prompt chunk.
- Implementation stage may use max 40% of full LLM context.
- Prefer many narrow slices over broad slices.
- Load only files needed for the current slice.
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
2. Read `SESSION.md`.
3. Read current slice from `SESSIONS/`.
4. Read only required `SKILLS/`.
5. Read only source/test files listed by slice.
6. Act only inside allowed paths.
7. Run required checks.
8. Update `SESSION.md`.
9. Report compact result.

## Plan Sync Rule

- Update `PLAN.md` after `SESSION.md` when the session change affects plan truth.
- Plan truth means slice status, ready queue, blocked state, dependencies, conflicts, or slice inventory.
- Do not update `PLAN.md` for session-only notes that do not change plan truth.

## Human Gates

There are two approval gates.

### Gate 1: Plan Approval

Required before implementation slices are marked `ready`.

Agent may:

- inspect `SPEC.md`
- inspect `PLAN.md`
- propose slice list
- propose dependencies
- mark blockers

Agent must not:

- write code
- create large slice batches without approval
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

Blocked:
- none | N items

Approve slice plan?
```

### Gate 2: Implementation Approval

Required before implementing a ready slice.

Agent may:

- inspect current slice
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

Slice:
- 0001-name

Will change:
- path/a
- path/b

Will test:
- command

Risk:
- low|medium|high

Approve?
```

After approval, implement only that slice.

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

Prefer:

- more pure-function slices
- more independent tests
- explicit dependencies
- explicit conflicts

Avoid:

- broad feature slices
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
