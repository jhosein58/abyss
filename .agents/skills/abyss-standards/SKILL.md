---
name: abyss-standards
description: >-
  Authoritative standards for Abyss language evolution, community tone, release engineering conventions,
  and structured release note specifications. Activate this skill whenever drafting release notes,
  creating release announcements, structuring changelogs, or standardizing community documentation.
---

# Abyss Engineering & Release Standards

This document establishes the project standards, community communication guidelines, and release engineering specifications for the Abyss programming language.

---

## 1. Core Philosophy & Project Identity

Abyss is built upon five inviolable design principles:
1. **Zero Runtime Overhead & Zero Bloat**: Emitted binaries contain only the exact code, structs, and symbols needed. No runtime metadata or background machinery is bundled unless explicitly invoked.
2. **Deterministic & Bit-Identical Reproducibility**: Compiler self-hosting must always reach a verified multi-generation fixed point where subsequent compilation rounds generate bit-identical C source code.
3. **Pure Compile-Time Evaluation (Comptime)**: Metaprogramming, source generation, and reflection are available at compile time through standard language constructs (`cmpt`, `#`, splices). Explicit runtime use of `@type_info` includes only the requested type information.
4. **Transparent Bootstrap**: The entire compiler can bootstrap from clean, standalone ISO C99 on any standard platform without needing external binaries.
5. **Orthogonal Simplicity**: Features remain orthogonal, explicit, and expression-oriented.

---

## 2. Repository & Development Standards

- **Source Code Cleanliness**:
  - Strictly **NO comments** (`//` or `--`) in any `.a` files. Code must be self-documenting, clean, and unambiguous.
- **Git Commit Discipline**:
  - Single-line English commit messages.
  - Start with an imperative action verb (e.g., `Implement host syscall dispatch in VM`, `Inline embedded lookup into indexer`).
  - High-frequency, atomic commits.
- **Verification Gates**:
  - Every release must pass `100%` of test suites (all unit, compiler, diagnostic, and pod tests).
  - Multi-round fixed-point bootstrap verification (`./abyss promote 3` or `./abyss release 3`) must produce matching cryptographic hashes across generations.

---

## 3. Standard Release Artifacts Catalog

Every formal release publishes a deterministic, curated set of artifacts:

| Artifact | Target Platform | Description |
| :--- | :--- | :--- |
| `abyss-linux-x86_64.tar.gz` | Linux (x86_64) | Full toolchain archive: `abyssc`, `sonar`, `abyssfmt`, standard library pods (`core`, `std`, `json`, `fmt`, `simd`, `test`). |
| `abyss-linux-x86_64.tar.gz.sha256` | Linux (x86_64) | SHA-256 checksum for the Linux archive. |
| `abyss-windows-x86_64.zip` | Windows (x86_64) | Full toolchain archive: `abyssc.exe`, `sonar.exe`, `abyssfmt.exe`, standard library pods. |
| `abyss-windows-x86_64.zip.sha256` | Windows (x86_64) | SHA-256 checksum for the Windows archive. |
| `abyssc.wasm` | WebAssembly (WASI) | Standalone WebAssembly compiler with pure compile-time embedded standard library; zero external file dependencies. |
| `abyss-bootstrap-c.tar.gz` | Multi-Platform | Standalone ISO C99 bootstrap sources for Linux, macOS (Darwin), Windows, and WebAssembly with build documentation. |
| `abyss-bootstrap-c.tar.gz.sha256` | Multi-Platform | SHA-256 checksum for the bootstrap bundle. |

*Note: Ephemeral, redundant, or virtual filesystem bundles (such as `vfs_bundle.json`) and individual OS bootstrap files (subsumed by `abyss-bootstrap-c.tar.gz`) are permanently excluded.*

---

## 4. Release Notes Tone & Specification

Use the sizes of the published assets and report only verification that ran. To update an existing GitHub release, wait for the tag workflow to publish its artifacts, then run `gh release edit <VERSION> --notes-file <FILE>`.

### Voice & Tone
- **Minimalist & Structured**: Concise, logical, and cleanly segmented.
- **English Only**: International, objective, technical prose.
- **Zero Fluff**: No hyperbolic marketing language ("revolutionary", "game-changing"). Let the technical merits and benchmarks speak.
- **Precise**: Reference concrete components (`compiler/vm/host.a`, `abyssc.wasm`, fixed-point hashes, test counts).

### Standard Release Note Template

```markdown
# Abyss <VERSION>

<One to two sentence executive summary stating the primary milestone or theme of the release.>

## Highlights

- **<Key Feature / Architectural Change>**: <Brief description of what was achieved and why it matters.>
- **<Key Feature / Performance Enhancement>**: <Brief description.>

## Changes & Improvements

### Compiler & Architecture
- <Bullet point describing compiler changes, VM improvements, or codegen adjustments.>

### Standard Library & Toolchain
- <Bullet point describing std, pods, or tooling updates.>

## Verification & Stability

- **Test Suite**: ✔ ALL <COUNT> TESTS PASSED (0 failures).
- **Bootstrap Stability**: 3-generation fixed-point bit-identical C hash: `<SHA256_HASH>`.
- **WASM Isolation**: Verified standalone execution without external filesystem access.

## Release Artifacts

| File | Size | Description |
| :--- | :--- | :--- |
| `abyss-linux-x86_64.tar.gz` | `<SIZE>` | Complete Linux x86_64 toolchain and standard library pods |
| `abyss-windows-x86_64.zip` | `<SIZE>` | Complete Windows x86_64 toolchain and standard library pods |
| `abyssc.wasm` | `<SIZE>` | Self-contained WebAssembly compiler with embedded stdlib |
| `abyss-bootstrap-c.tar.gz` | `<SIZE>` | Standalone C99 bootstrap sources for Linux, macOS, Windows, WASI |

## Installation & Quickstart

### Linux x86_64 (Bash)
```bash
curl -fsSL https://raw.githubusercontent.com/abysslang/abyss/main/scripts/install.sh | bash
```

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/abysslang/abyss/main/scripts/install.ps1 | iex
```

### Bootstrap from C (Linux, macOS, Windows, WASI)

Extract `abyss-bootstrap-c.tar.gz` and follow its `README.md` for your target. On Linux x86_64:

```bash
tar -xzf abyss-bootstrap-c.tar.gz
cd abyss-bootstrap-c
gcc -std=gnu99 -D_POSIX_C_SOURCE=200809L -O3 -fomit-frame-pointer bootstrap-linux-x86_64.c -o abyssc
./abyssc --version
```
```
