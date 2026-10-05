# Abyss Language Specification & Technical Reference Manual

Abyss is a minimal, data-oriented systems programming language featuring a Pratt parser, bidirectional type inference via Robinson unification, nominal type equivalence, compile-time symbol indexing, and direct C code generation.

---

## 1. Declarations & Bindings

Declarations separate compile-time bindings (`::`) from mutable runtime storage (`:=` and `:`).

| Syntax | Semantic | Lowering & Runtime Behavior |
| :--- | :--- | :--- |
| `Name :: expr` | Compile-time binding | Binds types, structs, or functions. Top-level scalar constants reaching runtime code are unsupported (use constant functions). |
| `name := expr` | Inferred mutable local | Allocates local variable; type is inferred from `expr`. |
| `name: Type = expr` | Annotated mutable local | Allocates local variable; unifies `expr` with `Type`. |
| `name: Type` | Uninitialized local | Lowers to C zero-initialization (`{0}`). |
| `name = expr` | Assignment statement | Mutates existing lvalue. |

```abyss
-- Type alias (transparent/structural equivalence)
Index :: u32

-- Compile-time constant function (preferred over runtime global constants)
default_cap :: () u32 { 16 }

-- Local variable bindings
setup :: () {
    a := 10            -- Inferred as untyped_int -> resolved to target context or i32
    b: u64 = 100       -- Explicitly typed
    c: u32             -- Zero-initialized (c == 0)
    c = 42             -- Reassignment
}

```

* **Scope & Shadowing:** Identifiers inside nested `{ ... }` blocks shadow outer declarations. Scopes restore on block exit.
* **Comments:** Single-line comments begin with `--` and continue to the newline. No block comment syntax exists.
* **Omitted Features:** No `let`, `var`, `fn`, `const`, or `mut` keywords.

---

## 2. Type System & Memory Primitives

### Built-in Types

| Category | Type Identifier | Emitted C Type | Width / Behavior |
| --- | --- | --- | --- |
| **Unsigned Int** | `u8`, `u16`, `u32`, `u64` | `uint8_t`, `uint16_t`, `uint32_t`, `uint64_t` | Fixed width |
| **Signed Int** | `i8`, `i16`, `i32`, `i64` | `int8_t`, `int16_t`, `int32_t`, `int64_t` | Two's-complement |
| **128-bit Int** | `u128`, `i128` | `unsigned __int128`, `__int128` | Requires C compiler extension |
| **Floating Point** | `f32`, `f64` | `float`, `double` | IEEE 754 |
| **Boolean** | `bool` | `bool` | `true`, `false` |
| **Unit** | `unit` | `void` | Zero-byte value. Illegal as parameter or local variable. |
| **Pointer** | `&T` | `T*` | Unmanaged raw memory address. |

### Literals & Inference Bounds

* **Integer Literals:** Parsed as decimal or hexadecimal (`0xFF`, `0xAB_CD`). Digits allow `_` separators. The parser token accumulator is limited to **`u32` max (`4,294,967,295`)**. Literals wider than 32 bits must be constructed using typed expressions: `(1 as u64) << (40 as u64)`.
* **Inference Fallback:** Untyped integers default to `int32_t`; untyped floats default to `double`.
* **Strings:** Double-quoted string literals (`"..."`) lower to C string literals cast to `((uint8_t*)"...")`. Their type is `&u8`.
* **Character Literals:** Single-quoted characters (`'A'`) are not implemented in the parser prefix dispatch; use byte dereference `*"A"` or numeric byte literals (`65 as u8`).

### Pointers & Address Operations

* **Address-Of (`&`):** Takes address of an lvalue (`&variable`).
* **Dereference (`*`):** Accesses memory (`*ptr = value`).
* **Null Pointer:** Represented via explicit zero-cast: `0 as &T`.
* **Pointer Arithmetic:** Operations `ptr + int`, `int + ptr`, and `ptr - int` operate in **stride units of `sizeof(T)**`, lowering directly to C pointer arithmetic. Byte offsets require casting to `&u8`.
* **Automatic Member Dereference:** `ptr.field` transparently dereferences arbitrary pointer indirection layers down to the base struct.

### Explicit Casting (`as`)

Conversions are never implicit across differing scalar widths or representations.

```abyss
raw_byte: u8 = 255
widened := raw_byte as u32
int_ptr := 0x1000 as &u32
back_int := int_ptr as u64
is_valid := int_ptr as bool

```

* **Valid Casts:** `Numeric/Bool <-> Numeric/Bool`, `Pointer <-> Pointer`, `Integer <-> Pointer`, `Pointer <-> Integer`, `Pointer -> Bool`.
* **Invalid Casts:** `Float/Bool <-> Pointer`, `Struct <-> Struct`, `Any -> unit`.

