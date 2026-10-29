#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="${1:-${ROOT_DIR}/abyssc}"
WORK_DIR=$(mktemp -d "${ROOT_DIR}/tmp/reflection_codegen.XXXXXX")
cat > "${WORK_DIR}/format.a" <<'ABYSS'
import fmt from "std/fmt"
import Arena from "std/allocator"
import print from "std/io"
main :: fn() {
    arena := Arena.new(64)
    values: [3]u16 = .[7, 8, 9]
    text := fmt(&arena, "__FMT_STATIC_OPEN__ {} {} __FMT_STATIC_CLOSE__", .{values, true})
    print(text)
    arena.destroy()
}
ABYSS
"${COMPILER}" b "${WORK_DIR}/format.a" "${WORK_DIR}/out.c"
python3 - "${ROOT_DIR}" "${WORK_DIR}/out.c" <<'PY'
from pathlib import Path
import re
import sys

root = Path(sys.argv[1])
source = Path(sys.argv[2]).read_text()
runtime = (root / 'runtime/prelude.c').read_text()
assert source.startswith(runtime), 'generated C must begin with the embedded runtime'
generated = source[len(runtime):]
functions = re.findall(r'^([^\n;{}]*\bsym_\d+\([^\n]*\)) \{\n(.*?)^\}', generated, re.M | re.S)
formats = [(signature, body) for signature, body in functions if '__FMT_STATIC_OPEN__' in body]
assert len(formats) == 1, 'expected one specialization for the format literal'
signature, body = formats[0]
assert not re.search(r'\b(?:while|for|malloc|realloc)\s*\(', body), 'format specialization contains runtime traversal or dispatch'
assert len(signature[signature.index('(') + 1:-1].split(',')) == 2, 'compile-time format string leaked into runtime parameters'
assert '__FMT_STATIC_OPEN__ {} {}' not in generated, 'format template was retained at runtime'
converters = [re.search(r'\bsym_\d+', signature).group() for signature, body in functions if '% 10' in body]
assert len(converters) == 1, 'expected one unsigned numeric conversion helper'
assert len(re.findall(r'\b' + converters[0] + r'\(', body)) == 3, 'fixed array elements must convert through three direct calls'
assert '[format error]' not in generated, 'format parser was emitted at runtime'
assert 'TypeInfo' not in generated and 'FieldInfo' not in generated, 'reflection metadata leaked into runtime code'
for path in (root / 'core').rglob('*.a'):
    text = path.read_text()
    assert 'std/' not in text, f'{path} depends on std'
    assert not re.search(r'\b(?:malloc|realloc|free|print_bytes|print_char)\b', text), f'{path} depends on allocation or I/O'
PY
gcc -std=gnu99 -D_POSIX_C_SOURCE=200809L -O0 -Werror=implicit-function-declaration -Werror=incompatible-pointer-types "${WORK_DIR}/out.c" -o "${WORK_DIR}/program"
"${WORK_DIR}/program" > "${WORK_DIR}/actual.out"
printf '__FMT_STATIC_OPEN__ [\n    7,\n    8,\n    9,\n] true __FMT_STATIC_CLOSE__\n' > "${WORK_DIR}/expected.out"
cmp "${WORK_DIR}/expected.out" "${WORK_DIR}/actual.out"
printf "  \033[1;32m✔ PASS\033[0m  %-36s \033[0;90m[Comptime]\033[0m\n" "format_and_reflection_metadata"
cat > "${WORK_DIR}/wide.a" <<'ABYSS'
import print from "std/io"
main :: fn() {
    maximum := ~(0 as u128)
    minimum := ((1 as u128) << 127) as i128
    print(maximum)
    print(minimum)
}
ABYSS
"${COMPILER}" b "${WORK_DIR}/wide.a" "${WORK_DIR}/wide.c"
gcc -std=gnu99 -D_POSIX_C_SOURCE=200809L -O0 -Werror=implicit-function-declaration "${WORK_DIR}/wide.c" -o "${WORK_DIR}/wide"
"${WORK_DIR}/wide" > "${WORK_DIR}/wide.out"
printf '340282366920938463463374607431768211455\n-170141183460469231731687303715884105728\n' > "${WORK_DIR}/wide.expected"
cmp "${WORK_DIR}/wide.expected" "${WORK_DIR}/wide.out"
printf "  \033[1;32m✔ PASS\033[0m  %-36s \033[0;90m[Precision]\033[0m\n" "wide_128bit_integer_preservation"
cat > "${WORK_DIR}/balance.a" <<'ABYSS'
import print, printf from "std/io"
import fall from "std/sys"
allocation_balance :: fn() u64
main :: fn() {
    i: u32 = 0
    while i < 1000 {
        print(i)
        printf("v={}\n", .{i})
        if allocation_balance() != 0 { fall() }
        i = i + 1
    }
}
ABYSS
cat > "${WORK_DIR}/balance.c" <<'C'
#include <stddef.h>
#include <stdint.h>
void *__real_malloc(size_t size);
void __real_free(void *ptr);
static uint64_t active_allocations;
void *__wrap_malloc(size_t size) {
    void *ptr = __real_malloc(size);
    if (ptr) active_allocations++;
    return ptr;
}
void __wrap_free(void *ptr) {
    if (ptr) active_allocations--;
    __real_free(ptr);
}
uint64_t allocation_balance(void) { return active_allocations; }
C
printf '#include <stdint.h>\nuint64_t allocation_balance(void);\n' > "${WORK_DIR}/balance.h"
"${COMPILER}" b "${WORK_DIR}/balance.a" "${WORK_DIR}/balance_program.c"
gcc -std=gnu99 -D_POSIX_C_SOURCE=200809L -O0 -include "${WORK_DIR}/balance.h" "${WORK_DIR}/balance_program.c" "${WORK_DIR}/balance.c" -Wl,--wrap=malloc,--wrap=free -o "${WORK_DIR}/balance"
"${WORK_DIR}/balance" > "${WORK_DIR}/balance.out"
printf "  \033[1;32m✔ PASS\033[0m  %-36s \033[0;90m[Memory]\033[0m\n" "temporary_arena_allocation_balance"
