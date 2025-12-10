# Abyss 🕳️

**High-level Syntax. Low-level Soul. JIT Powered.**

Abyss is an experimental, performance-oriented language designed for **Real-Time Audio, DSP, and Graphics**. It bridges the gap between modern ergonomics and raw machine code generation.

## ⚡ Under the Hood

Unlike typical interpreted languages, Abyss compiles your code directly to machine instructions at runtime using **Cranelift** (the same backend used by Wasmtime and Rust debug builds).

*   **JIT Compiled:** Scripts run at near-native speed. No interpreter overhead.
*   **Manual Memory Management:** No Garbage Collector. No pauses. You allocate, you free.
*   **Strict Dual-Typing:** Distinct separation between Logic (`i64`) and Math (`f64`) to ensure precision and performance.
*   **Zero Safety:** No borrow checker. No mandatory bounds checking. Just you, the CPU, and the raw pointers.

## 🦀 "Is this Rust?"

It looks like Rust. It feels like Rust. But... **the borrow checker went out for cigarettes and never came back.**

Think of Abyss as `unsafe { Rust }` by default, running on a hot-swappable JIT engine. We adopted the elegant syntax but stripped away the training wheels to give you the dangerous control required for low-level systems experimentation.

## 🩸 The Flavor

Abyss treats everything as numbers. Even strings are just floating-point arrays floating in the void.
```rust
fn main() {
    -- 1. Manual Allocation (No GC)
    let size = 1024
    let buffer = arr(size)

    -- 2. Strict Typing: 'i' is int, 'x' is float
    let i = 0
    let x = 0.0

    -- 3. Raw Speed Loop
    while i < size {
        -- Complex math runs at machine speed
        x = x * x + 0.5

        -- Hard clipping logic (DSP style)
        if x > 1000.0 {
            arr_set(buffer, i, 1000.0) 
            x = 0.0 -- Reset
        } else {
            arr_set(buffer, i, x)
        }

        i = i + 1
    }


    -- 5. Clean up your mess
    arr_free(buffer)
}
```

## 🚧 Status

**Experimental / Active Development**

The project is currently in the forge. The core engine is functional with:
*   **Cranelift JIT Backend**
*   **For Function Interface (FFI)** for Graphics & Audio
*   **Handle-based Heap Allocation**

---
*“Safety is an illusion. Speed is real.”*
.”*
