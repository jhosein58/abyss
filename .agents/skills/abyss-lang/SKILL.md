---
name: abyss-lang
description: >-
  Authoritative language specification, idiomatic coding patterns, grammar reference,
  and standard library conventions for the Abyss programming language (.a files).
  Activate this skill whenever reading, writing, refactoring, formatting, testing,
  or debugging Abyss code, compiler sources, stdlib, tools, or runtime.
---

# Abyss Programming Language Guide

Abyss is a statically typed, expression-oriented systems programming language featuring orthogonal struct namespaces, compile-time reflection, arena memory management, and zero-cost abstractions. Abyss emits C99 for native and WASI targets and executes bytecode on its VM. Compiler and standard-library runtime implementations are written in Abyss.

---

## 1. Golden Rules of Idiomatic Abyss

1. **Always use `Str` for text and paths:**
   - Prefer `Str` (`.{ ptr: &u8, len: u64 }`) for function parameters, return values, file paths, and string handling.
   - String literals `"..."` produce `Str`.
   - Only use `c"..."` (`&u8`) at the lowest boundary interfacing with external C runtime functions (`fopen`, `printf`).
   - Convert `&u8` to `Str` with `Str.of(cstr)`. Convert `Str` to null-terminated C string with `to_cstr(&arena, str)`.

2. **Dotted Import Syntax (`import <path>: <symbols>`):**
   - **Never** use the `from` keyword (it is completely eradicated from Abyss).
   - Absolute import: `import std.allocator: Arena` or `import std.fs: File`
   - Relative import: Single leading dot only: `import .token: Token` or `import .nexus.nexus: Nexus`
   - Parent directory relative imports (`..`) are strictly forbidden; use project-root absolute paths.
   - Module as namespace struct: `import std.mem` or `import std.string as string`
   - Scoped imports: Allowed inside functions, structs, enums, or blocks to keep bindings local.

3. **No Semicolons for Top-Level & Match Arms:**
   - Declarations and statements are separated by newlines or commas.
   - Semicolons at top level or in match arms cause syntax errors.

4. **Structs as Namespaces:**
   - Structs with static functions serve as namespaces (e.g., `File.read_to_string`, `Path.join`).
   - Methods taking `self &Self` are invoked with dot syntax (`stream.close()`).

5. **Memory & Allocations:**
   - Use `Arena` (`import std.allocator: Arena`) for temporary and phase-oriented allocations.
   - Pass `&Arena` explicitly to functions that construct dynamic data.
   - Call `.destroy()` on heap objects (`Vec`, `Arena`) when their lifecycle ends.

