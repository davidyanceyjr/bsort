# NAME

bsort - sort integers using bubble sort

# SYNOPSIS

```text
bsort [OPTIONS] [FILE]
```

# DESCRIPTION

`bsort` reads newline-separated signed integers, sorts them using bubble sort, and writes the sorted integers to stdout.

Input comes from `FILE` when provided. Otherwise input comes from stdin.

The default order is ascending.

# ACTORS

- User: runs the CLI.
- Shell: provides stdin, stdout, stderr, arguments, and exit status.
- CI system: runs automated checks.

# REQUIREMENTS

## Functional

REQ-001: The program must accept zero or one positional file argument.

REQ-002: If no file argument is provided, the program must read from stdin.

REQ-003: If one file argument is provided, the program must read from that file.

REQ-004: The program must reject more than one positional file argument.

REQ-005: The program must parse one signed integer per line.

REQ-006: The parser must allow leading and trailing whitespace.

REQ-007: Empty input must produce empty output and exit `0`.

REQ-008: Blank lines must be rejected unless `--ignore-blank` is provided.

REQ-009: Invalid integers must produce a line-numbered error.

REQ-010: The program must sort using bubble sort.

REQ-011: Default order must be ascending.

REQ-012: `--desc` must sort descending.

REQ-013: Duplicate integers must be preserved.

REQ-014: Negative integers must be supported.

REQ-015: `--unique` must remove duplicate values after sorting.

REQ-016: `--count` must print only the number of parsed integers.

REQ-017: `--check` must check whether input is already sorted and print no sorted output.

REQ-018: `--check` must exit `0` when input is sorted.

REQ-019: `--check` must exit `1` when input is not sorted.

REQ-020: `--help` must print usage and exit `0`.

REQ-021: `--version` must print version and exit `0`.

REQ-022: Normal sorted output must be written to stdout.

REQ-023: Errors must be written to stderr.

REQ-024: The program must expose stable exit codes.

## Non-Functional

NFR-001: The implementation must be dependency-light.

NFR-002: The program must run without network access.

NFR-003: The program must be testable with unit and CLI tests.

NFR-004: Sorting must use bubble sort even when a faster sort exists.

NFR-005: Workflow files must stay token-budgeted.

NFR-006: `SESSION.md` must remain under 2k tokens.

NFR-007: Each implementation slice must fit in one prompt chunk.

NFR-008: Implementation context for one slice must not exceed 20% of the model context window.

# OPTIONS

## `--desc`

Sort integers in descending order.

Cannot be combined with options that make ordering irrelevant unless explicitly allowed.

## `--unique`

Remove duplicate values after sorting.

## `--count`

Print only the number of parsed integers.

`--count` does not print sorted values.

## `--check`

Check whether input is already sorted.

Output:

- stdout: empty
- stderr: empty unless error
- exit `0`: sorted
- exit `1`: not sorted
- exit `2`: usage or parse error

## `--ignore-blank`

Ignore blank lines.

## `--help`

Print usage and exit `0`.

## `--version`

Print version and exit `0`.

# INPUTS

- stdin text
- optional file path
- command-line options

# OUTPUTS

## Sorted Output

```text
<integer>
<integer>
<integer>
```

Rules:

- one integer per line
- trailing newline required when at least one value is printed
- empty input prints nothing

## Count Output

```text
<number>
```

Rules:

- one unsigned decimal integer
- trailing newline required

## Error Output

Errors must be human-readable and written to stderr.

# EXIT STATUS

```text
0 success
1 check failed: input not sorted
2 usage error or parse error
3 file read error
```

# FILES

No config files.

# SCHEMAS

## Input Format

```text
<signed-integer>
<signed-integer>
```

Rules:

- one value per line
- whitespace around value allowed
- blank lines rejected unless `--ignore-blank`
- values must fit supported signed integer range

## Slice File Schema

Each slice file must include:

```yaml
id: string
title: string
status: proposed | approved | ready | in_progress | done | blocked
layer: L0 | L1 | L2 | L3 | L4 | L5 | L6
depends_on: string[]
parallel_safe: boolean
conflicts_with: string[]
risk: low | medium | high
allowed_paths: string[]
forbidden_paths: string[]
requires: string[]
emits: string[]
test_command: string
```

Required body sections:

```text
## Goal
## Scope
## Non-Scope
## Contract
## Acceptance
## Blockers
```

# STATE

The program is stateless.

No persistent files are created.

# ERRORS

## Too Many Arguments

Condition:

```text
more than one positional file argument
```

Behavior:

```text
stderr includes usage error
exit 2
```

## Invalid Integer

Condition:

```text
line cannot be parsed as signed integer
```

Behavior:

```text
stderr includes line number and original value
exit 2
```

## Blank Line

Condition:

```text
blank line found and --ignore-blank not set
```

Behavior:

```text
stderr includes line number
exit 2
```

## File Read Error

Condition:

```text
file cannot be opened or read
```

Behavior:

```text
stderr includes file path
exit 3
```

# EDGE CASES

- empty input
- single integer
- duplicate integers
- negative integers
- already sorted input
- reverse sorted input
- whitespace around integers
- blank lines
- invalid integer on first line
- invalid integer after valid lines
- unreadable file
- too many arguments
- conflicting or redundant flags

# HUMAN GATES

## Gate 1: Plan Approval

Required before slices are marked `ready`.

## Gate 2: Implementation Approval

Required before source or test files are changed.

# NON-GOALS

- Do not use built-in sort as the primary algorithm.
- Do not support floating-point numbers.
- Do not support string sorting.
- Do not support JSON.
- Do not support CSV.
- Do not support multiple input files.
- Do not add config files.
- Do not add network behavior.
- Do not add interactive prompts.
- Do not add a database.
- Do not add a web UI.

# EXAMPLES

## stdin ascending

Command:

```sh
printf "3\n1\n2\n" | bsort
```

Output:

```text
1
2
3
```

## descending

Command:

```sh
printf "3\n1\n2\n" | bsort --desc
```

Output:

```text
3
2
1
```

## unique

Command:

```sh
printf "3\n1\n3\n" | bsort --unique
```

Output:

```text
1
3
```

## count

Command:

```sh
printf "3\n1\n3\n" | bsort --count
```

Output:

```text
3
```

## check sorted

Command:

```sh
printf "1\n2\n3\n" | bsort --check
```

Exit:

```text
0
```

## check unsorted

Command:

```sh
printf "2\n1\n3\n" | bsort --check
```

Exit:

```text
1
```

## invalid input

Command:

```sh
printf "1\nabc\n3\n" | bsort
```

stderr:

```text
error: invalid integer on line 2: abc
```

Exit:

```text
2
```

# ACCEPTANCE

The project is acceptable when:

- stdin input works.
- file input works.
- ascending sort works.
- descending sort works.
- duplicates are preserved by default.
- `--unique` removes duplicates.
- `--count` prints parsed integer count.
- `--check` returns correct exit status.
- blank line behavior matches `--ignore-blank`.
- invalid integer errors include line number.
- too many arguments exit `2`.
- file read errors exit `3`.
- `--help` exits `0`.
- `--version` exits `0`.
- all required tests pass.
