# Abyss

**High-level Syntax. Low-level Soul.**

Abyss is a statically typed, compiled systems programming language designed for mechanical sympathy, predictability, and raw performance. It pairs clean, modern ergonomics inspired by Odin and Jai with the explicit transparency of C.

> **Status: 100% Self-Hosted**  
> The Abyss compiler is written in Abyss and emits deterministic, portable C99.

---

## Features

* **Ergonomic Syntax:** Clean declaration-first grammar without header files or preprocessor baggage.
* **Unification Type System:** Bidirectional Hindley-Milner-style type inference with strict numeric sizing.
* **Portable C99 Target:** Compiles directly to readable C99, leveraging GCC/Clang for optimizations and vectorization.
* **Zero-Cost Interop:** Native binding and direct execution of any C library without runtime wrappers.
* **Data-Oriented Compiler:** Built around flat arena tables and dense numeric IDs (`HirId`, `TypeId`) instead of pointer-heavy AST trees.
* **Minimal Core `std`:** Lightweight baseline modules covering Arena allocation, dynamic vectors, strings, and basic file I/O.

---

## Code at a Glance

```rust
print :: fn(s &u8) unit

main :: fn() {
    print("Hello, Abyss!")
}

```

---

## Quick Start

Abyss bootstraps from the included `abyssc` seed compiler:

```sh
mkdir -p tmp
./abyssc main.a tmp/out.c
gcc -O2 tmp/out.c -o tmp/abyssc
./tmp/abyssc main.a tmp/out.c

```

Run the self-hosting and compiler regression tests with:

```sh
./abyss bootstrap 5
./abyss test
```

The bootstrap test builds five compiler generations, checks identical C output,
runs the final compiler, and tests program execution and type-error diagnostics.
Each test run keeps its artifacts in a unique directory under `tmp/`.
Successful programs use checked-in `.out` files; rejected programs under
`tests/errors/` use `.err` files containing required diagnostic fragments.
The runner requires a nonzero compiler exit for rejected programs and checks
that no C file was emitted. Set `ABYSS_COMPILER` to test another compiler binary.


Import `print` from `std/io.a` to print signed and unsigned integers up to
64 bits, `f32`, `f64`, booleans, C strings, and `Str` values. Its generic type
argument is inferred, so calls such as `print(42)` and `print("hello")` work.
Unsupported types are rejected during compilation.

`./abyss r file.a` builds and runs that program. Without a file argument,
it builds and runs `main.a`, which is the compiler itself. Changes to `core`,
`std`, or the C runtime automatically refresh the embedded sources and rebuild
`abyssc` in two stages. The temporary stage uses `-O0`; the final compiler uses
`-O2`. Unchanged embedded sources skip this rebuild.

## Next Steps

With self-hosting stabilized, active development focuses on:

* [x] Self-hosted Stage 1 compiler
* [x] Primitives for memory (`Arena`), collections (`Vec`), and strings
* [x] Standard library expansion (formatting, math, system calls)
* [x] Module packaging & test runner
* [ ] Comptime evaluation & constant folding
---

## License

[MIT License](LICENSE) — Do whatever you want with the code, just keep my name on it!
