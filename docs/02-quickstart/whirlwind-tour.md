# Whirlwind Tour

This tour will get you comfortable with Abyss syntax in less than ten minutes.

---

## 1. Hello, World!

Every Abyss program starts at `main`:

```abyss
main :: fn() {
    print("Hello from the deep!")
}
```

Notice anything missing? There is no `void` or `unit` keyword. If a function doesn't return anything, you simply don't write a return type. `print` automatically appends a newline (`\n`). For custom compile-time formatted strings, `printf` is also available. 

---

## 2. Variables & Comments

Comments start with `--`. Use `:=` for type inference, or specify types explicitly:

```abyss
-- This is a single-line comment
x := 42                -- Inferred integer
pi: f32 = 3.14159      -- Explicit 32-bit float
name := "Abyss"        -- Inferred string
active := true         -- Boolean

-- Variables can be updated with '='
x = x + 10
```

Mutable variables can also be declared at module scope. Use `:=` with a compile-time constant initializer, or declare a type without an initializer to start at zero.

```abyss
initial :: 10
requests := initial * 2
failures: u64

record_failure :: fn() {
    failures = failures + 1
}
```

Each module has one instance of each global. Functions can read and update it, including through imports. A global's current value is a runtime value, so it cannot initialize another global or be read during compile-time evaluation. Taking the address of a global is allowed in an initializer because its storage is fixed.

---

## 3. Control Flow

### `if` / `else`
Parentheses around conditions are optional, but braces around blocks are required:

```abyss
depth := 500

if depth > 1000 {
    print("Crushing pressure!")
} else if depth > 200 {
    print("The twilight zone.")
} else {
    print("Near the surface.")
}
```

### Source locations and fatal errors

`@source_location()` gives you a compile-time value with a file path, a line, and a byte column. Lines and columns start at 1. Use `#fall` to print the location of a fatal error to stderr and exit with status 1.

```abyss
fall :: @import("std/sys").fall

check_depth :: fn(depth i32) {
    if depth < 0 { #fall }
}
```

### `while`
Straightforward loops without surprises. For formatted, newline-free printing, use `printf`:

```abyss
printf :: @import("std/io").printf
print :: @import("std/io").print

i := 0
while i < 5 {
    printf("{} ", .{i})
    i = i + 1
}
print("")
-- Outputs: 0 1 2 3 4
```

---

## 4. Functions & Methods

Functions are declared with `:: fn(...) [ReturnType]`:

```abyss
printf :: @import("std/io").printf

add :: fn(a i32, b i32) i32 {
    a + b
}

log_depth :: fn(meters i32) {
    printf("Current depth: {}m\n", .{meters})
}
```

The last expression in a block automatically acts as the return value—no explicit `return` keyword needed for simple expressions (though `ret` is available when you want an early exit).

---

## 5. Structs & Data Layout

Structs define custom data types. Field definitions use `name Type` syntax (no colons between name and type):

```abyss
Point :: struct {
    x, y i32

    -- Methods take 'self &Self' to access instance data
    dist_sq :: fn(self &Self) i32 {
        self.x * self.x + self.y * self.y
    }
}
```

Instantiating a struct is clean and expressive with struct literal syntax (`.{ ... }`). Combine with `printf` for zero-cost formatting:

```abyss
p: Point = .{ x: 3, y: 4 }
printf("Point ({}, {}), dist_sq = {}\n", .{p.x, p.y, p.dist_sq()})
-- Outputs: Point (3, 4), dist_sq = 25
```

---

## 6. Tuples & Arrays

Abyss supports structural tuples, destructuring, and fixed-size arrays with minimal syntax:

```abyss
-- Structural tuples and destructuring:
origin := .{10, 20}
x, y := origin

-- Fixed-size arrays:
numbers: [3]i32 = .[1, 2, 3]
printf("Item: {}, origin: ({}, {})\n", .{numbers[0], x, y})
```

---

## 7. Compile-Time Execution (`cmpt`)

Why calculate something at runtime when your CPU can do it once during compilation? 

The `cmpt` block executes arbitrary code at compile time using the internal bytecode VM:

```abyss
TABLE_SIZE := cmpt {
    base := 64
    factor := 4
    base * factor + 12
}

print(TABLE_SIZE)
-- TABLE_SIZE is baked into the binary as an exact constant (268)
```

---

## 8. First-Class Types

In Abyss, types are first-class values. You can pass them as arguments, return them from functions, and choose them conditionally:

```abyss
PrecisionType :: fn(high_precision bool) type {
    if high_precision { f64 } else { f32 }
}

-- Computes the type at compile-time without complex template metaprogramming
coordinate: PrecisionType(true) = 142.857
print(coordinate)
```

No template error messages stretching across five terminal screens. Types are just code.

## 9. Modules

Use `@import` anywhere an expression is allowed. It takes a compile-time `Str`
and returns a namespace struct for the file. Top-level bindings are available as
members, including types, functions, constants, and mutable globals.

```abyss
std :: @import("std")
Arena :: @import("std/allocator").Arena

main :: fn() {
    fs := @import("std/fs")
    print(@type_name(fs.File))
    print(std.ascii.is_digit('7'))
}
```

Use `./` or `../` for paths relative to the importing file. Other paths resolve
from the working directory and configured module search paths. The compiler
tries `name.a` before `name/name.a`. Use an explicit `.a` to select a specific file.

Each resolved file has one module identity per compilation. You can pass, store,
or return its namespace value using `@typeof(@import("path"))` as the type. Aliases
share the same definitions and mutable globals. Files can import each other and
use forward references; a binding whose value depends on itself is an error.

---

[Next: Tooling & Workflow &rarr;](workflow.md)
