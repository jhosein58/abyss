# Structs & Data Layout

A struct groups typed fields into one value. Named declarations establish identity; literals provide data. Methods resolve through that identity but do not add fields or dispatch tables to the object.

## Declare data directly

```abyss
Point :: struct {
    x: f32,
    y: f32
}

Segment :: struct {
    start: Point,
    end: Point
}
```

Each field uses `name: Type`. A field type must resolve to a type value. Commas are accepted between fields and after the last field. Newline-separated fields without commas occur throughout the library too.

The parser rejects duplicate field names, duplicate method names, and collisions between a field and a method. There is no field-default expression, visibility modifier, inheritance clause, packed attribute, or explicit alignment attribute in this grammar.

## Initialize with a named-field literal

The initializer starts with a dot followed by a brace: `.{ ... }`. It does not start with the type name. Give the literal an expected type through an annotation, function result, assignment, or argument.

```abyss
origin :: () Point {
    .{ x: 0.0, y: 0.0 }
}

construct :: () {
    point: Point = .{ y: 2.0, x: 1.0 }
    line: Segment = .{
        start: .{ x: 0.0, y: 0.0 },
        end: point
    }
}
```

For an expected named type, `check_value` verifies the field count, resolves every field name, checks every value against the required field type, and records the nominal type. Literal order can differ from declaration order because the checker matches by name and C emission uses designated initializers.

All fields must be supplied. Extra, missing, or repeated fields are errors. There is no spread syntax, shorthand `.{ x }`, or partial-update initializer.

## Anonymous literals and expected types

Without an expected nominal type, a literal begins as a structural record with inferred fields:

```abyss
anonymous_sum :: () u32 {
    values := .{ left: 3 as u32, right: 4 as u32 }
    values.left + values.right
}
```

This record has fields but no named declaration that owns methods. To use methods or pass a particular named type, supply that expected type when constructing the literal. An existing structural or different nominal variable is not automatically reclassified by assigning it to a named type.

```abyss
consume :: (point Point) f32 {
    point.x + point.y
}

pass_literal :: () f32 {
    consume(.{ x: 1.0, y: 2.0 })
}
```

## Read, write, and copy

Field access uses `value.field`. Fields are writable through a mutable local or pointer; pointer receiver layers are followed automatically.

```abyss
translate :: (point &Point, dx, dy f32) {
    point.x = point.x + dx
    (*point).y = (*point).y + dy
}

copy_point :: (source Point) Point {
    copy := source
    copy.x = copy.x + 1.0
    copy
}
```

Struct parameters and assignments lower to C value operations. Copying a struct copies its fields, including pointer values. It does not clone the memory referenced by a pointer. This matters for vectors, strings, and arenas: copying their headers can create shared storage and multiple apparent owners. [Memory](std.md#ownership-rules-to-keep-in-view) explains the consequences.

There is no struct `==` operation. Compare selected fields or write a function:

```abyss
point_equal :: (left, right Point) bool {
    (left.x == right.x) and (left.y == right.y)
}
```

## Zero-initialized locals

An annotated local with no initializer emits C initialization with `{0}`:

```abyss
zero_point :: () Point {
    point: Point
    point
}
```

For this example both fields become zero. This is a backend initialization rule, not a constructor call or a field-default mechanism. A zeroed arena or vector header is not necessarily a valid replacement for its `new()` constructor.

## Actual layout: C, after field canonicalization

`TypeStorage_alloc_struct` sorts fields by **interned name ID** before storing and interning a structural type. The C lowerer emits fields in that stored order, with names `_f<name-id>`. This is neither an alphabetical-layout contract nor a source-declaration-order contract: name IDs depend on interning history.

```abyss
Header :: struct {
    tag: u8,
    count: u64,
    active: bool
}
```

The generated C definition uses the resolved field order. Its target C compiler determines padding, alignment, and total size. Abyss exposes no general source `sizeof`, `alignof`, or `offsetof` operator here. Standard-library helpers such as `Str_size()` and `ArenaChunk.size()` return hard-coded values; they do not query this type's layout.

By-value nested structs are ordered for emission by the backend's type dependency sort. Pointer fields remain C pointers. These mechanisms support the existing generated program; they do not make arbitrary hand-written C structs interchangeable with Abyss structs. For C ABI work, inspect the emitted definition and target assumptions.

## Nominal identity and aliases

```abyss
Position :: struct { value: u32 }
Velocity :: struct { value: u32 }
PositionAlias :: Position
```

`Position` and `Velocity` have separate nominal origins even though their shapes match. `PositionAlias` preserves the identity of `Position`. A struct cast does not erase or convert identity; the cast checker accepts scalar and pointer categories, not record conversions.

Methods require a named struct binding. An anonymous struct with methods is rejected by checking. Nested struct declarations inside a struct method are rejected by parsing. See [Methods & Dogfooding](methods.md) for `Self`, static methods, and receiver ownership.

**Implementation notes:** `compiler/parser/parser.a` (`parse_struct`, `parse_struct_init`, `ensure_unique_field`), `compiler/typer/tyck.a` (`synth_var_or_binding`, `check_value`, `synth_struct_init`), `compiler/types.a` (`sort_struct_fields`, `TypeStorage_alloc_nominal`), and `compiler/lower.a` (`lower_expr`, `lower_function`).
