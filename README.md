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
print :: (s &u8) unit

main :: () {
    print("Hello, Abyss!")
}

```

---

## Quick Start

Abyss bootstraps directly via emitted C:

```sh
# 1. Build the compiler binary from the bootstrap seed
gcc -std=gnu11 -O2 out.c -o abyssc

# 2. Compile an Abyss source file
./abyssc main.a

# 3. Compile the generated C output
gcc out.c -o bin
./bin

```



## Next Steps

With self-hosting stabilized, active development focuses on:

* [x] Self-hosted Stage 1 compiler
* [x] Primitives for memory (`Arena`), collections (`Vec`), and strings
* [ ] Standard library expansion (formatting, math, system calls)
* [ ] Comptime evaluation & constant folding
* [ ] Module packaging & test runner

---

## License

[MIT License](LICENSE) — Do whatever you want with the code, just keep my name on it!


