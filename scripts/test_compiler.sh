#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="${1:-${ABYSS_COMPILER:-${ROOT_DIR}/abyssc}}"
FILTER="${2:-}"
cd "${ROOT_DIR}"
python3 "${ROOT_DIR}/scripts/embed_sources.py" --check
mkdir -p tmp
WORK_DIR=$(mktemp -d "${ROOT_DIR}/tmp/compiler_test.XXXXXX")
PASSED=0
FAILED=0
TOTAL=0

while IFS= read -r TEST_FILE; do
    if [[ "${TEST_FILE}" != tests/errors/* && ! "${TEST_FILE}" =~ ^tests/[^/]+\.a$ ]]; then
        continue
    fi
    TEST_NAME=$(basename "${TEST_FILE}" .a)
    if [[ -n "${FILTER}" && "${TEST_FILE}" != *"${FILTER}"* ]]; then
        continue
    fi
    TOTAL=$((TOTAL + 1))
    CASE_DIR="${WORK_DIR}/${TEST_NAME}"
    mkdir -p "${CASE_DIR}"
    C_OUT="${CASE_DIR}/out.c"
    LOG="${CASE_DIR}/compile.log"
    if [[ "${TEST_FILE}" == tests/errors/* ]]; then
        if timeout 30s "${COMPILER}" "${TEST_FILE}" "${C_OUT}" > "${LOG}" 2>&1; then
            echo "FAIL ${TEST_NAME}: invalid program was accepted"
            FAILED=$((FAILED + 1))
            continue
        else
            STATUS=$?
        fi
        if [[ ${STATUS} -ne 1 || -e "${C_OUT}" ]]; then
            echo "FAIL ${TEST_NAME}: compiler crashed, timed out, or emitted C (status ${STATUS})"
            cat "${LOG}"
            FAILED=$((FAILED + 1))
            continue
        fi
        if [[ ! -f "${TEST_FILE%.a}.err" ]]; then
            echo "FAIL ${TEST_NAME}: missing expected diagnostic"
            FAILED=$((FAILED + 1))
            continue
        fi
        MATCHED=true
        while IFS= read -r PATTERN; do
            if ! grep -Fq -- "${PATTERN}" "${LOG}"; then
                MATCHED=false
            fi
        done < "${TEST_FILE%.a}.err"
        if [[ "${MATCHED}" != true ]]; then
            echo "FAIL ${TEST_NAME}: diagnostic mismatch"
            cat "${LOG}"
            FAILED=$((FAILED + 1))
            continue
        fi
    else
        if ! timeout 30s "${COMPILER}" "${TEST_FILE}" "${C_OUT}" > "${LOG}" 2>&1; then
            echo "FAIL ${TEST_NAME}: Abyss compilation"
            cat "${LOG}"
            FAILED=$((FAILED + 1))
            continue
        fi
        if [[ ! -s "${C_OUT}" ]]; then
            echo "FAIL ${TEST_NAME}: compiler emitted no C"
            FAILED=$((FAILED + 1))
            continue
        fi
        if ! gcc -std=gnu99 -D_POSIX_C_SOURCE=200809L -O0 -Werror=implicit-function-declaration -Werror=incompatible-pointer-types "${C_OUT}" -o "${CASE_DIR}/program" > "${CASE_DIR}/gcc.log" 2>&1; then
            echo "FAIL ${TEST_NAME}: C compilation"
            cat "${CASE_DIR}/gcc.log"
            FAILED=$((FAILED + 1))
            continue
        fi
        if ! timeout 5s "${CASE_DIR}/program" > "${CASE_DIR}/actual.out" 2> "${CASE_DIR}/runtime.log"; then
            echo "FAIL ${TEST_NAME}: runtime"
            cat "${CASE_DIR}/runtime.log"
            FAILED=$((FAILED + 1))
            continue
        fi
        if [[ ! -f "${TEST_FILE%.a}.out" ]]; then
            echo "FAIL ${TEST_NAME}: missing expected output"
            FAILED=$((FAILED + 1))
            continue
        fi
        if ! diff -u "${TEST_FILE%.a}.out" "${CASE_DIR}/actual.out" > "${CASE_DIR}/diff.log"; then
            echo "FAIL ${TEST_NAME}: output mismatch"
            cat "${CASE_DIR}/diff.log"
            FAILED=$((FAILED + 1))
            continue
        fi
    fi
    echo "PASS ${TEST_NAME}"
    PASSED=$((PASSED + 1))
done < <(rg --files tests -g '*.a' | sort)

if [[ ${TOTAL} -eq 0 ]]; then
    echo "No tests matched: ${FILTER}"
    exit 1
fi

echo "${PASSED} passed, ${FAILED} failed (${TOTAL} total)"
echo "Artifacts: ${WORK_DIR}"
[[ ${FAILED} -eq 0 ]]
if [[ -z "${FILTER}" ]]; then
    bash "${ROOT_DIR}/scripts/test_standalone.sh" "${COMPILER}"
    bash "${ROOT_DIR}/scripts/test_cli_embedding.sh" "${COMPILER}"
fi
