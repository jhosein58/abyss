#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPILER=$(realpath "${1:-${ROOT_DIR}/abyssc}")
mkdir -p "${ROOT_DIR}/tmp"
WORK_DIR=$(mktemp -d "${ROOT_DIR}/tmp/compiler_test.XXXXXX")
mkdir -p "${WORK_DIR}/tmp"
ln -s "${ROOT_DIR}/std" "${WORK_DIR}/std"
cp "${ROOT_DIR}/prelude.c" "${WORK_DIR}/prelude.c"
cd "${WORK_DIR}"

cat > main.a <<'ABYSS'
import VecU32 from "std/vec/u32.a"

print :: (value &u8) unit
print_u32 :: (value u32) unit

Counter :: struct {
    value : u32,

    new :: (value u32) Self {
        .{ value: value }
    }

    add :: (value u32) {
        .value = .value + value
    }

    get :: () u32 {
        .value
    }
}

factorial :: (value u32) u32 {
    if value == 0 { ret 1 }
    value * factorial(value - 1)
}

main :: () {
    counter := Counter.new(40)
    counter.add(2)
    print_u32(counter.get())
    print("\n")
    values := VecU32.new()
    values.push(3)
    values.push(4)
    index: u64 = 0
    total: u32 = 0
    while index < values.len {
        total = total + values.get(index)
        index = index + 1
    }
    print_u32(total)
    print("\n")
    values.destroy()
    print_u32(factorial(5))
    print("\n")
}
ABYSS

"${COMPILER}"
gcc -O2 tmp/out.c -o tmp/program
printf '42\n7\n120\n' > expected.txt
./tmp/program > actual.txt
cmp expected.txt actual.txt
echo "[PASS] Struct methods, imports, vectors, loops and recursion"

expect_type_error() {
    local name="$1"
    local expected="$2"
    if "${COMPILER}" > diagnostic.txt 2>&1; then
        echo "[FAIL] ${name}: invalid program was accepted"
        exit 1
    fi
    if ! rg -Fq "${expected}" diagnostic.txt; then
        cat diagnostic.txt
        echo "[FAIL] ${name}: expected diagnostic missing"
        exit 1
    fi
    echo "[PASS] ${name}"
}

cat > main.a <<'ABYSS'
take :: (value u32) {}
main :: () {
    value: bool = true
    take(value)
}
ABYSS
expect_type_error "Argument type mismatch" "type mismatch: cannot unify"

cat > main.a <<'ABYSS'
main :: () {
    value: u32 = 1
    value = false
}
ABYSS
expect_type_error "Assignment type mismatch" "type mismatch: cannot unify"

cat > main.a <<'ABYSS'
take :: (value &u32) {}
main :: () {
    value: u8 = 1
    take(&value)
}
ABYSS
expect_type_error "Pointer type mismatch" "type mismatch: cannot unify"

cat > main.a <<'ABYSS'
Alpha :: struct { value : u32 }
Beta :: struct { value : u32 }
take :: (value Alpha) {}
main :: () {
    value: Beta = .{ value: 1 }
    take(value)
}
ABYSS
expect_type_error "Named struct identity" "Named structs require the same declaration"

echo "SUCCESS: Compiler regression tests passed."
echo "Artifacts: ${WORK_DIR}"
