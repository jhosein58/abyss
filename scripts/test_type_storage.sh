#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="${1:-${ROOT_DIR}/abyssc}"
WORK_DIR=$(mktemp -d "${ROOT_DIR}/tmp/type_storage.XXXXXX")
SOURCE="${ROOT_DIR}/tests/compiler/type_storage.a"
EXPECTED="${SOURCE%.a}.out"
C_COMPILER="${ABYSS_C_COMPILER:-}"
if [[ -z "${C_COMPILER}" ]]; then
    if command -v tcc >/dev/null 2>&1; then
        C_COMPILER="tcc"
    elif command -v gcc >/dev/null 2>&1; then
        C_COMPILER="gcc"
    elif command -v clang >/dev/null 2>&1; then
        C_COMPILER="clang"
    fi
fi

if [[ "${C_COMPILER}" == *"tcc"* ]]; then
    C_FLAGS="-std=c99 -D_POSIX_C_SOURCE=200809L"
else
    C_FLAGS="-std=gnu99 -D_POSIX_C_SOURCE=200809L -O0 -Werror=implicit-function-declaration -Werror=incompatible-pointer-types"
fi

timeout 30s "${COMPILER}" b "${SOURCE}" "${WORK_DIR}/out.c"
"${C_COMPILER}" ${C_FLAGS} "${WORK_DIR}/out.c" -o "${WORK_DIR}/program"
timeout 10s "${WORK_DIR}/program" > "${WORK_DIR}/c.out"
diff -u "${EXPECTED}" "${WORK_DIR}/c.out"
timeout 30s "${COMPILER}" r "${SOURCE}" > "${WORK_DIR}/vm.out"
diff -u "${EXPECTED}" "${WORK_DIR}/vm.out"
echo 'PASS type interning, collisions, borrowed slices, and formatting [C+VM]'
echo "Artifacts: ${WORK_DIR}"
