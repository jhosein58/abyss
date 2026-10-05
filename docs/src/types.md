# Type System

The checker assigns types through unification slots. Concrete numeric widths remain distinct; contextual inference resolves literals and anonymous struct fields. The result is a small static system with visible conversions and raw pointers.

## Scalars and C mappings

| Abyss | Emitted C | Role |
| --- | --- | --- |
| `u8`, `u16`, `u32`, `u64` | `uint8_t`, `uint16_t`, `uint32_t`, `uint64_t` | Unsigned integers |
| `i8`, `i16`, `i32`, `i64` | `int8_t`, `int16_t`, `int32_t`, `int64_t` | Signed integers |
| `u128` | `unsigned __int128` | Extension to the C target |
| `i128` | `__int128` | Extension to the C target |
| `f32` | `float` | Floating point |
| `f64` | `double` | Floating point |
| `bool` | `bool` | `true` or `false` |
| `unit` | `void` | No runtime value |
| `&T` | C spelling of `T` followed by `*` | Raw pointer |

```abyss
scalars :: () {
    byte: u8 = 255
    offset: i32 = -12
    count: u64 = 1_024
    large: u128 = (1 as u128) << (80 as u128)
    fraction: f32 = 0.25
    enabled: bool = true
}
```

The builtin type recognizer accepts integer widths 8, 16, 32, 64, and 128, and only floating widths 32 and 64. `f16`, `usize`, `isize`, and other widths are not builtin source types. A prelude function such as `print_f16` does not establish an Abyss `f16` type.