---

## 3. Operator Precedence & Syntax Rules

Binary expressions are left-associative; `:` and `=` are right-associative.

| Precedence | Operators | Category / Notes |
| --- | --- | --- |
| **160** | `.` | Member access, method calls |
| **150** | `(...)` | Function invocation |
| **140** | Prefix `-`, `not`, `~`, `&`, `*` | Unary negation, boolean not, bitwise not, address-of, deref |
| **130** | `as` | Type cast |
| **120** | `*`, `/`, `%` | Multiplicative (integers/floats; `%` is integer-only in C backend) |
| **110** | `+`, `-` | Additive / pointer arithmetic |
| **100** | `<<`, `>>` | Bitwise shift |
| **90** | `&` | Bitwise AND |
| **80** | `^` | Bitwise XOR |
| **70** | `|` | Bitwise OR |
| **60** | `==`, `!=`, `<`, `<=`, `>`, `>=` | Relational / Equality |
| **40** | `and` | Logical AND (short-circuit) |
| **30** | `or` | Logical OR (short-circuit) |
| **15** | `:` | Type annotation / variable declaration |
| **10** | `=` | Variable assignment |
| **5** | `::` | Compile-time binding |

* **Logical Keywords:** Use `not`, `and`, `or`. Operators `!`, `&&`, `||` are illegal tokens.
* **Compound Assignment:** Operators `+=`, `-=`, etc. are unsupported by the parser. Write `x = x + 1`.
* **Line Continuation:** Newlines terminate an expression unless the next token is a soft infix operator (`+`, `-`, `/`, `%`, `==`, `!=`, `<`, `>`, `and`, `or`, `as`, `::`, `:`, `=`). Member `.` and call `(` cannot begin on a new line.

---

## 4. Control Flow & Functions

### Function Signatures & ABI

Functions are declared using `name :: (param1 T1, param2 T2) ReturnType { ... }`.

```abyss
-- Multiple parameters sharing a trailing type
between :: (val, low, high u32) bool {
    (val >= low) and (val < high)
}

-- Unit function (explicit or omitted return type)
log_msg :: (msg &u8) {
    print(msg)
}

```

* **Trailing Expression Return:** The final expression in a block serves as the return value.
* **Early Return (`ret`):** `ret expr` returns early. `ret` followed immediately by a newline produces a bare return (`unit` or `_self`).
* **External C FFI:** Signatures **lacking a body block** resolve to external C symbols matching the identifier name:
```abyss
print :: (s &u8) unit
malloc :: (size u64) &u8

```


* **Entry Point:** The compiler looks for `main :: ()` returning `unit`.

### Conditionals (`if` / `else`)

`if` is parsed within the expression grammar, but **evaluates to `unit` in the current backend**. It cannot be used as a value expression initializing a variable.

```abyss
-- Correct: mutating state or early return
classify :: (v i32) i32 {
    res: i32 = 0
    if v < 0 {
        res = -1
    } else if v > 0 {
        res = 1
    }
    res
}

-- UNSUPPORTED in current compiler:
-- val := if cond { 10 } else { 20 }

```

### Iteration (`while`)

`while cond { ... }` executes while the boolean condition evaluates to `true`. Evaluates to `unit`.

* `break`: Exits nearest loop.
* `cont`: Skips to next iteration.

```abyss
sum_to :: (limit u32) u32 {
    total: u32 = 0
    i: u32 = 0
    while i < limit {
        i = i + 1
        if i % 2 == 0 { cont }
        total = total + i
    }
    total
}

```

---

## 5. Structs & Nominal Type System

### Struct Declarations

Structs group typed fields. A struct bound to a name via `::` constitutes a distinct **nominal type**.

```abyss
Point :: struct {
    x: f32,
    y: f32
}

```

* **Field Layout:** Struct fields are sorted internally by **interned name ID** prior to C emission. C alignment, padding, and field ordering follow the generated `_f<id>` layout.
* **Equality:** Structs do not support `==` or `!=`. Fields must be compared explicitly.
* **Zero-Init:** `p: Point` without an initializer generates C `{0}`.

### Anonymous Struct Literals (`.{ ... }`)

Instantiated with `.{ field1: val1, field2: val2 }`. Fields may be passed in any order; the compiler matches them by name against the expected nominal target.

```abyss
-- Coerced to nominal Point via explicit annotation
origin: Point = .{ y: 0.0, x: 0.0 }

-- Return value coercion
make_point :: () Point {
    .{ x: 1.0, y: 2.0 }
}

```

---

## 6. Methods & The `&Self` Chaining Convention

