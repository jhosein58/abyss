use abyss_diagnostics::Span;
use abyss_parser::ast::{BinaryOp, Lit, UnaryOp};

use crate::type_checker::types::Type;

#[derive(Debug, Clone)]
pub struct TypedExpr {
    pub kind: TypedExprKind,
    pub ty: Type,
    pub span: Span,
    pub id: u32,
}

#[derive(Debug, Clone)]
pub enum TypedExprKind {
    // --- Module & Scope ---
    Mod(Box<TypedExpr>, Box<TypedExpr>),
    Use(Box<TypedExpr>),

    // --- Sequences & Functions ---
    Sequence(Vec<TypedExpr>, Option<Box<TypedExpr>>),
    Signature(Vec<TypedExpr>, Option<Box<TypedExpr>>, Box<TypedExpr>),

    // --- Control Flow ---
    Ret(Option<Box<TypedExpr>>),
    Break,
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
    While(Box<TypedExpr>, Box<TypedExpr>),
    Forever(Box<TypedExpr>),
    Defer(Box<TypedExpr>),

    Lit(Lit),
    Ident(String),
    Binary(Box<TypedExpr>, BinaryOp, Box<TypedExpr>),
    Unary(UnaryOp, Box<TypedExpr>),
    Call(Box<TypedExpr>, Vec<TypedExpr>),
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

    ErrorPlaceholder,
}

#[derive(Debug, Clone)]
pub struct TypedMatchArm {
    pub pattern: Box<TypedExpr>,
    pub body: Box<TypedExpr>,
}

#[derive(Debug, Clone)]
pub struct TypedAttribute {
    pub name: String,
    pub args: Vec<TypedExpr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct TypedProgram {
    pub body: TypedExpr,
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
            TypedExprKind::Lit(l) => {
                println!("[Lit {:?}] :: {:?}", l, self.ty);
            }
            TypedExprKind::Ident(name) => {
                println!("[Ident '{}'] :: {:?}", name, self.ty);
            }
            TypedExprKind::Binary(left, op, right) => {
                println!("[Binary {:?}] :: {:?}", op, self.ty);
                left.print_recursive(indent + 1);
                right.print_recursive(indent + 1);
            }
            TypedExprKind::Unary(op, expr) => {
                println!("[Unary {:?}] :: {:?}", op, self.ty);
                expr.print_recursive(indent + 1);
            }
            TypedExprKind::Block(stmts) => {
                println!("[Block] :: {:?}", self.ty);
                for stmt in stmts {
                    stmt.print_recursive(indent + 1);
                }
            }
            TypedExprKind::If(cond, then_branch, else_branch) => {
                println!("[If] :: {:?}", self.ty);
                print!("{}  ├─ Cond: ", pad);
                println!("");
                cond.print_recursive(indent + 2);

                println!("{}  ├─ Then: ", pad);
                then_branch.print_recursive(indent + 2);

                if let Some(else_b) = else_branch {
                    println!("{}  └─ Else: ", pad);
                    else_b.print_recursive(indent + 2);
                }
            }
            TypedExprKind::While(cond, body) => {
                println!("[While] :: {:?}", self.ty);
                cond.print_recursive(indent + 1);
                body.print_recursive(indent + 1);
            }
            TypedExprKind::Call(func, args) => {
                println!("[Call] :: {:?}", self.ty);
                print!("{}  Fn: ", pad);
                println!("");
                func.print_recursive(indent + 2);
                for (i, arg) in args.iter().enumerate() {
                    println!("{}  Arg {}: ", pad, i);
                    arg.print_recursive(indent + 2);
                }
            }
            TypedExprKind::Ret(val) => {
                print!("[Return]");
                match val {
                    Some(v) => {
                        println!(" :: {:?}", self.ty);
                        v.print_recursive(indent + 1);
                    }
                    None => println!(" :: Void"),
                }
            }
            _ => {
                println!("[Unknown/Complex Kind] :: {:?}", self.ty);
            }
        }
    }
}
