use pyo3::prelude::*;
use std::rc::Rc;

#[pyclass]
#[derive(Clone)]
pub struct Node {
    expr: Option<IrExpr>,
    stmt: Option<IrStmt>,
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
        if let Some(stmt) = self.stmt {
            stmt
        } else if let Some(expr) = self.expr {
            IrStmt::Expr(expr)
        } else {
            panic!("Empty node")
        }
    }
}

use abyss_script::png::{DynamicSyntaxMagic, SyntaxCtx};
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

#[pyclass]
#[derive(Clone)]
pub struct PyExpr {
    pub inner: IrExpr,
}

#[pyclass]
#[derive(Clone)]
pub struct PyStmt {
    pub inner: IrStmt,
}

#[pyclass]
pub struct IR;

#[pymethods]
impl IR {
    #[staticmethod]
    fn int(val: i64) -> Node {
        Node::from_expr(Ir::int(val))
    }

    #[staticmethod]
    fn bool_val(val: bool) -> Node {
        Node::from_expr(Ir::bool(val))
    }

    #[staticmethod]
    fn var(name: String) -> Node {
        Node::from_expr(Ir::var(name))
    }

    #[staticmethod]
    fn add(l: &Node, r: &Node) -> Node {
        Node::from_expr(Ir::add(l.expr.clone().unwrap(), r.expr.clone().unwrap()))
    }

    #[staticmethod]
    fn sub(l: &Node, r: &Node) -> Node {
        Node::from_expr(Ir::sub(l.expr.clone().unwrap(), r.expr.clone().unwrap()))
    }

    #[staticmethod]
    fn mul(l: &Node, r: &Node) -> Node {
        Node::from_expr(Ir::mul(l.expr.clone().unwrap(), r.expr.clone().unwrap()))
    }

    #[staticmethod]
    fn div(l: &Node, r: &Node) -> Node {
        Node::from_expr(Ir::div(l.expr.clone().unwrap(), r.expr.clone().unwrap()))
    }

    #[staticmethod]
    fn eq(l: &Node, r: &Node) -> Node {
        Node::from_expr(Ir::eq(l.expr.clone().unwrap(), r.expr.clone().unwrap()))
    }

    #[staticmethod]
    fn lt(l: &Node, r: &Node) -> Node {
        Node::from_expr(Ir::lt(l.expr.clone().unwrap(), r.expr.clone().unwrap()))
    }

    #[staticmethod]
    fn gt(l: &Node, r: &Node) -> Node {
        Node::from_expr(Ir::gt(l.expr.clone().unwrap(), r.expr.clone().unwrap()))
    }

    #[staticmethod]
    fn call(func: &Node, args: Vec<Node>) -> Node {
        let name = match &func.expr.as_ref().unwrap().kind {
            IrExprKind::VarRef(n) => n.clone(),
            _ => panic!("Function must be a variable"),
        };
        Node::from_expr(Ir::call(
            name,
            args.into_iter().map(|a| a.expr.unwrap()).collect(),
        ))
    }

    #[staticmethod]
    fn var_decl(name: String, val: &Node) -> Node {
        Node::from_stmt(Ir::var_dec(name, val.expr.clone().unwrap()))
    }

    #[staticmethod]
    fn assign(target: &Node, val: &Node) -> Node {
        let name = match &target.expr.as_ref().unwrap().kind {
            IrExprKind::VarRef(n) => n.clone(),
            _ => panic!("Assignment target must be a variable"),
        };
        Node::from_stmt(IrStmt::Assign {
            target: name,
            val: val.expr.clone().unwrap(),
        })
    }

    #[staticmethod]
    fn if_stmt(cond: &Node, then_body: Vec<Node>, else_body: Vec<Node>) -> Node {
        Node::from_stmt(IrStmt::If(
            cond.expr.clone().unwrap(),
            then_body.into_iter().map(|s| s.to_stmt()).collect(),
            else_body.into_iter().map(|s| s.to_stmt()).collect(),
        ))
    }

    #[staticmethod]
    fn while_stmt(cond: &Node, body: Vec<Node>) -> Node {
        Node::from_stmt(IrStmt::While {
            cond: cond.expr.clone().unwrap(),
            body: body.into_iter().map(|s| s.to_stmt()).collect(),
        })
    }

    #[staticmethod]
    fn expr_stmt(expr: &Node) -> Node {
        Node::from_stmt(IrStmt::Expr(expr.expr.clone().unwrap()))
    }
}

#[pyclass(unsendable)]
pub struct Ctx {
    ctx: SyntaxCtx<PyObject>,
}

#[pymethods]
impl Ctx {
    fn node(&self, py: Python, name: &str) -> PyResult<PyObject> {
        Ok(self.ctx.get_node(name).clone_ref(py))
    }

    fn nodes(&self, py: Python, name: &str) -> PyResult<Vec<PyObject>> {
        Ok(self
            .ctx
            .get_node_list(name)
            .into_iter()
            .map(|o| o.clone_ref(py))
            .collect())
    }

    fn ident(&self, _py: Python, name: &str) -> PyResult<String> {
        Ok(self.ctx.get_ident(name))
    }
}

#[pyclass(unsendable)]
pub struct Abyss {
    source: String,
    tokens: Vec<(String, String)>,
    ignored: Vec<String>,
    expr_rules: Vec<(String, u8, PyObject)>,
    stmt_rules: Vec<(String, PyObject)>,
    number_rule: Option<(String, PyObject)>,
    ident_rule: Option<(String, PyObject)>,
}

#[pymethods]
impl Abyss {
    #[new]
    fn new(source: String) -> Self {
        Abyss {
            source,
            tokens: Vec::new(),
            ignored: Vec::new(),
            expr_rules: Vec::new(),
            stmt_rules: Vec::new(),
            number_rule: None,
            ident_rule: None,
        }
    }

