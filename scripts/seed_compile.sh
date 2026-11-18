#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="$1"
OUTPUT="$2"
BUILD_DIR="$(dirname "${OUTPUT}")"

if "${COMPILER}" --version 2>/dev/null | grep -qx 'abyssc target-aware'; then
    exec "${COMPILER}" "${ROOT_DIR}/compiler/main.a" "${OUTPUT}"
fi

case "$(uname -s)" in
    Linux) ABI=linux ;;
    Darwin) ABI=darwin ;;
    *) printf 'The legacy seed requires a Linux or Darwin host.\n' >&2; exit 1 ;;
esac

SDK="${BUILD_DIR}/seed-sdk"
mkdir -p "${SDK}/pods/std/sys"
ln -s "${ROOT_DIR}/pods/std/sys/sys.${ABI}.a" "${SDK}/pods/std/sys/sys.a"
ln -s "${ROOT_DIR}/pods/std/sys/abi.${ABI}.a" "${SDK}/pods/std/sys/abi.a"

source_literal() {
    awk -v declaration="$2 :: fn() Str {" '
        $0 == declaration { getline; sub(/^[[:blank:]]*`/, ""); print; body = 1; next }
        body && /^`$/ { exit }
        body { print }
    ' "$1"
}

SEED_HEADER="${SDK}/seed.h"
{
    printf '%s\n' '#define _GNU_SOURCE' '#include <stdint.h>' '#include <stdbool.h>' '#include <stddef.h>' '#include <stdio.h>' '#include <stdlib.h>' '#include <string.h>' '#ifdef __TINYC__' '#define __int128 long long' 'typedef uint16_t _Float16;' '#endif'
    printf '%s\n' '#define abyss_process_argc abyss_argc' '#define abyss_process_argv abyss_argv'
    source_literal "${ROOT_DIR}/compiler/platform_c.a" process_source
    source_literal "${ROOT_DIR}/compiler/platform_c.a" host_source
    source_literal "${ROOT_DIR}/compiler/vm/native.a" vm_ffi_source
} > "${SEED_HEADER}"

cd "${ROOT_DIR}"
ABYSS_DEV="${SDK}" ABYSS_PRELUDE="${SEED_HEADER}" "${COMPILER}" "${ROOT_DIR}/compiler/main.a" "${OUTPUT}"
