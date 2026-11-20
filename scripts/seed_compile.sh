#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="$(realpath "$1")"
mkdir -p "$(dirname "$2")"
OUTPUT="$(realpath "$2")"
cd "${ROOT_DIR}"
export ABYSS_DEV="${ROOT_DIR}"

WORK_DIR=$(mktemp -d "$(dirname "${OUTPUT}")/seed-bridge.XXXXXX")
trap 'rm -rf "${WORK_DIR}"' EXIT

build_bridge() {
    local revision="$1"
    local label="$2"
    local bridge_dir="${WORK_DIR}/${label}"
    mkdir -p "${bridge_dir}"
    if ! git archive "${revision}" compiler pods scripts/seed_compile.sh | tar -x -C "${bridge_dir}"; then
        printf 'This seed lacks %s support. Use a newer compiler or a checkout containing %s.\n' "${label}" "${revision}" >&2
        exit 1
    fi
    bash "${bridge_dir}/scripts/seed_compile.sh" "${COMPILER}" "${bridge_dir}/bootstrap.c"
    if command -v tcc >/dev/null 2>&1; then
        tcc -std=c99 -D_POSIX_C_SOURCE=200809L "${bridge_dir}/bootstrap.c" -o "${bridge_dir}/abyssc"
    else
        "${CC:-cc}" -std=gnu99 -D_POSIX_C_SOURCE=200809L -O0 "${bridge_dir}/bootstrap.c" -o "${bridge_dir}/abyssc"
    fi
    COMPILER="${bridge_dir}/abyssc"
}

if ! "${COMPILER}" --version 2>/dev/null | grep -qx 'abyssc target-aware'; then
    build_bridge ec958cf93edcd0c877177f52286ee8cdace6f1d8 target
fi

printf '%s\n' 'main :: fn() i32 { @import("core/target").target.ptr_bits as i32 }' > "${WORK_DIR}/probe.a"
if "${COMPILER}" "${WORK_DIR}/probe.a" "${WORK_DIR}/probe.c" > "${WORK_DIR}/probe.log" 2>&1; then
    "${COMPILER}" "${ROOT_DIR}/compiler/main.a" "${OUTPUT}"
    exit
fi

STAGE="${WORK_DIR}/legacy"
mkdir -p "${STAGE}/pods"
cp -R compiler "${STAGE}/compiler"
cp -R pods/core pods/std "${STAGE}/pods/"
while IFS= read -r -d '' source; do
    awk -f "${ROOT_DIR}/scripts/seed_imports.awk" "${source}" > "${source}.seed"
    mv "${source}.seed" "${source}"
done < <(find "${STAGE}/compiler" "${STAGE}/pods" -name '*.a' -print0)

printf '%s\n' 'main :: fn() i32 { @source_location().line as i32 }' > "${WORK_DIR}/probe.a"
if ! (cd "${STAGE}" && ABYSS_DEV="${STAGE}" "${COMPILER}" "${WORK_DIR}/probe.a" "${WORK_DIR}/probe.c") > "${WORK_DIR}/probe.log" 2>&1 &&
    grep -aFq "unknown builtin function '@source_location'" "${WORK_DIR}/probe.log"; then
    build_bridge 1c10ec868b91068e42dad421c9fccfbc7f0cb0ed source-location
fi

cd "${STAGE}"
ABYSS_DEV="${STAGE}" "${COMPILER}" compiler/main.a "${OUTPUT}"
