const EXAMPLES = {
    hello: {
        "main.a": `import std.io: printf

main :: fn() {
    printf("Welcome to the Abyss VFS Playground!\\n", .{})
    printf("Compiled and executed in-browser with complete filesystem abstraction.\\n", .{})
}
`
    },
    multifile: {
        "main.a": `import std.io: printf
import math: add_numbers, multiply
import models: User

main :: fn() {
    u := User.new(Str.of("Hosein"), 25)
    sum := add_numbers(15, 27)
    prod := multiply(6, 7)
    
    printf("User: {}, age = {}\\n", .{u.name, u.age})
    printf("Math operations from math.a: sum = {}, product = {}\\n", .{sum, prod})
}
`,
        "math.a": `add_numbers :: fn(a i32, b i32) i32 {
    a + b
}

multiply :: fn(a i32, b i32) i32 {
    a * b
}
`,
        "models.a": `User :: struct {
    name Str
    age  i32

    new :: fn(name Str, age i32) Self {
        .{ name: name, age: age }
    }
}
`
    },
    json_pod: {
        "main.a": `import std.io: printf
import std.allocator: Arena
import pods.json.json: parse_json

main :: fn() {
    arena := Arena.new(1024)
    raw := Str.of("{\\"language\\": \\"Abyss\\", \\"version\\": \\"0.1.0\\", \\"status\\": \\"active\\"}")
    
    printf("Parsing JSON using pods/json pod:\\n", .{})
    opt := parse_json(&arena, raw)
    if opt.is_some() {
        val := opt.unwrap()
        lang := val.get_str(Str.of("language")).unwrap_or(Str.of("?"))
        ver := val.get_str(Str.of("version")).unwrap_or(Str.of("?"))
        status := val.get_str(Str.of("status")).unwrap_or(Str.of("?"))
        
        printf("  Language : {}\\n", .{lang})
        printf("  Version  : {}\\n", .{ver})
        printf("  Status   : {}\\n", .{status})
    }
}
`,
        "pod.json": `{
  "name": "my-app",
  "version": "1.0.0",
  "dependencies": {
    "json": "^0.1.0"
  }
}
`
    },
    fibonacci: {
        "main.a": `import std.io: printf

fib :: fn(n i32) i32 {
    if n <= 1 { ret n }
    fib(n - 1) + fib(n - 2)
}

main :: fn() {
    printf("Calculating Fibonacci sequence:\\n", .{})
    i := 0
    while i <= 10 {
        printf("  fib({}) = {}\\n", .{i, fib(i)})
        i = i + 1
    }
}
`
    },
    structs: {
        "main.a": `import std.io: printf

Point :: struct {
    x i32
    y i32

    new :: fn(x i32, y i32) Self {
        .{ x: x, y: y }
    }

    sum :: fn(self &Self) i32 {
        self.x + self.y
    }
}

main :: fn() {
    p := Point.new(15, 27)
    printf("Point coords: ({}, {})\\n", .{p.x, p.y})
    printf("Point sum: {}\\n", .{p.sum()})
}
`
    }
};

let wasmModule = null;
let vfsBundle = null;
let projectFiles = {};
let activeFileName = "main.a";

const editor = document.getElementById("code-editor");
const lineNumbers = document.getElementById("line-numbers");
const output = document.getElementById("output");
const runBtn = document.getElementById("run-btn");
const clearBtn = document.getElementById("clear-btn");
const copyBtn = document.getElementById("copy-btn");
const resetBtn = document.getElementById("reset-btn");
const newFileBtn = document.getElementById("new-file-btn");
const exampleSelect = document.getElementById("example-select");
const statusPill = document.getElementById("status-pill");
const statusText = document.getElementById("status-text");
const execTime = document.getElementById("exec-time");
const userFileList = document.getElementById("user-file-list");
const tabsContainer = document.getElementById("tabs-container");

function updateLineNumbers() {
    const lines = editor.value.split("\n").length;
    let numbers = "";
    for (let i = 1; i <= lines; i++) {
        numbers += i + "\n";
    }
    lineNumbers.textContent = numbers;
}

function syncScroll() {
    lineNumbers.scrollTop = editor.scrollTop;
}

function setStatus(status, text) {
    statusPill.className = "status-pill " + status;
    statusText.textContent = text;
}

function escapeHtml(text) {
    return text
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;");
}

