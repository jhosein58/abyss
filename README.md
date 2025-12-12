# Abyss 🕳️

**High-level Syntax. Low-level Soul. TCC Powered.**

Abyss is an experimental, performance-oriented language designed for **Real-Time Audio, DSP, and Graphics**. It bridges the gap between modern ergonomics and raw C-level manipulation.

## ⚡ Under the Hood

Abyss is no longer just an interpreter. It leverages the **Tiny C Compiler (TCC)** backend to JIT-compile your scripts directly into machine code with blistering speed.

*   **TCC Backend:** Blends the compilation speed of a script with the execution speed of C.
*   **Zero Overhead:** No Garbage Collector. No runtime safety nets. You allocate, you free.
*   **C Interop (FFI):** Seamlessly call host Rust functions or external C libraries.
*   **Raw Memory Access:** Arrays are just pointers. Structs are just arrays. You have total control.

## 🦀 "Is this Rust?"

It looks like Rust. It feels like Rust. But... **the borrow checker went out for cigarettes and never came back.**

Think of Abyss as `unsafe { Rust }` running on a hot-swappable JIT engine. We adopted the elegant syntax but stripped away the training wheels to give you the dangerous control required for low-level systems experimentation.

## 🩸 The Flavor: Build Your Own Vector

Abyss doesn't give you a standard library with bloat. It gives you the tools to build one. Here is how you implement a dynamic Vector using raw memory references and stack arrays:
```rust
-- A helper to push values into our "Vector" structure
-- vec structure: [capacity, length, data_pointer]
fn vec_push(vec: &i64, val: i64) {
    let len: i64 = vec[1]      -- Get current length
    let ref: &i64 = vec[2]     -- Get pointer to data

    ref[len] = val             -- Write to memory directly
    vec[1] = len + 1           -- Update length
}

-- A helper to print the vector content
fn vec_print(vec: &i64) {
    let len: i64 = vec[1]
    let ref: &i64 = vec[2]

    print("[")
    -- Robust loops
    for i in 0 -> len {
        print_i64(ref[i])
        if i < len - 1 {
            print(", ")
        }
    }
    print("]\n")
}

fn entry {
    -- 1. Allocate raw storage on the stack
    let arr: i64[100]

    -- 2. Define our Vector "Header"
    -- [Capacity: 100, Length: 0, Pointer to Storage: &arr]
    let vec = [100, 0, &arr]

    -- 3. Manipulate via functions
    vec_push(vec, 5)
    vec_push(vec, 100)
    vec_push(vec, -3)

    -- Output: [5, 100, -3]
    vec_print(vec)
}
```
## 🚧 Status

**Active Development**

The project has evolved significantly. The core engine now features:
*   **TCC JIT Integration:** Instant compilation and execution.
*   **Pointer Arithmetic:** Direct access to memory addresses using `&` syntax.
*   **Rust FFI:** Two-way communication between Abyss scripts and the Rust host.
*   **Control Flow:** Robust `for` and `while` loops for complex logic.

---
*“Safety is an illusion. Speed is real.”*
