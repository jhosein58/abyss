#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="${1:-${ABYSS_COMPILER:-${ROOT_DIR}/abyssc}}"
[[ "${COMPILER}" = /* ]] || COMPILER="${PWD}/${COMPILER}"
FILTER="${2:-}"
cd "${ROOT_DIR}"
python3 "${ROOT_DIR}/scripts/embed_sources.py" --check
mkdir -p tmp
WORK_DIR=$(mktemp -d "${ROOT_DIR}/tmp/compiler_test.XXXXXX")
PASSED=0
FAILED=0
TOTAL=0

if [[ -z "${FILTER}" ]]; then
    echo "Verifying compiler self-hosting fixed-point..."
    SELF_HOST_DIR="${WORK_DIR}/self_host"
    mkdir -p "${SELF_HOST_DIR}"
    if ! timeout 60s "${COMPILER}" b "${ROOT_DIR}/main.a" "${SELF_HOST_DIR}/stage1.c" > "${SELF_HOST_DIR}/stage1_compile.log" 2>&1; then
        echo "FAIL self-hosting: generation 0 compilation failed"
        cat "${SELF_HOST_DIR}/stage1_compile.log"
        exit 1
    fi
    if ! gcc -std=gnu99 -D_POSIX_C_SOURCE=200809L -O2 "${SELF_HOST_DIR}/stage1.c" -o "${SELF_HOST_DIR}/stage1_bin" > "${SELF_HOST_DIR}/gcc.log" 2>&1; then
        echo "FAIL self-hosting: gcc build of stage1 failed"
        cat "${SELF_HOST_DIR}/gcc.log"
        exit 1
    fi
    if ! timeout 60s "${SELF_HOST_DIR}/stage1_bin" b "${ROOT_DIR}/main.a" "${SELF_HOST_DIR}/stage2.c" > "${SELF_HOST_DIR}/stage2_compile.log" 2>&1; then
        echo "FAIL self-hosting: generation 1 compilation failed"
        cat "${SELF_HOST_DIR}/stage2_compile.log"
        exit 1
    fi
    if ! cmp -s "${SELF_HOST_DIR}/stage1.c" "${SELF_HOST_DIR}/stage2.c"; then
        echo "FAIL self-hosting: output differs from previous generation"
        diff -u "${SELF_HOST_DIR}/stage1.c" "${SELF_HOST_DIR}/stage2.c" | head -n 30
        exit 1
    fi
    echo "PASS self-hosting fixed-point verification"
fi

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

    if [[ "${TEST_FILE}" == tests/errors/* ]]; then
        C_OUT="${CASE_DIR}/out.c"
        C_LOG="${CASE_DIR}/c_compile.log"
        if timeout 30s "${COMPILER}" b "${TEST_FILE}" "${C_OUT}" > "${C_LOG}" 2>&1; then
            echo "FAIL ${TEST_NAME}: [C] invalid program was accepted"
            FAILED=$((FAILED + 1))
            continue
        else
            STATUS=$?
        fi
        if [[ ${STATUS} -ne 1 || -e "${C_OUT}" ]]; then
            echo "FAIL ${TEST_NAME}: [C] compiler crashed, timed out, or emitted C (status ${STATUS})"
            cat "${C_LOG}"
            FAILED=$((FAILED + 1))
            continue
        fi
        if [[ ! -f "${TEST_FILE%.a}.err" ]]; then
            echo "FAIL ${TEST_NAME}: [C] missing expected diagnostic"
            FAILED=$((FAILED + 1))
            continue
        fi
        MATCHED=true
        while IFS= read -r PATTERN; do
            if ! grep -Fq -- "${PATTERN}" "${C_LOG}"; then
                MATCHED=false
            fi
        done < "${TEST_FILE%.a}.err"
        if [[ "${MATCHED}" != true ]]; then
            echo "FAIL ${TEST_NAME}: [C] diagnostic mismatch"
            cat "${C_LOG}"
            FAILED=$((FAILED + 1))
            continue
        fi

        VM_LOG="${CASE_DIR}/vm_compile.log"
        if timeout 30s "${COMPILER}" r "${TEST_FILE}" > "${VM_LOG}" 2>&1; then
            echo "FAIL ${TEST_NAME}: [VM] invalid program was accepted"
            FAILED=$((FAILED + 1))
            continue
        else
            VM_STATUS=$?
        fi
        if [[ ${VM_STATUS} -ne 1 ]]; then
            echo "FAIL ${TEST_NAME}: [VM] compiler crashed or timed out (status ${VM_STATUS})"
            cat "${VM_LOG}"
            FAILED=$((FAILED + 1))
            continue
        fi
        VM_MATCHED=true
        while IFS= read -r PATTERN; do
            if ! grep -Fq -- "${PATTERN}" "${VM_LOG}"; then
                VM_MATCHED=false
            fi
        done < "${TEST_FILE%.a}.err"
        if [[ "${VM_MATCHED}" != true ]]; then
            echo "FAIL ${TEST_NAME}: [VM] diagnostic mismatch"
            cat "${VM_LOG}"
            FAILED=$((FAILED + 1))
            continue
        fi
    else
        C_OUT="${CASE_DIR}/out.c"
        C_LOG="${CASE_DIR}/c_compile.log"
        if ! timeout 30s "${COMPILER}" b "${TEST_FILE}" "${C_OUT}" > "${C_LOG}" 2>&1; then
            echo "FAIL ${TEST_NAME}: [C] Abyss compilation"
            cat "${C_LOG}"
            FAILED=$((FAILED + 1))
            continue
        fi
        if [[ ! -s "${C_OUT}" ]]; then
            echo "FAIL ${TEST_NAME}: [C] compiler emitted no C"
            FAILED=$((FAILED + 1))
            continue
        fi
        if ! gcc -std=gnu99 -D_POSIX_C_SOURCE=200809L -O0 -Werror=implicit-function-declaration -Werror=incompatible-pointer-types "${C_OUT}" -o "${CASE_DIR}/c_program" > "${CASE_DIR}/gcc.log" 2>&1; then
            echo "FAIL ${TEST_NAME}: [C] C compilation"
            cat "${CASE_DIR}/gcc.log"
            FAILED=$((FAILED + 1))
            continue
        fi
        if ! timeout 5s "${CASE_DIR}/c_program" > "${CASE_DIR}/c_actual.out" 2> "${CASE_DIR}/c_runtime.log"; then
            echo "FAIL ${TEST_NAME}: [C] runtime"
            cat "${CASE_DIR}/c_runtime.log"
            FAILED=$((FAILED + 1))
            continue
        fi
        if [[ ! -f "${TEST_FILE%.a}.out" ]]; then
            echo "FAIL ${TEST_NAME}: missing expected output file"
            FAILED=$((FAILED + 1))
            continue
        fi
        if ! diff --strip-trailing-cr -u "${TEST_FILE%.a}.out" "${CASE_DIR}/c_actual.out" > "${CASE_DIR}/c_diff.log"; then
            echo "FAIL ${TEST_NAME}: [C] output mismatch with expected"
            cat "${CASE_DIR}/c_diff.log"
            FAILED=$((FAILED + 1))
            continue
        fi

        VM_OUT="${CASE_DIR}/vm_actual.out"
        VM_LOG="${CASE_DIR}/vm_runtime.log"
        if ! timeout 10s "${COMPILER}" r "${TEST_FILE}" > "${VM_OUT}" 2> "${VM_LOG}"; then
            echo "FAIL ${TEST_NAME}: [VM] execution failed"
            cat "${VM_LOG}"
            FAILED=$((FAILED + 1))
            continue
        fi
        if ! diff --strip-trailing-cr -u "${TEST_FILE%.a}.out" "${VM_OUT}" > "${CASE_DIR}/vm_diff.log"; then
            echo "FAIL ${TEST_NAME}: [VM] output mismatch with expected"
            cat "${CASE_DIR}/vm_diff.log"
            FAILED=$((FAILED + 1))
            continue
        fi
        if ! diff --strip-trailing-cr -u "${CASE_DIR}/c_actual.out" "${VM_OUT}" > "${CASE_DIR}/c_vm_diff.log"; then
            echo "FAIL ${TEST_NAME}: C and VM outputs diverge"
            cat "${CASE_DIR}/c_vm_diff.log"
            FAILED=$((FAILED + 1))
            continue
        fi
    fi
    echo "PASS ${TEST_NAME} [C+VM]"
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
