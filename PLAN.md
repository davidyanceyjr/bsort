# PLAN

## Decisions

- Stack: Rust CLI with `Cargo.toml`, `src/main.rs`, `src/lib.rs`, and `tests/`.
- Integer range: signed 64-bit integers.
- `--desc` with `--count` or `--check`: usage error, exit `2`.

## Slice Index

| ID | Layer | Title | Status | Depends On | Parallel | Conflicts | Risk |
|---|---|---|---|---|---|---|---|
| 0001 | L0 | toolchain-and-skeleton | done | none | false | 0012, 0013, 0014 | medium |
| 0002 | L0 | exit-codes-and-shared-types | done | 0001 | true | none | low |
| 0003 | L1 | bubble-sort-core | done | 0001 | true | none | low |
| 0004 | L1 | order-helpers | done | 0001 | true | none | low |
| 0005 | L1 | sortedness-check | done | 0003, 0004 | true | none | low |
| 0006 | L1 | unique-filter | done | 0003 | true | none | low |
| 0007 | L2 | line-parser-and-parse-errors | done | 0001, 0002 | true | none | medium |
| 0008 | L3 | option-parser-and-usage-errors | done | 0001, 0002, 0004 | true | 0012, 0013, 0014 | medium |
| 0009 | L4 | stdin-reader | done | 0001 | true | none | low |
| 0010 | L4 | file-reader-and-exit3 | done | 0001, 0002 | true | none | low |
| 0011 | L4 | output-formatters-and-count | done | 0001, 0002 | true | none | low |
| 0012 | L5 | sort-flow-main | done | 0002, 0003, 0004, 0006, 0007, 0008, 0009, 0010, 0011 | false | 0001, 0013, 0014 | high |
| 0013 | L5 | check-flow-main | ready | 0002, 0004, 0005, 0007, 0008, 0009, 0010 | false | 0001, 0012, 0014 | medium |
| 0014 | L6 | help-and-version | done | 0001, 0002, 0008 | false | 0001, 0012, 0013 | low |
| 0015 | L5 | cli-acceptance-tests | approved | 0012, 0013, 0014 | false | none | medium |
| 0016 | L0 | github-repo-scaffold | done | 0001 | true | none | low |
| 0017 | L0 | test-command-wiring | done | 0001 | true | none | low |
| 0018 | L1 | unit-test-skeleton | done | 0017 | true | none | low |
| 0019 | L6 | ci-test-workflow | done | 0017, 0018 | true | none | medium |
| 0020 | L6 | ci-quality-gates | done | 0019 | false | none | medium |
| 0021 | L6 | release-workflow-draft | done | 0016, 0017 | true | none | medium |
| 0022 | L6 | release-artifact-contract | done | 0021 | false | none | medium |
| 0023 | L0 | local-git-init | done | 0016 | false | 0024, 0025 | low |
| 0024 | L0 | local-git-hygiene | done | 0023, 0016, 0017 | false | 0023, 0025 | low |
| 0025 | L6 | gh-remote-wiring | done | 0023, 0024 | false | 0023, 0024 | medium |
| 0026 | L0 | branch-bundle-policy | done | 0025 | false | 0027, 0028 | low |
| 0027 | L1 | pr-template-for-slice-bundles | done | 0026 | false | 0028 | low |
| 0028 | L2 | pr-policy-ci-check | done | 0026, 0027 | false | none | medium |

## Ready Queue

- 0013-check-flow-main

## Blocked

- none
