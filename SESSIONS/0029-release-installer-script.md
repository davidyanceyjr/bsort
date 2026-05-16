---
id: 0029
title: release-installer-script
status: done
layer: L6
depends_on: ["0022"]
parallel_safe: false
conflicts_with: []
risk: medium
allowed_paths: [".github/workflows/release.yml", "README.md", "scripts/install.sh"]
forbidden_paths: ["BOOTSTRAP.md", "PLAN.md", "SESSION.md", "SESSIONS/**", "src/**", "tests/**", ".github/workflows/ci.yml", "Cargo.toml", "rust-toolchain.toml"]
requires: ["0022 release artifact contract", "Gate 2 approval"]
emits: ["Release artifact that includes a POSIX installer script for the built binary"]
test_command: "cargo test"
---

## Goal

Ship a small POSIX installer script with release output so users can install the built `bsort` binary.

## Scope

- Add a checked-in `scripts/install.sh`.
- Stage the installer into the release artifact beside the built binary.
- Document basic installer usage in `README.md`.

## Non-Scope

- No package manager integration.
- No multi-platform installer logic beyond POSIX `sh`.
- No source or test behavior changes for the CLI itself.

## Contract

Inputs:
- built release binary from the existing release workflow
- optional destination directory argument to `install.sh`

Outputs:
- release artifact containing `bsort` and `install.sh`
- installer copies `bsort` into the destination directory

Errors:
- installer exits non-zero with stderr text when `bsort` is missing
- installer exits non-zero with stderr text when the destination is not writable

## Acceptance

- Release workflow stages `install.sh` into the uploaded artifact.
- `install.sh` runs under POSIX `sh`.
- Running `sh install.sh` installs `bsort` into `/usr/local/bin` by default.
- Running `sh install.sh /custom/bin` installs into the provided directory.
- README shows artifact installer usage.

## Blockers

- none
