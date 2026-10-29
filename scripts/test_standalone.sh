#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="${1:-${ABYSS_COMPILER:-${ROOT_DIR}/abyssc}}"
WORK_DIR=$(mktemp -d "${ROOT_DIR}/tmp/standalone_test.XXXXXX")
C_COMPILER="${ABYSS_C_COMPILER:-}"
if [[ -z "${C_COMPILER}" ]]; then
    if [[ -x "${ROOT_DIR}/tools/bin/tcc" ]]; then
        C_COMPILER="${ROOT_DIR}/tools/bin/tcc"
    elif command -v tcc >/dev/null 2>&1; then
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
cp "${COMPILER}" "${WORK_DIR}/abyssc"
cp "${ROOT_DIR}/tests/23_embedded_core.a" "${WORK_DIR}/main.a"
(
    cd "${WORK_DIR}"
    env -u ABYSS_SRC -u ABYSS_OUT ABYSS_PRELUDE=/nonexistent/prelude.c timeout 30s ./abyssc
    "${C_COMPILER}" ${C_FLAGS} out.c -o program
    timeout 5s ./program > actual.out
    diff -u "${ROOT_DIR}/tests/23_embedded_core.out" actual.out
    for TEST_FILE in "${ROOT_DIR}"/tests/[0-9]*.a; do
        if [[ "${TEST_FILE}" == */22_import_aliases.a ]]; then
            continue
        fi
        cp "${TEST_FILE}" main.a
        timeout 30s ./abyssc main.a relocated.c
        "${C_COMPILER}" ${C_FLAGS} relocated.c -o relocated
        timeout 5s ./relocated > relocated.out
        diff -u "${TEST_FILE%.a}.out" relocated.out
    done
    cp "${ROOT_DIR}/tests/23_embedded_core.a" main.a
    mkdir -p std/collections core runtime
    printf 'this is invalid Abyss\n' > std/collections/vec.a
    printf 'this is invalid Abyss\n' > core/prelude.a
    printf 'this is invalid C\n' > runtime/prelude.c
    timeout 30s ./abyssc main.a shadow.c
    cmp out.c shadow.c
    printf 'import Missing from "std/missing"\nmain :: fn() {}\n' > missing.a
    printf 'Missing :: fn() u32 { 99 }\n' > std/missing.a
    if timeout 30s ./abyssc missing.a missing.c > missing.log 2>&1; then
        echo 'FAIL standalone: missing embedded module loaded from disk'
        exit 1
    else
        STATUS=$?
    fi
    [[ ${STATUS} -eq 1 && ! -e missing.c ]]
    rg -q 'embedded module not found' missing.log
)
printf "  \033[1;32m✔ PASS\033[0m  %-36s \033[0;90m[Isolation]\033[0m\n" "standalone_compiler_isolation"
