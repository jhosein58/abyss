import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
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

async function run(args) {
    const stdout = [];
    const stderr = [];
    const root = mount({ "main.a": source });
    const wasi = new WASI(["abyssc", ...args], ["ENTRY=environment"], [
        new OpenFile(new File([])),
        ConsoleStdout.lineBuffered(line => stdout.push(line)),
        ConsoleStdout.lineBuffered(line => stderr.push(line)),
        root,
    ]);
    const instance = await WebAssembly.instantiate(wasm, { wasi_snapshot_preview1: wasi.wasiImport });
    const result = wasi.start(instance);
    assert.equal(result, 0, [...stdout, ...stderr].join("\n"));
    return { stdout, root };
}

const interpreted = await run(["r", "main.a", "vm-argument"]);
assert.deepEqual(interpreted.stdout, ["first-second", "value", "vm-argument", "environment", "true", "1.250000"]);
const compiled = await run(["b", "main.a", "out.c"]);
const emitted = new TextDecoder().decode(compiled.root.dir.contents.get("out.c").data);
assert.match(emitted, /import_module\("wasi_snapshot_preview1"\), import_name\("fd_write"\)/);
assert.match(emitted, /void _start\(void\)/);
console.log("WASI Preview 1 compiler passed VM and C emission through browser_wasi_shim");
