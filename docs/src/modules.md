# Modules & Project Architecture

The file is the module boundary. Imports select explicit names from another source file. Indexing discovers declarations and import edges; the engine parses and checks requested symbols as their types are needed.

## Import explicit names

```abyss
import Arena from "std/allocator.a"
import Str, Str_from, Str_eq from "std/string.a"

same_text :: (text Str) bool {
    Str_eq(text, Str_from("Abyss"))
}
```

The file path is quoted. A grouped import creates local names for the selected symbols; it does not create a namespace object. The standard library paths in this fragment assume a source file at repository root. Imports must be at module scope: they are processed by the indexer, not the expression parser.

Long lists can span lines:

```abyss
import String, String_new, String_push_str,
       String_as_str from "std/string.a"
```

The indexer accepts identifier/comma tokens until `from`. Write commas even though the scan is more permissive than a formal list grammar. Missing names or a missing quoted path are errors.

There are no wildcard imports, package search paths, versioned module specifications, or `as` clauses in the import grammar. `as` belongs to value casting.

## Rename a symbol

The explicit local alias form is:

```abyss
Pool :: import Arena from "std/allocator.a"

make_pool :: () Pool {
    Pool.new(4096)
}
```

The indexer records the local name, remote name, and target file, then allocates a symbol alias when the remote symbol is known. A type alias preserves nominal identity; it does not create another struct declaration. Use this form for a defined function or type. External C function aliases have the [name-emission limitation](control-flow.md#external-functions-and-c-integration) described in the functions chapter.

## Paths are resolved from the importer

Suppose the source file `app/reader.a` imports `../std/string.a`. The indexer combines the importing directory with that path, removes `.` segments, and collapses resolvable `..` segments. It compares normalized, interned path strings to avoid loading the same target repeatedly during traversal.

```abyss
-- Inside app/reader.a
import Str from "../std/string.a"

read_byte :: (text Str, offset u64) u8 {
    *(text.ptr + offset)
}
```

This is string-based slash path handling, not a filesystem realpath resolver. It does not establish symlink identity or a robust absolute-path/cross-platform path contract. Use relative slash-separated paths as the existing project does. Normalization is per indexing traversal; do not assume a persistent global module cache.

The loader uses `read_file`. An inaccessible file produces its file error and terminates through `fall()`. A file that loads successfully but lacks an imported name eventually produces `symbol not found` from import resolution.

## Discovery before parsing

`index_file` maintains file and token-range queues. It scans top-level names followed by `::` and records their declaration token spans. Import processing enqueues additional files without immediately parsing every definition.

After scanning the reachable import graph, it repeatedly tries pending imports. Each newly resolved name can make another alias available; the pass continues until no progress remains. Remaining unresolved imports are errors. This permits discovery through import/alias chains, but is not a proof that every cyclic value dependency can be checked.

```abyss
-- arithmetic.a
add :: (left, right u32) u32 {
    left + right
}
```

```abyss
-- main.a beside arithmetic.a
import add from "arithmetic.a"

main :: () {
    answer := add(20, 22)
}
```

The engine can know `add` exists before parsing its function body. `Engine_slot_of` follows aliases to the canonical symbol, ensures parsing/checking as needed, and exposes its type slot.

## What incremental resolution means here

There are three separate mechanisms:

1. File traversal avoids repeated loads for already recorded paths in that indexing operation.
2. Declaration resolution requests types lazily, with an “already resolving” marker to limit re-entry.
3. Code emission visits reachable functions and their method targets, tracking emitted items.

```abyss
root :: () u32 {
    helper()
}

helper :: () u32 { 42 }
```

Top-level symbol indexing allows the reference to a declaration appearing later in the file. This is a symbol-resolution design, not disk-backed incremental compilation, a package manager, or automatic whole-project testing. Do not infer that every unreachable parser/type/backend defect is validated by successfully compiling one entry.

Some graph errors appear at different stages. Every indexed import must resolve, while a body is ordinarily parsed when the engine requests its symbol. An imported type's methods are checked through the owning declaration; unsupported lowerer paths may surface only when reached for emission.

## Organize by data and operation

Keep a named struct with its inline methods, and import that type where needed. Free utility functions require their own explicit imports. A method name alone is not an exported top-level function.

```abyss
import VecU32 from "std/vec/u32.a"

make_ids :: () VecU32 {
    VecU32.new()
}
```

This needs `VecU32`; it does not need an invented `VecU32_new` declaration. In contrast, `String_new` is a real free function in `std/string.a`, so importing `String` alone does not supply that symbol.

There are no `pub`/`private` declarations in this grammar. Names are selected explicitly at import sites; underscore naming is a convention, not an access-control rule. The standard library specializes slices and vectors with separate concrete files and type aliases. Its “Generics” comments do not define a source generic mechanism.

**Implementation notes:** `compiler/indexer.a` (`path_resolve`, `resolve_and_load_file`, `scan_file_symbols`, `index_file`), `compiler/engine.a` (`Engine_ensure_resolved`, `Engine_slot_of`, `Engine_compile`), and `compiler/parser/env.a` (`ScopeEnv_lookup`).
