# Abyss

**High-level Syntax. Low-level Soul.**

Abyss is a statically typed systems programming language designed for predictability, mechanical sympathy, and raw performance. It pairs clean, modern ergonomics inspired by Odin and Jai with the transparency of C.

> **Status:** 100% Self-Hosted. Emits deterministic GNU C99.

---

## Code at a Glance

```abyss
import print from "std/io.a"

Point :: struct {
    x i32
    y i32
}

main :: fn() {
    p: Point = .{ x: 10, y: 20 }
    print("Point: ")
    print(p.x)
    print(", ")
    print(p.y)
    print("\n")
}
```

---

## Quick Start

### 1. Build the Compiler

Using the repository's seed compiler:

```sh
./abyss b
```

Or bootstrap directly from the release C source:

```sh
gcc -std=gnu99 -O2 bootstrap.c -o abyssc
```

### 2. Run a Program

```sh
./abyss r program.a
```

---

## Tooling CLI

The `./abyss` script provides a unified developer workflow:

| Command | Description |
| :--- | :--- |
| `./abyss r [file.a]` | Build and run in development mode (live `std` reload) |
| `./abyss b [file.a]` | Build an optimized binary (`-O3`) |
| `./abyss b-win` | Cross-compile standalone Windows binary (`.exe`) via MinGW |
| `./abyss test [filter]` | Run language and compiler regression test suite |
| `./abyss bootstrap [n]` | Run multi-round fixed-point bootstrap verification |
| `./abyss release` | Package standalone release artifacts (`bootstrap.c`, Linux, Windows) |

---

## Features

- **Ergonomic Grammar:** Clean declaration syntax without header files or preprocessor baggage.
- **Unification Type System:** Bidirectional type inference with strict numeric sizing.
- **GNU C99 Target:** Emits clean GNU C99, leveraging GCC and Clang for optimizations.
- **Data-Oriented Core:** Built around dense linear tables and numeric IDs (`HirId`, `TypeId`) instead of pointer-heavy AST trees.
- **Zero-Dependency Bootstrap:** A single portable C file (`bootstrap.c`) bootstraps the entire language from scratch.

---

## License

[MIT License](LICENSE)
