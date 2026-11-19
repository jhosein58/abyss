# Abyss Grammar & Syntax Specification

This reference details the grammar, tokenization, type system, and AST/HIR rules of the Abyss programming language.

---

## 1. Keywords & Tokens

### Keywords
`fn`, `struct`, `enum`, `union`, `import`, `as`, `if`, `else`, `while`, `for`, `in`, `match`, `ret`, `break`, `cont`, `cmpt`, `Self`, `true`, `false`, `null`.

### Operators & Delimiters
- Comments: `--` (single-line only, up to the end of the line)
- Assignment: `:=` (declaration), `=` (assignment), `::` (compile-time binding / function definition)
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Logical: `and`, `or`, `not`
- Access: `.` (member access with automatic deep pointer dereferencing), `:` (type annotation / import symbol list), `::` (comptime / namespace)
- Sequencing: `,` (separates multiple expressions/statements on a single line, e.g. `{ a := 1, a + 2 }`; completely optional whenever breaking to a new line)
- Splice: `#` (compile-time D-style mixin splice; expands string expressions into AST; supports nested multi-level splices `##`, `###`, `####`)
- Builtins:
  - `@source_location()`: Returns a compile-time `SourceLocation` containing the source path and 1-based line and byte column. In generated code, the location is the enclosing splice invocation.
  - `@sizeof(T)`: Evaluates memory size of type `T` in bytes as `u64`.
  - `@typeof(x)`: Evaluates the static type of expression `x`.
  - `@is(val, Type)`: Tests if tagged union `val` currently holds variant `Type` (returns `bool`).
  - `@tag(val)`: Returns the numeric discriminant tag of tagged union `val` (`u32`).
  - `@type_name(T)`: Returns the canonical name of type `T` as a compile-time `Str` literal.
  - `@type_info(T)`: Returns a pointer `&TypeInfo()` with full compile-time/runtime reflection metadata.
  - `@type_from_id(id)`: Reconstructs a `type` from a compile-time integer type ID.
  - `@element_type(ArrayOrPtr)`: Extracts the underlying element type from an array `[N]T` or pointer `&T`.
  - `@field_type(StructOrTuple, name_or_index)`: Extracts the type of a struct field by name string or tuple field by index.
- Comptime keyword: `cmpt` (forces compile-time constant evaluation via bytecode VM for expressions, variables, or function parameters)

---

## 2. Imports Grammar

The import grammar is orthogonal, strict, and scoped:

```ebnf
ImportDecl ::= "import" ( RelativePath | AbsolutePath ) [ "as" Ident ] [ ":" SymbolList ]

RelativePath ::= "." Ident ( "." Ident )*
AbsolutePath ::= Ident ( "." Ident )*

SymbolList ::= SymbolEntry ( ( "," | Newline ) SymbolEntry )* [ "," ]
SymbolEntry ::= Ident [ "as" Ident ]
```

### Path Resolution
1. **Leading `.`**: Indicates a relative path starting from the directory of the current file.
2. **No leading `.`**: Absolute path rooted at the project workspace directory (e.g., `std`, `core`, `compiler`).
3. **Multiple dots (`..`)**: Forbidden by the compiler.
4. **Directory resolution**: If a path refers to a directory `<dir>`, the compiler automatically looks for `<dir>/<basename>.a` (e.g. `std.test` resolves to `std/test/test.a` if `std/test.a` does not exist).
5. **No `.a` in import path**: Import paths must omit the `.a` extension.
6. **Scoped Imports**: Can appear at top-level, or inside any block `{ ... }`, function `fn`, or struct declaration, shadowing higher-level symbols only within that scope.

---

## 3. Types

### Primitive Types
- Unsigned integers: `u8`, `u16`, `u32`, `u64`
- Signed integers: `i8`, `i16`, `i32`, `i64`
- Floating-point: `f32`, `f64`
- Boolean: `bool`
- Void: `void`
- Untyped constants: untyped int, untyped float, untyped str (coerced during type checking).
  - *Note on characters:* Abyss has **no** `char` primitive type. Character literals like `'x'` are compile-time `untyped int` and coerce to any integer or float.

### Compound Types
- **Pointers**: `&T` (raw memory reference)
- **Functions**: `fn(A, B) C`
- **Fixed Arrays**: `[N]T` (e.g. `[4]i32`, initialized with `.[1, 2, 3, 4]`)
- **Slices**: `[]T` or `Slice(T)`
- **Tuples**: `struct { T1, T2 }` (initialized with `.{ v1, v2 }`, indexed with `.0`, `.1`, destructuring `a, b := tuple`)
- **Structs**: `struct { field1 T1, field2 T2 }` (NO colons in field definitions; grouped fields `x, y i32` supported)
- **Enums**: `enum [ReprType] { A, B, C }`
- **Tagged Unions (Sum Types)**: `T1 | T2` (checked via `@is(val, T)`, downcast via `val as T`, tag via `@tag(val)`)
- **Type Functions (Generics)**: `fn(T type) type { struct { ... } }` (instantiated via `TypeFn(T)`)

---

## 4. Struct & Namespace Semantics

Structs serve both as data containers and as orthogonal namespaces:

```abyss
-- Data Struct
Vector2 :: struct {
    x f32
    y f32

    new :: fn(x f32, y f32) Self {
        .{ x: x, y: y }
    }

    add :: fn(self &Self, other &Vector2) Vector2 {
        .{ x: self.x + other.x, y: self.y + other.y }
    }
}

-- Namespace Struct (Zero Fields)
Math :: struct {
    PI :: 3.141592653589793

    abs :: fn(x f32) f32 {
        if x < 0.0 { -x } else { x }
    }
}
```

- When an imported module has no symbol list (`import std.math`), the module is loaded as a struct whose fields are the module's exported declarations.
- Method calls `val.method(args)` are syntactic sugar for `Type.method(&val, args)` when the first parameter is `self &Self`.

---

## 5. Match Expressions (Enums & Tagged Unions Only)

> [!IMPORTANT]
> In Abyss, `match` is **strictly restricted to Enum and Tagged Union values** (`TyKind.enum_` and `TyKind.union_`). It is **not** a generic C-style `switch` and **cannot** match on integers (`u8`, `i32`, `u64`), floats, or strings. Matching on any other type produces a compiler error: `match requires an enum or union value`.

### 1. Enum Matching
- Patterns are enum variant names.
- Multiple variants can be grouped in a single arm with commas `,`.
- Must be exhaustive (cover all variants or provide an `else` arm).

```abyss
Color :: enum { red, green, blue }

match c {
    red { 10 }
    green, blue { 20 }
}
```

### 2. Tagged Union Matching with Binding (`match x in y`)
- Binds `x` to the inner payload value narrowed to the matched type within the arm:

```abyss
match val in (u: i32 | bool) {
    i32  { print_i32(val) }
    bool { print_bool(val) }
}
```
*Note: Binding cannot be used in arms with multiple comma-separated types.*

### 3. Tagged Union Matching without Binding
- Groups multiple types together or uses `else`:

```abyss
match u {
    i32, f32 { handle_number() }
    else     { handle_other() }
}
```

### 4. Syntax Rules:
- Arm bodies use `{ ... }` blocks (never `=>`).
- Separated by newlines or commas (never semicolons `;`).
- Exactly one `else` arm allowed.
