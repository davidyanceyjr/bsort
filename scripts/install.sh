#!/bin/sh

set -eu

dest_dir=${1:-/usr/local/bin}
script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
binary_path="${script_dir}/bsort"
dest_path="${dest_dir}/bsort"

fail() {
  printf '%s\n' "$1" >&2
  exit 1
}

if [ ! -f "${binary_path}" ]; then
  fail "install.sh: missing binary: ${binary_path}"
fi

if [ ! -x "${binary_path}" ]; then
  fail "install.sh: binary is not executable: ${binary_path}"
fi

if [ ! -d "${dest_dir}" ]; then
  fail "install.sh: destination directory not found: ${dest_dir}"
fi

if [ ! -w "${dest_dir}" ]; then
  fail "install.sh: destination directory not writable: ${dest_dir}"
fi

cp "${binary_path}" "${dest_path}"
chmod 755 "${dest_path}"

printf 'installed %s\n' "${dest_path}"
