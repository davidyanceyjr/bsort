---
name: bootstrap
description: First-run SPEC.md to PLAN.md, SESSION.md, and SESSIONS/*.md generation. Use when bootstrapping or after major SPEC.md changes.
---

# Skill: Bootstrap

Purpose: generate workflow state from `SPEC.md`.

Read:

- `AGENTS.md`
- `BOOTSTRAP.md`
- `SPEC.md`

May create/update:

- `PLAN.md`
- `SESSION.md`
- `SESSIONS/*.md`

Must not:

- edit source code
- edit tests
- install dependencies
- mark slices ready without Gate 1
- create extra workflow files

## Process

1. Extract facts from `SPEC.md`.
2. Identify layers:

```text
L0 skeleton/types/constants
L1 pure functions
L2 parsing/validation
L3 CLI args
L4 IO wiring
L5 integration behavior
L6 docs/help/release
```

3. Create narrow slices.
4. Assign dependencies.
5. Mark `parallel_safe`.
6. Mark `conflicts_with`.
7. Mark unclear work blocked.
8. Write `PLAN.md`.
9. Write `SESSION.md`.
10. Write one file per slice under `SESSIONS/`.
11. Stop at Gate 1.

## Ambiguity

One bootstrap clarification batch only.

Ask only planning-blocking questions.

Max 10 questions.

Each question must include:

- options
- recommendation
- blocked/affected slices

If unanswered:

- generate safe slices
- mark affected slices blocked
- stop at Gate 1

## Output

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
