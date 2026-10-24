#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPILER="${1:-${ROOT_DIR}/abyssc}"
WORK_DIR=$(mktemp -d "${ROOT_DIR}/tmp/reflection_codegen.XXXXXX")
"${COMPILER}" b "${ROOT_DIR}/tests/51_reflection_zero_cost.a" "${WORK_DIR}/out.c"
python3 - "${ROOT_DIR}" "${WORK_DIR}/out.c" <<'PY'
from pathlib import Path
import re
import sys

root = Path(sys.argv[1])
source = Path(sys.argv[2]).read_text()
runtime = (root / 'runtime/prelude.c').read_text()
assert source.startswith(runtime), 'generated C must begin with the embedded runtime'
generated = source[len(runtime):]
assert not re.search(r'\b(?:while|for|malloc|realloc)\s*\(', generated), 'runtime traversal or allocation found'
functions = re.findall(r'^void (sym_\d+)\([^;{}]*\) \{', generated, re.M)
assert len(functions) == 1, 'macro printing must expand inside the caller without printer wrappers'
assert len(re.findall(r'\bprint_(?:i32|bool|u16)\(', generated)) == 5, 'fixed fields and array elements must print directly'
assert 'TypeInfo' not in generated and 'FieldInfo' not in generated, 'reflection metadata leaked into runtime code'
PY
gcc -std=gnu99 -D_POSIX_C_SOURCE=200809L -O0 -Werror=implicit-function-declaration -Werror=incompatible-pointer-types "${WORK_DIR}/out.c" -o "${WORK_DIR}/program"
"${WORK_DIR}/program" > "${WORK_DIR}/actual.out"
cmp "${ROOT_DIR}/tests/51_reflection_zero_cost.out" "${WORK_DIR}/actual.out"
echo 'PASS reflection expands to direct calls without runtime traversal or allocation'
echo "Artifacts: ${WORK_DIR}"
cat > "${WORK_DIR}/wide.a" <<'ABYSS'
import print from "std/io"
main :: fn() {
    maximum := ~(0 as u128)
    minimum := ((1 as u128) << 127) as i128
    #print(maximum)
    print("\n")
    #print(minimum)
    print("\n")
}
ABYSS
"${COMPILER}" b "${WORK_DIR}/wide.a" "${WORK_DIR}/wide.c"
gcc -std=gnu99 -D_POSIX_C_SOURCE=200809L -O0 -Werror=implicit-function-declaration "${WORK_DIR}/wide.c" -o "${WORK_DIR}/wide"
"${WORK_DIR}/wide" > "${WORK_DIR}/wide.out"
printf '340282366920938463463374607431768211455\n-170141183460469231731687303715884105728\n' > "${WORK_DIR}/wide.expected"
cmp "${WORK_DIR}/wide.expected" "${WORK_DIR}/wide.out"
echo 'PASS native printing preserves all 128 bits'
