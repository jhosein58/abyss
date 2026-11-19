# Idiomatic Abyss Coding Patterns & Standard Library

This guide provides practical patterns and best practices for writing idiomatic, high-performance Abyss code.

---

## 1. Strings: Prefer `Str` Over `&u8`

In Abyss, `Str` is a first-class slice structure representing a UTF-8 string:
```abyss
Str :: struct {
    ptr &u8
    len u64
}
```

### Best Practices:
1. **Function Parameters & Return Values**: Always use `Str` instead of `&u8`:
   ```abyss
   -- IDIOMATIC
   greet :: fn(name Str) {
       printf("Hello, {}!\n", .{name})
   }

   -- ANTI-PATTERN (avoid unless interfacing with C runtime)
   greet_raw :: fn(name &u8) { ... }
   ```
2. **String Literals**:
   - `"hello"` produces a `Str`.
   - `c"hello"` produces a null-terminated `&u8`.
3. **Conversions**:
   - `Str.of(cstr)` converts `&u8` to `Str` (calculating length).
   - `to_cstr(&arena, str)` converts `Str` to a null-terminated `&u8` allocated in the arena.

---

## 2. File I/O with `std.fs: File`

Always use the high-level `File` struct with `Str` paths:

```abyss
import std.fs: File
import std.allocator: Arena

read_and_process :: fn(path Str) {
    arena := Arena.new(65536)

    -- File.read_to_string accepts Str path and an Arena
    opt := File.read_to_string(&arena, path)
    if opt.is_some() {
        content := opt.unwrap()
        printf("Read {} bytes from {}\n", .{content.len, path})
    }

    -- File.write_string accepts Str path and Str content
    ok := File.write_string("output.txt", "Processed data\n")

    arena.destroy()
}
```

---

## 3. Memory Allocation: The Arena Idiom

Abyss favors scoped arena allocation over pervasive individual heap allocations:

```abyss
import std.allocator: Arena
import std.collections.vec: Vec

process_batch :: fn(items &Vec(Str)) {
    -- 1. Create a local scratch arena
    scratch := Arena.new(1024 * 1024)

    -- 2. Perform allocations using scratch
    temp_buf := scratch.alloc(4096)
    -- ...

    -- 3. Destroy all allocations in a single call (O(1))
    scratch.destroy()
}
```

---

## 4. Collections: `Vec(T)`

`Vec(T)` is a dynamically resizable array:

```abyss
import std.collections.vec: Vec

list := Vec(u32).new()
list.push(10)
list.push(20)
list.push(30)

i: u64 = 0
while i < list.len {
    val := list.get(i)
    printf("Item: {}\n", .{val})
    i = i + 1
}

list.destroy()
```

---

## 5. Optional Values: `Maybe(T)`

Use `Maybe(T)` (automatically available via `core/prelude`, no import needed) for fallible operations or optional values:

```abyss
-- Maybe is part of prelude - no import required!
find_value :: fn(key u32) Maybe(u32) {
    if key == 42 {
        ret Maybe(u32).some(100)
    }
    Maybe(u32).none()
}

val_opt := find_value(42)
if val_opt.is_some() {
    v := val_opt.unwrap()
    printf("Found: {}\n", .{v})
}
```

---

## 6. Formatting & Style Rules

1. **4 Spaces Indentation**: Never use tabs.
2. **Explicit Member Initialization**: Use `.{ field1: val1, field2: val2 }`.
3. **No Redundant Type Annotations**: Let the compiler infer local bindings with `:=`.
4. **Clean Codebase**: Maintain self-documenting naming (`is_open`, `try_load_source`, `resolve_module`).

---

## 7. The Prelude Ecosystem: Zero-Import Standard Types

Abyss automatically brings foundational primitives into scope via `core/prelude` and `std/prelude`. **Do NOT write explicit imports for these types:**

Symbol | Source | Purpose
:--- | :--- | :---
`Maybe(T)` | `core/prelude` | Optional value container (`some`, `none`, `unwrap_or`, `unwrap`).
`Str` | `core/prelude` | Native UTF-8 string slice struct.
`Vec(T)` | `std/prelude` | Dynamically growing heap array.
`HashSet(T)` / `HashMap(K, V)` | `std/prelude` | High-performance hash collections.
`printf` / `print` | `std/prelude` | Standard I/O output functions.
`Range`, `Slice` | `core/prelude` | Range iterators and memory slice views.

