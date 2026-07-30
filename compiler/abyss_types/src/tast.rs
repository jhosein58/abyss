use abyss_diagnostics::Span;
use abyss_parser::ast::{BinaryOp, Lit, UnaryOp};

use crate::types::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypedExpr {
    pub kind: TypedExprKind,
    pub ty: Type,
    pub span: Span,
    pub id: u32,
}

impl TypedExpr {
    pub fn span_expr(&self) -> Span {
        match self.kind {
            TypedExprKind::Binary(ref l, _, ref r) => l.span_expr().merge(r.span_expr()),
            TypedExprKind::If(_, ref then_b, ref else_b) => {
                if let Some(eb) = else_b.clone() {
                    self.span.clone().merge(eb.span_expr())
                } else {
                    self.span.clone().merge(then_b.span_expr())
                }
            }
            _ => self.span.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypedExprKind {
    // --- Module & Scope ---
    Mod(Box<TypedExpr>, Box<TypedExpr>),
    Use(Box<TypedExpr>),

    // --- Sequences & Functions & dec ---
    SequenceInit(Vec<SequenceElement>),
    FunctionDef {
        name: String,
        args: Vec<TypedExpr>,
        ret_ty: Type,
        body: Box<TypedExpr>,
        is_native: bool,
    },
    FuncRef(String),
    VarDec(String, Type, Option<Box<TypedExpr>>),
    Def(String, Box<TypedExpr>),
    Comptime(Box<TypedExpr>),

    // --- Control Flow ---
    Ret(Option<Box<TypedExpr>>),
    Out(Option<Box<TypedExpr>>),
    Continue,
    Block(Vec<TypedExpr>),
    If(Box<TypedExpr>, Box<TypedExpr>, Option<Box<TypedExpr>>),

    // --- Loops ---
    For(Box<TypedExpr>, Box<TypedExpr>, Box<TypedExpr>),
    Range {
        start: Option<Box<TypedExpr>>,
        end: Option<Box<TypedExpr>>,
        step: Option<Box<TypedExpr>>,
        inclusive: bool,
    },
    While(Box<TypedExpr>, Box<TypedExpr>, Option<Box<TypedExpr>>),
    Forever(Box<TypedExpr>),
    Defer(Box<TypedExpr>),

    Lit(Lit),
    Ident(String),
    Binary(Box<TypedExpr>, BinaryOp, Box<TypedExpr>),
    Unary(UnaryOp, Box<TypedExpr>),
    Call(Box<TypedExpr>, Vec<TypedExpr>, bool), // calle, args, is_native
    Index(Box<TypedExpr>, Box<TypedExpr>),      // a[b]
    FieldAccess(Box<TypedExpr>, String),        // a.b
    Cast(Box<TypedExpr>, Box<TypedExpr>),
    Is(Box<TypedExpr>, Option<Box<TypedExpr>>),
    Member(Box<TypedExpr>, String),
    BoundMethod {
        receiver: Box<TypedExpr>,
        method_name: String,
    },
    SizeOf(Option<Box<TypedExpr>>),

    Match {
        subject: Box<TypedExpr>,
        arms: Vec<TypedMatchArm>,
    },
    Then(Box<TypedExpr>, Box<TypedExpr>),
    TypeOf(Box<TypedExpr>),
    Refinement(Option<Box<TypedExpr>>, Box<TypedExpr>),
    Attributed(Vec<TypedAttribute>, Box<TypedExpr>),
    Wildcard,
    Type(Type),
    ErrorPlaceholder,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SequenceElement {
    pub label: Option<String>,
    pub expr: TypedExpr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypedMatchArm {
    pub pattern: Box<TypedExpr>,
    pub body: Box<TypedExpr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypedAttribute {
    pub name: String,
    pub args: Vec<TypedExpr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct TypedProgram {
    pub body: TypedExpr,
    pub globals: Vec<(String, TypedExpr)>,
}

impl TypedProgram {
    pub fn print_tree(&self) {
        print!("{}", self.format_tree());
    }

    pub fn format_tree(&self) -> String {
        let mut out = String::new();
        out.push_str("\x1b[1;36mTAST Program Root:\x1b[0m\n");

        let has_globals = !self.globals.is_empty();

        if has_globals {
            out.push_str(&format!(
                "├── \x1b[1;34mGlobals\x1b[0m ({} items)\n",
                self.globals.len()
            ));
            for (i, (name, expr)) in self.globals.iter().enumerate() {
                let is_last_global = i == self.globals.len() - 1;
                expr.format_recursive(&mut out, "│   ", is_last_global, false, Some(name));
            }
        }

        out.push_str("└── \x1b[1;35mBody\x1b[0m\n");
        self.body
            .format_recursive(&mut out, "    ", true, false, None);

        out
    }
}

impl TypedExpr {
    pub fn print_tree(&self) {
        print!("{}", self.format_tree());
    }

    pub fn format_tree(&self) -> String {
        let mut out = String::new();
        out.push_str("\x1b[1;36mTAST Tree Root:\x1b[0m\n");
        self.format_recursive(&mut out, "", true, true, None);
        out
    }

    fn format_recursive(
        &self,
        out: &mut String,
        prefix: &str,
        is_last: bool,
        is_root: bool,
        edge_label: Option<&str>,
    ) {
        let connector = if is_root {
            ""
        } else if is_last {
            "└── "
        } else {
            "├── "
        };

        let label_str = if let Some(l) = edge_label {
            format!("\x1b[38;5;243m{}:\x1b[0m ", l)
        } else {
            String::new()
        };

        let node_info: String;
        let mut children: Vec<(Option<String>, &TypedExpr)> = Vec::new();

        macro_rules! add_child {
            ($label:expr, $expr:expr) => {
                children.push((Some($label.to_string()), $expr))
            };
            ($expr:expr) => {
                children.push((None, $expr))
            };
        }

        macro_rules! add_opt_child {
            ($label:expr, $opt_expr:expr) => {
                if let Some(e) = $opt_expr {
                    children.push((Some($label.to_string()), e))
                }
            };
        }

        match &self.kind {
            TypedExprKind::Mod(l, r) => {
                node_info = "\x1b[1;35mMod\x1b[0m".to_string();
                add_child!("left", l);
                add_child!("right", r);
            }
            TypedExprKind::Use(e) => {
                node_info = "\x1b[1;35mUse\x1b[0m".to_string();
                add_child!(e);
            }
            TypedExprKind::SequenceInit(elems) => {
                node_info = format!("\x1b[1;35mSequenceInit\x1b[0m ({} items)", elems.len());
                for (i, elem) in elems.iter().enumerate() {
                    let lbl = elem.label.clone().unwrap_or_else(|| format!("[{}]", i));
                    children.push((Some(lbl), &elem.expr));
                }
            }
            TypedExprKind::FunctionDef {
                name,
                args,
                ret_ty,
                body,
                is_native,
            } => {
                let native_tag = if *is_native {
                    " \x1b[33m[native]\x1b[0m"
                } else {
                    ""
                };
                node_info = format!(
                    "\x1b[1;32mƒ {}\x1b[0m{} → \x1b[36m{}\x1b[0m",
                    name,
                    native_tag,
                    ret_ty.name()
                );
                for (i, arg) in args.iter().enumerate() {
                    children.push((Some(format!("arg{}", i)), arg));
                }
                add_child!("body", body);
            }
            TypedExprKind::FuncRef(name) => {
                node_info = format!("\x1b[1;33m&{}\x1b[0m", name);
            }
            TypedExprKind::VarDec(name, ty, init) => {
                node_info = format!(
                    "\x1b[1;34mvar {}\x1b[0m: \x1b[36m{}\x1b[0m",
                    name,
                    ty.name()
                );
                add_opt_child!("=", init);
            }
            TypedExprKind::Def(name, expr) => {
                node_info = format!("\x1b[1;34mdef {}\x1b[0m", name);
                add_child!(":=", expr);
            }
            TypedExprKind::Comptime(expr) => {
                node_info = "\x1b[1;95mcomptime\x1b[0m".to_string();
                add_child!(expr);
            }
            TypedExprKind::Ret(expr) => {
                node_info = "\x1b[1;31mreturn\x1b[0m".to_string();
                add_opt_child!("", expr);
            }
            TypedExprKind::Out(expr) => {
                node_info = "\x1b[1;31mout\x1b[0m".to_string();
                add_opt_child!("", expr);
            }
            TypedExprKind::Continue => {
                node_info = "\x1b[1;31mcontinue\x1b[0m".to_string();
            }
            TypedExprKind::Block(stmts) => {
                node_info = format!("\x1b[1;90m{{}}\x1b[0m ({} stmts)", stmts.len());
                for stmt in stmts {
                    add_child!(stmt);
                }
            }
            TypedExprKind::If(cond, then_b, else_b) => {
                node_info = "\x1b[1;33mif\x1b[0m".to_string();
                add_child!("cond", cond);
                add_child!("then", then_b);
                add_opt_child!("else", else_b);
            }
            TypedExprKind::For(init, cond, step) => {
                node_info = "\x1b[1;33mfor\x1b[0m".to_string();
                add_child!("init", init);
                add_child!("cond", cond);
                add_child!("step", step);
            }
            TypedExprKind::Range {
                start,
                end,
                step,
                inclusive,
            } => {
                let inc_char = if *inclusive { "..=" } else { ".." };
                node_info = format!("\x1b[1;90mrange {}\x1b[0m", inc_char);
                add_opt_child!("start", start);
                add_opt_child!("end", end);
                add_opt_child!("step", step);
            }
            TypedExprKind::While(cond, body, else_b) => {
                node_info = "\x1b[1;33mwhile\x1b[0m".to_string();
                add_child!("cond", cond);
                add_child!("body", body);
                add_opt_child!("else", else_b);
            }
            TypedExprKind::Forever(body) => {
                node_info = "\x1b[1;33mforever\x1b[0m".to_string();
                add_child!("body", body);
            }
            TypedExprKind::Defer(expr) => {
                node_info = "\x1b[1;90mdefer\x1b[0m".to_string();
                add_child!(expr);
            }
            TypedExprKind::Lit(lit) => {
                node_info = match lit {
                    Lit::Int(n) => format!("\x1b[38;5;208m{}\x1b[0m", n),
                    Lit::Float(f) => format!("\x1b[38;5;213m{}\x1b[0m", f.0),
                    Lit::Bool(b) => format!("\x1b[38;5;85m{}\x1b[0m", b),
                    Lit::Str(s) => format!("\x1b[38;5;118m\"{}\"\x1b[0m", s),
                    Lit::Cstr(s) => format!("\x1b[38;5;196mc\"{}\"\x1b[0m", s),
                    Lit::Char(c) => format!("\x1b[38;5;159m'{}'\x1b[0m", c),
                };
            }
            TypedExprKind::Ident(name) => {
                node_info = format!("\x1b[1;37m{}\x1b[0m", name);
            }
            TypedExprKind::Binary(l, op, r) => {
                let op_str = format!("{:?}", op);
                node_info = format!("\x1b[1;90m{}\x1b[0m", op_str);
                add_child!("lhs", l);
                add_child!("rhs", r);
            }
            TypedExprKind::Unary(op, expr) => {
                let op_str = format!("{:?}", op);
                node_info = format!("\x1b[1;90m{}\x1b[0m", op_str);
                add_child!(expr);
            }
            TypedExprKind::Call(callee, args, is_native) => {
                let native_tag = if *is_native {
                    " \x1b[33m[native]\x1b[0m"
                } else {
                    ""
                };
                node_info = format!("\x1b[1;35mcall\x1b[0m{}", native_tag);
                add_child!("fn", callee);
                for (i, arg) in args.iter().enumerate() {
                    children.push((Some(format!("{}", i)), arg));
                }
            }
            TypedExprKind::Index(expr, idx) => {
                node_info = "\x1b[1;90m[]\x1b[0m".to_string();
                add_child!("array", expr);
                add_child!("index", idx);
            }
            TypedExprKind::FieldAccess(expr, field) => {
                node_info = format!("\x1b[1;90m.{}\x1b[0m", field);
                add_child!("object", expr);
            }
            TypedExprKind::Cast(expr, target) => {
                node_info = "\x1b[1;90mas\x1b[0m".to_string();
                add_child!("expr", expr);
                add_child!("→", target);
            }
            TypedExprKind::Is(expr, target) => {
                node_info = "\x1b[1;90mis\x1b[0m".to_string();
                add_child!("expr", expr);
                add_opt_child!("?", target);
            }
            TypedExprKind::Member(expr, name) => {
                node_info = format!("\x1b[1;90m.{}()\x1b[0m", name);
                add_child!("object", expr);
            }
            TypedExprKind::SizeOf(expr) => {
                node_info = "\x1b[1;90msizeof\x1b[0m".to_string();
                add_opt_child!("type", expr);
            }
            TypedExprKind::Match { subject, arms } => {
                node_info = format!("\x1b[1;33mmatch\x1b[0m ({} arms)", arms.len());
                add_child!("subject", subject);
                for (i, arm) in arms.iter().enumerate() {
                    children.push((Some(format!("arm{}·pat", i)), &arm.pattern));
                    children.push((Some(format!("arm{}·body", i)), &arm.body));
                }
            }
            TypedExprKind::Then(e1, e2) => {
                node_info = "\x1b[1;90mthen\x1b[0m".to_string();
                add_child!("first", e1);
                add_child!("second", e2);
            }
            TypedExprKind::TypeOf(expr) => {
                node_info = "\x1b[1;90mtypeof\x1b[0m".to_string();
                add_child!(expr);
            }
            TypedExprKind::Refinement(cond, expr) => {
                node_info = "\x1b[1;90mrefine\x1b[0m".to_string();
                add_opt_child!("when", cond);
                add_child!("expr", expr);
            }
            TypedExprKind::Attributed(attrs, expr) => {
                let attr_names: Vec<String> = attrs.iter().map(|a| a.name.clone()).collect();
                node_info = format!("\x1b[1;90m#[{}]\x1b[0m", attr_names.join(", "));
                add_child!("target", expr);
            }
            TypedExprKind::Wildcard => {
                node_info = "\x1b[2;90m_\x1b[0m".to_string();
            }
            TypedExprKind::Type(ty) => {
                node_info = format!("\x1b[1;36mtype {}\x1b[0m", ty.name());
            }
            TypedExprKind::ErrorPlaceholder => {
                node_info = "\x1b[1;41;97m ERROR \x1b[0m".to_string();
            }
            _ => panic!(),
        }

        let type_str = format!("\x1b[38;5;245m: {}\x1b[0m", self.ty.name());
        let id_str = format!("\x1b[2;38;5;238m[id:{}]\x1b[0m", self.id);

        out.push_str(&format!(
            "{}{}{}{} {} {}\n",
            prefix, connector, label_str, node_info, type_str, id_str
        ));

        let child_prefix = if is_root {
            "".to_string()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        let num_children = children.len();
        for (i, (child_label, child_expr)) in children.into_iter().enumerate() {
            let is_last_child = i == num_children - 1;
            child_expr.format_recursive(
                out,
                &child_prefix,
                is_last_child,
                false,
                child_label.as_deref(),
            );
        }
    }
}
