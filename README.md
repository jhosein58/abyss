# Abyss 🕳️

**High-level Syntax. Low-level Soul.**

> **⚠️ Status Update: I Blew It Up Again**  
> Honest truth? The compiler was actually working great. So... naturally, I scrapped the whole codebase.  
> Right now, **literally nothing works**. I'm doing a massive, ground-zero refactor to rebuild Abyss from scratch—this time with zero architectural compromises.

---

## 🩸 What It Looks Like Now

The syntax is shifting towards a clean, no-nonsense style (inspired by Odin and Jai):
```rust
main :: () {
    print("Hello, Abyss!\n")
}
```
---

## The Plan: Why Nuke It?

I'm rebuilding Abyss as a **data-oriented, query-based compiler engine** designed for sheer execution speed:

* **Ultra-Fast Data-Oriented Architecture (DOD):** Cache-friendly arenas, contiguous arrays, and stable IDs over slow tree-walking and pointer-chasing.
* **Query-Based & Compiler as a Library:** Everything is parsed and type-checked lazily on demand. This makes building an LSP, IDE tools, or static analyzers practically free.
* **Microsecond Incremental Compilation:** Change one function, and only that slice gets re-indexed. No more full-file or full-project re-parses.
* **Lock-Free Parallel Pipelines:** Unlocked, thread-safe type checking across top-level symbols so multi-threading feels effortless.
* **Linearized Heavy Passes:** Flattening complex passes like type checking into fast, batchable assembly-line operations.


---

## 📜 License

[MIT License](LICENSE) — Do whatever you want with the code, just keep my name on it!
