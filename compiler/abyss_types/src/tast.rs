use std::collections::HashMap;

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
    ArrayInit(Vec<TypedExpr>, Option<Box<TypedExpr>>),
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
    Index(Box<TypedExpr>, Box<TypedExpr>),
    Cast(Box<TypedExpr>, Option<Box<TypedExpr>>),
    Is(Box<TypedExpr>, Option<Box<TypedExpr>>),
    Member(Box<TypedExpr>, String),
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
    pub globals: HashMap<String, TypedExpr>,
}

impl TypedExpr {
    pub fn print_tree(&self) {
        self.print_recursive(0);
    }
    fn print_recursive(&self, indent: usize) {
        let pad = "  ".repeat(indent);
        let arrow = if indent > 0 { "└─ " } else { "" };

        print!("{}{}", pad, arrow);

        match &self.kind {
            TypedExprKind::FunctionDef {
                name,
                args,
                ret_ty,
                body,
                is_native,
            } => {
                let _ = is_native;
                println!(
                    "[FunctionDef '{}' -> {}] :: {}",
                    name,
                    ret_ty.name(),
                    self.ty.name()
                );
                for arg in args {
                    arg.print_recursive(indent + 1);
                }
                body.print_recursive(indent + 1);
            }
            TypedExprKind::FuncRef(name) => {
                println!("[FuncRef '{}'] :: {}", name, self.ty.name());
            }
            TypedExprKind::Lit(l) => {
                println!("[Lit {:?}] :: {}", l, self.ty.name());
            }
            TypedExprKind::Ident(name) => {
                println!("[Ident '{}'] :: {}", name, self.ty.name());
            }
            TypedExprKind::VarDec(name, ty, init_opt) => {
                println!("[VarDec '{}'] :: {}", name, ty.name());
                if let Some(init_expr) = init_opt {
                    init_expr.print_recursive(indent + 1);
                }
            }
            TypedExprKind::Binary(left, op, right) => {
                println!("[Binary {:?}] :: {}", op, self.ty.name());
                left.print_recursive(indent + 1);
                right.print_recursive(indent + 1);
            }
            TypedExprKind::Unary(op, expr) => {
                println!("[Unary {:?}] :: {}", op, self.ty.name());
                expr.print_recursive(indent + 1);
            }
            TypedExprKind::Block(stmts) => {
                println!("[Block] :: {}", self.ty.name());
                for stmt in stmts {
                    stmt.print_recursive(indent + 1);
                }
            }
            TypedExprKind::If(cond, then_branch, else_branch) => {
                println!("[If] :: {}", self.ty.name());
                cond.print_recursive(indent + 1);
                then_branch.print_recursive(indent + 1);
                if let Some(else_b) = else_branch {
                    else_b.print_recursive(indent + 1);
                }
            }
            TypedExprKind::While(cond, body, else_b) => {
                println!("[While] :: {}", self.ty.name());
                cond.print_recursive(indent + 1);
                body.print_recursive(indent + 1);
                if let Some(eb) = else_b {
                    eb.print_recursive(indent + 1);
                }
            }
            TypedExprKind::Call(func, args, _) => {
                println!("[Call] :: {}", self.ty.name());
                func.print_recursive(indent + 1);
                for arg in args {
                    arg.print_recursive(indent + 1);
                }
            }
            TypedExprKind::Ret(val) => {
                println!("[Return] :: {}", self.ty.name());
                if let Some(v) = val {
                    v.print_recursive(indent + 1);
                }
            }
            TypedExprKind::Out(val) => {
                println!("[out] :: {}", self.ty.name());
                if let Some(v) = val {
                    v.print_recursive(indent + 1);
                }
            }
            TypedExprKind::Continue => println!("[Continue]"),
            TypedExprKind::Mod(left, right) => {
                println!("[Mod] :: {}", self.ty.name());
                left.print_recursive(indent + 1);
                right.print_recursive(indent + 1);
            }
            TypedExprKind::Member(expr, name) => {
                println!("[Member '.{}'] :: {}", name, self.ty.name());
                expr.print_recursive(indent + 1);
            }
            TypedExprKind::Index(expr, idx) => {
                println!("[Index] :: {}", self.ty.name());
                expr.print_recursive(indent + 1);
                idx.print_recursive(indent + 1);
            }
            TypedExprKind::Cast(expr, _) => {
                println!("[Cast] :: {}", self.ty.name());
                expr.print_recursive(indent + 1);
            }
            TypedExprKind::ErrorPlaceholder => {
                println!("[ErrorPlaceholder] :: {}", self.ty.name());
            }

            other => {
                let debug_str = format!("{:?}", other);
                let variant_name = debug_str
                    .split('(')
                    .next()
                    .unwrap_or("Unknown")
                    .split('{')
                    .next()
                    .unwrap_or("Unknown");
                println!("[{}] :: {}", variant_name, self.ty.name());
            }
        }
    }
}
