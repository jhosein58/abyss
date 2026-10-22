# Architecture

Under the hood, Abyss is designed as a fast, deterministic pipeline with zero bloat. It rejects the modern trend of multi-gigabyte compiler toolchains that turn every build into an excuse to go grab coffee.

---

## The Dual-Engine Model

Traditional compilers force a painful compromise: either you use a sluggish interpreter with instant startup, or an optimizing native compiler where you wait seconds (or minutes) for intermediate representations to optimize.

Abyss includes **two coordinated execution engines** in a single binary:

```
                  ┌──────────────────────┐
                  │  Abyss Source (.a)   │
                  └──────────┬───────────┘
                             │
                  ┌──────────▼───────────┐
                  │     DOP Compiler     │
                  │ (Lex, Parse, Types)  │
                  └──────────┬───────────┘
                             │
             ┌───────────────┴───────────────┐
             │                               │
    ┌────────▼────────┐             ┌────────▼────────┐
    │   Bytecode VM   │             │   C99 Emitter   │
    ├─────────────────┤             ├─────────────────┤
    │ • Zero latency  │             │ • Portable C99  │
    │ • Tests & dev   │             │ • Maximum -O3   │
    │ • 'cmpt' blocks │             │ • GCC / Clang   │
    └─────────────────┘             └─────────────────┘
```

1. **The Bytecode VM:**
   The compiler can translate source directly into compact bytecode instructions and execute them instantly in-memory. This powers:
   - Zero-latency test runs (`./abyss r main.a`).
   - Compile-time evaluation (`cmpt { ... }`).
   - Interactive developer workflows.

2. **The C99 Emitter:**
   When you need maximum throughput, Abyss emits pure ISO C99. This output contains no compiler-specific extensions (no GNU statement expressions or nested labels) and compiles cleanly with GCC, Clang, or MSVC. You get industry-standard native code optimization without carrying the weight of a heavy backend inside the language itself.

---

## Data-Oriented Compiler Core (DOP)

Most classic compilers represent code as deeply nested, pointer-heavy Abstract Syntax Trees (ASTs). When traversing an AST, the CPU constantly stalls on cache misses, jumping randomly across the heap.

The Abyss compiler is architected around **Data-Oriented Programming (DOP)**:
- Code nodes, types, and symbols are stored in **dense, linear tables**.
- References are compact 32-bit integer IDs (`HirId`, `TyId`, `SymbolId`) rather than 64-bit heap pointers.
- Traversal sweeps sequentially across contiguous memory, keeping CPU caches warm and parsing blazing fast.

---

## Self-Hosting & Fixed-Point Determinism

Abyss is **100% self-hosted**. The compiler compiles itself:

$$\text{abyssc}_{G_1}(\text{main.a}) \rightarrow \text{stage2.c} \rightarrow \text{abyssc}_{G_2}$$
$$\text{abyssc}_{G_2}(\text{main.a}) \rightarrow \text{stage3.c} \rightarrow \text{abyssc}_{G_3}$$

The output of Generation 2 and Generation 3 is mathematically verified to be **bit-for-bit identical** ($G_2 \equiv G_3$). There is no undefined behavior, no memory corruption, and no environmental noise leaking into the build.

---

## Zero-Dependency Bootstrap

How do you build a self-hosted compiler on a machine that doesn't have the compiler yet?

Abyss solves the "chicken-and-egg" bootstrap problem cleanly. Every release packages `dist/bootstrap.c`—a single, standalone C99 source file containing the entire compiler.

On any machine with a basic C compiler:
```sh
cc -std=c99 -O3 -rdynamic dist/bootstrap.c -o abyssc
```

Done. No package managers, no downloading toolchains, no external libraries.

---

[Next: Whirlwind Tour &rarr;](../02-quickstart/whirlwind-tour.md)
