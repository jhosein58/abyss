#!/usr/bin/env bash
set -euo pipefail

ROUNDS=${1:-5}

GREEN='\033[1;32m'
RED='\033[1;31m'
CYAN='\033[1;36m'
YELLOW='\033[1;33m'
RESET='\033[0m'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${ROOT_DIR}/tmp/bootstrap_test"
SEED_COMPILER="${ROOT_DIR}/abyssc"

GCC_FLAGS="-O3 -march=native -mtune=native -fomit-frame-pointer -funroll-loops"

echo -e "${CYAN}====================================================${RESET}"
echo -e "${CYAN}        ABYSS COMPILER BOOTSTRAP STABILITY TEST     ${RESET}"
echo -e "${CYAN}====================================================${RESET}"

if [ ! -f "${SEED_COMPILER}" ]; then
    echo -e "${RED}[ERROR] Seed compiler '${SEED_COMPILER}' not found!${RESET}"
    exit 1
fi

rm -rf "${WORK_DIR}"
mkdir -p "${WORK_DIR}"

CURRENT_COMPILER="${SEED_COMPILER}"
PREV_C_HASH=""
FIXED_POINT_REACHED=false

for i in $(seq 1 "${ROUNDS}"); do
    echo -e "\n${YELLOW}>>> [Round ${i}/${ROUNDS}] Compiling compiler with: $(basename "${CURRENT_COMPILER}")...${RESET}"

    if ! "${CURRENT_COMPILER}"; then
        echo -e "${RED}[FAIL] Compiler crashed at Generation ${i}!${RESET}"
        exit 1
    fi

    OUT_C="${ROOT_DIR}/tmp/out.c"
    if [ ! -f "${OUT_C}" ]; then
        echo -e "${RED}[FAIL] Generation ${i} did not produce '${OUT_C}'!${RESET}"
        exit 1
    fi

    GEN_C="${WORK_DIR}/gen_${i}.c"
    cp "${OUT_C}" "${GEN_C}"

    GEN_BIN="${WORK_DIR}/abyss_gen_${i}"
    echo -e "    Compiling C output via GCC..."
    if ! gcc ${GCC_FLAGS} "${GEN_C}" -o "${GEN_BIN}"; then
        echo -e "${RED}[FAIL] GCC compilation failed for Generation ${i}!${RESET}"
        exit 1
    fi

    CURR_C_HASH=$(sha256sum "${GEN_C}" | awk '{print $1}')
    echo -e "    Gen ${i} C Hash: ${CYAN}${CURR_C_HASH}${RESET}"

    if [ -n "${PREV_C_HASH}" ]; then
        if [ "${CURR_C_HASH}" == "${PREV_C_HASH}" ]; then
            echo -e "    ${GREEN}✔ C output is BIT-IDENTICAL to Generation $((i-1))!${RESET}"
            FIXED_POINT_REACHED=true
        else
            if [ "${i}" -gt 2 ]; then
                echo -e "    ${RED}✘ Divergence detected! Gen ${i} differs from Gen $((i-1))!${RESET}"
                echo -e "    Diff summary:"
                diff -u "${WORK_DIR}/gen_$((i-1)).c" "${GEN_C}" | head -n 20 || true
                exit 1
            else
                echo -e "    ${YELLOW}ℹ Transitioning from Seed to Self-Hosted code generation.${RESET}"
            fi
        fi
    fi

    PREV_C_HASH="${CURR_C_HASH}"
    CURRENT_COMPILER="${GEN_BIN}"
done

echo -e "\n${CYAN}====================================================${RESET}"
if [ "${FIXED_POINT_REACHED}" = true ]; then
    echo -e "${GREEN}SUCCESS: Fixed-point bootstrap verified across generations!${RESET}"
    echo -e "Artifacts kept safe inside: ${WORK_DIR}"
    echo -e "To promote the verified compiler, run:"
    echo -e "  ${YELLOW}cp ${WORK_DIR}/abyss_gen_2 ./abyssc${RESET}"
else
    echo -e "${YELLOW}WARNING: Completed rounds without reaching a stable fixed-point.${RESET}"
fi
echo -e "${CYAN}====================================================${RESET}"