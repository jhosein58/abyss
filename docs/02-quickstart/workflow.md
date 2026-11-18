# Tooling & Workflow

Abyss comes with a developer driver script in the repository root: `./abyss`. It handles everything from zero-latency script-like execution to native cross-compilation.

---

## The Fast Loop: Run with the VM

During active development, waiting for a C compiler to link binaries adds unnecessary friction. Use the Bytecode VM for instantaneous turnaround:

```sh
./abyss r main.a
```

The compiler parses your code, checks types, translates it to bytecode, and executes it immediately in memory. Zero disk clutter, zero linker latency.

---

## The Native Loop: Optimized C99 Binaries

When you need peak execution performance:

```sh
# Compile to an optimized native binary (-O3)
./abyss b main.a

# Or compile and run in a single command
./abyss br main.a
```

This passes the emitted C99 code to your native C compiler (`gcc` or `clang`) with maximum optimization flags enabled.

---

## Windows Cross-Compilation

Writing code on Linux and deploying to Windows? Abyss supports one-command standalone Windows cross-compilation via MinGW:

```sh
./abyss b-win main.a
```

The resulting `.exe` binary is completely standalone—it links directly to standard Windows system libraries (`KERNEL32.dll` and `msvcrt.dll`) with zero third-party dependencies or runtime bloat.

---

## Target Selection

The compiler defaults to the host OS and architecture. Use `--target linux|windows|darwin|wasi|freestanding` and, if needed, `--arch x86_64|aarch64|wasm32` to select a different profile. WASI defaults to `wasm32`. The flags select the ABI and module sources; use a C toolchain for the same target when linking.

```sh
./abyss emit b --target wasi main.a program.c
clang --target=wasm32-wasi -ffreestanding -nostdlib program.c -o program.wasm
```

An import of `platform` selects `platform.<os>.<arch>.a`, then `platform.<os>.a`, then `platform.a`. Folder modules use the same suffixes on their entry file. Import `Os`, `Arch` and the typed `target` constant from `core.target` to inspect the selected profile. Use `--library` to emit C exports without an executable entry point. A freestanding target also omits the entry point.

`./abyss wasm` produces a standalone `dist/abyssc.wasm` with `core` and `std` embedded directly into the binary. For a browser playground, add the user's source file to a `browser_wasi_shim` `PreopenDirectory` at `/` without needing any external standard library bundles, and pass arguments such as `["abyssc", "r", "main.a"]` to the shim. Instantiate the module with the shim's `wasi_snapshot_preview1` imports and call `wasi.start(instance)`. To emit C, use `["abyssc", "b", "main.a", "out.c"]` and read `out.c` from the same directory.

---

## Testing & Quality Assurance

Abyss has an automated test suite verifying both execution backends:

```sh
# Run all tests against both the C99 and VM backends
./abyss test

# Or run tests on Windows binaries using Wine
./abyss test-win
```

Every language feature is cross-tested to guarantee that VM execution and native C execution produce identical output.

---

## Release Packaging & Bootstrap Verification

To verify self-hosting determinism and generate standalone release packages:

```sh
# Run multi-round fixed-point bootstrap verification
./abyss bootstrap

# Generate distribution artifacts into dist/
./abyss release
```

This verifies bit-for-bit generational equality ($G_2 \equiv G_3$) and updates `dist/bootstrap.c` for distribution.

---

[Next Part: The Guide &rarr;](../03-the-guide/01-basics.md)
