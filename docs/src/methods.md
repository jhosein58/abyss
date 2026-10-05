# Methods & Dogfooding

Methods organize operations around a named struct while preserving direct calls and explicit mutation. Receiver classification is unusually concrete: the compiler looks for implicit leading-dot accesses in the method's HIR.

## Static and instance methods

```abyss
Counter :: struct {
    value: u32,

    new :: (initial u32) Self {
        .{ value: initial }
    }

    add :: (amount u32) {
        .value = .value + amount
    }

    read :: () u32 {
        .value
    }
}

use_counter :: () u32 {
    counter := Counter.new(3)
    counter.add(4)
    counter.read()
}
```

`new` has no implicit receiver access, so it is static and requires the named type at the call site. `add` and `read` use `.value`, so they are instance methods and require a value or pointer. Methods are declared with `::` and must have function bodies; external method declarations are rejected by the struct parser.

Methods occupy no runtime storage in the struct. Their functions are emitted separately. There is no virtual lookup, interface table, inheritance, or overload selection by operator.

## Self and method ownership

In a named struct method's signature and body, `Self` resolves to the struct's nominal type. The typer checks the HIR node's recorded method owner before ordinary identifier resolution.

```abyss
Cell :: struct {
    contents: u8,

    of :: (contents u8) Self {
        .{ contents: contents }
    }
}
```

`Self` is not a reserved lexer keyword, and there is no globally injected `Self` in ordinary free functions. Older library modules sometimes declare a module alias such as `Self :: ArenaVecU8`; that is an ordinary type alias with a separate purpose.

Ownership here means **which declaration owns the method**, not memory ownership or borrow checking. The parser records the owning struct on method HIR nodes. After creating the named struct type, the checker uses that owner for `Self`, implicit members, and the chaining convention.

Methods on unnamed structural types are rejected. Nested functions and nested struct declarations inside methods are also rejected. Method values are unsupported: `counter.read()` is valid, but storing `counter.read` for later invocation is not.

## Leading-dot receiver access

Inside a method, `.field` means a member on the implicit receiver. It also permits calling another receiver method:

```abyss
Counter :: struct {
    value: u32,

    add :: (amount u32) {
        .value = .value + amount
    }

    increment :: () {
        .add(1)
    }
}
```

The parser requires a struct-method context for leading-dot member access. The literal `.{ ... }` is a separate form and remains available outside methods.

There is no implicit unqualified lookup that turns a bare `value` into `.value`. Use the dot. This matters in the current standard library: some `clear` methods use bare `len = 0` and are defective; [Vectors](std.md#heap-vectors) documents their status.

The receiver is not introduced simply because a method returns `Self` or `&Self`. `method_has_receiver` scans for a member node with an absent left-hand side. A `.method()` call creates such a node too. Consequently, refactoring the last implicit member out of a method changes its classification to static.

## Receivers at the call site

```abyss
read_counter :: (pointer &Counter) u32 {
    pointer.read()
}
```

The lowerer adjusts a receiver as follows:

| Call target | Receiver passed to emitted C function |
| --- | --- |
| Implicit `.method()` | Existing `_self` pointer |
| Named local struct value | Address of that value |
| Pointer to struct | Pointer itself |
| Multiple pointer layers | Dereferenced until one struct pointer remains |
| Non-addressable struct result | Address of a C compound-literal array element holding the result |

Automatic receiver adjustment does not apply to an explicit free-function pointer parameter. For `modify :: (counter &Counter)`, call `modify(&counter)` on a struct local.

Instance methods cannot be called through the type name, and static methods cannot be called on an instance. Both errors are diagnosed by member checking. Argument counts refer to the explicitly declared parameters; the generated receiver is additional and does not count as a user argument.

Temporary receiver storage follows generated C lifetime rules. Do not keep a chained receiver pointer after its backing local or temporary ceases to exist. Abyss does not track this lifetime.

## The &Self chaining convention

A receiver method whose declared result resolves to a pointer to its owning nominal type gets special return behavior. The backend emits `return _self;` at ordinary fallthrough, and a bare `ret` returns `_self` early. The checker does not require the final body expression to produce the receiver pointer.

```abyss
Counter :: struct {
    value: u32,

    new :: (initial u32) Self {
        .{ value: initial }
    }

    add :: (amount u32) &Self {
        if amount == 0 { ret }
        .value = .value + amount
    }

    read :: () u32 {
        .value
    }
}

chained :: () u32 {
    counter := Counter.new(10)
    counter.add(2).add(3).read()
}
```

This mutates `counter` and reads 15. The chain shares one receiver; it does not construct a fresh counter at each call.

The test is based on type identity, not solely on the text `&Self`: an equivalent pointer-to-owner annotation can trigger it. A static method returning `&Self` does **not** trigger it, because it has no receiver. The allocator's `ArenaChunk.new` is such a static constructor; it returns an explicitly allocated pointer.

An explicit `ret pointer` remains an explicit return and is checked against the result type. For receiver chaining, use bare `ret` or fallthrough unless you intentionally return another valid pointer.

## How the library exercises the feature

`VecU32.new()` calls `Self.with_cap(4)`, a static constructor. `push` calls `.grow()`, which calls `.reserve(1)`. `Arena.alloc` uses `.current`, while its static `new` constructs the arena header. `Str.of` is static and `Str.print` reads `.ptr` and `.len` through a receiver.

```abyss
import VecU32 from "std/vec/u32.a"

library_methods :: () u32 {
    values := VecU32.new()
    values.push(42)
    answer := values.get(0)
    values.destroy()
    answer
}
```

Legacy free functions such as `String_push_str` and `ArenaVecU8_push` are real symbols with explicit pointer parameters. They coexist with inline methods. Do not assume an inline method automatically exports a symbol named `Type_method`; an import must name an actual module-level declaration.

## Lowering in concrete terms

The following is schematic C, with names shortened for readability:

```c
void method_add(CounterType * _self, uint32_t amount) {
    _self->_f_value = _self->_f_value + amount;
    return;
}
```

Actual C names use numeric IDs. Method targets are recorded during member checking; lowering queues the chosen method and emits a direct function call. Implicit `.field` becomes `_self->_f<id>`. There is no method object to allocate or resolve at runtime.

**Implementation notes:** `compiler/parser/parser.a` (`parse_struct`, `parse_fn_tail`, leading-dot dispatch), `compiler/typer/tyck.a` (`synth_ident`, `method_has_receiver`, `synth_member`, `method_returns_ref_self`), and `compiler/lower.a` (`lower_expr`, `lower_function`).