Methods must be declared inside a named struct body using `::`.

```abyss
Counter :: struct {
    val: u32,

    -- Static Method (No implicit leading dot in body)
    new :: (init u32) Self {
        .{ val: init }
    }

    -- Instance Method (Contains implicit leading dot .val)
    add :: (n u32) &Self {
        .val = .val + n
        -- Returns &Self implicitly
    }

    read :: () u32 {
        .val
    }
}

```

### Static vs. Instance Dispatch Rules

1. **Receiver Detection:** A method is classified as an **instance method** if and only if its HIR contains an implicit leading-dot access (`.field` or `.method()`).
2. **Static Method:** A method containing no leading-dot accesses is static. It must be called through the type name: `Counter.new(0)`.
3. **Implicit Instance Receiver (`_self`):** At the call site (`instance.add(5)`), the compiler passes `_self` automatically:
* Value instance -> address is taken (`&instance`).
* Pointer instance -> pointer is forwarded directly.
* Non-addressable temporary -> materialized in a compound literal array `(&((T[]){expr})[0])`.


4. **The `&Self` Return Convention:** If an instance method returns a pointer to its enclosing struct (`&Self` or `&Type`), the compiler:
* Emits `return _self;` at fallthrough.
* Emits `return _self;` on bare `ret`.
* Allows fluent method chaining: `c.add(1).add(2).read()`.



---

## 7. Modules & Import System

The file system defines the module boundary. Compilation resolves declarations on demand via symbol indexing.

```abyss
-- Sibling import
import VecU32 from "std/vec/u32.a"

-- Multi-symbol relative import
import Str, Str_from, Str_eq from "../std/string.a"

-- Renaming / Symbol alias
ArenaPool :: import Arena from "std/allocator.a"

```

* **Resolution:** Paths are resolved relative to the importing file using slash delimiters (`./`, `../`).
* **Struct Imports:** Importing a named struct automatically imports all associated static and instance methods.
* **No Wildcards:** Wildcard imports (`*`) and `import ... as ...` are unsupported.

---

## 8. Standard Library Core Reference

### 8.1. Memory & Allocators (`std/allocator.a`, `std/mem.a`)

#### `Arena` (`std/allocator.a`)

Linear chunk-based allocator. Alignment is aligned to 8-byte boundaries via `(size + 7) & ~7`.

```abyss
ArenaChunk :: struct {
    next_chunk: &u8,
    memory: &u8,
    offset: u64,
    cap: u64
}

Arena :: struct {
    current: &ArenaChunk,
    default_chunk_size: u64
}

```

| Method / Signature | Semantic / Preconditions |
| --- | --- |
| `Arena.new(default_chunk_size u64) Arena` | Allocates initial chunk buffer via C `malloc`. |
| `alloc(size u64) &u8` | Returns 8-byte aligned raw pointer; prepends new chunk if size exceeds space. |
| `realloc(old &u8, old_size u64, new_size u64) &u8` | Extends allocation if last in chunk; otherwise allocates new block and copies bytes. |
| `reset() unit` | Sets chunk offset to zero across all chained chunks. Retains memory buffers. |
| `destroy() unit` | Traverses chunk linked-list and releases all buffers via C `free`. |

#### Raw Memory (`std/mem.a`)

* `malloc(size u64) &u8`: Allocates heap memory.
* `realloc(ptr &u8, size u64) &u8`: Resizes heap allocation.
* `free(ptr &u8) unit`: Releases heap memory.
* `mem_copy(dest &u8, src &u8, n u64) unit`: Copies memory bytes (prelude limits single copies to < 100 MiB).
* `mem_set(dest &u8, val u8, n u64) unit`: Fills memory block.

---

### 8.2. Heap Vectors (`std/vec/*.a`)

Concrete implementations: `VecU8`, `VecU32`, `VecI32`, `VecBool`, `VecStr`. Elements allocate on heap via `malloc`/`realloc` and release via `free`.

```abyss
VecT :: struct {
    ptr: &T,
    len: u64,
    cap: u64
}

```