function parseAnsi(text) {
    const colors = {
        "30": "ansi-black",
        "31": "ansi-red",
        "32": "ansi-green",
        "33": "ansi-yellow",
        "34": "ansi-blue",
        "35": "ansi-magenta",
        "36": "ansi-cyan",
        "37": "ansi-white"
    };

    let result = "";
    let i = 0;
    let activeTags = 0;

    while (i < text.length) {
        if (text.charCodeAt(i) === 27 && text[i + 1] === "[") {
            const end = text.indexOf("m", i + 2);
            if (end !== -1) {
                const codes = text.substring(i + 2, end).split(";");
                i = end + 1;

                if (codes.includes("0") || codes[0] === "") {
                    while (activeTags > 0) {
                        result += "</span>";
                        activeTags--;
                    }
                }

                let classes = [];
                for (const c of codes) {
                    if (c === "1") classes.push("ansi-bold");
                    if (colors[c]) classes.push(colors[c]);
                }

                if (classes.length > 0) {
                    result += `<span class="${classes.join(" ")}">`;
                    activeTags++;
                }
                continue;
            }
        }
        result += escapeHtml(text[i]);
        i++;
    }

    while (activeTags > 0) {
        result += "</span>";
        activeTags--;
    }

    return result;
}

function renderUI() {
    userFileList.innerHTML = "";
    tabsContainer.innerHTML = "";

    for (const fileName of Object.keys(projectFiles)) {
        const item = document.createElement("div");
        item.className = "tree-item" + (fileName === activeFileName ? " active" : "");
        item.innerHTML = `<span>📄 ${escapeHtml(fileName)}</span>`;
        if (fileName !== "main.a") {
            const del = document.createElement("span");
            del.className = "item-delete";
            del.textContent = "✕";
            del.title = "Delete file";
            del.onclick = (e) => {
                e.stopPropagation();
                delete projectFiles[fileName];
                if (activeFileName === fileName) {
                    activeFileName = "main.a";
                }
                renderUI();
            };
            item.appendChild(del);
        }
        item.onclick = () => selectFile(fileName);
        userFileList.appendChild(item);

        const tab = document.createElement("div");
        tab.className = "tab" + (fileName === activeFileName ? " active" : "");
        tab.textContent = fileName;
        tab.onclick = () => selectFile(fileName);
        tabsContainer.appendChild(tab);
    }

    editor.value = projectFiles[activeFileName] || "";
    updateLineNumbers();
}

function selectFile(fileName) {
    if (projectFiles[activeFileName] !== undefined) {
        projectFiles[activeFileName] = editor.value;
    }
    activeFileName = fileName;
    renderUI();
}

function loadExample(key) {
    if (EXAMPLES[key]) {
        projectFiles = JSON.parse(JSON.stringify(EXAMPLES[key]));
        activeFileName = "main.a";
        renderUI();
    }
}

async function initEnvironment() {
    try {
        setStatus("loading", "Loading VFS Bundle...");
        runBtn.disabled = true;

        const [wasmRes, bundleRes] = await Promise.all([
            fetch("abyssc.wasm"),
            fetch("vfs_bundle.json")
        ]);

        if (!wasmRes.ok) throw new Error(`Failed to load abyssc.wasm (${wasmRes.status})`);
        if (!bundleRes.ok) throw new Error(`Failed to load vfs_bundle.json (${bundleRes.status})`);

        const wasmBuffer = await wasmRes.arrayBuffer();
        vfsBundle = await bundleRes.json();
        wasmModule = await WebAssembly.compile(wasmBuffer);

        setStatus("ready", "VFS Engine Ready");
        runBtn.disabled = false;
    } catch (err) {
        setStatus("error", "Init Error");
        output.innerHTML = `<span class="ansi-red">Failed to initialize WebAssembly VFS: ${escapeHtml(err.message)}</span>`;
    }
}