128-bit support requires a C compiler and target supporting `__int128`. The integer literal parser still has a [u32-sized limit](syntax.md#integer-and-floating-literals).

## Literal inference and concrete values

Integer literals initially have the internal type `untyped_int`; decimal floats have `untyped_float`. An integer literal can unify with a signed, unsigned, or floating numeric context. A floating literal can unify with a concrete floating type. It cannot implicitly become an integer.

```abyss
scale :: (factor f32) f32 {
    factor * 2
}

add_byte :: (value u8) u8 {
    value + 1
}
```

Here `2` and `1` take the concrete type required by the other operand. This rule does not insert a conversion between two independently typed variables:

```abyss
add_widths :: (small u8, large u32) u32 {
    (small as u32) + large
}
```

Without the `as u32`, `u8` and `u32` fail to unify. The same is true of `i32` with `u32`, and `f32` with `f64`.

When no other context resolves an untyped numeric value, the backend spells an integer as `int32_t` and a float as `double`. These are code-emission fallbacks, not a general numeric promotion system. Annotate storage where width matters.

## Boolean operations and comparisons

`not` requires a boolean. `and` and `or` require two booleans and lower to C short-circuit operators. Conditions for `if` and `while` also require `bool`; there is no implicit integer truthiness.

```abyss
in_range :: (value, low, high u32) bool {
    (value >= low) and (value < high)
}

is_absent :: (pointer &u8) bool {
    pointer == (0 as &u8)
}
```

Numeric comparisons require compatible operand types. Boolean and pointer operands support `==` and `!=`; relational pointer comparisons and struct equality are rejected. Two unrelated pointer element types do not unify merely because both values are pointers.

Arithmetic supports numeric operands; bitwise operations and shifts require integer operands. Shift operands are unified too, so use compatible types for both sides. Pointer arithmetic has its own rules below. Floating `%` is a current checker/backend gap: the arithmetic checker admits numeric operands, but C `%` requires integers. Use integer remainder; there is no builtin floating remainder operation.

## Unit and returns

`unit` describes effects without a returned runtime value. Omit the return annotation or write `unit` explicitly:

```abyss
print :: (text &u8) unit

announce :: () unit {
    print("ready\n")
}
```

The type checker rejects `unit` parameters. The lowerer rejects `unit` locals. Empty blocks and control-flow statements have unit type; they cannot initialize a usable value. The internal `never` type marks `ret` and can unify with a surrounding result, but there is no public `!` type syntax in this parser.

## Explicit casts

`value as Type` emits a C cast. It is not an overflow check, a bounds check, or a data-layout transformation.

| Source/target categories | Accepted by `synth_cast` |
| --- | --- |
| Numeric or bool -> numeric or bool | Yes |
| Pointer -> pointer | Yes |
| Integer -> pointer | Yes |
| Pointer -> integer | Yes |
| Pointer -> bool | Yes |
| Float or bool -> pointer | No |
| Struct -> another struct | No |
| Value -> unit | No |

```abyss
conversions :: (byte u8, raw &u8) {
    widened: u32 = byte as u32
    fraction: f64 = widened as f64
    typed := raw as &u32
    present := raw as bool
}
```

The target must resolve to a type. Casting a pointer changes how subsequent operations interpret its storage; it does not allocate or validate that storage. C rules govern narrowing and float-to-integer conversion. Pointer-to-integer portability depends on an appropriate width and target representation.

## Addresses, dereferences, and null

`&T` constructs a pointer type in a type context. `&value` takes an address in a value context. `*pointer` dereferences; dereferencing a non-pointer reports a type error.

```abyss
write_value :: (target &u32, value u32) {
    *target = value
}

address_example :: () {
    value: u32 = 7
    pointer := &value
    write_value(pointer, 9)
}
```

Use an addressable object for `&`: the backend emits a direct C address operation and does not generally materialize arbitrary expressions. Pointers have no borrow checking, ownership qualifiers, const qualifier, or automatic null check. A null is written with an integer-to-pointer cast:

```abyss
empty :: () &u8 {
    0 as &u8
}
```

Returning the address of a local produces a dangling pointer. Casting a string literal to a mutable pointer does not make its storage safely writable. Lifetime and writable-storage discipline remain the program's responsibility.

## Pointer arithmetic and member access

The arithmetic checker accepts pointer plus integer, integer plus pointer, and pointer minus integer. It does not implement pointer subtraction returning a distance. The result keeps the pointer type.

```abyss
read_at :: (bytes &u8, index u64) u8 {
    *(bytes + index)
}

write_at :: (values &u32, index u64, value u32) {
    *(values + index) = value
}
```

Because these operations lower directly to C pointer arithmetic, the increment is in **elements**: `&u32 + 1` advances one `u32`, not one byte. Use `&u8` for byte offsets. There is no range check, and the pointer must refer to suitable live storage.

For struct fields, the checker and lowerer automatically follow pointer layers. `pointer.field` and `(*pointer).field` both work; there is no separate `->` operator. Method calls use related receiver adjustment, covered in [Methods](methods.md#receivers-at-the-call-site).

## Nominal and structural identity

A named `Name :: struct { ... }` definition creates a nominal type tied to its declaration origin. Two distinct declarations with identical fields remain different types. A type alias refers to the same type, so it preserves identity.

```abyss
Meters :: struct { value: f32 }
Distance :: Meters

read_distance :: (distance Distance) f32 {
    distance.value
}
```

Anonymous struct literals initially receive structural types with inferred field types. An expected named type checks every field and attaches that nominal identity to the literal. Conversion of one existing nominal value into another is not automatic. [Structs & Data Layout](structs.md) gives initialization examples.

The unifier uses disjoint-set parents and ranks for slots, recursively unifies pointer elements and anonymous fields, and interns resolved types. It has no generic type-parameter syntax, subtype hierarchy, trait constraints, or automatic numeric widening.

## Internal types versus source features

The storage includes array, function, inference, error, and never kinds. Arrays have internal element/length data and a unification rule, but the parser lacks array literals, array-type prefixes, and a working indexing handler. `c_type_str` also lacks general array and function-pointer mappings. Use the pointer/length [slice modules](std.md#slices-pointer-and-length) instead of inventing `[T; N]` source syntax.

**Implementation notes:** `compiler/typer/tyck.a` (`parse_builtin_type`, `synth_binary`, `synth_cast`, `validate_binary_comp`, `synth_unary_addr_of`), `compiler/unify.a` (`UnifyStorage_unify_types`), `compiler/types.a`, and `compiler/lower.a` (`c_type_str`).
