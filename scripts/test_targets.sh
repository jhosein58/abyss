#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="$1"
WORK_DIR=$(mktemp -d "${ROOT_DIR}/tmp/target_test.XXXXXX")
mkdir -p "${WORK_DIR}/platform" "${WORK_DIR}/sdk/pods/core"

cat > "${WORK_DIR}/main.a" <<'ABYSS'
import .platform: marker
import core.target: Arch, Os, Target, target
main :: fn() {
    profile: Target = target
    print(marker())
    print(profile.os == Os.wasi)
    print(profile.arch == Arch.wasm32)
    print(profile.ptr_bits)
}
ABYSS
printf 'marker :: fn() Str { "fallback" }\n' > "${WORK_DIR}/platform/platform.a"
printf 'marker :: fn() Str { "linux" }\n' > "${WORK_DIR}/platform/platform.linux.a"
printf 'marker :: fn() Str { "linux-aarch64" }\n' > "${WORK_DIR}/platform/platform.linux.aarch64.a"
printf 'marker :: fn() Str { "wasi" }\n' > "${WORK_DIR}/platform/platform.wasi.a"
printf 'invalid :: "not a valid source\n' > "${WORK_DIR}/platform/platform.windows.a"

"${COMPILER}" b --target linux --arch x86_64 "${WORK_DIR}/main.a" "${WORK_DIR}/linux.c"
"${COMPILER}" b --target linux --arch aarch64 "${WORK_DIR}/main.a" "${WORK_DIR}/arm.c"
"${COMPILER}" b --target darwin "${WORK_DIR}/main.a" "${WORK_DIR}/darwin.c"
"${COMPILER}" b --target wasi "${WORK_DIR}/main.a" "${WORK_DIR}/wasi.c"
grep -q '"linux"' "${WORK_DIR}/linux.c"
grep -q '"linux-aarch64"' "${WORK_DIR}/arm.c"
grep -q '"fallback"' "${WORK_DIR}/darwin.c"
grep -q '"wasi"' "${WORK_DIR}/wasi.c"
grep -q 'import_module("wasi_snapshot_preview1")' "${WORK_DIR}/wasi.c"
grep -q '^void _start(' "${WORK_DIR}/wasi.c"
grep -q '^int main(int argc, char \*\*argv) { return (int)abyss_main(argc, (uint8_t \*\*)argv); }$' "${WORK_DIR}/linux.c"
if grep -q '^int main(' "${WORK_DIR}/wasi.c"; then exit 1; fi

"${CC:-gcc}" -std=gnu99 -O0 "${WORK_DIR}/linux.c" -o "${WORK_DIR}/linux"
printf 'linux\nfalse\nfalse\n64\n' > "${WORK_DIR}/expected"
"${WORK_DIR}/linux" > "${WORK_DIR}/actual"
cmp "${WORK_DIR}/expected" "${WORK_DIR}/actual"

cat > "${WORK_DIR}/library.a" <<'ABYSS'
copy :: extern fn(dest, from_ptr &u8, size u64) {
    @memcpy(dest, from_ptr, size)
}
ABYSS
"${COMPILER}" b --library "${WORK_DIR}/library.a" "${WORK_DIR}/library.c"
"${COMPILER}" b --target freestanding "${WORK_DIR}/library.a" "${WORK_DIR}/free.c"
for output in library free; do
    if grep -Eq '^int main\(|^void _start\(' "${WORK_DIR}/${output}.c"; then exit 1; fi
    "${CC:-gcc}" -std=gnu99 -ffreestanding -c "${WORK_DIR}/${output}.c" -o "${WORK_DIR}/${output}.o"
done

printf 'import core.foreign\nmain :: fn() {}\n' > "${WORK_DIR}/reject.a"
printf 'Forbidden :: extern struct { value i32 }\n' > "${WORK_DIR}/sdk/pods/core/foreign.a"
if ABYSS_DEV="${WORK_DIR}/sdk" "${COMPILER}" b "${WORK_DIR}/reject.a" "${WORK_DIR}/reject.c" >"${WORK_DIR}/reject.log" 2>&1; then
    echo 'FAIL target checks: unused extern in core was accepted'
    exit 1
fi
grep -q 'core modules cannot declare extern' "${WORK_DIR}/reject.log"
printf '  ✔ PASS  %-36s [Targets]\n' 'platform_module_resolution_and_abi'
