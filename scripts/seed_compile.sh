#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="$(realpath "$1")"
mkdir -p "$(dirname "$2")"
OUTPUT="$(realpath "$2")"
cd "${ROOT_DIR}"
export ABYSS_DEV="${ROOT_DIR}"

if ! "${COMPILER}" --version 2>/dev/null | grep -qx 'abyssc target-aware'; then
    BRIDGE_REV=ec958cf93edcd0c877177f52286ee8cdace6f1d8
    BRIDGE_DIR=$(mktemp -d "$(dirname "${OUTPUT}")/seed-bridge.XXXXXX")
    trap 'rm -rf "${BRIDGE_DIR}"' EXIT
    if ! git archive "${BRIDGE_REV}" compiler pods scripts/seed_compile.sh | tar -x -C "${BRIDGE_DIR}"; then
        printf 'This seed predates target support. Use a target-aware compiler or a checkout containing %s.\n' "${BRIDGE_REV}" >&2
        exit 1
    fi
    (
        cd "${BRIDGE_DIR}"
        bash scripts/seed_compile.sh "${COMPILER}" "${BRIDGE_DIR}/bootstrap.c"
    )
    if command -v tcc >/dev/null 2>&1; then
        tcc -std=c99 -D_POSIX_C_SOURCE=200809L "${BRIDGE_DIR}/bootstrap.c" -o "${BRIDGE_DIR}/abyssc"
    else
        "${CC:-cc}" -std=gnu99 -D_POSIX_C_SOURCE=200809L -O0 "${BRIDGE_DIR}/bootstrap.c" -o "${BRIDGE_DIR}/abyssc"
    fi
    COMPILER="${BRIDGE_DIR}/abyssc"
fi

"${COMPILER}" "${ROOT_DIR}/compiler/main.a" "${OUTPUT}"
