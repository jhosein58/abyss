/* tslint:disable */
/* eslint-disable */

export class Abyss {
    free(): void;
    [Symbol.dispose](): void;
    expr(pattern: string, precedence: number, callback: Function): void;
    ident(token_name: string, callback: Function): void;
    ignore(name: string): void;
    constructor(source: string);
    number(token_name: string, callback: Function): void;
    parse(): Node[];
    run(host_functions: object): void;
    stmt(pattern: string, callback: Function): void;
    token(name: string, pattern: string): void;
}

export class Ctx {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    ident(name: string): string;
    node(name: string): Node;
    nodes(name: string): Node[];
}

export class IR {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    static add(l: Node, r: Node): Node;
    static assign(target: Node, val: Node): Node;
    static bool(val: boolean): Node;
    static call(func: Node, args: Node[]): Node;
    static div(l: Node, r: Node): Node;
    static eq(l: Node, r: Node): Node;
    static exprStmt(expr: Node): Node;
    static gt(l: Node, r: Node): Node;
    static ifStmt(cond: Node, then_body: Node[], else_body: Node[]): Node;
    static int(val: number): Node;
    static lt(l: Node, r: Node): Node;
    static mul(l: Node, r: Node): Node;
    static neq(l: Node, r: Node): Node;
    static sub(l: Node, r: Node): Node;
    static var(name: string): Node;
    static varDecl(name: string, val: Node): Node;
    static whileStmt(cond: Node, body: Node[]): Node;
}

export class Node {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    __stash(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_abyss_free: (a: number, b: number) => void;
    readonly __wbg_ctx_free: (a: number, b: number) => void;
    readonly __wbg_ir_free: (a: number, b: number) => void;
    readonly __wbg_node_free: (a: number, b: number) => void;
    readonly abyss_expr: (a: number, b: number, c: number, d: number, e: any) => void;
    readonly abyss_ident: (a: number, b: number, c: number, d: any) => void;
    readonly abyss_ignore: (a: number, b: number, c: number) => void;
    readonly abyss_new: (a: number, b: number) => number;
    readonly abyss_number: (a: number, b: number, c: number, d: any) => void;
    readonly abyss_parse: (a: number) => [number, number, number, number];
    readonly abyss_run: (a: number, b: any) => [number, number];
    readonly abyss_stmt: (a: number, b: number, c: number, d: any) => void;
    readonly abyss_token: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly ctx_ident: (a: number, b: number, c: number) => [number, number, number, number];
    readonly ctx_node: (a: number, b: number, c: number) => [number, number, number];
    readonly ctx_nodes: (a: number, b: number, c: number) => [number, number, number, number];
    readonly ir_add: (a: number, b: number) => number;
    readonly ir_assign: (a: number, b: number) => number;
    readonly ir_bool: (a: number) => number;
    readonly ir_call: (a: number, b: number, c: number) => number;
    readonly ir_div: (a: number, b: number) => number;
    readonly ir_eq: (a: number, b: number) => number;
    readonly ir_exprStmt: (a: number) => number;
    readonly ir_gt: (a: number, b: number) => number;
    readonly ir_ifStmt: (a: number, b: number, c: number, d: number, e: number) => number;
    readonly ir_int: (a: number) => number;
    readonly ir_lt: (a: number, b: number) => number;
    readonly ir_mul: (a: number, b: number) => number;
    readonly ir_neq: (a: number, b: number) => number;
    readonly ir_sub: (a: number, b: number) => number;
    readonly ir_var: (a: number, b: number) => number;
    readonly ir_varDecl: (a: number, b: number, c: number) => number;
    readonly ir_whileStmt: (a: number, b: number, c: number) => number;
    readonly node___stash: (a: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __externref_drop_slice: (a: number, b: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
