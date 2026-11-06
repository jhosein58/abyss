#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="${1:-${ROOT_DIR}/abyssc}"
export ABYSS_DEV="${ROOT_DIR}"
export ABYSS_COMPILER="${COMPILER}"

SONAR_BIN="${ROOT_DIR}/sonar"
if [ ! -x "${SONAR_BIN}" ]; then
    "${COMPILER}" --map json="${ROOT_DIR}/pods/json/json" b "${ROOT_DIR}/tools/sonar/main.a" "${ROOT_DIR}/tmp/sonar_bs.c"
    gcc -std=gnu99 -D_POSIX_C_SOURCE=200809L -O2 -fomit-frame-pointer -Wno-error=incompatible-pointer-types -Wno-implicit-function-declaration "${ROOT_DIR}/tmp/sonar_bs.c" -o "${SONAR_BIN}"
fi

run_pod_test() {
    local POD_NAME="$1"
    local POD_PATH="pods/${POD_NAME}"
    local T_START=$(date +%s%3N)
    local LOG_FILE="${ROOT_DIR}/tmp/test_pod_${POD_NAME}.log"
    mkdir -p "${ROOT_DIR}/tmp"

    if "${SONAR_BIN}" test "${POD_PATH}" > "${LOG_FILE}" 2>&1; then
        local T_END=$(date +%s%3N)
        local T_MS=$((T_END - T_START))
        printf "  \033[1;32m✔ PASS\033[0m  %-36s \033[0;90m[Pod: %s]\033[0m \033[0;90m(%sms)\033[0m\n" "${POD_NAME}_unit_tests" "${POD_NAME}" "${T_MS}"
    else
        printf "  \033[1;31m✘ FAIL\033[0m  %-36s \033[1;31m[Pod: %s]\033[0m\n" "${POD_NAME}_unit_tests" "${POD_NAME}"
        cat "${LOG_FILE}"
        exit 1
    fi
}

run_pod_test "core"
run_pod_test "std"
run_pod_test "test"
run_pod_test "json"
run_pod_test "simd"
