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
