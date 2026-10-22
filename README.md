<p align="center">
  <img src="assets/logo.svg" alt="Abyss Logo" width="128" height="128" />
</p>

<h1 align="center">Abyss</h1>

<p align="center">
  <strong>High-level Syntax. Low-level Soul.</strong><br>
  A statically typed systems programming language designed for mechanical sympathy, predictability, and raw performance.
</p>

<p align="center">
  <em>100% Self-Hosted &bull; Pure C99 Target &bull; Integrated Bytecode VM &bull; Bit-Identical Determinism</em>
</p>

---

## Code at a Glance

```abyss
import print from "std/io"

Point :: struct {
    x, y i32

    dist_sq :: fn(self &Self) i32 {
        self.x * self.x + self.y * self.y
    }
}

TypeByCond :: fn(cond bool) type {
    if cond { i32 } else { f32 }
}

main :: fn() {
    p: Point = .{ x: 3, y: 4 }
    print("Distance squared: ")
    print(p.dist_sq())
    print("\n")

    val := cmpt {
        base := 20
        base * 2 + 2
    }
    print("Compile-time evaluated: ")
    print(val)
    print("\n")

    x: TypeByCond(true) = 100
    print("Computed type value: ")
    print(x)
    print("\n")
}
```

---

## Key Highlights

- **Dual-Engine Architecture:** Instant Bytecode VM for development and testing, alongside an optimizing portable C99 code generator.
- **Compile-Time Execution (`cmpt`):** Evaluate arbitrary expressions and blocks at compile-time via the VM engine.
- **First-Class Types:** Types are first-class values that can be passed to functions, returned from type functions, or selected via conditional branches.
- **Data-Oriented Core (DOP):** Built around dense linear tables and compact integer IDs instead of pointer-heavy AST graphs.
- **Multiplatform & Standalone:** Native Linux binaries and standalone Windows binaries with zero third-party runtime dependencies (linking only to `KERNEL32.dll` and `msvcrt.dll`).
- **Deterministic Fixed-Point Self-Hosting:** Verified bit-for-bit identical across bootstrap generations ($G_1 \equiv G_2 \equiv G_3$).
- **Zero-Dependency Bootstrap:** A single portable C99 source file (`bootstrap.c`) bootstraps the entire compiler from scratch on any machine with a C compiler.

---

## Quick Start

### 1. Build the Compiler

Using the repository's seed compiler:

```sh
./abyss b
```

Or bootstrap directly from the standalone C source:

```sh
gcc -std=c99 -O3 -rdynamic dist/bootstrap.c -o abyssc
```

### 2. Run a Program

Instant execution on the Bytecode VM:

```sh
./abyss r program.a
```

Compile and run an optimized native binary:

```sh
./abyss br program.a
```

---

## Tooling CLI

The `./abyss` workflow script provides end-to-end tooling:

| Command | Description |
| :--- | :--- |
| `./abyss r [file.a]` | Run immediately on the Bytecode VM (zero compile latency) |
| `./abyss b [file.a]` | Compile to an optimized native binary (`-O3`) |
| `./abyss br [file.a]` | Compile to native binary and run |
| `./abyss b-win [file.a]` | Cross-compile standalone Windows binary (`.exe`) via MinGW |
| `./abyss test [filter]` | Run test suite against both C and VM backends |
| `./abyss test-win [filter]` | Run test suite against Windows binary via Wine |
| `./abyss bootstrap [n]` | Run multi-round bit-identical bootstrap verification |
| `./abyss release` | Package and verify release artifacts (`bootstrap.c`, Linux, Windows) |
| `./abyss clean` | Clean temporary build artifacts |

---

## License

[MIT License](LICENSE)
