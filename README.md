# <img src="assets/logo.svg" alt="A" height="28" />byss

**Built for the deep.**

A statically typed, data-oriented systems programming language. Pairs modern ergonomics with the transparency and speed of C.

- **100% Self-Hosted:** Bit-for-bit deterministic bootstrap verification.
- **Pure C99 Target:** Emits standard portable C. Compiles with GCC, Clang, or MSVC.
- **Integrated Bytecode VM:** Instant compile-and-run without external toolchain latency.
- **Zero-Cost Formatting & Reflection:** Compile-time format verification and type inspection with zero runtime overhead.
- **Compile-Time Evaluation:** Run arbitrary logic at compile time via `cmpt` and macros.
- **First-Class Types:** Compute, return, and pass types dynamically at compile time.
- **Zero-Dependency Bootstrap:** Standalone C99 source compiles anywhere with a single `cc` command.

🌐 **Website & Docs:** [**jhosein58.github.io/abyss**](https://jhosein58.github.io/abyss)

---

## Code at a Glance

A stack-based bytecode virtual machine in Abyss:

```zig
Op :: enum {
    halt
    push
    add
    sub
    mul
    print
}

main :: fn() {
    program := .[
        Op.push
        Op.push
        Op.add
        Op.push
        Op.mul
        Op.print
        Op.halt
    ]

    s := Vec(i32).new()

    for inst in program {
        match inst {
            halt  { break }

            push  { s.push(42) }
            add   { s.push(s.pop() + s.pop()) }
            sub   { s.push(s.pop() - s.pop()) }
            mul   { s.push(s.pop() * s.pop()) }

            print { 
                printf("result = {}\n", .{s.pop()}) 
            }
        }
    }
}
```

---

## Installation

```sh
# Linux / macOS
curl -fsSL https://raw.githubusercontent.com/jhosein58/abyss/main/scripts/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/jhosein58/abyss/main/scripts/install.ps1 | iex
```

---

## Quick Start

```sh
# Run immediately via VM (zero compile latency)
./abyss r main.a

# Compile and run optimized native binary (-O3)
./abyss br main.a

# Fast test suite
./abyss test

# Package manager
./abyss sonar help
```

---

## License

[MIT](LICENSE)

