#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="${1:-${ROOT_DIR}/abyssc}"
WORK_DIR=$(mktemp -d "${ROOT_DIR}/tmp/lexer_test.XXXXXX")
SOURCE="${ROOT_DIR}/tests/compiler/lexer.a"
EXPECTED="${SOURCE%.a}.out"
timeout 30s "${COMPILER}" b "${SOURCE}" "${WORK_DIR}/out.c"
gcc -std=gnu99 -D_POSIX_C_SOURCE=200809L -O0 -Werror=implicit-function-declaration -Werror=incompatible-pointer-types "${WORK_DIR}/out.c" -o "${WORK_DIR}/program"
timeout 10s "${WORK_DIR}/program" > "${WORK_DIR}/c.out"
diff -u "${EXPECTED}" "${WORK_DIR}/c.out"
timeout 30s "${COMPILER}" r "${SOURCE}" > "${WORK_DIR}/vm.out"
diff -u "${EXPECTED}" "${WORK_DIR}/vm.out"
echo 'PASS compiler lexer unit tests [C+VM]'
echo "Artifacts: ${WORK_DIR}"
