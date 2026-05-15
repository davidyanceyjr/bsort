---
name: session-update
description: Keep SESSION.md compact, current, and under 2k tokens.
---

# Skill: Session Update

Purpose: keep `SESSION.md` as tiny current-state pointer.

Hard cap:

- `SESSION.md` max 2k tokens

Do not include:

- diary
- long history
- pasted diffs
- repeated spec text
- long explanations
- full test logs

## Format

```md
# SESSION

current_slice:
last_completed:
status:
branch:
test_command:

## Ready

- none

## Active

- none

## Blocked

- none

## Batch

gate:
approved_layers:

## Notes

- short note
```

## Status

Use compact values:

```text
planning
idle
in_progress
blocked
failed
```

## On Bootstrap

```md
status: planning

## Notes

- Bootstrap generated from SPEC.md.
- Awaiting Gate 1 approval.
```

## On Slice Start

```md
current_slice: slice-id
status: in_progress
```

## On Slice Done

```md
last_completed: slice-id
current_slice:
status: idle
```

## On Blocked

```md
current_slice: slice-id
status: blocked

## Blocked

- slice-id: short reason
```

Keep only actionable current state.
