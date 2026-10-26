const EXAMPLES = {
    hello: `import print_cstr from "std/prelude"

main :: fn() {
    print_cstr(c"Welcome to the Abyss Playground!\\n")
    print_cstr(c"Compiled and executed entirely inside WebAssembly.\\n")
}
`,
    fibonacci: `import print_cstr, print_i32 from "std/prelude"

fib :: fn(n i32) i32 {
    if n <= 1 { ret n }
    fib(n - 1) + fib(n - 2)
}

main :: fn() {
    print_cstr(c"Calculating Fibonacci numbers:\\n")
    i := 0
    while i <= 10 {
        print_cstr(c"fib(")
        print_i32(i)
        print_cstr(c") = ")
        print_i32(fib(i))
        print_cstr(c"\\n")
        i = i + 1
    }
}
`,
    structs: `import print_cstr, print_i32 from "std/prelude"

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
    print_cstr(c"Point coords: (")
    print_i32(p.x)
    print_cstr(c", ")
    print_i32(p.y)
    print_cstr(c")\\n")
    print_cstr(c"Point sum: ")
    print_i32(p.sum())
    print_cstr(c"\\n")
}
`,
    strings: `import print_cstr, print_i32, print_char from "std/prelude"
import Str from "std/string"

main :: fn() {
    msg := Str.of("Abyss Language")
    print_cstr(c"Message: ")
    print_cstr(msg.ptr)
    print_cstr(c"\\nLength: ")
    print_i32(msg.len as i32)
    print_cstr(c"\\nCharacters: ")
    i: u64 = 0
    while i < msg.len {
        c := *(msg.ptr + i)
        print_char(c)
        print_char(32 as u8)
        i = i + 1
    }
    print_cstr(c"\\n")
}
`,
    comptime: `import print_cstr, print_i32 from "std/prelude"
import Str from "std/string"

gen_code :: fn() Str {
    Str.of("6 * 7")
}

main :: fn() {
    val := #gen_code()
    print_cstr(c"Macro code splice result = ")
    print_i32(val)
    print_cstr(c"\\n")
}
`
};

let wasmModule = null;
const editor = document.getElementById("code-editor");
const lineNumbers = document.getElementById("line-numbers");
const output = document.getElementById("output");
const runBtn = document.getElementById("run-btn");
const clearBtn = document.getElementById("clear-btn");
const copyBtn = document.getElementById("copy-btn");
const resetBtn = document.getElementById("reset-btn");
const exampleSelect = document.getElementById("example-select");
const statusPill = document.getElementById("status-pill");
const statusText = document.getElementById("status-text");
const execTime = document.getElementById("exec-time");

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

async function loadWasm() {
    try {
        setStatus("loading", "Compiling Wasm...");
        runBtn.disabled = true;

        const response = await fetch("abyssc.wasm");
        if (!response.ok) {
            throw new Error(`Failed to load abyssc.wasm (${response.status})`);
        }

        const buffer = await response.arrayBuffer();
        wasmModule = await WebAssembly.compile(buffer);

        setStatus("ready", "Wasm Ready");
        runBtn.disabled = false;
    } catch (err) {
        setStatus("error", "Wasm Error");
        output.innerHTML = `<span class="ansi-red">Failed to initialize WebAssembly engine: ${escapeHtml(err.message)}</span>`;
    }
}

async function runCode() {
    if (!wasmModule) return;

    setStatus("running", "Executing...");
    runBtn.disabled = true;
    execTime.style.display = "none";

    let stdoutText = "";
    const decoder = new TextDecoder();

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

        const encoder = new TextEncoder();
        const codeBytes = encoder.encode(editor.value);
        const ptr = instance.exports.alloc_source_buffer(codeBytes.length + 1);
        const mem = new Uint8Array(instance.exports.memory.buffer);
        mem.set(codeBytes, ptr);
        mem[ptr + codeBytes.length] = 0;

        const t0 = performance.now();
        let exitCode = 0;
        try {
            exitCode = instance.exports.run_abyss(ptr, codeBytes.length);
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
            setStatus("ready", "Wasm Ready");
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

editor.addEventListener("input", updateLineNumbers);
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
        updateLineNumbers();
    }
});

exampleSelect.addEventListener("change", (e) => {
    const val = e.target.value;
    if (EXAMPLES[val]) {
        editor.value = EXAMPLES[val];
        updateLineNumbers();
    }
});

resetBtn.addEventListener("click", () => {
    const val = exampleSelect.value;
    if (EXAMPLES[val]) {
        editor.value = EXAMPLES[val];
        updateLineNumbers();
    }
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

editor.value = EXAMPLES.hello;
updateLineNumbers();
loadWasm();
