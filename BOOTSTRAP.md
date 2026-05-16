# BOOTSTRAP.md

Purpose: first-run setup from `SPEC.md`.

Use once per major spec version.

## Inputs

Required:

- `AGENTS.md`
- `SPEC.md`

Optional:

- existing `PLAN.md`
- existing `SESSION.md`
- existing `SESSIONS/*.md`

## Output Files

Create or update:

- `PLAN.md`
- `SESSION.md`
- `SESSIONS/*.md`

Do not create extra workflow files.

## Rules

- `SPEC.md` is source of truth.
- Do not implement code.
- Do not edit source files.
- Do not install dependencies.
- Do not create broad slices.
- Prefer many narrow slices.
- Each slice must fit one prompt chunk.
- Do not force one session file per atomic slice.
- `SESSION.md` must stay under 2k tokens.
- Mark unclear work as `blocked`.
- Do not guess through ambiguity.
- Stop at Gate 1.

## Bootstrap Process

1. Read `SPEC.md`.
2. Extract requirements, non-goals, inputs, outputs, state, errors, acceptance.
3. Identify implementation layers:

```text
L0: project skeleton, constants, types
L1: pure functions
L2: parsing and validation
L3: CLI argument handling
L4: IO wiring
L5: integration behavior
L6: docs/help/release polish
```

4. Create narrow slices.
5. Assign dependencies.
6. Mark parallel safety.
7. Mark conflicts.
8. Build bundle candidates from the atomic slice graph.
9. Run ambiguity scan.
10. Ask one compact planning-level question batch only if needed.
11. Write `PLAN.md`.
12. Write one file per bundle under `SESSIONS/`.
13. Initialize or update `SESSION.md`.
14. Stop for Gate 1 approval.

## Ambiguity Handling

Bootstrap must perform one ambiguity scan.

Classify ambiguity:

```text
planning-blocker: prevents safe slice graph
slice-blocker: affects implementation of specific slices later
non-blocking: use documented default or existing convention
out-of-scope: ignore
```

Bootstrap may ask one compact batch of clarification questions before Gate 1.

Question batch rules:

- Ask only planning-blocker questions.
- Batch all questions together.
- Max 10 questions.
- Each question must include options.
- Each question must include default recommendation.
- Each question must list affected slices or planning area.
- If unanswered, generate the plan anyway and mark affected slices `blocked`.
- Do not continue asking follow-up questions unless human explicitly requests it.

## Clarification Output

Use this format:

```text
CLARIFICATION REQUIRED

Blocking planning questions:

Q1. Question?
Options:
A. option
B. option
C. option
Recommend: A
Affects:
- slice-id or planning area

Answer like:
Q1=A
```

If no answer is provided:

```text
UNRESOLVED

Action:
- affected slices marked blocked
- unaffected slices remain proposed
- Gate 1 requested for unblocked plan
```

## Slice-Level Ambiguity

Bootstrap must not resolve all future ambiguity.

Implementation agents may later block a specific slice when new ambiguity appears.

Rule:

```text
Global ambiguity gets one bootstrap batch.
Local ambiguity gets one blocked slice.
```

## Slice Sizing

A slice should:

- change one behavior
- touch 1 to 5 files
- have clear acceptance
- have clear allowed paths
- have clear forbidden paths
- have test command
- be independently reviewable

Reject broad slices.

## Bundle Sizing

A bundle should:

- contain 1 to 3 slices
- group related work only
- stay reviewable in one session
- allow upstream plus downstream slices together only when both fit
- allow independent sibling slices together only when allowed paths do not overlap

Reject broad bundles.

Bad:

```text
implement CLI
```

Good:

```text
0008-parse-valid-integers
0012-parse-invalid-line-error
0015-stdin-read-path
```

## PLAN.md Format

Use this format:

```md
# PLAN

## Decisions

- none

## Slice Index

| ID | Layer | Title | Status | Depends On | Parallel | Conflicts | Risk |
|---|---|---|---|---|---|---|---|

## Ready Queue

- none

## Blocked

- none
```

Status values:

```text
proposed
approved
ready
in_progress
done
blocked
```

`PLAN.md` is atomic slice truth.

Do not require one `SESSIONS/*.md` file per atomic slice.

## SESSION.md Format

Keep under 2k tokens.

```md
# SESSION

current_slice:
last_completed:
status: planning
branch:
test_command:

## Ready

- none

## Active

- none

## Blocked

- none

## Notes

- Bootstrap generated from SPEC.md.
- Awaiting Gate 1 approval.
```

`SESSION.md` should name ready bundles, not raw slices, after bootstrap.

## Bundle File Format

```md
---
bundle_id:
title:
status: proposed
slice_ids: []
execution_order: []
parallel_groups: []
risk: low
allowed_paths: []
forbidden_paths: []
test_command:
blocked_by: []
---

## Goal

One sentence.

## Slices

- 0001-name

## Why This Bundle

One short paragraph.

## Acceptance

- item
```

## Gate 1 Output

After bootstrap, stop and print:

```text
GATE 1 REQUIRED

Generated:
- PLAN.md
- SESSION.md
- SESSIONS/*.md

Atomic slices:
- count: N

Proposed bundles:
- bundle-id: slice-a
- bundle-id: slice-b, slice-c

Parallel:
- bundle-id: yes|no

Blocked:
- none | N items

Approve bundle plan?
```
