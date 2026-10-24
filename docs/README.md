# Into the Abyss

*The Official Guide to the Abyss Programming Language*

Welcome to **Into the Abyss**.

This book is designed for programmers who want to build fast, predictable software without fighting layers of unnecessary abstraction. Abyss was built for the deep—where memory has addresses, caches have line sizes, and code runs directly on the machine.

There are no hundred-page theoretical preambles here. Every chapter is short, code-first, and gets straight to the point.

---

## Table of Contents

### Part I: Overview
Get into the mindset behind the language and understand how the compiler works.
- [Philosophy](01-overview/philosophy.md) — Mechanical sympathy, zero hidden runtime, and control.
- [Architecture](01-overview/architecture.md) — The dual-engine design (VM + C99), data-oriented compiler, and bootstrapping.

### Part II: Quick Start
Start writing and running code in less than ten minutes.
- [Whirlwind Tour](02-quickstart/whirlwind-tour.md) — Syntax, types, control flow, structs, and compile-time execution at a glance.
- [Compile-Time Reflection](02-quickstart/reflection.md) — Type metadata, generated printing, and macro templates.
- [Tooling & Workflow](02-quickstart/workflow.md) — Using the `./abyss` driver for instant VM runs and optimized native builds.

### Part III: The Guide *(In Progress)*
A deep, structured dive into every language feature.
- `01 - Basics` — Primitive types, variables, arithmetic, and operators.
- `02 - Control Flow` — Branching with `if/else`, looping with `while` and `for`, and pattern matching.
- `03 - Functions & Methods` — Function declarations, clean return syntax, and method receivers.
- `04 - Structs & Types` — Data layout, constructors, methods, and first-class types.
- `05 - Pointers & Memory` — Addressing, dereferencing, manual memory control, and layout guarantees.
- `06 - Comptime & The VM` — Running arbitrary code during compilation with `cmpt`.

### Part IV: Reference *(In Progress)*
Comprehensive API documentation for the standard library.
- `core` — The foundation: `String`, `Str`, and core intrinsics.
- `std/io` — Formatted printing and basic I/O.
- `std/fs` — File system operations and path manipulation.
- `std/mem` — Allocators, manual heap management, and memory blocks.
- `std/env` — Command-line argument parsing and environment variables.
- `std/sys` — Platform bindings and system-level primitives.

---

> *"Built for the deep."*
