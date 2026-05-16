# Contributing

## Branch Policy

- Default policy: one branch per small related slice bundle.
- Prefer one slice on one branch when the slice is already small and self-contained.
- Do not put unrelated slices on the same branch.
- Use branch names shaped like `slice/<slice-ids>-<short-topic>`.
- Single-slice example: `slice/0003-bubble-sort-core`
- Bundle example: `slice/0026-0028-repo-workflow`

## Bundle Rules

- Group only slices that are closely related and can be reviewed together.
- Keep the bundle small enough to review in one PR.
- If a later slice depends on an earlier slice in the same topic, they may share a branch.
- If slices change unrelated behavior, split them onto separate branches.

## Pull Requests

- List all slice IDs in the PR body.
- State why the branch is a single slice or a small bundle.
- Record tests run and note any CI-relevant context.
- Record risk and rollback notes.
