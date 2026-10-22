# <img src="assets/logo.svg" alt="Abyss" width="36" height="36" align="middle" /> Abyss

**Built for the deep.**

A statically typed, data-oriented systems programming language. Pairs modern ergonomics with the transparency and speed of C.

- **100% Self-Hosted:** Bit-for-bit deterministic bootstrap verification.
- **Pure C99 Target:** Emits standard portable C. Compiles with GCC, Clang, or MSVC.
- **Integrated Bytecode VM:** Instant compile-and-run without external toolchain latency.
- **Compile-Time Evaluation:** Run arbitrary logic at compile time via `cmpt`.
- **First-Class Types:** Compute, return, and pass types dynamically at compile time.
- **Zero-Dependency Bootstrap:** `dist/bootstrap.c` compiles anywhere with a single `cc` command.

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

## Quick Start

```sh
# Build compiler (using seed binary)
./abyss b

# Or bootstrap from standalone C99 source
gcc -std=c99 -O3 -rdynamic dist/bootstrap.c -o abyssc

# Run immediately via VM
./abyss r main.a

# Compile native binary and run
./abyss br main.a
```

---

## Tooling

| Command | Description |
| :--- | :--- |
| `./abyss r <file>` | Run on Bytecode VM (zero compile latency) |
| `./abyss b <file>` | Compile to native binary (`-O3`) |
| `./abyss br <file>` | Compile and run native binary |
| `./abyss b-win <file>` | Cross-compile standalone Windows `.exe` |
| `./abyss test` | Run test suite (C + VM backends) |
| `./abyss test-win` | Run Windows test suite via Wine |
| `./abyss release` | Package release binaries and `bootstrap.c` |

---

## License

[MIT](LICENSE)