### Idiomatic `Maybe(T)` Usage:
```abyss
-- Functions returning optional results
find_user :: fn(id u32) Maybe(Str) {
    if id == 1 { ret Maybe(Str).some("Alice") }
    Maybe(Str).none()
}

-- Consuming with fallback
name := find_user(user_id).unwrap_or("Guest")

-- Conditional inspection
opt := find_user(user_id)
if opt.is_some() {
    printf("Found: {}\n", .{opt.unwrap()})
}
```
Avoid sentinel values like `(0 as &u8)` or special error codes when `Maybe(T)` expresses intent cleanly.

---

## 8. Iterators & Modern `for` Loops

Abyss provides structural iterators through `Iterator` and `RefIterator` from `core/prelude`. Any type with a `next(self &Self) Maybe(T)` method can be looped over directly with `for`.

### Implementing an Iterator:
```abyss
MyIter :: struct {
    items &Vec(u32)
    idx   u64

    next :: fn(self &Self) Maybe(u32) {
        if self.idx < self.items.len {
            val := self.items.get(self.idx)
            self.idx = self.idx + 1
            ret Maybe(u32).some(val)
        }
        Maybe(u32).none()
    }
}
```

### Iteration via `for`:
```abyss
-- Idiomatic, concise, and expressive:
for item in my_collection {
    process(item)
}
```
Prefer `for in` over verbose `while i < len` loops to keep logic clean, modern, and compact.

---

## 9. Automatic Dereferencing (Auto-Deref)

Abyss handles pointer dereferencing on member access automatically and deeply:

```abyss
-- IDIOMATIC:
user.address.city.len
node.next.next.val

-- ANTI-PATTERN (avoid manual clumsy dereferencing):
(*user).address.city.len
(*(*node).next).val
```
Never wrap pointers in parentheses with asterisks `(*a).b` for field access or method calls. The compiler cleanly resolves the dereferencing chain.

---

## 10. Optional Commas Across Newlines

Newlines serve as universal statement and element delimiters. Commas are completely optional whenever breaking to a new line:

```abyss
-- Multiline array without commas:
numbers := .[
    10
    20
    30
]

-- Multiline struct initialization without commas:
user := User.{
    id: 101
    name: "Alex"
    active: true
}

-- Multiline function calls without commas:
result := compute_heavy(
    arena
    primary_dataset
    configuration_flags
)

-- Multiline imports without commas:
import std.collections.hash_map:
    HashMap
    MapEntry
```
Omit commas across newlines to produce clean, vertical, modern Abyss code.

---

## 11. Deferred Type Inference in Practice

Abyss has a bidirectional constraint-unification engine. Avoid cluttering function bodies with redundant type annotations or explicit generic parameters:

```abyss
-- IDIOMATIC (Clean, lean, inferred):
list := Vec().new()
list.push(42)              -- Infers Vec(i32) automatically!

names := Vec().with_cap(8)
names.push("Alice")        -- Infers Vec(Str) automatically!

box := Box().new("Value")  -- Infers Box(Str) automatically!

-- ANTI-PATTERN (Verbose, redundant):
list: Vec(i32) = Vec(i32).new()
box: Box(Str) = Box(Str).new("Value")
```
Reserve explicit type annotations exclusively for boundary definitions: function parameters and function return types.

---

## 12. Metaprogramming & Reflection Recipes

### D-Style String Mixins with `#`:
The `#` splice operator takes any string expression (or function returning `Str`) and parses it directly into the AST at compile time:

```abyss
-- Simple expression splice:
x := #"100 + 200"

-- Multiline template splice via backticks:
#`
    print("Zero-cost metaprogramming\n")
`

-- Function-based code generator:
make_getter :: fn(name Str) Str {
    -- generate method source code
    "..."
}

#make_getter("id")
```

### Compile-Time Reflection (`@` Builtins) & Zero-Cost Formatting:
Combine `@type_info` with `#` splices to inspect types and generate specialized serialization/formatting code at compile time with zero runtime overhead:

```abyss
inspect_fields :: fn(T type) Str {
    info := @type_info(T)
    -- generate printing code for all fields at compile time
    "..."
}

#inspect_fields(User)
```

### Multi-Level Nested Splices:
Metaprogramming can be staged through multiple levels:
- `#code`: Level 1 splice (string -> AST).
- `##code`: Level 2 splice (unquotes string -> compiles string -> AST).
- `###code`, `####code`: Arbitrary-depth compile-time code synthesis.
- Use `cmpt` to force compile-time evaluation on the bytecode VM before codegen.
