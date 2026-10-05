# Getting Started & Philosophy

**Keep the data visible. Keep the machine close.** Abyss combines a small language core with explicit memory operations and a C backend. Its compiler is written in Abyss: the implementation exercises the same structs, pointers, methods, arenas, and vectors you use in a program.

This manual describes the source in this checkout. A token in the lexer, a type in an internal table, and a feature that survives C emission are three different levels of support. The chapters call out those distinctions, including unfinished behavior. Examples labeled *rejected* or *unsupported* illustrate a boundary; other examples use implemented forms. Small fragments belong inside a function unless a complete declaration or module is shown.

## Start with a complete function

```abyss
print :: (text &u8) unit

main :: () {
    print("Hello, Abyss!\n")
}
```

`print` has a signature and no body, so it is an external function. The project's `prelude.c` supplies its C definition. `main` has a body and no result annotation, so it returns `unit`. The [functions chapter](control-flow.md#external-functions-and-c-integration) explains this integration model.

The current root `main.a` is a compiler driver. It reads a file literally named `main.a`, finds its `main` symbol, emits C, prepends the file named `prelude.c`, and writes `tmp/out.c`. It does **not** parse command-line source paths. Consequently, `./abyssc program.a` is not a supported compile command for this driver.

To inspect the self-hosted pipeline from the repository root, the existing scripts use this sequence:

```sh
./abyssc
gcc -O3 -march=x86-64 -fomit-frame-pointer -funroll-loops tmp/out.c -o tmp/out
```

This writes build artifacts under the root `tmp/` directory; it compiles the current root `main.a`. The included executable is the available seed in this checkout. The README mentions `bootstrap.c`, but that file is absent here; do not assume a bootstrap source distribution exists locally.

For a separate program, use a working directory containing its own `main.a`, `prelude.c`, and writable `tmp/` directory, and invoke the compiler executable by its path. Imports are relative to the source file, so adjust paths to the standard library accordingly. The companion examples and verification command described [below](#building-and-checking-this-reference) demonstrate this arrangement under `docs/`.

## Radical simplicity, with concrete costs

The implementation has no language garbage collector, virtual method dispatch, class hierarchy, or operator-overloading system. A struct is data. A method resolves to a direct generated C function. Address-taking and casts appear in source.

```abyss
Pair :: struct {
    left: u32,
    right: u32
}

pair_sum :: (pair Pair) u32 {
    pair.left + pair.right
}
```

“Zero runtime overhead” describes the direction: the core does not insert a managed runtime or dynamic dispatch machinery. It is not a timing guarantee. C calls, copied structs, vector growth, arena chunks, string scans, and temporary receivers all have costs. The current prelude uses the C library for allocation, output, and files.

```abyss
import Arena from "std/allocator.a"

scratch :: () {
    arena := Arena.new(4096)
    bytes := arena.alloc(128)
    *bytes = 65 as u8
    arena.destroy()
}
```

The path assumes a source file at the repository root. [Memory & Standard Library Core](std.md) gives the lifetime and alignment rules; an arena is a policy you select, not an allocation strategy imposed on every value.

## A compiler built around tables

The compiler uses dense numeric IDs and parallel vectors. `TokenStream` stores token kinds, offsets, lengths, text views, and newline flags in separate typed vectors. HIR nodes and type data are also stored in tables. This organization keeps identity cheap to carry and makes passes explicit.

```abyss
import VecU32 from "std/vec/u32.a"

collect_ids :: () {
    ids := VecU32.new()
    ids.push(7)
    ids.push(19)
    first := ids.get(0)
    ids.destroy()
}
```

These are concrete typed containers, not a generic `Vec(T)` facility. The compiler's own usage is the best guide to supported idioms.

## From source to C

```text
source files
  -> lexer and import/definition indexer
  -> symbol-driven parser
  -> HIR tables and unification slots
  -> deferred call/member/operator checking
  -> reachable functions and required struct definitions
  -> C source plus the caller-supplied prelude
  -> C compiler and executable
```

The engine resolves declarations on demand. The backend emits names such as `sym_<id>`, `method_<id>`, and `_f<id>`, so emitted C is useful for tracing lowering but does not preserve all source names. External function names retain their spelling. Generated C includes prototypes, structs, functions, and a wrapper `int main(void)` that invokes the selected Abyss entry and returns zero.

C emission gives access to existing optimizers and debuggers. Portability still depends on the target: `i128`/`u128` map to C compiler extensions, layout follows emitted C, and library size helpers assume specific sizes. Do not treat “C99 target” as a promise of identical ABI on every machine.

## Building and checking this reference

```sh
mdbook build docs
node docs/verify.mjs
```

The first command builds the HTML book. The second checks chapter and generated HTML links, exercises the bundled Highlight.js grammar, and compiles and runs the companion Abyss example using the local seed compiler and GCC. Generated artifacts stay in `docs/.verification/`.

Both `abyss` and `a` Markdown fences select the same grammar:

```a
double :: (value u32) u32 {
    value * 2
}
```

The grammar loads after Highlight.js and before the theme's `book.js`, which performs highlighting. Character and compound-assignment tokens are colored for lexical readability even though their parser support is incomplete; highlighting is not a syntax validator.

## Implementation map

| Area | Ground truth |
| --- | --- |
| Tokens, keywords, literals | `compiler/token.a`, `compiler/lexer.a` |
| Grammar and precedence | `compiler/parser/core.a`, `parser.a`, `prec.a`, `env.a` |
| Types and identity | `compiler/types.a` |
| Unification and literal inference | `compiler/unify.a` |
| Checking, `Self`, receivers, casts | `compiler/typer/tyck.a` |
| Imports and lazy declarations | `compiler/indexer.a`, `compiler/engine.a` |
| Expression and function lowering | `compiler/lower.a` |
| C text emission and main wrapper | `compiler/codegen.a` |
| Allocation and collections | `std/allocator.a`, `std/string.a`, `std/vec/`, `std/avec/`, `std/slice/` |

Source paths identify repository files; they are not links to missing copies in the rendered book.