    fn token(&mut self, name: String, pattern: String) {
        self.tokens.push((name, pattern));
    }

    fn ignore(&mut self, name: String) {
        self.ignored.push(name);
    }

    fn number(&mut self, token_name: String, callback: PyObject) {
        self.number_rule = Some((token_name, callback));
    }

    fn ident(&mut self, token_name: String, callback: PyObject) {
        self.ident_rule = Some((token_name, callback));
    }

    fn expr(&mut self, pattern: String, precedence: u8, callback: PyObject) {
        self.expr_rules.push((pattern, precedence, callback));
    }

    fn stmt(&mut self, pattern: String, callback: PyObject) {
        self.stmt_rules.push((pattern, callback));
    }

    fn parse(&self, py: Python) -> PyResult<Vec<PyObject>> {
        let mut lexer_rules = DynamicLexerRules::new();
        for (name, regex) in &self.tokens {
            lexer_rules.add_token(name, regex);
        }

        let scanner = Scanner::new(&self.source, lexer_rules);
        let mut parser: DynamicPrattParser<PyObject> = DynamicPrattParser::new(scanner, 0);

        for ig in &self.ignored {
            parser.ignore_token(ig);
        }

        if let Some((token_name, cb)) = &self.number_rule {
            let cb = cb.clone();
            parser.register_rule(
                token_name,
                0,
                Some(Rc::new(move |_, tk| {
                    Python::with_gil(|py| match cb.call1(py, (tk.text,)) {
                        Ok(res) => Ok(res.into_py(py)),
                        Err(e) => {
                            e.print(py);
                            Err("Number callback failed".to_string())
                        }
                    })
                })),
                None,
            );
        }

        if let Some((token_name, cb)) = &self.ident_rule {
            let cb = cb.clone();
            parser.register_rule(
                token_name,
                0,
                Some(Rc::new(move |_, tk| {
                    Python::with_gil(|py| match cb.call1(py, (tk.text,)) {
                        Ok(res) => Ok(res.into_py(py)),
                        Err(e) => {
                            e.print(py);
                            Err("Ident callback failed".to_string())
                        }
                    })
                })),
                None,
            );
        }

        for (pattern, precedence, cb) in &self.expr_rules {
            let cb = cb.clone();
            parser.define_expr(pattern, *precedence, move |ctx| {
                Python::with_gil(|py| {
                    let py_ctx = Ctx { ctx };
                    let py_ctx_obj = Py::new(py, py_ctx).unwrap();
                    match cb.call1(py, (py_ctx_obj,)) {
                        Ok(res) => res.into_py(py),
                        Err(e) => {
                            e.print(py);
                            py.None()
                        }
                    }
                })
            });
        }

        for (pattern, cb) in &self.stmt_rules {
            let cb = cb.clone();
            parser.define_stmt(pattern, move |ctx| {
                Python::with_gil(|py| {
                    let py_ctx = Ctx { ctx };
                    let py_ctx_obj = Py::new(py, py_ctx).unwrap();
                    match cb.call1(py, (py_ctx_obj,)) {
                        Ok(res) => res.into_py(py),
                        Err(e) => {
                            e.print(py);
                            py.None()
                        }
                    }
                })
            });
        }

        parser.advance();
        match parser.parse_program() {
            Ok(nodes) => Ok(nodes),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PySyntaxError, _>(e)),
        }
    }

    fn run(&self, py: Python, host_functions: Option<PyObject>) -> PyResult<()> {
        let nodes = self.parse(py)?;

        let stmts: Vec<IrStmt> = nodes
            .into_iter()
            .map(|obj| {
                if let Ok(node) = obj.extract::<Node>(py) {
                    if let Some(stmt) = node.stmt {
                        stmt
                    } else if let Some(expr) = node.expr {
                        IrStmt::Expr(expr)
                    } else {
                        panic!("Empty node");
                    }
                } else {
                    panic!("Invalid node type: expected Node");
                }
            })
            .collect();

        let ir_program = Ir::program(stmts);
        let mut compiler = IrCompiler::new();

        if let Some(ref funcs) = host_functions {
            if let Ok(dict) = funcs.downcast::<pyo3::types::PyDict>(py) {
                for (name, _) in dict.iter() {
                    let name_str = name.extract::<String>()?;
                    compiler.register_extern(&name_str, vec![IrType::I64], IrType::I64);
                }
            }
        }

        let (instructions, constants, extern_defs) = compiler.compile(&ir_program);
        let mut vm = AbyssVm::new(instructions, constants);

        if let Some(funcs) = host_functions {
            if let Ok(dict) = funcs.downcast::<pyo3::types::PyDict>(py) {
                for (name, func) in dict.iter() {
                    let name_str = name.extract::<String>()?;
                    let func_obj = func.to_object(py);

                    let host_fn = Rc::new(move |args: &[u64], _heap: &mut [u8]| -> u64 {
                        Python::with_gil(|py| {
                            let py_args: Vec<i64> = args.iter().map(|&v| v as i64).collect();
                            match func_obj.call1(py, (py_args,)) {
                                Ok(_) => 0,
                                Err(e) => {
                                    e.print(py);
                                    0
                                }
                            }
                        })
                    });

                    vm.register_host_function(&name_str, 1, vec![false], host_fn);
                }
            }
        }

        vm.load_imports(&extern_defs);
        vm.init_globals(ir_program.globals.len());
        vm.run();

        Ok(())
    }
}

#[pymodule]
fn abyss_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Abyss>()?;
    m.add_class::<Ctx>()?;
    m.add_class::<Node>()?;
    m.add_class::<IR>()?;
    Ok(())
}
