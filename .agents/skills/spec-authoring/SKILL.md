---
name: spec-authoring
description: Write or revise SPEC.md in man-page-plus-contract style for implementation slicing.
---

# Skill: Spec Authoring

Purpose: make `SPEC.md` precise, compact, stable, and sliceable.

Use man-page style plus technical contracts.

## Recommended Sections

```md
# NAME
# SYNOPSIS
# DESCRIPTION
# ACTORS
# REQUIREMENTS
# OPTIONS
# INPUTS
# OUTPUTS
# FILES
# SCHEMAS
# STATE
# ERRORS
# EDGE CASES
# HUMAN GATES
# NON-GOALS
# EXAMPLES
# ACCEPTANCE
```

## Rules

- product behavior only
- no slice task lists
- no implementation diary
- no prompt essays
- no unresolved guesses
- requirements must be testable
- non-goals must be explicit
- examples should show exact input/output/error behavior

## Requirement Style

Good:

```text
REQ-008: Blank lines must be rejected unless --ignore-blank is provided.
```

Bad:

```text
The parser should be smart and easy to use.
```

## Contract Rule

Each important behavior should fit:

```text
Given X
When Y
Then Z
```

or:

```text
The system must/must not ...
```

## Ambiguity

If unclear, add compact open question or non-goal.

Do not hide ambiguity in vague prose.
