# Compile-time reflection and generated printing

`@type_info(T)` accepts either a type or an expression and returns `&TypeInfo()`. It inspects the expression's type without evaluating the expression. Metadata exists only during compilation: consume it in `cmpt` or in a string-producing macro. Runtime use produces a diagnostic.

```abyss
import print from "std/io"

User :: struct {
    name Str
    scores [3]i32
}

main :: fn() {
    user: User = .{ name: "Ada", scores: .[10, 20, 30] }
    #print(user)
    print("\n")
    count := cmpt { info := @type_info(User) info.field_count }
    print(count)
}
```

`#print(user)` expands the generic printer's macro template inside the caller. The argument is evaluated once. Struct fields, tuple positions, and fixed array elements become direct calls to primitive printing functions. Metadata, generator functions, heap allocations, traversal loops, and type dispatch for those shapes are absent from the emitted program, including builds with C optimization disabled. Adjacent punctuation and indentation are combined into string literals.

Ordinary `print(user)` uses the same compile-time generator in a function specialized for the argument type. It preserves the existing scalar and string printing behavior.

`TypeInfo()`, `FieldInfo()`, and `TypeKind` are available through the core prelude.

| Metadata field | Meaning |
| --- | --- |
| `id` | Compilation-local type identifier |
| `name` | Readable type name, including generic arguments |
| `kind` | `TypeKind`, with distinct struct, tuple, array, string, enum, union, pointer, and function cases |
| `bits` | Integer or floating-point width |
| `length` | Fixed array length |
| `fields`, `field_count` | Array of `&FieldInfo()` pointers and its length |
| `element` | Pointer target, array element, or enum backing type |
| `members`, `member_count` | Array of union member metadata pointers and its length |
| `params`, `param_count` | Array of function parameter metadata pointers and its length |
| `result`, `is_extern` | Function return metadata and external linkage flag |

A field has `name` and `type_info`. Enum variants also have `value`, their declared discriminant, and `type_info` describes the backing type. Fields of named structs follow declaration order; tuples follow numeric position. Recursive types share descriptors, so following pointer targets can lead back to an existing descriptor. Bound recursive metadata walks or track visited `id` values.

```abyss
field_names :: fn(info &TypeInfo()) Str {
    code := Str.empty()
    i: u64 = 0
    while i < info.field_count {
        field := *(info.fields + i)
        code = code.concat(`print("`).concat(field.name).concat(`\n")`)
        i = i + 1
    }
    code
}
```

Invoke it with `#field_names(@type_info(User))`. The loop executes in the compiler VM and emits one print statement per field. A metadata-derived `Str`, boolean, or integer can also be returned from `cmpt` and embedded into runtime code. Metadata pointers cannot escape compilation.

| Builtin | Result |
| --- | --- |
| `@type_name(T)` | Type name as `Str`, embedded as a string literal |
| `@field_type(T, "name")` | Field type |
| `@field_type(T, index)` | Field type at a constant index, using reflection order |
| `@element_type(T)` | Array element or pointer target type |
| `@type_from_id(id)` | Type reconstructed from a constant metadata identifier |

Type identifiers belong to one compilation. A generator can insert `@type_from_id(123)` into its generated source to refer to an anonymous type or a type outside the expansion's import scope. Do not persist identifiers across compilations.

A unit-returning function whose body consists of one dynamic macro is a macro template. Prefixing its call with `#` binds its arguments once and expands its generated body in the caller. This mechanism also works with aliases, inferred generic arguments, explicit type arguments, and multiple value arguments.

Printing covers numeric values, booleans, strings, nested structs, tuples, fixed arrays, enums, unions, function signatures, and pointers. Nested strings are quoted and escaped. Enum and union selection depends on runtime values and emits conditional branches. Arbitrary pointers print their type and numeric address without dereferencing them, which keeps null and recursive pointers safe; byte pointers retain C-string printing. A slice prints its pointer and length as struct fields. Traversing runtime-sized data, escaping dynamic strings, converting numbers, and performing I/O necessarily require runtime work. The VM retains its existing 64-bit value representation; full-width 128-bit numeric printing is available in native builds.
