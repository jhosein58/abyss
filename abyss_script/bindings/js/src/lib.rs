use js_sys::{Array, Function, Object, Reflect};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use abyss_script::png::DynamicSyntaxMagic;
use abyss_script::{
    lexer::{DynamicLexerRules, Scanner},
    parser::DynamicPrattParser,
};

use abyss_ir::ir::IrExprKind;
use abyss_ir::{
    facade::Ir,
    ir::{IrExpr, IrStmt, IrType},
};
use abyss_vm::{codegen::IrCompiler, vm::core::AbyssVm};

thread_local! {
    static CALLBACKS: RefCell<HashMap<u32, Function>> = RefCell::new(HashMap::new());
    static STASHED_NODE: RefCell<Option<Node>> = RefCell::new(None);
}

static mut CALLBACK_ID: u32 = 0;

fn register_callback(func: Function) -> u32 {
    unsafe {
        CALLBACK_ID += 1;
        let id = CALLBACK_ID;
        CALLBACKS.with(|cbs| cbs.borrow_mut().insert(id, func));
        id
    }
}

fn call_callback(id: u32, arg: &JsValue) -> Result<JsValue, JsValue> {
    CALLBACKS.with(|cbs| {
        cbs.borrow()
            .get(&id)
            .ok_or_else(|| JsValue::from_str("Callback not found"))?
            .call1(&JsValue::NULL, arg)
    })
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Node {
    expr: Option<IrExpr>,
    stmt: Option<IrStmt>,
}

#[wasm_bindgen]
impl Node {
    #[wasm_bindgen(js_name = __stash)]
    pub fn stash(&self) {
        STASHED_NODE.with(|n| *n.borrow_mut() = Some(self.clone()));
    }
}

impl Node {
    fn from_expr(expr: IrExpr) -> Self {
        Node {
            expr: Some(expr),
            stmt: None,
        }
    }

    fn from_stmt(stmt: IrStmt) -> Self {
        Node {
            expr: None,
            stmt: Some(stmt),
        }
    }

    fn to_stmt(self) -> IrStmt {
        self.stmt
            .or_else(|| self.expr.map(IrStmt::Expr))
            .expect("Empty node")
    }
}

fn extract_node(val: JsValue) -> Result<Node, JsValue> {
    let stash_str = JsValue::from_str("__stash");
    let func_val = js_sys::Reflect::get(&val, &stash_str)?;

    if func_val.is_undefined() {
        return Err(JsValue::from_str("Expected a Node object"));
    }

    let func = func_val.unchecked_into::<js_sys::Function>();
    func.call0(&val)?;

    STASHED_NODE.with(|n| {
        n.borrow_mut()
            .take()
            .ok_or_else(|| JsValue::from_str("Failed to extract Node"))
    })
}

#[wasm_bindgen]
pub struct IR;

#[wasm_bindgen]
impl IR {
    pub fn int(val: i32) -> Node {
        Node::from_expr(Ir::int(val as i64))
    }

    #[wasm_bindgen(js_name = bool)]
    pub fn bool_val(val: bool) -> Node {
        Node::from_expr(Ir::bool(val))
    }

    #[wasm_bindgen(js_name = var)]
    pub fn var_ref(name: String) -> Node {
        Node::from_expr(Ir::var(name))
    }

    pub fn add(l: &Node, r: &Node) -> Node {
        Node::from_expr(Ir::add(
            l.expr.clone().expect("Left must be expr"),
            r.expr.clone().expect("Right must be expr"),
        ))
    }

    pub fn sub(l: &Node, r: &Node) -> Node {
        Node::from_expr(Ir::sub(
            l.expr.clone().expect("Left must be expr"),
            r.expr.clone().expect("Right must be expr"),
        ))
    }

    pub fn mul(l: &Node, r: &Node) -> Node {
        Node::from_expr(Ir::mul(
            l.expr.clone().expect("Left must be expr"),
            r.expr.clone().expect("Right must be expr"),
        ))
    }

    pub fn div(l: &Node, r: &Node) -> Node {
        Node::from_expr(Ir::div(
            l.expr.clone().expect("Left must be expr"),
            r.expr.clone().expect("Right must be expr"),
        ))
    }

    pub fn eq(l: &Node, r: &Node) -> Node {
        Node::from_expr(Ir::eq(
            l.expr.clone().expect("Left must be expr"),
            r.expr.clone().expect("Right must be expr"),
        ))
    }

    pub fn lt(l: &Node, r: &Node) -> Node {
        Node::from_expr(Ir::lt(
            l.expr.clone().expect("Left must be expr"),
            r.expr.clone().expect("Right must be expr"),
        ))
    }

    pub fn gt(l: &Node, r: &Node) -> Node {
        Node::from_expr(Ir::gt(
            l.expr.clone().expect("Left must be expr"),
            r.expr.clone().expect("Right must be expr"),
        ))
    }

    pub fn neq(l: &Node, r: &Node) -> Node {
        Node::from_expr(Ir::neq(
            l.expr.clone().expect("Left must be expr"),
            r.expr.clone().expect("Right must be expr"),
        ))
    }

    pub fn call(func: &Node, args: Vec<Node>) -> Node {
        let name = match &func.expr.as_ref().expect("Func must be expr").kind {
            IrExprKind::VarRef(n) => n.clone(),
            _ => panic!("Function must be variable"),
        };
        Node::from_expr(Ir::call(
            name,
            args.into_iter()
                .map(|a| a.expr.expect("Arg must be expr"))
                .collect(),
        ))
    }

    #[wasm_bindgen(js_name = varDecl)]
    pub fn var_decl(name: String, val: &Node) -> Node {
        Node::from_stmt(Ir::var_dec(
            name,
            val.expr.clone().expect("Val must be expr"),
        ))
    }

    pub fn assign(target: &Node, val: &Node) -> Node {
        let name = match &target.expr.as_ref().expect("Target must be expr").kind {
            IrExprKind::VarRef(n) => n.clone(),
            _ => panic!("Target must be variable"),
        };
        Node::from_stmt(IrStmt::Assign {
            target: name,
            val: val.expr.clone().expect("Val must be expr"),
        })
    }

    #[wasm_bindgen(js_name = ifStmt)]
    pub fn if_stmt(cond: &Node, then_body: Vec<Node>, else_body: Vec<Node>) -> Node {
        Node::from_stmt(IrStmt::If(
            cond.expr.clone().expect("Cond must be expr"),
            then_body.into_iter().map(|s| s.to_stmt()).collect(),
            else_body.into_iter().map(|s| s.to_stmt()).collect(),
        ))
    }

    #[wasm_bindgen(js_name = whileStmt)]
    pub fn while_stmt(cond: &Node, body: Vec<Node>) -> Node {
        Node::from_stmt(IrStmt::While {
            cond: cond.expr.clone().expect("Cond must be expr"),
            body: body.into_iter().map(|s| s.to_stmt()).collect(),
        })
    }

    #[wasm_bindgen(js_name = exprStmt)]
    pub fn expr_stmt(expr: &Node) -> Node {
        Node::from_stmt(IrStmt::Expr(expr.expr.clone().expect("Must be expr")))
    }
}

#[wasm_bindgen]
pub struct Ctx {
    nodes: HashMap<String, Node>,
    lists: HashMap<String, Vec<Node>>,
    idents: HashMap<String, String>,
}

#[wasm_bindgen]
impl Ctx {
    pub fn node(&self, name: &str) -> Result<Node, JsValue> {
        self.nodes
            .get(name)
            .cloned()
            .ok_or_else(|| JsValue::from_str("Node not found"))
    }

    pub fn nodes(&self, name: &str) -> Result<Vec<Node>, JsValue> {
        self.lists
            .get(name)
            .cloned()
            .ok_or_else(|| JsValue::from_str("List not found"))
    }

    pub fn ident(&self, name: &str) -> Result<String, JsValue> {
        self.idents
            .get(name)
            .cloned()
            .ok_or_else(|| JsValue::from_str("Ident not found"))
    }
}

impl Ctx {
    fn new() -> Self {
        Ctx {
            nodes: HashMap::new(),
            lists: HashMap::new(),
            idents: HashMap::new(),
        }
    }

    fn set_node(&mut self, name: &str, node: Node) {
        self.nodes.insert(name.to_string(), node);
    }

    fn set_nodes(&mut self, name: &str, nodes: Vec<Node>) {
        self.lists.insert(name.to_string(), nodes);
    }

    fn set_ident(&mut self, name: &str, ident: String) {
        self.idents.insert(name.to_string(), ident);
    }
}

#[wasm_bindgen]
pub struct Abyss {
    source: String,
    tokens: Vec<(String, String)>,
    ignored: Vec<String>,
    number_rule: Option<(String, u32)>,
    ident_rule: Option<(String, u32)>,
    expr_rules: Vec<(String, u8, u32)>,
    stmt_rules: Vec<(String, u32)>,
}

#[wasm_bindgen]
impl Abyss {
    #[wasm_bindgen(constructor)]
    pub fn new(source: String) -> Self {
        Abyss {
            source,
            tokens: Vec::new(),
            ignored: Vec::new(),
            number_rule: None,
            ident_rule: None,
            expr_rules: Vec::new(),
            stmt_rules: Vec::new(),
        }
    }

    pub fn token(&mut self, name: String, pattern: String) {
        self.tokens.push((name, pattern));
    }

    pub fn ignore(&mut self, name: String) {
        self.ignored.push(name);
    }

    pub fn number(&mut self, token_name: String, callback: Function) {
        self.number_rule = Some((token_name, register_callback(callback)));
    }

    pub fn ident(&mut self, token_name: String, callback: Function) {
        self.ident_rule = Some((token_name, register_callback(callback)));
    }

    pub fn expr(&mut self, pattern: String, precedence: u8, callback: Function) {
        self.expr_rules
            .push((pattern, precedence, register_callback(callback)));
    }

    pub fn stmt(&mut self, pattern: String, callback: Function) {
        self.stmt_rules.push((pattern, register_callback(callback)));
    }

    fn build_parser(&self) -> Result<DynamicPrattParser<'_, Node>, JsValue> {
        let mut lexer_rules = DynamicLexerRules::new();
        for (name, pattern) in &self.tokens {
            lexer_rules.add_token(name, pattern);
        }

        let scanner = Scanner::new(&self.source, lexer_rules);
        let mut parser = DynamicPrattParser::new(scanner, 0);

        for ig in &self.ignored {
            parser.ignore_token(ig);
        }

        if let Some((token_name, cb_id)) = &self.number_rule {
            let cb_id = *cb_id;
            parser.register_rule(
                token_name,
                0,
                Some(Rc::new(move |_, tk| {
                    match call_callback(cb_id, &JsValue::from_str(tk.text)) {
                        Ok(val) => {
                            extract_node(val).map_err(|_| "Callback must return Node".to_string())
                        }
                        Err(e) => Err(format!("{:?}", e)),
                    }
                })),
                None,
            );
        }

        if let Some((token_name, cb_id)) = &self.ident_rule {
            let cb_id = *cb_id;
            parser.register_rule(
                token_name,
                0,
                Some(Rc::new(move |_, tk| {
                    match call_callback(cb_id, &JsValue::from_str(tk.text)) {
                        Ok(val) => {
                            extract_node(val).map_err(|_| "Callback must return Node".to_string())
                        }
                        Err(e) => Err(format!("{:?}", e)),
                    }
                })),
                None,
            );
        }

        for (pattern, precedence, cb_id) in &self.expr_rules {
            let cb_id = *cb_id;
            parser.define_expr(pattern, *precedence, move |ctx| {
                let mut js_ctx = Ctx::new();

                for (name, var) in &ctx.vars {
                    match var {
                        abyss_script::png::SyntaxVar::Expr(node) => {
                            js_ctx.set_node(name, node.clone());
                        }
                        abyss_script::png::SyntaxVar::Ident(s) => {
                            js_ctx.set_ident(name, s.clone());
                        }
                        abyss_script::png::SyntaxVar::List(list) => {
                            let nodes: Vec<Node> = list
                                .iter()
                                .filter_map(|v| match v {
                                    abyss_script::png::SyntaxVar::Expr(n) => Some(n.clone()),
                                    _ => None,
                                })
                                .collect();
                            js_ctx.set_nodes(name, nodes);
                        }
                    }
                }

                match call_callback(cb_id, &JsValue::from(js_ctx)) {
                    Ok(val) => extract_node(val).unwrap_or_else(|_| Node::from_expr(Ir::int(0))),
                    Err(_) => Node::from_expr(Ir::int(0)),
                }
            });
        }

        for (pattern, cb_id) in &self.stmt_rules {
            let cb_id = *cb_id;
            parser.define_stmt(pattern, move |ctx| {
                let mut js_ctx = Ctx::new();

                for (name, var) in &ctx.vars {
                    match var {
                        abyss_script::png::SyntaxVar::Expr(node) => {
                            js_ctx.set_node(name, node.clone());
                        }
                        abyss_script::png::SyntaxVar::Ident(s) => {
                            js_ctx.set_ident(name, s.clone());
                        }
                        abyss_script::png::SyntaxVar::List(list) => {
                            let nodes: Vec<Node> = list
                                .iter()
                                .filter_map(|v| match v {
                                    abyss_script::png::SyntaxVar::Expr(n) => Some(n.clone()),
                                    _ => None,
                                })
                                .collect();
                            js_ctx.set_nodes(name, nodes);
                        }
                    }
                }

                match call_callback(cb_id, &JsValue::from(js_ctx)) {
                    Ok(val) => extract_node(val).unwrap_or_else(|_| Node::from_expr(Ir::int(0))),
                    Err(_) => Node::from_expr(Ir::int(0)),
                }
            });
        }

        Ok(parser)
    }

    pub fn parse(&self) -> Result<Vec<Node>, JsValue> {
        let mut parser = self.build_parser()?;
        parser.advance();
        parser.parse_program().map_err(|e| JsValue::from_str(&e))
    }

    pub fn run(&self, host_functions: Object) -> Result<(), JsValue> {
        let nodes = self.parse()?;
        let stmts: Vec<IrStmt> = nodes.into_iter().map(|n| n.to_stmt()).collect();
        let ir_program = Ir::program(stmts);

        let mut compiler = IrCompiler::new();

        let keys = Object::keys(&host_functions);
        for i in 0..keys.length() {
            let key = keys.get(i).as_string().unwrap();
            compiler.register_extern(&key, vec![IrType::I64], IrType::I64);
        }

        let (instructions, constants, extern_defs) = compiler.compile(&ir_program);
        let mut vm = AbyssVm::new(instructions, constants);

        for i in 0..keys.length() {
            let key = keys.get(i).as_string().unwrap();
            let func: Function = Reflect::get(&host_functions, &JsValue::from_str(&key))?.into();
            let func_id = register_callback(func);

            let host_fn = Rc::new(move |args: &[u64], _: &mut [u8]| -> u64 {
                let js_args = Array::new();
                for &arg in args {
                    js_args.push(&JsValue::from_f64(arg as i64 as f64));
                }
                call_callback(func_id, &js_args.into()).ok();
                0
            });

            vm.register_host_function(&key, 1, vec![false], host_fn);
        }

        vm.load_imports(&extern_defs);
        vm.init_globals(ir_program.globals.len());
        vm.run();

        Ok(())
    }
}
