#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="$(realpath "$1")"
export ABYSS_DEV="${ABYSS_DEV:-${ROOT_DIR}}"
WORK_DIR=$(mktemp -d "${ROOT_DIR}/tmp/target_test.XXXXXX")
mkdir -p "${WORK_DIR}/platform" "${WORK_DIR}/sdk/pods/core"

cat > "${WORK_DIR}/main.a" <<'ABYSS'
marker :: @import("./platform").marker
Arch :: @import("core/target").Arch
Os :: @import("core/target").Os
Target :: @import("core/target").Target
target :: @import("core/target").target
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
grep -q '^int main(int argc, char \*\*argv) { abyss_process_set_args(argc, (uint8_t \*\*)argv);' "${WORK_DIR}/linux.c"
if grep -q '^int main(' "${WORK_DIR}/wasi.c"; then exit 1; fi

"${CC:-gcc}" -std=gnu99 -O0 "${WORK_DIR}/linux.c" -o "${WORK_DIR}/linux"
printf 'linux\nfalse\nfalse\n64\n' > "${WORK_DIR}/expected"
"${WORK_DIR}/linux" > "${WORK_DIR}/actual"
cmp "${WORK_DIR}/expected" "${WORK_DIR}/actual"

cat > "${WORK_DIR}/args.a" <<'ABYSS'
Env :: @import("std/env").Env

main :: fn() i32 {
    if Env.args_len() != 3 or Env.arg(-1) != (0 as &u8) or Env.arg(3) != (0 as &u8) { ret 1 }
    if not Str.of(Env.arg(1)).eq("first") { ret 2 }
    Env.shift_args(1)
    if Env.args_len() != 2 or not Str.of(Env.arg(0)).eq("first") { ret 3 }
    if not Str.of(Env.arg(1)).eq("second") { ret 4 }
    42
}
ABYSS
"${COMPILER}" b "${WORK_DIR}/args.a" "${WORK_DIR}/args.c"
"${CC:-gcc}" -std=gnu99 -O0 "${WORK_DIR}/args.c" -o "${WORK_DIR}/args"
status=0
"${WORK_DIR}/args" first second || status=$?
[[ "${status}" == 42 ]]

cat > "${WORK_DIR}/library.a" <<'ABYSS'
Env :: @import("std/env").Env

library_arg_count :: extern fn() i32 { Env.args_len() }

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

"${CC:-gcc}" -std=gnu99 -shared -fPIC -Wl,--no-undefined "${WORK_DIR}/library.c" -o "${WORK_DIR}/library.so"
python3 - "${WORK_DIR}/library.so" <<'PYTHON'
import ctypes
import sys

library = ctypes.CDLL(sys.argv[1])
assert library.library_arg_count() == 0
assert not hasattr(library, "abyss_process_arg_count")
PYTHON

printf 'main :: fn() { @import("core/foreign") }\n' > "${WORK_DIR}/reject.a"
printf 'Forbidden :: extern struct { value i32 }\n' > "${WORK_DIR}/sdk/pods/core/foreign.a"
if ABYSS_DEV="${WORK_DIR}/sdk" "${COMPILER}" b "${WORK_DIR}/reject.a" "${WORK_DIR}/reject.c" >"${WORK_DIR}/reject.log" 2>&1; then
    echo 'FAIL target checks: unused extern in core was accepted'
    exit 1
fi
grep -q 'core modules cannot declare extern' "${WORK_DIR}/reject.log"
mkdir -p "${WORK_DIR}/imports/deps/probe"
cat > "${WORK_DIR}/imports/deps/probe/probe.a" <<'ABYSS'
calls: i32 = 0

module_probe :: extern fn() i32 {
    calls = calls + 1
    calls
}
ABYSS
cat > "${WORK_DIR}/imports/main.a" <<ABYSS
main :: fn() {
    first := @import("probe")
    second: @typeof(first) = @import("alias")
    third: @typeof(first) = @import("./deps/probe/./probe.a")
    fourth: @typeof(first) = @import("${WORK_DIR}/imports/deps/probe/probe.a")
    print(first.module_probe())
    print(second.module_probe())
    print(third.module_probe())
    print(fourth.module_probe())
}
ABYSS
(
    cd "${WORK_DIR}/imports"
    "${COMPILER}" b --map alias=deps/probe/probe -I deps main.a modules.c
    [[ $(grep -c '^int32_t module_probe(void) {' modules.c) -eq 1 ]]
    "${CC:-gcc}" -std=gnu99 -O0 modules.c -o modules
    printf '1\n2\n3\n4\n' > expected
    ./modules > native.out
    "${COMPILER}" r --map alias=deps/probe/probe -I deps main.a > vm.out
    cmp expected native.out
    cmp expected vm.out
)
cat > "${WORK_DIR}/imports/relative.a" <<'ABYSS'
main :: fn() {
    print(@import("probe").module_probe())
    print(@import("./probe").module_probe())
}
ABYSS
cat > "${WORK_DIR}/imports/explicit.a" <<'ABYSS'
main :: fn() {
    print(@import("./deps/probe").module_probe())
    print(@import("./deps/probe.a").module_probe())
}
ABYSS
(
    cd "${WORK_DIR}/imports"
    if "${COMPILER}" b -I deps relative.a relative.c > relative.log 2>&1; then exit 1; fi
    grep -Fq 'cannot open module: ./probe' relative.log
    if "${COMPILER}" b explicit.a explicit.c > explicit.log 2>&1; then exit 1; fi
    grep -Fq 'cannot open module: ./deps/probe.a' explicit.log
)
printf '  ✔ PASS  %-36s [Targets]\n' 'platform_module_resolution_and_abi'
