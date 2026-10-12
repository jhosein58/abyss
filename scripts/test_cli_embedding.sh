#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="${1:-${ABYSS_COMPILER:-${ROOT_DIR}/abyssc}}"
WORK_DIR=$(mktemp -d "${ROOT_DIR}/tmp/cli_embedding_test.XXXXXX")
cp -r "${ROOT_DIR}/compiler" "${ROOT_DIR}/core" "${ROOT_DIR}/std" "${ROOT_DIR}/runtime" "${WORK_DIR}/"
cp "${ROOT_DIR}/abyss" "${ROOT_DIR}/main.a" "${WORK_DIR}/"
cp "${COMPILER}" "${WORK_DIR}/abyssc"
mkdir -p "${WORK_DIR}/scripts"
cp "${ROOT_DIR}/scripts/embed_sources.py" "${WORK_DIR}/scripts/"
(
    cd "${WORK_DIR}"
    printf 'main :: () { print("automatic embedding\\n") }\n' > program.a
    printf '\n#define ABYSS_CLI_EMBED_TEST 1\n' >> runtime/prelude.c
    timeout 120s ./abyss b program.a
    rg -q '^#define ABYSS_CLI_EMBED_TEST 1$' tmp/out.c
    ./tmp/out > actual.out
    printf 'automatic embedding\n' > expected.out
    diff -u expected.out actual.out
    BEFORE=$(stat -c '%y' abyssc compiler/embedded.a)
    timeout 30s ./abyss r program.a > actual.out
    diff -u expected.out actual.out
    timeout 30s ./abyss br program.a > actual.out
    diff -u expected.out actual.out
    [[ "${BEFORE}" == "$(stat -c '%y' abyssc compiler/embedded.a)" ]]
    printf '\n#error ABYSS_CLI_EMBED_FAILURE\n' >> runtime/prelude.c
    cp abyssc previous_compiler
    if timeout 120s ./abyss b program.a > failure.log 2>&1; then
        echo 'FAIL CLI embedding: invalid runtime was accepted'
        exit 1
    fi
    rg -q ABYSS_CLI_EMBED_FAILURE failure.log
    cmp abyssc previous_compiler
)
echo 'PASS CLI automatic embedding, cached builds, and failed rebuild preservation'
echo "Artifacts: ${WORK_DIR}"
