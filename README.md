# <img src="assets/logo.svg" alt="A" height="28" />byss

**Built for the deep.**

A statically typed, data-oriented systems programming language. Pairs modern ergonomics with the transparency and speed of C.

- **100% Self-Hosted:** Bit-for-bit deterministic bootstrap verification.
- **Pure C99 Target:** Emits standard portable C. Compiles with GCC, Clang, or MSVC.
- **Integrated Bytecode VM:** Instant compile-and-run without external toolchain latency.
- **Zero-Cost Formatting & Reflection:** Compile-time format verification and type inspection with zero runtime overhead.
- **Compile-Time Evaluation:** Run arbitrary logic at compile time via `cmpt` and macros.
- **First-Class Types:** Compute, return, and pass types dynamically at compile time.
- **Zero-Dependency Bootstrap:** `dist/bootstrap.c` compiles anywhere with a single `cc` command.

📖 **Guide:** [**Into the Abyss**](docs/README.md) — The official language documentation and book.

---

## Code at a Glance

<p align="center">
  <img src="assets/code-preview.svg?v=2" alt="Abyss Code Preview" width="100%" />
</p>

<details>
<summary><b>📄 View raw code</b></summary>

```abyss
import print, printf from "std/io"

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
    printf("Point ({}, {}), dist_sq = {}\n", .{p.x, p.y, p.dist_sq()})

    val := cmpt {
        base := 20
        base * 2 + 2
    }
    printf("Compile-time evaluated: {}\n", .{val})

    x: TypeByCond(true) = 100
    print(x)
}
```
</details>

---

## Quick Start

```sh
# Run immediately via VM (zero compile latency)
./abyss r main.a

# Compile and run native binary with full optimization (-O3)
./abyss br main.a

# Fast test suite (C + VM backends via TCC/GCC)
./abyss test

# Full deep verification suite
./abyss test-all

# Or bootstrap from zero-dependency standalone C99 source
gcc -std=c99 -O3 -rdynamic dist/bootstrap.c -o abyssc
```

---

## Tooling

| Command | Description |
| :--- | :--- |
| `./abyss r <file>` | Run on Bytecode VM (auto-rebuilds compiler if sources changed) |
| `./abyss b <file>` | Compile to optimized native binary (`-O3 -march=native`) |
| `./abyss br <file>` | Compile and run native binary |
| `./abyss test [filter]` | Fast dev test suite with instant sub-second feedback |
| `./abyss test-all [filter]` | Full verification suite (fixed-point, reflection, isolation, CLI) |
| `./abyss promote [rounds]` | Multi-stage bootstrap & test verification before promoting to `./abyssc` |
| `./abyss bootstrap [rounds]` | Multi-generation bit-identical fixed-point stability test |
| `./abyss b-win <file>` | Cross-compile standalone Windows `.exe` |
| `./abyss r-win <file>` | Cross-compile and run Windows binary via Wine |
| `./abyss test-win [filter]` | Run test suite on Windows binary via Wine |
| `./abyss wasm` | Build WebAssembly standalone compiler (`abyssc.wasm`) |
| `./abyss release [rounds]` | Package release artifacts (`bootstrap.c`, Linux, Windows, Wasm) |
| `./abyss clean` | Clean temporary build artifacts |

---

## License

[MIT](LICENSE)
