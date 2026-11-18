# Calling C from Abyss

Use a top-level `extern fn` declaration to import a C symbol. Without an explicit name, Abyss uses the binding name as the linker symbol. With a name, the binding and the C identifier can differ.

```abyss
puts :: extern fn(message &u8) i32
c_abs :: extern "abs" fn(i32) i32

main :: fn() {
    puts(c"Hello from Abyss")
    print(c_abs(-42))
}
```

Parameter names are optional, and consecutive names can share a type (`a, b i32`). Use a newline after an anonymous named type when it precedes a group of named parameters. A declaration without a return type returns no value. Generic, compile-time, and variadic parameters are invalid on external functions. Use a fixed-signature C wrapper for a variadic API.

A body turns the declaration into an export. The native backend emits the function under its C linker name, including exports that Abyss code never calls. In a VM run, the body executes as bytecode. Pass the function as a callback to share it with C. Exported names are linker symbols only in native builds.

```abyss
Point :: extern struct {
    x f64
    y f64
}

distance_sq :: extern "abyss_distance_sq" fn(p Point) f64 {
    p.x * p.x + p.y * p.y
}

Distance :: fn(Point) f64
```

`extern struct` preserves declaration order and uses C alignment and padding. Match the fields and their types on the C side. A bodyless declaration such as `Distance`, without `extern`, is a function type alias. Use it in declarations that accept or return function pointers.

For native builds, link the generated C with the objects or libraries that provide the imports. For VM calls, symbols must already be visible to the running process. On Linux, for example, load a shared library with `LD_PRELOAD`:

```sh
cc -shared -fPIC helper.c -o helper.so
LD_PRELOAD="$PWD/helper.so" ./abyss r main.a
```

Typed VM foreign calls require a compiler built with the libffi headers available, plus a libffi shared library at runtime. The VM bridge supports booleans, 8- to 64-bit integers, `f32`, `f64`, pointers, callbacks, and records made from those types. Keep callbacks on the VM's owning thread and call them only while that VM is alive. WebAssembly does not load native libraries.
