# Abyss

**High-level Syntax. Low-level Soul.**

Abyss is a statically typed, compiled systems programming language engineered for mechanical sympathy, predictability, and uncompromising performance. It combines modern syntactic ergonomics (inspired by Odin and Jai) with the raw control and transparency of C.

> **Status: Stage 0 is Operational & Self-Hosting Has Begun**  
> The architectural rewrite is complete. The core compiler pipeline—including the data-oriented Nexus store, Pratt parser, unification-based type checker, and C code generator—is fully functional. Implementation of the standard library (`std`) and the Stage 1 self-hosted compiler is currently in progress.

---

## Code at a Glance

Abyss compiles directly to portable C99 with seamless libc interoperability and explicit memory management:

```rust
print :: (s &u8) unit

main :: () {
    print("Hello, Abyss!")
}
```

---

## Architectural Highlights

* **Data-Oriented (DOD) Nexus Engine:** Replaces heavy pointer-chasing AST nodes with cache-aligned arenas, flat storage tables, and dense numeric IDs (`HirId`, `SymbolId`, `TypeId`).
* **Unification-Based Type Inference:** Type checking driven by union-find logic, resolving nested structs, pointer indirection, and literal coercions bidirectionally.
* **C as Intermediate Representation:** Emits clean, portable C99 code, leveraging GCC and Clang for backend code optimization, instruction scheduling, and architecture-specific vectorization.
* **Zero-Cost Interop:** Native binding and direct execution of any C library without runtime wrappers or marshaling overhead.

---

## Roadmap & Progress

* [x] Data-oriented storage architecture (`abyss_nexus`)
* [x] Pratt parser with lookahead header resolution (`abyss_parser`)
* [x] Hindley-Milner / Unification type system (`abyss_typer`)
* [x] C code generator with topological struct resolution (`abyss_lower`)
* [x] Zero-overhead C FFI and minimal runtime (`prelude.c`)
* [x] Dynamic collections (`Vec`) with direct pointer arithmetic (`std/vec`)
* [ ] Core standard library (File I/O, Arena Allocator, String Utilities)
* [ ] Self-hosting bootstrap (Stage 1 compiler written in Abyss)

---

## License

[MIT License](LICENSE) — Do whatever you want with the code, just keep my name on it!


