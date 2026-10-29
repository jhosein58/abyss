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

A stack-based bytecode virtual machine in Abyss:

```zig
import printf from "std/io"

Op :: enum {
    push,
    add,
    sub,
    mul,
    print,
    halt,
}

Inst :: struct { Op, i32 }

main :: fn() {
    program: [7]Inst = .[
        .{ Op.push,  15 },
        .{ Op.push,  25 },
        .{ Op.add,   0 },
        .{ Op.push,  2 },
        .{ Op.mul,   0 },
        .{ Op.print, 0 },
        .{ Op.halt,  0 },
    ]

    stack := Vec(i32).new()

    for inst in program {
        match inst.0 {
            push  { stack.push(inst.1) }
            add   { stack.push(stack.pop() + stack.pop()) }
            sub   { stack.push(stack.pop() - stack.pop()) }
            mul   { stack.push(stack.pop() * stack.pop()) }
            print { printf("result = {}\n", .{stack.pop()}) }
            halt  { break }
        }
    }
}
```

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
