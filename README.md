# Abyss 🕳️

**High-level Syntax. Low-level Soul. TCC Powered.**

Abyss is an experimental, performance-oriented language designed for **Real-Time Audio, DSP, and Graphics**. It bridges the gap between modern ergonomics and raw C-level manipulation.

## ⚡ Under the Hood

Abyss has evolved. It leverages the **Tiny C Compiler (TCC)** backend to JIT-compile your scripts directly into machine code with blistering speed. It now features a fully-fledged type system that maps directly to C memory layouts.

*   **Native Structs:** Define data layouts that match C structs bit-for-bit.
*   **Methods & Impls:** Organize logic with `impl` blocks. It looks like OOP, compiles to flat functions.
*   **Zero Overhead:** No Garbage Collector. No runtime safety nets. You allocate, you free.
*   **Full C Ecosystem:** Call `malloc`, `memcpy`, `printf` or any shared library directly.
*   **Raw Memory Access:** Pointers are first-class citizens. Array indexing works on raw pointers.

## 🦀 "Is this Rust?"

It looks like Rust. It feels like Rust. But... **the borrow checker went out for cigarettes and never came back.**

Think of Abyss as `unsafe { Rust }` running on a hot-swappable JIT engine. We adopted the elegant syntax—structs, impl blocks, and type inference—but stripped away the training wheels to give you the dangerous control required for low-level systems experimentation.

## 🩸 The Flavor: Zero-Cost Abstractions

Abyss doesn't force a standard library on you. It gives you the power to build one from scratch using `libc` primitives.

Here is a fully functional **Dynamic Vector** implementation. Note how we mix high-level method syntax (`self.push`) with raw C memory management (`malloc`/`free`):
```rust
-- Define the memory layout (Matches C struct perfectly)
struct Vec {
    p: &i64,  -- Pointer to data
    l: i64,   -- Length
    c: i64    -- Capacity
}

-- Constructor
fn NewVec: Vec {
    ret Vec { p: 0, l: 0, c: 0 }
}

-- Add behavior to the data
impl Vec {
    -- Manual memory management, wrapped in a nice API
    fn resize(self: &Vec, new_cap: i64) {
        if new_cap > self.c {
            -- Direct call to libc malloc/realloc logic
            let new_ptr: &i64 = malloc(new_cap * size(i64)) as &i64

            if self.p != 0 {
                memcpy(new_ptr, self.p, self.l * size(i64))
            }

            free(self.p)
            self.p = new_ptr
            self.c = new_cap
        } 
    }

    fn push(self: &Vec, val: i64) {
        if self.l == self.c {
            -- "self.c" is syntactic sugar for accessing struct fields via pointer
            self.resize(self.c + 1)
        }
        -- Array indexing on raw pointers!
        self.p[self.l] = val
        self.l += 1
    }


    fn get(self: &Vec, idx: i64): i64 {
        ret self.p[idx]
    }
}

fn entry {
    -- Stack allocation of the struct header
    let my_vec: Vec = NewVec()

    -- Method call syntax (Automatically passes &my_vec as 'self')
    my_vec.push(1)
    my_vec.push(20)
    my_vec.push(300)

    let val = my_vec.get(1)
    printf("Vector value at index 1: %d\n", val)

    -- Remember: In Abyss, you are the Garbage Collector.
    -- free(my_vec.p) would go here in a real app.
}
```

## 🚧 Status

**Active Development**

The language has reached a major milestone: **Self-Hosted Data Structures**.
Current capabilities include:

*   **Structs & Member Access:** Dot notation (`obj.field`) works seamlessly on values and pointers.
*   **Impl Blocks:** Define methods associated with types. `obj.method()` syntax automatically handles name mangling and pointer passing.
*   **C Interop (FFI):** Seamlessly call `libc` functions or host Rust functions.
*   **Pointer Arithmetic:** Treat pointers like arrays when needed.
*   **Control Flow:** Robust `if`, `while`, and `ret` support.

---
*“Safety is an illusion. Speed is real.”*
