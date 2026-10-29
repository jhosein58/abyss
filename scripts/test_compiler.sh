#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="${1:-${ABYSS_COMPILER:-${ROOT_DIR}/abyssc}}"
[[ "${COMPILER}" = /* ]] || COMPILER="${PWD}/${COMPILER}"
FILTER="${2:-}"
FULL_TEST="${ABYSS_TEST_FULL:-false}"

if [[ "${FILTER}" == "--full" || "${FILTER}" == "all" ]]; then
    FULL_TEST=true
    FILTER=""
fi

cd "${ROOT_DIR}"
rm -f "${ROOT_DIR}/vm_dyn_io.txt"
trap 'rm -f "${ROOT_DIR}/vm_dyn_io.txt"' EXIT
mkdir -p tmp
WORK_DIR=$(mktemp -d "${ROOT_DIR}/tmp/compiler_test.XXXXXX")
PASSED=0
FAILED=0
TOTAL=0

if [ -t 1 ] && [ -z "${NO_COLOR:-}" ]; then
    GREEN='\033[1;32m'
    RED='\033[1;31m'
    YELLOW='\033[1;33m'
    CYAN='\033[1;36m'
    BOLD='\033[1m'
    DIM='\033[0;90m'
    RESET='\033[0m'
else
    GREEN=''
    RED=''
    YELLOW=''
    CYAN=''
    BOLD=''
    DIM=''
    RESET=''
fi

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

SUITE_START=$(date +%s%3N)

echo -e "${CYAN}========================================================================${RESET}"
echo -e "${BOLD}   ABYSS TEST RUNNER${RESET}  ${DIM}(Backend: ${C_COMPILER} | Target: $(basename "${COMPILER}"))${RESET}"
echo -e "${CYAN}========================================================================${RESET}"

if [[ "${FULL_TEST}" == true && -z "${FILTER}" ]]; then
    FP_START=$(date +%s%3N)
    SELF_HOST_DIR="${WORK_DIR}/self_host"
    mkdir -p "${SELF_HOST_DIR}"
    if ! timeout 60s "${COMPILER}" b "${ROOT_DIR}/compiler/main.a" "${SELF_HOST_DIR}/stage1.c" > "${SELF_HOST_DIR}/stage1_compile.log" 2>&1; then
        printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}stage 1 compilation failed${RESET}\n" "fixed_point_verification"
        cat "${SELF_HOST_DIR}/stage1_compile.log"
        exit 1
    fi
    if ! "${C_COMPILER}" ${C_FLAGS} "${SELF_HOST_DIR}/stage1.c" -o "${SELF_HOST_DIR}/stage1_bin" > "${SELF_HOST_DIR}/stage1_c.log" 2>&1; then
        printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}stage 1 C build failed${RESET}\n" "fixed_point_verification"
        cat "${SELF_HOST_DIR}/stage1_c.log"
        exit 1
    fi
    if ! timeout 60s "${SELF_HOST_DIR}/stage1_bin" b "${ROOT_DIR}/compiler/main.a" "${SELF_HOST_DIR}/stage2.c" > "${SELF_HOST_DIR}/stage2_compile.log" 2>&1; then
        printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}stage 2 self-hosting failed${RESET}\n" "fixed_point_verification"
        cat "${SELF_HOST_DIR}/stage2_compile.log"
        exit 1
    fi
    if ! cmp -s "${SELF_HOST_DIR}/stage1.c" "${SELF_HOST_DIR}/stage2.c"; then
        printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}stage 1 and 2 outputs diverge${RESET}\n" "fixed_point_verification"
        diff -u "${SELF_HOST_DIR}/stage1.c" "${SELF_HOST_DIR}/stage2.c" | head -n 30
        exit 1
    fi
    FP_END=$(date +%s%3N)
    FP_MS=$((FP_END - FP_START))
    printf "  ${GREEN}✔ PASS${RESET}  %-36s ${DIM}[Fixed-Point]${RESET} ${DIM}(%sms)${RESET}\n" "compiler_self_hosting" "${FP_MS}"
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
    CASE_START=$(date +%s%3N)

    if [[ "${TEST_FILE}" == tests/errors/* ]]; then
        C_OUT="${CASE_DIR}/out.c"
        C_LOG="${CASE_DIR}/c_compile.log"
        if timeout 30s "${COMPILER}" b "${TEST_FILE}" "${C_OUT}" > "${C_LOG}" 2>&1; then
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}[C] invalid program accepted${RESET}\n" "${TEST_NAME}"
            FAILED=$((FAILED + 1))
            continue
        else
            STATUS=$?
        fi
        if [[ ${STATUS} -ne 1 || -e "${C_OUT}" ]]; then
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}[C] crash or emitted C (status ${STATUS})${RESET}\n" "${TEST_NAME}"
            cat "${C_LOG}"
            FAILED=$((FAILED + 1))
            continue
        fi
        if [[ ! -f "${TEST_FILE%.a}.err" ]]; then
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}[C] missing expected diagnostic${RESET}\n" "${TEST_NAME}"
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
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}[C] diagnostic mismatch${RESET}\n" "${TEST_NAME}"
            cat "${C_LOG}"
            FAILED=$((FAILED + 1))
            continue
        fi

        VM_LOG="${CASE_DIR}/vm_compile.log"
        if timeout 30s "${COMPILER}" r "${TEST_FILE}" > "${VM_LOG}" 2>&1; then
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}[VM] invalid program accepted${RESET}\n" "${TEST_NAME}"
            FAILED=$((FAILED + 1))
            continue
        else
            VM_STATUS=$?
        fi
        if [[ ${VM_STATUS} -ne 1 ]]; then
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}[VM] crash or timeout (status ${VM_STATUS})${RESET}\n" "${TEST_NAME}"
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
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}[VM] diagnostic mismatch${RESET}\n" "${TEST_NAME}"
            cat "${VM_LOG}"
            FAILED=$((FAILED + 1))
            continue
        fi
        CASE_END=$(date +%s%3N)
        CASE_MS=$((CASE_END - CASE_START))
        printf "  ${GREEN}✔ PASS${RESET}  %-36s ${DIM}[Errors]${RESET} ${DIM}(%sms)${RESET}\n" "${TEST_NAME}" "${CASE_MS}"
    else
        C_OUT="${CASE_DIR}/out.c"
        C_LOG="${CASE_DIR}/c_compile.log"
        if ! timeout 30s "${COMPILER}" b "${TEST_FILE}" "${C_OUT}" > "${C_LOG}" 2>&1; then
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}[C] Abyss compilation error${RESET}\n" "${TEST_NAME}"
            cat "${C_LOG}"
            FAILED=$((FAILED + 1))
            continue
        fi
        if [[ ! -s "${C_OUT}" ]]; then
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}[C] compiler emitted empty C${RESET}\n" "${TEST_NAME}"
            FAILED=$((FAILED + 1))
            continue
        fi
        if ! "${C_COMPILER}" ${C_FLAGS} "${C_OUT}" -o "${CASE_DIR}/c_program" > "${CASE_DIR}/c_build.log" 2>&1; then
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}[C] C backend build error${RESET}\n" "${TEST_NAME}"
            cat "${CASE_DIR}/c_build.log"
            FAILED=$((FAILED + 1))
            continue
        fi
        if ! timeout 5s "${CASE_DIR}/c_program" > "${CASE_DIR}/c_actual.out" 2> "${CASE_DIR}/c_runtime.log"; then
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}[C] runtime error${RESET}\n" "${TEST_NAME}"
            cat "${CASE_DIR}/c_runtime.log"
            FAILED=$((FAILED + 1))
            continue
        fi
        if [[ ! -f "${TEST_FILE%.a}.out" ]]; then
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}missing .out snapshot${RESET}\n" "${TEST_NAME}"
            FAILED=$((FAILED + 1))
            continue
        fi
        if ! diff --strip-trailing-cr -u "${TEST_FILE%.a}.out" "${CASE_DIR}/c_actual.out" > "${CASE_DIR}/c_diff.log"; then
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}[C] output mismatch${RESET}\n" "${TEST_NAME}"
            cat "${CASE_DIR}/c_diff.log"
            FAILED=$((FAILED + 1))
            continue
        fi

        VM_OUT="${CASE_DIR}/vm_actual.out"
        VM_LOG="${CASE_DIR}/vm_runtime.log"
        if ! timeout 10s "${COMPILER}" r "${TEST_FILE}" > "${VM_OUT}" 2> "${VM_LOG}"; then
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}[VM] execution failed${RESET}\n" "${TEST_NAME}"
            cat "${VM_LOG}"
            FAILED=$((FAILED + 1))
            continue
        fi
        if ! diff --strip-trailing-cr -u "${TEST_FILE%.a}.out" "${VM_OUT}" > "${CASE_DIR}/vm_diff.log"; then
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}[VM] output mismatch${RESET}\n" "${TEST_NAME}"
            cat "${CASE_DIR}/vm_diff.log"
            FAILED=$((FAILED + 1))
            continue
        fi
        if ! diff --strip-trailing-cr -u "${CASE_DIR}/c_actual.out" "${VM_OUT}" > "${CASE_DIR}/c_vm_diff.log"; then
            printf "  ${RED}✘ FAIL${RESET}  %-36s ${RED}C and VM output diverge${RESET}\n" "${TEST_NAME}"
            cat "${CASE_DIR}/c_vm_diff.log"
            FAILED=$((FAILED + 1))
            continue
        fi
        CASE_END=$(date +%s%3N)
        CASE_MS=$((CASE_END - CASE_START))
        printf "  ${GREEN}✔ PASS${RESET}  %-36s ${DIM}[C+VM]${RESET} ${DIM}(%sms)${RESET}\n" "${TEST_NAME}" "${CASE_MS}"
    fi
    PASSED=$((PASSED + 1))
done < <(rg --files tests -g '*.a' | sort)

if [[ ${TOTAL} -eq 0 ]]; then
    echo -e "${YELLOW}No tests matched: ${FILTER}${RESET}"
    exit 1
fi

if [[ -z "${FILTER}" ]]; then
    bash "${ROOT_DIR}/scripts/test_type_storage.sh" "${COMPILER}"
    bash "${ROOT_DIR}/scripts/test_lexer.sh" "${COMPILER}"
    if [[ "${FULL_TEST}" == true ]]; then
        bash "${ROOT_DIR}/scripts/test_reflection_codegen.sh" "${COMPILER}"
        bash "${ROOT_DIR}/scripts/test_standalone.sh" "${COMPILER}"
        bash "${ROOT_DIR}/scripts/test_cli_embedding.sh" "${COMPILER}"
    fi
fi

SUITE_END=$(date +%s%3N)
TOTAL_MS=$((SUITE_END - SUITE_START))
TOTAL_SEC=$(awk "BEGIN {printf \"%.2f\", ${TOTAL_MS}/1000}")

echo -e "\n${CYAN}========================================================================${RESET}"
echo -e "${BOLD}   TEST RUNNER SUMMARY${RESET}"
echo -e "${CYAN}========================================================================${RESET}"
echo -e "  Backend:    ${CYAN}${C_COMPILER}${RESET} ${DIM}(${C_FLAGS})${RESET}"
echo -e "  Duration:   ${BOLD}${TOTAL_SEC}s${RESET} ${DIM}(${TOTAL_MS}ms)${RESET}"
if [[ ${FAILED} -eq 0 ]]; then
    echo -e "  Status:     ${GREEN}✔ ALL ${PASSED} TESTS PASSED${RESET} ${DIM}(0 failures)${RESET}"
else
    echo -e "  Status:     ${RED}✘ ${FAILED} FAILED${RESET}, ${GREEN}${PASSED} passed${RESET} (${TOTAL} total)"
fi
echo -e "  Artifacts:  ${DIM}${WORK_DIR}${RESET}"
echo -e "${CYAN}========================================================================${RESET}"

[[ ${FAILED} -eq 0 ]]