6. **Leverage `core` and `std` Prelude Primitives:**
   - **Zero-Import Preludes**: The compiler automatically injects `core/prelude` and `std/prelude` into all source files. The following symbols are **globally available without any `import` statement**:
     - From `core/prelude`: `Maybe`, `Str`, `CharIter`, `Iterator`, `RefIterator`, `Range`, `Choice`, `Slice`, `SourceLocation`, `TypeInfo`, `FieldInfo`, `TypeKind`.
     - From `std/prelude`: `Vec`, `HashSet`, `HashMap`, `Worklist`, `print`, `printf`.
   - **`print` vs `write` Output Semantics**:
     - `print(value)` in `std.io` **automatically outputs a trailing newline (`\n`)** after the formatted value! It behaves like `println` in other languages. Never append `\n` to `print` or wrap it with an extra newline printer, or you will create double blank lines.
     - `write(value)` in `std.io` formats and prints `value` **without** a trailing newline. Use `write` for inline output, concatenations, or progress indicators.
     - `printf(pattern, args)` in `std.io` formats according to `pattern` without an automatic trailing newline.
   - **Prefer High-Level Abstractions**: Never write low-level C-style null checks, ad-hoc arrays, or manual loop copies when standard collections (`Vec`, `HashMap`, `HashSet`) or abstractions (`Maybe`) can make the code clean, concise, and expressive.
   - **Embrace `Maybe` (Abyss's `Option`)**: Use `Maybe` for any optional or fallible value instead of sentinel values (`-1`, null pointers). Use `.is_some()`, `.is_none()`, `.unwrap()`, and `.unwrap_or(default)`.

7. **Iterators, `for` Loops, and DOP Design:**
   - Prefer custom iterators combined with `for item in collection` loops over manual index-based `while` loops wherever applicable.
   - Embrace Data-Oriented Programming (DOP) principles: keep data structures contiguous and flat, separate data from transformation, and prioritize speed, beauty, and conciseness.

8. **Comments, Characters, and Single-Line Sequencing:**
   - **Comments:** Single-line only, written with `--` (e.g. `-- note`). Double-slash `//` is NOT valid Abyss syntax.
   - **No `char` Type:** Abyss does not have a `char` primitive. Character literals like `'a'` are compile-time `untyped int` and coerce to any integer or float.
   - **Comma Expression Sequencing:** Because Abyss has no semicolons, multiple expressions or statements on the same line are separated by commas `,`:
     ```abyss
     val := { a := 2, a + 3 }
     print(1), print(2)
     ```

9. **Automatic Dereference (Auto-Deref):**
   - **Never write clumsy manual dereferences like `(*a).b`, `(*ptr).field`, `&(*eng).nexus`, or `(tbl).lhs(id)`!**
   - Abyss has seamless automatic dereferencing for all member and field accesses: write `a.b.c.d` and `tbl.lhs(id)` directly.
   - Pointers to fields/subsystems take the address directly without dereferencing: write `db := &eng.nexus` and `tbl := &db.hir`. The compiler automatically dereferences pointers as deeply as necessary without extra parentheses or asterisks.

10. **Newlines as Separators:**
    - A newline can replace the comma between statements or list items. Examples:
      - Array literals:
        ```abyss
        items := .[
            1
            2
            3
        ]
        ```
      - Struct literals: `.{ x: 10 \n y: 20 }`
      - Function arguments / parameters: `foo(a \n b)`
      - Imports: `import std.mem: \n malloc \n free`
    - Keep commas within grouped names such as `a, b i32` and within multiple match patterns on the same line.
    - Keep trailing comments on the line they describe and retain a blank line between related groups of statements or fields. Keep short literals compact and put one entry per line when a list needs to wrap.

11. **Deferred Type Inference & Lean Annotations:**
    - Rely on the bidirectional unification engine: omit redundant type annotations inside function bodies.
    - Type arguments at the beginning of generic functions/types are optional: write `Vec().new()` instead of `Vec(i32).new()`; the compiler infers `T` from subsequent usage (e.g., the first `.push(item)`).
    - Explicit type annotations are strictly reserved for boundary points: function parameters and explicit function return types.

12. **Metaprogramming via String Splices (`#`), `cmpt`, and Multi-Level Quotes:**
    - **Splice `#` (D-style Mixin):** The `#` operator evaluates a string expression at compile-time and inlines the generated code directly into the AST:
      ```abyss
      a := #"3"
      #`print("inlined code\n")`
      #generate_methods()
      ```
    - **Source Locations:** `@source_location()` is a compile-time `SourceLocation` value with `file Str`, `line u32`, and `column u32`. Line and column start at 1. Code generated by a splice uses the source location of the splice invocation, including in nested splices.
    - **Fatal Errors:** Import `fall` from `std.sys` and use `#fall` to report the splice site to stderr and exit with status 1. Use `fall_at(location)` when you already have a `SourceLocation`.
    - **`cmpt` Keyword:** Marks compile-time constant variables, parameters, or expressions evaluated on the bytecode VM before codegen.
    - **Multi-Level Splices:** `##str`, `###str`, `####str` allow multi-level quote/unquote metaprogramming where code generates code that generates code.
    - **Raw Multiline Strings:** Backticks `` `...` `` define multiline raw string literals ideal for code templates.

---

## 2. Quick Syntax Cheat Sheet

### Variables & Constants
```abyss
x := 42                 -- Mutable binding with inferred type
count: u64 = 100        -- Explicit type annotation
PI :: 3.14159           -- Constant binding (evaluated at comptime)
```

### Functions & Methods
```abyss
-- Standalone function
add :: fn(a: i32, b: i32) i32 {
    a + b               -- Implicit tail return
}

-- Struct with constructors and methods
Point :: struct {
    x i32
    y i32

    new :: fn(x i32, y i32) Self {
        .{ x: x, y: y }
    }

    distance_sq :: fn(self &Self) i32 {
        self.x * self.x + self.y * self.y
    }
}
```

### Control Flow
```abyss
-- If-else (can be an expression)
val := if cond { 10 } else { 20 }

-- While loop
while i < len {
    if should_skip(i) { cont }
    if should_stop(i) { break }
    i = i + 1
}

-- For loop over iterators
for item in collection {
    process(item)
}

-- Pattern matching (STRICTLY for Enums and Unions only! Cannot match on integers/strings)
-- 1. Matching Enum variants:
match status {
    Status.ok    { handle_ok() }
    Status.error { handle_err() }
    else         { fallback() }
}

-- 2. Matching Union types with binding:
match val in union_val {
    i32  { print_i32(val) }
    bool { print_bool(val) }
}

-- 3. Matching Union types with grouped types (without binding):
match union_val {
    i32, f32 { handle_numeric() }
    else     { fallback() }
}
```

### Memory Management with Arena
```abyss
arena := Arena.new(65536)

-- Allocate memory or use standard collections
list := Vec(Str).new()
list.push("hello")
list.push("world")

-- Clean up
list.destroy()
arena.destroy()
```

---

## 3. Development & Verification Workflows

The repository provides the `./abyss` driver script:

Command | Purpose
:--- | :---
`./abyss r <file.a>` | Run `.a` file on Bytecode VM (fast development loop).
`./abyss b <file.a>` | Compile `.a` file to optimized native binary via C backend.
`./abyss br <file.a>` | Build and immediately execute native binary.
`./abyss test [filter]` | Run the compiler test suite across C and VM backends.
`./abyss bootstrap [n]` | Verify multi-generation bit-identical self-hosting fixed-point.
`./abyss promote [n]` | Verify and promote compiler binary to `./abyssc`.
`./abyss fmt [--width N] <path> ...` | Format files or directories in place. The default target is 100 columns.
`./abyss b-fmt` | Rebuild the native formatter after changing `pods/fmt`.
`./abyss sonar <cmd>` | Sonar package manager (or invoke `./sonar` directly): `init`, `add`, `update`, `vendor`, `build`, `run`.
`./abyss sonar-test` | Run Sonar automated unit and integration test suite.
`./abyss wasm` | Build the WebAssembly compiler with embedded core and std pods (`dist/abyssc.wasm`).
`./abyss release [n]` | Build full release packages in `dist/` (Linux tar.gz, Windows zip, Wasm, SHA256).

---

## 4. Package & Pod Management with Sonar (`tools/sonar/` and `./sonar`)

Sonar is the official package manager, registry client, and polyglot build orchestrator for Abyss. Packages in Abyss are known as **Pods**, defined by a `pod.json` manifest. The core compiler remains decoupled and agnostic of pod manifests.

### Official Installers
- **Linux / macOS:** `curl -fsSL https://raw.githubusercontent.com/jhosein58/abyss/main/scripts/install.sh | bash`
- **Windows (PowerShell):** `irm https://raw.githubusercontent.com/jhosein58/abyss/main/scripts/install.ps1 | iex`

### CLI Subcommands
- `sonar init [name]`: Initialize a standard `pod.json` manifest.
- `sonar add <pod> [ver]`: Query the `abyss-index` registry, add the pod to `dependencies`, clone Git repo, and vendor dependencies.
- `sonar update`: Pull latest pod definitions from the central `abyss-index` repository.
- `sonar vendor`: Resolve dependencies (MVS, cycle check, conflict check) and vendor into `.vendor/`.
- `sonar build [src] [out]`: Compile native C sources, link objects, and invoke compiler with `--map` module remapping.
- `sonar run [src]`: Build and immediately execute project binary.
- `sonar test [target]`: Auto-discover and run all unit test suites in `tests/` or a specified test target with ANSI color formatting. Auto-maps `test` pod and pod under test.

### Pod Manifest (`pod.json`)
```json
{
  "name": "my-app",
  "version": "1.0.0",
  "entry": "src/main.a",
  "dependencies": {
    "json": {
      "version": "^0.1.0",
      "git": "https://github.com/jhosein58/abyss.git",
      "subdir": "pods/json"
    },
    "math": {
      "version": "^1.2.0",
      "path": "../local_libs/math"
    },
    "raylib_native": {
      "version": "^5.0.0",
      "path": "./deps/raylib_native"
    }
  },
  "c_build": {
    "sources": ["native/helper.c"],
    "include_dirs": ["native/include"],
    "c_flags": ["-O3"],
    "ld_flags": ["-lraylib", "-lGL", "-lm", "-lpthread", "-ldl"]
  }
}
```

### Core Architecture & Invariants
1. **Minimal Version Selection (MVS) & Elm-Style Conflict Detection**:
   - Transitive dependencies are traversed recursively.
   - For dependencies sharing the same major version, Sonar selects the minimum sufficient version (MVS: highest required minor/patch).
   - Incompatible major versions (e.g., `^1.0.0` vs `^2.0.0`) trigger immediate strict compilation errors.
2. **Circular Dependency Handling**:
   - DFS active-stack tracking detects mutual cycles (`A <-> B`) cleanly without infinite recursion.
3. **Polyglot & C Wrappers**:
   - Pods can contain native C sources (`.c`, `.h`).
   - Sonar compiles C sources into cached objects in `.vendor/.cache/` and links them into the final binary with specified `ld_flags`.
4. **Compiler Module Flags**:
   - `-I <dir>`: Generic search directory for module imports.
   - `--map <prefix>=<path>`: Generic path remapping for module namespaces.

---

## 5. Modern Testing Framework (`pods/test`)

Abyss features an integrated unit testing and assertion framework built around compile-time reflection, ANSI color diagnostics, and non-fatal context tracking.

### Core Modules & Primitives
- **Import:** `import test: Suite, Test, assert_eq, assert_ne, assert_str_eq, assert_true, assert_false, assert_gt, assert_lt`
- **Fatal Assertions:** Standalone functions that halt test execution immediately on failure (`assert_eq(T, actual, expected, hint)`, `assert_str_eq(actual, expected, hint)`, `assert_null(T, ptr, hint)`, etc.) with colored diffs and type names via `@type_name(T)`.
- **`Test` Context:** Non-fatal assertions attached to `t &Test` (`t.assert_eq`, `t.assert_str_eq`, `t.assert_true`, etc.) that collect all assertion failures within a test case without premature exit.
- **`Suite` Runner:** Registers and executes test cases (`suite.add("name", test_func)`), tracks passed/failed counts, prints formatted status lines (`✓`/`✗`), and exits with appropriate process exit code via `suite.run_and_exit()`.

### Example Test Suite (`tests/test_example.a`)
```abyss
import std.allocator: Arena
import test: Suite, Test, assert_eq, assert_str_eq, assert_true

test_math_operations :: fn(t &Test) {
    t.assert_eq(i32, 2 + 2, 4, "addition must be correct")
    t.assert_true(10 > 5, "inequality check")
}

test_type_reflection :: fn(t &Test) {
    t.assert_str_eq(@type_name(i32), "i32", "")
    type_name := cmpt {
        info := @type_info(Point)
        info.name
    }
    t.assert_str_eq(type_name, "Point", "")
}

main :: fn() {
    arena := Arena.new(4096)
    suite := Suite.new(&arena, "My Test Suite")
    suite.add("math_operations", test_math_operations)
    suite.add("type_reflection", test_type_reflection)
    suite.run_and_exit()
    arena.destroy()
}
```

---

## 6. Modern SIMD Vector Pod (`pods/simd`)

The SIMD pod (`pods/simd`) implements fixed-width vectors and lane operations in Abyss. Exact scalar math calls the platform math library through typed `extern fn` declarations. The fast variants use Abyss approximations where applicable.

### Modular Architecture
- `pods/simd/simd.a`: Entry point re-exporting the entire public API.
- `pods/simd/vector.a`: Generic `Simd(T, cmpt N u64)` struct, memory layout, arithmetic, logic, and transcendental math.
- `pods/simd/mask.a`: `SimdMask(cmpt N u64)` boolean predicate masks and lane manipulation.
- `pods/simd/math.a`: Functional math API wrappers (`sin(v)`, `cos_fast(v)`, `exp(v)`, etc.).
- `pods/simd/aliases.a`: Standard vector types (`f32x4`, `f32x8`, `f64x2`, `f64x4`, `i32x4`, `i32x8`, `u32x4`, `u32x8`, `i64x2`, `i64x4`, `i16x8`, `u16x8`, `i8x16`, `u8x16`, `m2`, `m4`, `m8`, `m16`).
- `pods/simd/scalar.a`: Scalar math ABI declarations and approximations in Abyss.

### Vector Operations & APIs
- **Constructors:** `splat(val)`, `from_array(arr)`, `zero()`, `load(ptr)`, `store(ptr)`
- **Lane Access:** `get(idx)`, `set(idx, val)`, `extract(idx)`, `insert(idx, val)`, `reverse()`
- **Vector Arithmetic:** `add`, `sub`, `mul`, `div`, `neg`, `abs`, `min`, `max`, `clamp(low, high)`, `fma(b, c)`
- **Bitwise Logic (Integer):** `bit_and` / `and_`, `bit_or` / `or_`, `bit_xor` / `xor_`, `bit_not` / `not_`
- **Horizontal Reductions:** `sum()`, `product()`, `reduce_min()`, `reduce_max()`, `dot(other)`
- **Comparisons & Masks:** `cmp_eq` / `cmpeq`, `cmp_ne` / `cmpne`, `cmp_lt` / `cmplt`, `cmp_le` / `cmple`, `cmp_gt` / `cmpgt`, `cmp_ge` / `cmpge`
- **Mask Predicates:** `mask.all()`, `mask.any()`, `mask.none()`, `mask.count_ones()`, `mask.get(idx)`, `mask.set(idx, val)`
- **Selection:** `Simd(T, N).select(mask, on_true, on_false)` chooses lane values based on boolean mask.
- **Target Features:** `abyss_simd_cpu_features()` reports portable and target baseline flags.
- **Dual-Precision Vector Math:**
  - **System math:** `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `exp`, `exp2`, `ln`, `log2`, `log10`, `pow`, `sqrt`, `rsqrt`, `cbrt`, `hypot`, `sinh`, `cosh`, `tanh`
  - **Fast variants:** `sin_fast`, `cos_fast`, `tan_fast`, `exp_fast`, `ln_fast`, `rsqrt_fast`

### Example SIMD Usage
```abyss
import simd: f32x4, f64x2, i32x4, Simd, sin_fast, rsqrt_fast

run_simd :: fn() {
    a := f32x4.splat(1.5)
    b := f32x4.splat(2.0)
    c := a.fma(b, f32x4.splat(0.5))
    total := c.sum()

    s_fast := c.sin_fast()
    s_fn := sin_fast(c)
    inv_root := rsqrt_fast(c)

    mask := a.cmp_lt(b)
    chosen := f32x4.select(mask, a, b)
}
```

---

## 7. C ABI Interoperability & Two-Way FFI

Abyss features first-class, two-way C ABI interoperability, enabling Abyss programs to call foreign C libraries and export Abyss functions with unmangled C linker symbols.

### 7.1 Foreign C Function Declarations (`extern fn`)

Foreign C functions are declared using the `extern fn` syntax without a function body:

- **Standard declaration** (uses binding identifier as C linker symbol):
  ```abyss
  puts :: extern fn(s &u8) i32
  ```

- **Custom C linker symbol** (binds an arbitrary C linker symbol name):
  ```abyss
  print :: extern "printf" fn(s &u8) i32
  c_cos :: extern "cos" fn(x f64) f64
  ```

- **Parameter styles**:
  - **Named parameters**:
    ```abyss
    add :: extern fn(a, b i32) i32
    ```
  - **Anonymous parameters** (type only, no identifier):
    ```abyss
    add :: extern fn(i32, i32) i32
    print_str :: extern fn(&u8) i32
    ```

### 7.2 Two-Way C ABI: Exporting Abyss Functions to C

Abyss functions can be exported to foreign C code with unmangled C symbols by defining an `extern fn` with a block body `{ ... }`:

```abyss
-- Exported with C symbol 'abyss_add'
abyss_add :: extern fn(a, b i32) i32 {
    a + b
}

-- Exported with custom C linker symbol 'my_custom_c_func'
compute :: extern "my_custom_c_func" fn(x i32, y i32) i32 {
    x * 2 + y
}
```

The C backend emits an unmangled C function definition matching standard C calling conventions, directly callable by external C code or dynamic linkers. In VM mode, calls to exported functions within Abyss are dispatched directly as bytecode routines.

### 7.3 Extern Structs & C Memory Layout (`extern struct`)

Declare C-compatible structs using `extern struct`:

```abyss
Point :: extern struct {
    x i32
    y i32
}

Header :: extern struct {
    magic u32
    flags u16
    tag   u8
}
```

- **Strict Declaration Order**: Unlike standard Abyss structs (which may reorder fields for optimal packing), `extern struct` strictly preserves field declaration order without sorting.
- **Natural C Alignment & Padding**: Follows standard platform C ABI alignment and padding rules (e.g. System V AMD64 / Windows x64).
- **FFI Cleanliness**: Can be passed directly by value or by reference across foreign C boundaries.

### 7.4 Function Type Alias Disambiguation

Top-level function signatures declared **without** `extern` and **without** a body are parsed and bound as **function type aliases** (binding to `ty_id_type()`):

```abyss
Callback :: fn(i32) bool
BinaryOp :: fn(i32, i32) i32
UnaryOp  :: fn(i32) i32

-- Using function type aliases in signatures:
apply :: fn(cb Callback, val i32) bool {
    cb(val)
}
```

### 7.5 Runtime Boundaries

- Use `extern "module" "name" fn(...)` for a WASI import. Define platform entry points and ABI shims with exported Abyss functions. Only `extern` definitions are public in emitted native objects.
- Keep hosted process argument state in `std.sys.process`. The C backend emits the hosted entry adapter; WASI startup and freestanding memory routines are in `std.sys.start.wasi` and `std.sys.memory.wasi`.
- The VM binding code is in `compiler/vm/native.a`. It loads libffi at runtime on native targets. Compiler builds do not use libffi headers.
- Use `@memcpy` to copy the bits of an ABI pointer or scalar between equal-size storage types when a numeric cast would change its value.

### 7.6 Strict FFI Constraints & Type Diagnostics

- **No Generics in Extern Functions**: External C functions cannot have generic type parameters (`T type`). Generic extern declarations trigger a compile-time error: `external functions cannot be generic`.
- **No Variadics (`...`)**: C-style variadic arguments (`...`) are strictly prohibited in function declarations: `variadic parameters are not supported; use tuples for type-safe formatting`.
- **Fixed-Width Primitives**: Abyss uses explicit fixed-width types (`i8`..`i128`, `u8`..`u128`, `f32`, `f64`, `bool`, `&u8`). Using C type names (`int`, `char`, `short`, `long`, `float`, `double`, `unsigned`, `void`, `size_t`) triggers helpful diagnostic suggestions (e.g. `unknown type 'int'; did you mean 'i32'?`).

---

## 8. Extended References

Read these detailed documents when implementing complex features:
- [Compiler Architecture & Codebase Layout](./references/architecture.md): Subsystems, pipeline phases, Nexus data structures, and bootstrap invariants.
- [Grammar & Syntax Reference](./references/grammar.md): Full formal grammar, expressions, types, and compiler internals.
- [Idiomatic Patterns & Standard Library](./references/idioms.md): Detailed recipes for `Str`, `Maybe`, `Vec`, memory ownership, and error handling.
