import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const shim = process.argv[2] ? pathToFileURL(resolve(process.argv[2])).href : "@bjorn3/browser_wasi_shim";
const { WASI, File, OpenFile, ConsoleStdout, Directory, PreopenDirectory } = await import(shim);
const distribution = new URL("../dist/", import.meta.url);
const wasm = await WebAssembly.compile(await readFile(new URL("abyssc.wasm", distribution)));
const imports = WebAssembly.Module.imports(wasm);
assert.ok(imports.length > 0);
assert.ok(imports.every(entry => entry.module === "wasi_snapshot_preview1"));
assert.ok(WebAssembly.Module.exports(wasm).some(entry => entry.name === "_start"));

const source = `import core.target: Arch, Os, target
import std.allocator: Arena
import std.env: Env
import std.fs: Dir, File
import std.sys: fall

main :: fn() {
    if not Dir.create(c"scratch") { fall() }
    if not File.write_string("scratch/value", "first") { fall() }
    if not File.append_string("scratch/value", "-second") { fall() }
    arena := Arena.new(4096)
    print(File.read_to_string(&arena, "/scratch/value").unwrap())
    entries := Dir.open(c"scratch").unwrap()
    print(entries.next().unwrap().name)
    entries.close()
    print(Env.arg(1))
    print(Env.get(c"ENTRY"))
    print(target.os == Os.wasi and target.arch == Arch.wasm32 and target.ptr_bits == 32)
    print(1.25)
    arena.destroy()
}
`;

function mount(files) {
    const root = new Map();
    const encoder = new TextEncoder();
    for (const [name, content] of Object.entries(files)) {
        const segments = name.split("/");
        const filename = segments.pop();
        let directory = root;
        for (const segment of segments) {
            if (!directory.has(segment)) directory.set(segment, new Directory(new Map()));
            directory = directory.get(segment).contents;
        }
        directory.set(filename, new File(encoder.encode(content)));
    }
    return new PreopenDirectory("/", root);
}

async function run(args, entry = source, module = wasm, expectedStatus = 0) {
    const stdout = [];
    const stderr = [];
    const root = mount({ "main.a": entry });
    const wasi = new WASI(["abyssc", ...args], ["ENTRY=environment"], [
        new OpenFile(new File([])),
        ConsoleStdout.lineBuffered(line => stdout.push(line)),
        ConsoleStdout.lineBuffered(line => stderr.push(line)),
        root,
    ]);
    const instance = await WebAssembly.instantiate(module, { wasi_snapshot_preview1: wasi.wasiImport });
    const result = wasi.start(instance);
    assert.equal(result, expectedStatus, [...stdout, ...stderr].join("\n"));
    return { stdout, root };
}

const interpreted = await run(["r", "main.a", "vm-argument"]);
assert.deepEqual(interpreted.stdout, ["first-second", "value", "vm-argument", "environment", "true", "1.250000"]);
const compiled = await run(["b", "main.a", "out.c"]);
const emitted = new TextDecoder().decode(compiled.root.dir.contents.get("out.c").data);
assert.match(emitted, /import_module\("wasi_snapshot_preview1"\), import_name\("fd_write"\)/);
assert.match(emitted, /void _start\(void\)/);
const memorySource = `import std.env: Env
import std.mem: free, malloc, realloc

shift_bytes :: extern "memmove" fn(dest, source &u8, count u32) &u8

main :: fn() i32 {
    if Env.args_len() != 3 or not Str.of(Env.arg(1)).eq("first") { ret 1 }
    Env.shift_args(1)
    if Env.args_len() != 2 or not Str.of(Env.arg(1)).eq("second") { ret 2 }
    left := malloc(64)
    right := malloc(64)
    guard := malloc(64)
    if left == (0 as &u8) or right == (0 as &u8) or guard == (0 as &u8) { ret 3 }
    if (left as u32) % 16 != 0 { ret 4 }
    free(left)
    free(right)
    joined := malloc(128)
    if joined != left { ret 5 }
    joined[0] = 7
    joined[1] = 8
    joined[2] = 9
    shift_bytes(joined + 1, joined, 3)
    if joined[1] != 7 or joined[2] != 8 or joined[3] != 9 { ret 6 }
    grown := realloc(joined, 131072)
    if grown == (0 as &u8) or grown[3] != 9 { ret 7 }
    grown[131071] = 11
    if grown[131071] != 11 or malloc(0xffffffff as u64) != (0 as &u8) { ret 8 }
    if realloc(grown, 0) != (0 as &u8) { ret 9 }
    free(guard)
    print("WASI memory verified")
    42
}
`;
const memory = await run(["b", "main.a", "out.c"], memorySource);
const directory = await mkdtemp(resolve(tmpdir(), "abyss-wasi-"));
try {
    const cFile = resolve(directory, "memory.c");
    const wasmFile = resolve(directory, "memory.wasm");
    await writeFile(cFile, memory.root.dir.contents.get("out.c").data);
    execFileSync(process.env.CLANG ?? "clang", [
        "-w", "--target=wasm32-wasi", "-ffreestanding", "-nostdlib", "-O3", cFile, "-o", wasmFile,
    ]);
    const program = await WebAssembly.compile(await readFile(wasmFile));
    const execution = await run(["first", "second"], "", program, 42);
    assert.deepEqual(execution.stdout, ["WASI memory verified"]);
} finally {
    await rm(directory, { recursive: true });
}
console.log("WASI Preview 1 compiler and generated runtime passed through browser_wasi_shim");