| Method Signature | Behavior & Preconditions |
| --- | --- |
| `VecT.new() Self` | Constructs empty vector with capacity clamped to 4 (`with_cap(4)`). |
| `VecT.with_cap(capacity u64) Self` | Allocates element buffer; sets initial capacity (minimum 4). |
| `reserve(additional u64) &Self` | Ensures space for `len + additional`. Doubles capacity on overflow. |
| `grow() &Self` | Reserves 1 additional slot when `len == cap`. |
| `push(value T) &Self` | Appends element; triggers reallocation if capacity is reached. |
| `get(index u64) T` | Unchecked element access: `*(.ptr + index)`. |
| `set(index u64, value T) unit` | Unchecked element assignment: `*(.ptr + index) = value`. |
| `pop() T` | Decrements `len` and returns element at `len`. Unchecked on empty vector. |
| `last() T` | Reads element at index `len - 1`. Unchecked on empty vector. |
| `is_empty() bool` | Returns `.len == 0`. |
| `extend(src &T, count u64) &Self` | Reserves `count` elements and copies from `src`. Source must not overlap vector. |
| `mark() u64` | Returns current length as logical marker. |
| `reset(mark u64) unit` | Shrinks vector length to `mark` without releasing capacity. |
| `clear() unit` | Logically empties vector. **Note:** Only `VecU32` implements `.len = 0` correctly in this checkout; for other vector types use `.reset(0)`. |
| `ptr_at(offset u64) &T` | Returns raw pointer offset: `.ptr + offset`. |
| `swap_remove(index u64) T` | Replaces slot at `index` with last element and decrements length. Unchecked. |
| `as_slice() SliceT` | Returns non-owning slice `{ ptr: .ptr, len: .len }`. |
| `destroy() unit` | Frees buffer via C `free`; zeros pointer, length, and capacity fields. |

---

### 8.3. Arena Vectors (`std/avec/*.a`)

Modules: `ArenaVecU8` (`std/avec/u8.a`), `ArenaVecI32` (`std/avec/i32.a`). Procedural API taking explicit pointer receivers; memory is bound to the lifespan of the underlying `Arena`.

| Function Signature | Description |
| --- | --- |
| `ArenaVecT_new(arena &Arena) ArenaVecT` | Initializes vector backed by arena with default capacity 4. |
| `ArenaVecT_with_cap(arena &Arena, cap u64) ArenaVecT` | Initializes vector backed by arena with specified capacity. |
| `ArenaVecT_push(v &ArenaVecT, val T) unit` | Appends element; delegates growth to `arena.realloc`. |
| `ArenaVecT_get(v &ArenaVecT, index u64) T` | Unchecked element read. |
| `ArenaVecT_set(v &ArenaVecT, index u64, val T) unit` | Unchecked element write. |
| `ArenaVecT_pop(v &ArenaVecT) T` | Decrements length and returns popped element. |
| `ArenaVecT_clear(v &ArenaVecT) unit` | Resets `len = 0` while retaining allocated buffer. |

---

### 8.4. Slices & String Primitives (`std/slice/*.a`, `std/string.a`)

#### Slices (`std/slice/*.a`)

Modules: `SliceU8`, `SliceU32`, `SliceI32`, `SliceBool`, `SliceStr`.

* **Layout:** Struct containing `ptr: &T, len: u64`.
* **Indexing:** No `slice[i]` syntax. Use pointer arithmetic: `*(slice.ptr + i)`.
* **Initialization:** Construct via struct literals (`.{ ptr: p, len: l }`) or `vec.as_slice()`. Avoid `Slice*_new` constructors in this revision.

#### String Views & Builders (`std/string.a`)

* **`Str`:** Immutable borrowed byte view `{ ptr: &u8, len: u64 }`. Does not own memory.
* `Str.of(c_str &u8) Str` / `Str_from(c_str &u8) Str`: Constructs view by measuring zero-terminated string length.
* `Str_sub(s Str, start u64, len u64) Str`: Unchecked sub-slice borrowing bytes.
* `Str_eq(a Str, b Str) bool`: Returns `true` if lengths and byte sequences match.
* `s.print() unit` / `Str_println(s Str) unit`: Outputs byte contents to `stdout`.


* **`String`:** Arena-backed growable string builder wrapping `ArenaVecU8`.
* `String_new(arena &Arena) String`: Constructs empty string builder.
* `String_push_str(s &String, view Str) unit`: Appends byte slice.
* `String_push_char(s &String, ch u8) unit`: Appends single byte.
* `String_push_u32(s &String, val u32) unit`: Appends decimal ASCII representation.
* `String_as_str(s &String) Str`: Returns borrowed `Str` view of accumulated bytes (not null-terminated).



---

### 8.5. System & File Operations (`std/fs.a`, `std/sys.a`)

* `read_file(arena &Arena, path &u8) Str`: Reads entire file into arena-allocated memory, appends terminating zero, and returns `Str`. Halts via `fall()` on error.
* `write_file(path &u8, content Str) bool`: Writes bytes to file path via C `fopen`/`fwrite`. Returns `true` on success.
* `append_file(path &u8, content Str) bool`: Appends bytes to file path.
* `fall() unit`: Halts execution immediately (maps to `abyss_fall` in `prelude.c`).

