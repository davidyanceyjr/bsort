# bsort

[![CI](https://github.com/davidyanceyjr/bsort/actions/workflows/ci.yml/badge.svg)](https://github.com/davidyanceyjr/bsort/actions/workflows/ci.yml)

Small Rust CLI that sorts newline-separated signed integers using bubble sort.

## Status

Work in progress.

## Behavior

- Reads from stdin or one input file.
- Parses one signed integer per line.
- Sorts ascending by default.
- Uses bubble sort by contract.

## Options

```text
--desc
--unique
--count
--check
--ignore-blank
--help
--version
```

## Usage

```text
bsort [OPTIONS] [FILE]
```

Examples:

```text
printf "3\n1\n2\n" | bsort
printf "3\n1\n2\n" | bsort --desc
printf "1\n1\n2\n" | bsort --unique
printf "1\n2\n3\n" | bsort --check
bsort numbers.txt
```

## Output

- Sorted mode prints one integer per line to stdout.
- `--count` prints the parsed item count.
- `--check` prints nothing and exits `0` if already sorted, `1` if not sorted.
- Usage and parse errors exit `2`.
- File read errors exit `3`.

## Development

Requirements:

- Rust stable toolchain with Cargo support.

Checks:

```text
cargo test
```

## Git Workflow

- Default policy: use one branch per small related slice bundle.
- Use one slice alone when the work is already small and self-contained.
- Do not mix unrelated slices on one branch.
- Branch names use the shape `slice/<slice-ids>-<short-topic>`.
- Single-slice example: `slice/0003-bubble-sort-core`
- Bundle example: `slice/0026-0028-repo-workflow`

See `.github/CONTRIBUTING.md` for contributor rules and PR expectations.