function mountVfs(instance, enc) {
    if (vfsBundle) {
        for (const [path, content] of Object.entries(vfsBundle)) {
            const pBytes = enc.encode(path + "\0");
            const dBytes = enc.encode(content);
            const pPtr = instance.exports.alloc_source_buffer(pBytes.length);
            const dPtr = instance.exports.alloc_source_buffer(dBytes.length);
            new Uint8Array(instance.exports.memory.buffer).set(pBytes, pPtr);
            new Uint8Array(instance.exports.memory.buffer).set(dBytes, dPtr);
            instance.exports.abyss_vfs_write(pPtr, dPtr, dBytes.length);
        }
    }

    for (const [path, content] of Object.entries(projectFiles)) {
        const pBytes = enc.encode(path + "\0");
        const dBytes = enc.encode(content);
        const pPtr = instance.exports.alloc_source_buffer(pBytes.length);
        const dPtr = instance.exports.alloc_source_buffer(dBytes.length);
        new Uint8Array(instance.exports.memory.buffer).set(pBytes, pPtr);
        new Uint8Array(instance.exports.memory.buffer).set(dBytes, dPtr);
        instance.exports.abyss_vfs_write(pPtr, dPtr, dBytes.length);
    }
}

async function runCode() {
    if (!wasmModule) return;

    if (projectFiles[activeFileName] !== undefined) {
        projectFiles[activeFileName] = editor.value;
    }

    setStatus("running", "Executing...");
    runBtn.disabled = true;
    execTime.style.display = "none";

    let stdoutText = "";
    const decoder = new TextDecoder();
    const encoder = new TextEncoder();

    try {
        let instance;
        const imports = {
            env: {
                js_print_bytes: (ptr, len) => {
                    const bytes = new Uint8Array(instance.exports.memory.buffer, ptr, len);
                    stdoutText += decoder.decode(bytes);
                },
                js_trap: () => {
                    throw new Error("abyss_trap");
                }
            }
        };

        instance = new WebAssembly.Instance(wasmModule, imports);
        mountVfs(instance, encoder);

        const mBytes = encoder.encode("main.a\0");
        const mPtr = instance.exports.alloc_source_buffer(mBytes.length);
        new Uint8Array(instance.exports.memory.buffer).set(mBytes, mPtr);

        const t0 = performance.now();
        let exitCode = 0;
        try {
            exitCode = instance.exports.run_abyss_file(mPtr, 6);
        } catch (trapErr) {
            if (trapErr.message !== "abyss_trap") {
                stdoutText += `\nRuntime error: ${trapErr.message}\n`;
            }
            exitCode = 1;
        }
        const t1 = performance.now();
        const elapsed = (t1 - t0).toFixed(1);

        output.innerHTML = parseAnsi(stdoutText);
        execTime.textContent = `${elapsed} ms`;
        execTime.style.display = "inline-block";

        if (exitCode === 0) {
            setStatus("ready", "VFS Engine Ready");
        } else {
            setStatus("error", "Process Exited");
        }
    } catch (err) {
        output.innerHTML = `<span class="ansi-red">Execution error: ${escapeHtml(err.message)}</span>`;
        setStatus("error", "Error");
    } finally {
        runBtn.disabled = false;
    }
}

editor.addEventListener("input", () => {
    projectFiles[activeFileName] = editor.value;
    updateLineNumbers();
});
editor.addEventListener("scroll", syncScroll);

editor.addEventListener("keydown", (e) => {
    if ((e.ctrlKey || e.metaKey) && e.key === "Enter") {
        e.preventDefault();
        runCode();
        return;
    }

    if (e.key === "Tab") {
        e.preventDefault();
        const start = editor.selectionStart;
        const end = editor.selectionEnd;
        editor.value = editor.value.substring(0, start) + "    " + editor.value.substring(end);
        editor.selectionStart = editor.selectionEnd = start + 4;
        projectFiles[activeFileName] = editor.value;
        updateLineNumbers();
    }
});

newFileBtn.addEventListener("click", () => {
    const name = prompt("Enter file name (e.g. math.a, helper.a, pod.json):");
    if (name && name.trim()) {
        const cleanName = name.trim();
        if (!projectFiles[cleanName]) {
            projectFiles[cleanName] = "";
        }
        selectFile(cleanName);
    }
});

exampleSelect.addEventListener("change", (e) => {
    loadExample(e.target.value);
});

resetBtn.addEventListener("click", () => {
    loadExample(exampleSelect.value);
});

copyBtn.addEventListener("click", () => {
    navigator.clipboard.writeText(editor.value).then(() => {
        const originalTitle = copyBtn.getAttribute("title");
        copyBtn.setAttribute("title", "Copied!");
        setTimeout(() => copyBtn.setAttribute("title", originalTitle), 1500);
    });
});

clearBtn.addEventListener("click", () => {
    output.textContent = "";
    execTime.style.display = "none";
});

runBtn.addEventListener("click", runCode);

loadExample("hello");
initEnvironment();
