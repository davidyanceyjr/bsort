---
name: slice-implementation
description: Implement exactly one approved slice with tests, bounded context, and slice-level blocker questions.
---

# Skill: Slice Implementation

Purpose: implement one approved slice.

Before editing:

1. Read `AGENTS.md`.
2. Read `SESSION.md`.
3. Read `PLAN.md` only as needed.
4. Read current slice file.
5. Read only required source/test files.
6. Verify dependencies are done.
7. Stop at Gate 2.

## Gate 2

Required before source/test edits.

```text
GATE 2 REQUIRED

Slice:
- slice-id

Will change:
- path

Will test:
- command

Risk:
- low|medium|high

Approve?
```

## After Approval

Allowed:

- edit only `allowed_paths`
- implement smallest change
- add/update required tests
- run `test_command`
- update `SESSION.md`
- update current slice status
- update `PLAN.md` status for current slice only

Forbidden:

- future slices
- unrelated refactors
- new dependencies without approval
- forbidden paths
- broad formatting
- deleting/weakening tests
- public API changes unless slice says so

## Slice Blocker Questions

Stop if implementation is unsafe.

Ask only questions required to unblock the slice.

Max 3 questions per blocked slice.

Format:

```text
BLOCKED

Slice:
- slice-id

Reason:
- short reason

Question:
- exact question

Options:
A. ...
B. ...
C. ...

Recommend:
- A

Affects:
- slice-id
- dependent-slice-id
```

Do not edit code while blocked.

## Done Output

```text
DONE

Slice:
- slice-id

Changed:
- file

Tests:
- command: pass|fail|not run

SESSION.md:
- updated

Notes:
- none
```

## Failed Output

```text
FAILED

Slice:
- slice-id

Changed:
- file

Failure:
- short failure

Cause:
- short cause

Next:
- suggested next step
```
