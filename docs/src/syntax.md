# Lexical Structure & Syntax

Abyss uses a Pratt expression parser with explicit precedence and a small set of prefixes. Read the binding forms first: they tell you whether a name defines a declaration, allocates a local variable, or refers to an existing value.

## Identifiers and keywords

Identifiers follow `[A-Za-z_][A-Za-z0-9_]*`. Names are case-sensitive; Unicode identifier rules are not implemented. The lexer treats these words specially:

```text
if else while ret break cont struct import from
true false as and or not
```

Type spellings and `Self` are identifier tokens. Their meanings are supplied by type checking. `Self` has a contextual meaning inside named struct methods, described in [Methods & Dogfooding](methods.md#self-and-method-ownership).

```abyss
BufferIndex :: u32

next_index :: (current BufferIndex) BufferIndex {
    current + 1
}
```

There is no `fn`, `let`, `var`, `return`, `continue`, `for`, or `null` keyword. Use `ret`, `cont`, and a typed zero for a null pointer.

## Whitespace and comments

Spaces and tabs are whitespace. Both carriage return and line feed count as newlines. `--` starts a comment ending at the next newline; there is no block-comment syntax.

```abyss
advance :: (offset u64) u64 {
    -- Move past the current byte.
    offset + 1
}
```

Two consecutive minus characters always begin a comment. To subtract a negative expression, write `left - (-right)` or separate the minuses with whitespace. `//` and `/* ... */` are not comments.

## Integer and floating literals

| Form | Example | Implementation rule |
| --- | --- | --- |
| Decimal integer | `1024`, `1_024` | Separators must occur between digits |
| Hexadecimal integer | `0xFF`, `0XAB_CD` | Same separator rule; at least one digit |
| Decimal float | `0.5`, `12.25`, `.5` | Digits required after the decimal point |
| Negative value | `-12` | Unary minus applied to an integer token |

The parser accumulates every integer literal as a `u32`. Its maximum token value is **4,294,967,295**, even when the destination type is `u64` or `u128`. Wider values can be formed with typed arithmetic. An annotation does not increase the literal parser's limit.

```abyss
numeric_forms :: () {
    count: u32 = 1_024
    bits: u32 = 0xAB_CD
    fraction: f64 = .5
    large: u64 = (1 as u64) << (40 as u64)
}
```

The lexer rejects underscores in floats. Scientific notation, binary prefixes, numeric suffixes, and a trailing-dot floating literal are not implemented. A leading zero does not select an octal integer base in the parser. Integer tokens lower to decimal text, while floating literal text is passed through to C.

There is no dedicated literal-range check against a narrow destination such as `u8`; do not infer checked narrowing from successful type inference. C conversion and overflow rules still matter.

## Strings and byte access

Strings use double quotes and cannot contain a physical newline. The lexer recognizes C-style escape starters: `a`, `b`, `f`, `n`, `r`, `t`, `v`, backslash, single and double quotes, `?`, octal digits, and `x` followed by a hexadecimal digit.

```abyss
print :: (text &u8) unit

show_text :: () {
    print("Abyss\n")
    print("quote: \"\n")
    letter: u8 = *"A"
}
```

The lexer validates escape starters rather than decoding a complete escape value. Raw quoted text survives parsing and is emitted as a C string literal cast to `uint8_t*`. C supplies escape decoding and the terminating zero; strings have type `&u8`, not `Str` or `String`. See [Strings](std.md#str-a-borrowed-byte-view) for the difference between C strings and length-delimited views.

Single-quoted character literals are recognized by the lexer, but `dispatch_prefix` has no `lit_char` branch. They cannot currently be used as parsed value expressions. Use `*"A"` or an explicitly typed numeric byte instead.

## Declaration forms

| Spelling | Meaning | Practical status |
| --- | --- | --- |
| `Name :: expression` | Definition/binding | Functions and type aliases/structs supported |
| `name := expression` | Inferred mutable local | Supported inside functions |
| `name: Type = expression` | Annotated mutable local | Supported |
| `name: Type` | Local without a source initializer | C emitter initializes it with `{0}` |
| `name = expression` | Assignment | Use an existing writable value |
| `name: Type: expression` | Annotated binding node | Parsed; runtime lowering has no general binding case |

```abyss
Index :: u32

advance :: (start Index) Index {
    next := start
    step: Index = 1
    zero: Index
    next = next + step + zero
    next
}
```

`:=` is lexed as `:` followed by `=`. The colon handler recognizes that sequence and creates an inferred local. Whitespace between those tokens does not change the form.

Local lookup walks scope entries backward before consulting module symbols; this permits shadowing. Block exit restores the earlier scope. Do not use a local in its own initializer: the parser binds the new name before parsing that initializer.

```abyss
scopes :: () {
    value: u32 = 3
    {
        value: u32 = 9
        value = value + 1
    }
    value = value + 1
}
```

`::` is not a fully implemented arbitrary compile-time evaluator. A global numeric binding can be parsed and typed, but a runtime reference reaches the backend's `unsupported global value` error. The library uses constant-returning functions:

```abyss
capacity :: () u32 { 64 }

use_capacity :: () u32 {
    capacity()
}
```

## Blocks, values, and statement effects

A block is a sequence of expressions. Its type is the type of its last item; an empty block is `unit`. The backend emits earlier items for their effects and keeps the final emitted value for a surrounding use or a function return.

```abyss
sum :: (left, right u32) u32 {
    result := left + right
    result
}
```

Parsing an expression does not guarantee it supplies a C value. Declarations, `if`, `while`, `ret`, `break`, and `cont` emit statements. An `if` node has `unit` type even when both branches end with numbers. See [Control Flow & Functions](control-flow.md#if-is-parsed-as-an-expression-but-produces-unit).

Semicolons are tokenized but never consumed as block separators. Write newline-separated expressions, without a trailing semicolon. Commas separate call arguments and can separate struct fields and literal entries; they are not general statement separators.

## Line continuation

At a newline, `parse_expr` stops unless the following token is a “soft” infix token. The soft set includes `+`, `-`, `/`, `%`, comparisons, shifts, `|`, `^`, assignment tokens, `:`, `::`, `and`, `or`, and `as`. It excludes `*`, `&`, `.`, and `(`.

```abyss
combine :: (left, middle, right u32) u32 {
    total := left
        + middle
        + right
    total
}
```

For multiplication, put the operator before the newline if you split the expression: its right operand is parsed after the operator has been consumed. Keep member chains and calls on one line. A leading `.` on the next line starts a separate implicit receiver expression inside a method, and is an error elsewhere.

## Operator precedence

Higher rows bind more tightly. Binary arithmetic and comparisons are left-associative; `::`, `:`, and `=` use right binding power.

| Binding power | Operators/forms |
| --- | --- |
| 160 | `.` member |
| 150 | `(...)` call |
| 140 | Prefix `-`, `not`, `~`, `&`, `*` |
| 130 | `as` |
| 120 | `*`, `/`, `%` |
| 110 | `+`, `-` |
| 100 | `<<`, `>>` |
| 90 | `&` bitwise |
| 80 | `^` |
| 70 | `\|` |
| 60 | `==`, `!=`, `<`, `<=`, `>`, `>=` |
| 40 | `and` |
| 30 | `or` |
| 15 | `:` declaration |
| 10 | `=` assignment |
| 5 | `::` binding |

```abyss
masked :: (value u32) u32 {
    (value & 0xFF) + 1
}
```

Use parentheses around bitwise operations when combined with comparisons: bitwise operators bind more tightly. Use `not`, `and`, and `or`; standalone `!` is an unknown character, and `&&`/`||` are not logical operator tokens.

Compound assignments (`+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=`) have tokens and binding powers but no implementation in `parse_binary`; they report `unsupported binary operator`. Write `value = value + amount`.

Bracket tokens and `#` also exist without a working source array/index/directive grammar. Internal representation alone does not establish a supported feature.

**Implementation notes:** `compiler/lexer.a` (`scan_number`, `scan_string`, `scan_char`, `match_keyword`), `compiler/parser/core.a` (`str_to_u32`), `compiler/parser/parser.a` (`dispatch_prefix`, `parse_var_decl`, `parse_block`), `compiler/parser/prec.a`, and `compiler/lower.a` (`lower_expr`).
