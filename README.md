# Abyss 🕳️

**High-level Syntax. Low-level Soul. LLVM Powered.**

Abyss has been completely rewritten from the ground up. It is an experimental, high-performance systems programming language designed for maximum control, blistering speed, and modern ergonomics. 

The original TCC prototype has been scrapped. Welcome to the new Abyss.

## ⚡ Under the Hood

Abyss is now powered by **LLVM**, bringing industry-standard optimizations and raw native performance. Furthermore, it introduces a dual-target architecture:

*   **LLVM Backend:** Compiles directly to highly optimized native machine code.
*   **Custom Abyss VM:** A dedicated virtual machine target that ensures true portability ("Run Anywhere") and unlocks powerful **Comptime** (compile-time execution) capabilities.
*   **Zero Overhead:** No Garbage Collector. You are in full control of memory.
*   **Seamless C Interop:** Calling external libraries requires zero boilerplate.

## 🩸 The Flavor: Raw Power, Clean Syntax

The syntax has evolved to be cleaner and more direct, especially when interacting with the outside world. You don't need complex bindings to talk to C; you just define the signature and go.

Here is a minimal example showing how effortless FFI (Foreign Function Interface) is in the new Abyss:

```rust
-- Declare an external C function with zero friction
def printf(s: &u8): i32 _

-- Call it directly using a C-string literal
printf(c"Hello, Abyss!\n")
```

## 🧠 Comptime & VM Magic

By leveraging the custom Abyss VM, the compiler can execute your code *during compilation*. This allows for advanced metaprogramming, compile-time data generation, and type checking without the need for complex macro systems. 

## 🚧 Status

**Reborn & Active Development**

The language has transitioned from a JIT prototype to a serious compiled language. 
Current focus areas:
*   Stabilizing the LLVM IR generation.
*   Expanding VM instructions for broader `comptime` support.
*   Fleshing out the core syntax and type system.
