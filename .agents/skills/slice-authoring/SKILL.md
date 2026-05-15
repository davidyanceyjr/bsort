---
name: slice-authoring
description: Create or revise small schema-valid implementation slice files in SESSIONS/.
---

# Skill: Slice Authoring

Purpose: convert approved spec/plan work into tiny implementation slices.

Rules:

- small beats broad
- one behavior per slice
- each slice fits one prompt chunk
- downstream dependencies first
- contracts before implementations
- tests with each slice when practical
- no code edits during authoring
- stop at Gate 1 when slice plan changes

## Required Slice Schema

```md
---
id:
title:
status: proposed
layer:
depends_on: []
parallel_safe: true
conflicts_with: []
risk: low
allowed_paths: []
forbidden_paths: []
requires: []
emits: []
test_command:
---

## Goal

One sentence.

## Scope

- item

## Non-Scope

- item

## Contract

Inputs:
- item

Outputs:
- item

Errors:
- item

## Acceptance

- item

## Blockers

- none
```

## Status Values

```text
proposed
approved
ready
in_progress
done
blocked
```

## Sizing

Good slice:

- one behavior
- 1 to 5 likely files
- explicit tests/checks
- explicit allowed paths
- explicit forbidden paths

Bad slice:

```text
implement CLI
```

Good slices:

```text
0013-cli-argument-shape
0015-stdin-read-path
0016-file-read-path
```

## Gate 1 Output

```text
GATE 1 REQUIRED

Proposed slices:
- ...

Blocked:
- none | ...

Approve slice plan?
```
