#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="${1:-${ABYSS_COMPILER:-${ROOT_DIR}/abyssc}}"
WORK_DIR=$(mktemp -d "${ROOT_DIR}/tmp/cli_embedding_test.XXXXXX")
cp -r "${ROOT_DIR}/compiler" "${ROOT_DIR}/core" "${ROOT_DIR}/std" "${ROOT_DIR}/runtime" "${WORK_DIR}/"
cp "${ROOT_DIR}/abyss" "${WORK_DIR}/"
cp "${COMPILER}" "${WORK_DIR}/abyssc"
(
    cd "${WORK_DIR}"
    printf 'import write as print, embedding_probe from "std/io"\nmain :: fn() { print("automatic embedding\\n"), print(embedding_probe()), print(" "), print(true), print("\\n") }\n' > program.a
    printf '\nembedding_probe :: fn() i32 { 7 }\n' >> std/io.a
    printf 'invalid user program\n' > main.a
    printf '\n#define ABYSS_CLI_EMBED_TEST 1\n' >> runtime/prelude.c
    timeout 120s ./abyss b program.a
    rg -q '^#define ABYSS_CLI_EMBED_TEST 1$' tmp/out.c
    ./tmp/out > actual.out
    printf 'automatic embedding\n7 true\n' > expected.out
    diff -u expected.out actual.out
    BEFORE=$(stat -c '%y' abyssc)
    timeout 30s ./abyss r program.a > actual.out
    diff -u expected.out actual.out
    timeout 30s ./abyss br program.a > actual.out
    diff -u expected.out actual.out
    [[ "${BEFORE}" == "$(stat -c '%y' abyssc)" ]]
    printf '\n#error ABYSS_CLI_EMBED_FAILURE\n' >> runtime/prelude.c
    cp abyssc previous_compiler
    if timeout 120s ./abyss b program.a > failure.log 2>&1; then
        echo 'FAIL CLI embedding: invalid runtime was accepted'
        exit 1
    fi
    rg -q ABYSS_CLI_EMBED_FAILURE failure.log
    cmp abyssc previous_compiler
)
printf "  \033[1;32m✔ PASS\033[0m  %-36s \033[0;90m[CLI]\033[0m\n" "cli_embedding_and_cache"
