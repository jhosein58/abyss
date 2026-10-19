#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="${1:-${ABYSS_COMPILER:-${ROOT_DIR}/abyssc}}"
cd "${ROOT_DIR}"
WORK_DIR=$(mktemp -d "${ROOT_DIR}/tmp/dummy_backend.XXXXXX")
trap 'rm -rf "${WORK_DIR}"' EXIT

timeout 30s "${COMPILER}" compiler/dummy_driver.a "${WORK_DIR}/driver.c"
gcc -std=gnu99 -D_POSIX_C_SOURCE=200809L -O0 "${WORK_DIR}/driver.c" -o "${WORK_DIR}/driver"
timeout 30s "${WORK_DIR}/driver" tests/01_literals.a "${WORK_DIR}/dummy.out"
test -s "${WORK_DIR}/dummy.out"
rg -q '^dummy_start_function$' "${WORK_DIR}/dummy.out"
rg -q '^dummy_end_function$' "${WORK_DIR}/dummy.out"
if rg -q '#include' "${WORK_DIR}/dummy.out"; then
    exit 1
fi
echo "PASS dummy backend"
