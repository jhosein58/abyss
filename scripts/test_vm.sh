#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="${1:-${ROOT_DIR}/abyssc}"
FILTER="${2:-}"
PASSED=0
FAILED=0
TOTAL=0

for TEST_FILE in $(find "${ROOT_DIR}/tests" -maxdepth 1 -name '*.a' | sort); do
    TEST_NAME=$(basename "${TEST_FILE}" .a)
    if [[ -n "${FILTER}" && "${TEST_NAME}" != *"${FILTER}"* ]]; then
        continue
    fi
    OUT_FILE="${ROOT_DIR}/tests/${TEST_NAME}.out"
    if [[ ! -f "${OUT_FILE}" ]]; then
        continue
    fi
    TOTAL=$((TOTAL + 1))
    ACTUAL=$(ABYSS_VM=1 "${COMPILER}" "${TEST_FILE}" 2>&1 || true)
    if diff -u "${OUT_FILE}" <(echo "${ACTUAL}") > /dev/null 2>&1; then
        echo "PASS ${TEST_NAME}"
        PASSED=$((PASSED + 1))
    else
        echo "FAIL ${TEST_NAME}"
        diff -u "${OUT_FILE}" <(echo "${ACTUAL}") || true
        FAILED=$((FAILED + 1))
    fi
done

echo ""
echo "${PASSED}/${TOTAL} passed (${FAILED} failed)"
[[ ${FAILED} -eq 0 ]]
