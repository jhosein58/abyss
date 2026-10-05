# Control Flow & Functions

Functions supply the executable units of the language. Control flow is parsed within the expression grammar, but the current backend implements conditionals and loops as statements. Keep that distinction clear when writing value-producing functions.

## Function signatures and calls

```abyss
add :: (left u32, right u32) u32 {
    left + right
}

use_add :: () u32 {
    add(3, 4)
}
```

There is no `fn` keyword or return arrow. A definition is a name, `::`, a parameter list, an optional return type, and a block. Call arguments are positional and comma-separated. The checker requires the number of arguments to match exactly and checks each against the parameter type. Calls accept a trailing comma.

Adjacent parameter names can share the type that follows the group:

```abyss
between :: (value, low, high u32) bool {
    (value >= low) and (value < high)
}
```

This is equivalent to three explicit `u32` parameter annotations. The parser permits missing annotations, and the checker can use an already resolved parameter slot, but unresolved parameter types are an error. It is not a generic specialization mechanism. Prefer fully annotated signatures for stable APIs.

There are no default parameters, named call arguments, variadic signatures, multiple result syntax, closures, or a general supported function-pointer type mapping in this pipeline. Methods must be called directly. Nested functions inside methods are explicitly forbidden; do not assume general nested closures elsewhere.

## Tail values and unit results

A non-unit function can return its block's final value without `ret`:

```abyss
twice :: (value u64) u64 {
    result := value * 2
    result
}
```

Omitting the return annotation selects `unit`; the checker does not infer a result signature from a numeric tail. Unit functions emit any final value expression for its effects, then return without a value.

```abyss
print :: (text &u8) unit

announce :: () {
    print("ready\n")
}
```

A numeric result needs a numeric annotation. `unit` cannot be a parameter type or a usable local value.

## Early return

`ret value` returns from the function. The value must appear on the same line as `ret`; a newline immediately after it creates a bare return. `ret` itself has the internal `never` type.

```abyss
maximum :: (left, right i32) i32 {
    if left > right {
        ret left
    }
    right
}
```

Use bare `ret` in a unit function, or in the special [receiver chaining convention](methods.md#the-self-chaining-convention). A bare return in an ordinary non-unit function conflicts with its result type.

The current checker is not a complete control-flow proof that every execution path returns a value. Keep a clear typed tail or explicit returns, and check generated C for functions with complicated exit paths.

## If is parsed as an expression but produces unit

```abyss
select :: (condition bool) u32 {
    selected: u32 = 0
    if condition {
        selected = 10
    } else {
        selected = 20
    }
    selected
}
```

The parser accepts `if condition then-expression` and an optional `else else-expression`. Braced blocks are the idiomatic branches, and `else if` works by nesting another `if`. The condition must be `bool`.

In this checkout, `synth_if` binds the node to `unit` regardless of branch results. The lowerer emits an ordinary C `if`/`else` and returns no value string. Branch results are not unified into a conditional result.

The following is **unsupported**, even though many expression-oriented languages accept it:

```abyss
unsupported :: (condition bool) u32 {
    chosen := if condition { 10 } else { 20 }
    chosen
}
```

Use assignments followed by a tail value, as in `select`, or early return followed by a fallback tail. An `if` without `else` also has unit type. This is an implementation boundary, not a claim that conditional value expressions are impossible to add later.

```abyss
classify :: (value i32) i32 {
    result: i32 = 0
    if value < 0 {
        result = -1
    } else if value > 0 {
        result = 1
    }
    result
}
```

## While loops

`while condition { ... }` repeats while the boolean condition is true and has unit type. There is no iterator protocol, range loop, or `for` grammar.

```abyss
sum_before :: (limit u32) u32 {
    total: u32 = 0
    index: u32 = 0
    while index < limit {
        total = total + index
        index = index + 1
    }
    total
}
```

Write a plain boolean expression or a parenthesized expression in the condition. The parser explicitly disables function-header recognition while parsing `if` and `while` conditions. Older library code sometimes adds `or false` after a parenthesized condition as a historical disambiguation workaround; it is not an extra loop requirement in the current parser.

The lowerer emits the condition as C expression text. Keep complex block expressions with statement effects out of loop conditions: statement emission during lowering is not a general guarantee of per-iteration evaluation. Ordinary comparisons, boolean combinations, and calls are the intended forms.

## Break and cont

`break` exits the nearest loop; `cont` starts its next iteration. The backend emits C `break` and `continue`. The current typer assigns unit to both and does not track a loop-context validity rule; use them inside a loop so the emitted C is valid.

```abyss
sum_odd :: (limit u32) u32 {
    total: u32 = 0
    index: u32 = 0
    while true {
        if index >= limit { break }
        current := index
        index = index + 1
        if current % 2 == 0 { cont }
        total = total + current
    }
    total
}
```

Advance loop state before a `cont` when skipping work; otherwise the loop may repeatedly inspect the same value. There are no labels or `break value` results.

## External functions and C integration

A signature with no body marks an external function. The caller supplies its C declaration or definition through the prelude or other C integration. External lowering uses its symbol spelling, while defined Abyss functions use generated `sym_<id>` names.

```abyss
print_u32 :: (value u32) unit

report :: (value u32) {
    print_u32(value)
}
```

This declaration describes a function already defined in the project prelude. The language does not automatically discover C headers, include them, or link arbitrary libraries. Imported Abyss modules may declare external functions, but an import alone cannot provide their native implementations.

The lowerer returns early for external functions without adding a generated prototype in that path. Supply a compatible prototype/definition before generated calls. Type and ABI compatibility are essential: spellings such as `&u8` are the current library convention, not a universal FFI type system.

Aliasing a defined Abyss function through the module system resolves to its canonical generated symbol. An alias of an external function can expose a backend limitation because external calls use the referring identifier spelling; avoid renaming an external symbol without a matching C name or wrapper.

## Entry points

The engine validates that the selected entry is a defined function with no parameters. The generated C wrapper calls it and returns zero; it does not forward an Abyss return value as the process exit code.

```a
main :: () {
}
```

No automatic argument vector is injected. File selection and entry selection are performed by the driver, not by a module-level `main` keyword.

**Implementation notes:** `compiler/parser/parser.a` (`parse_paren`, `parse_fn_tail`, `parse_return`, `parse_if`, `parse_while`), `compiler/typer/tyck.a` (`synth_func`, `synth_call`, `synth_if`, `synth_while`, `synth_break_cont`), `compiler/lower.a`, and `compiler/engine.a` (`Engine_abyss_main`).
