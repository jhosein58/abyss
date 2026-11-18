#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT_DIR}"
COMPILER="${1:-${ABYSS_COMPILER:-${ROOT_DIR}/abyssc}}"
FIXTURES=tests/extern
mkdir -p "${ROOT_DIR}/tmp"
WORK_DIR=$(mktemp -d "${ROOT_DIR}/tmp/extern_test.XXXXXX")
export ABYSS_DEV="${ROOT_DIR}"
CC="${ABYSS_FFI_CC:-gcc}"

trap 'if [[ $? -ne 0 ]]; then printf "C ABI test artifacts: %s\n" "${WORK_DIR}" >&2; fi' EXIT

timeout 30s "${COMPILER}" b "${FIXTURES}/interop.a" "${WORK_DIR}/interop.c"
"${CC}" -std=c99 -D_POSIX_C_SOURCE=200809L -Werror=implicit-function-declaration -Werror=incompatible-pointer-types \
    -Dmain=abyss_generated_main -c "${WORK_DIR}/interop.c" -o "${WORK_DIR}/interop.o"
"${CC}" -std=c99 -Wall -Wextra -Wpedantic -Werror \
    "${WORK_DIR}/interop.o" "${FIXTURES}/fixture.c" "${FIXTURES}/native.c" -o "${WORK_DIR}/interop"
printf 'extern ABI verified\n' > "${WORK_DIR}/expected"
timeout 10s "${WORK_DIR}/interop" > "${WORK_DIR}/native.out"
diff -u "${WORK_DIR}/expected" "${WORK_DIR}/native.out"

if [[ "${ABYSS_TEST_WINE:-false}" != true ]]; then
    "${CC}" -std=c99 -Wall -Wextra -Wpedantic -Werror -fPIC -shared \
        "${FIXTURES}/fixture.c" -o "${WORK_DIR}/fixture.so"
    timeout 10s env LD_PRELOAD="${WORK_DIR}/fixture.so" "${COMPILER}" r "${FIXTURES}/interop.a" > "${WORK_DIR}/vm.out"
    diff -u "${WORK_DIR}/expected" "${WORK_DIR}/vm.out"
fi

count=0
while IFS=$'\t' read -r name line message source; do
    case_file="${WORK_DIR}/${name}.a"
    output="${WORK_DIR}/${name}.c"
    printf '%s\n' "${source//|/$'\n'}" > "${case_file}"
    for mode in b r; do
        log="${WORK_DIR}/${name}.${mode}.log"
        status=0
        timeout 10s "${COMPILER}" "${mode}" "${case_file}" "${output}" > "${log}" 2>&1 || status=$?
        if [[ ${status} -ne 1 || -e "${output}" ]] ||
            ! grep -Fq -- "${message}" "${log}" ||
            ! grep -Fq -- "${name}.a:${line}:" "${log}"; then
            printf 'FAIL extern/%s (%s, exit %s)\n' "${name}" "${mode}" "${status}" >&2
            cat "${log}" >&2
            exit 1
        fi
    done
    count=$((count + 1))
done < "${FIXTURES}/diagnostics.tsv"

if [[ "${ABYSS_TEST_WINE:-false}" == true ]]; then
    printf '  ✔ PASS  %-36s [C; %s compile diagnostics per backend; VM shared fixture skipped under Wine]\n' "extern_abi" "${count}"
    exit 0
fi

printf '%s\n' 'missing :: extern "abyss_no_such_test_symbol" fn()' 'main :: fn() { missing() }' > "${WORK_DIR}/missing.a"
status=0
timeout 10s "${COMPILER}" r "${WORK_DIR}/missing.a" > "${WORK_DIR}/missing.log" 2>&1 || status=$?
if [[ ${status} -ne 1 ]] || ! grep -Fq 'undefined external symbol: abyss_no_such_test_symbol' "${WORK_DIR}/missing.log"; then
    cat "${WORK_DIR}/missing.log" >&2
    exit 1
fi

printf '  ✔ PASS  %-36s [C+VM; %s compile diagnostics per backend and one VM runtime diagnostic]\n' "extern_abi" "${count}"
