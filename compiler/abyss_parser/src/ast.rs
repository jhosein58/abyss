use std::hash::{Hash, Hasher};

use abyss_diagnostics::Span;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
    pub id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprKind {
    Mod(Box<Expr>, Box<Expr>),                          // Mod(Name, Body?)
    Use(Box<Expr>),                                     // Use(Module)
    Sequence(Vec<Expr>, Option<Box<Expr>>), // [expr: expr, expr: expr] or [expr, epxr] or [expr; len]
    Signature(Vec<Expr>, Option<Box<Expr>>, Box<Expr>), // Signature(args, return, body)

    Ret(Option<Box<Expr>>),
    Out(Option<Box<Expr>>),                      // out (break loop)
    Continue,                                    // next
    Block(Vec<Expr>),                            // Block(statements)
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>), // If(condition, then, else)
    For(Box<Expr>, Box<Expr>, Box<Expr>),        // For(pattern, range, body)
    Range {
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
        step: Option<Box<Expr>>,
        inclusive: bool,
    },
    While(Box<Expr>, Box<Expr>), // While(condition, body)
    Forever(Box<Expr>),

    Defer(Box<Expr>), // Defer(expression)

    // ---------------------
    Lit(Lit),
    Ident(String),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>), // call(callee, args)
    Index(Box<Expr>, Box<Expr>),
    Cast(Box<Expr>, Option<Box<Expr>>),
    Is(Box<Expr>, Option<Box<Expr>>),
    Member(Box<Expr>, String),
    SizeOf(Option<Box<Expr>>),
    Match {
        subject: Box<Expr>,
        arms: Vec<MatchArm>,
    },
    Then(Box<Expr>, Box<Expr>), // Then(first, second)
    TypeOf(Box<Expr>),
    Refinement(Option<Box<Expr>>, Box<Expr>),
    Attributed(Vec<Attribute>, Box<Expr>),
    Wildcard, // _
}

#[derive(Debug, Clone, Copy)]
pub struct OrderedFloat(pub f64);

impl PartialEq for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for OrderedFloat {}

impl Hash for OrderedFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lit {
    Int(i64),
    Float(OrderedFloat),
    Bool(bool),
    Str(String),
    Cstr(String),
    Char(char),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub enum BinaryOp {
    Assign,       // =
    AssignAdd,    // +=
    AssignSub,    // -=
    AssignMul,    // *=
    AssignDiv,    // /=
    AssignMod,    // %=
    AssignBitAnd, // &=
    AssignBitOr,  // |=
    AssignBitXor, // ^=
    AssignShl,    // <<=
    AssignShr,    // >>=
    Add,          // +
    Sub,          // -
    Mul,          // *
    Div,          // /
    Mod,          // %
    Eq,           // ==
    Neq,          // !=
    Lt,           // <
    Gt,           // >
    Lte,          // <=
    Gte,          // >=
    And,          // and
    Or,           // or
    BitAnd,       // &
    Pipe,         // |    Union & BitOr
    BitXor,       // ^
    Shl,          // <<
    Shr,          // >>
    KeyValue,     // :    TypeAnnot
    ConstDef,     // ::
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Neg,    // -x
    Not,    // not x
    BitNot, // ~x
    Deref,  // *x
    AddrOf, // &x
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchArm {
    pattern: Box<Expr>,
    body: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub name: String,
    pub args: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub body: Expr,
}

impl Program {
    pub fn print_indented(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
    ) -> std::fmt::Result {
        self.body.print_indented(f, indent)
    }
}

impl Expr {
    pub fn print_indented(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
    ) -> std::fmt::Result {
        self.kind.print_indented(f, indent)
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_indented(f, 0)
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_indented(f, 0)
    }
}

impl std::fmt::Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lit::Int(v) => write!(f, "{}", v),
            Lit::Float(v) => write!(f, "{}", v.0),
            Lit::Bool(v) => write!(f, "{}", v),
            Lit::Str(v) => write!(f, "\"{}\"", v),
            Lit::Cstr(v) => write!(f, "c\"{}\"", v),
            Lit::Char(v) => write!(f, "'{}'", v),
        }
    }
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BinaryOp::Assign => "=",
            BinaryOp::AssignAdd => "+=",
            BinaryOp::AssignSub => "-=",
            BinaryOp::AssignMul => "*=",
            BinaryOp::AssignDiv => "/=",
            BinaryOp::AssignMod => "%=",
            BinaryOp::AssignBitAnd => "&=",
            BinaryOp::AssignBitOr => "|=",
            BinaryOp::AssignBitXor => "^=",
            BinaryOp::AssignShl => "<<=",
            BinaryOp::AssignShr => ">>=",
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::Eq => "==",
            BinaryOp::Neq => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Gt => ">",
            BinaryOp::Lte => "<=",
            BinaryOp::Gte => ">=",
            BinaryOp::And => "and",
            BinaryOp::Or => "or",
            BinaryOp::BitAnd => "&",
            BinaryOp::Pipe => "|",
            BinaryOp::BitXor => "^",
            BinaryOp::Shl => "<<",
            BinaryOp::Shr => ">>",
            BinaryOp::KeyValue => ":",
            BinaryOp::ConstDef => "::",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "not",
            UnaryOp::BitNot => "~",
            UnaryOp::Deref => "*",
            UnaryOp::AddrOf => "&",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.name)?;
        if !self.args.is_empty() {
            write!(f, "(")?;
            for (i, arg) in self.args.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", arg)?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl ExprKind {
    pub fn print_indented(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
    ) -> std::fmt::Result {
        let pad = "  ".repeat(indent);
        let inner_pad = "  ".repeat(indent + 1);

        match self {
            ExprKind::Wildcard => write!(f, "_"),
            ExprKind::Lit(lit) => write!(f, "{}", lit),
            ExprKind::Ident(path) => write!(f, "{}", path),
            ExprKind::Out(val) => {
                if let Some(v) = val {
                    writeln!(f, "Out {{")?;
                    write!(f, "{}", inner_pad)?;
                    v.print_indented(f, indent + 1)?;
                    writeln!(f)?;
                    write!(f, "{}}}", pad)
                } else {
                    write!(f, "Out")
                }
            }

            ExprKind::Continue => write!(f, "Continue"),
            ExprKind::Use(path) => write!(f, "Use({})", path),
            ExprKind::SizeOf(ty) => {
                if let Some(t) = ty {
                    writeln!(f, "SizeOf {{")?;
                    write!(f, "{}", inner_pad)?;
                    t.print_indented(f, indent + 1)?;
                    writeln!(f)?;
                    write!(f, "{}}}", pad)
                } else {
                    write!(f, "SizeOf {{ _ }}")
                }
            }

            ExprKind::Block(stmts) => {
                if stmts.is_empty() {
                    return write!(f, "Block {{}}");
                }
                writeln!(f, "Block {{")?;
                for s in stmts {
                    write!(f, "{}", inner_pad)?;
                    s.print_indented(f, indent + 1)?;
                    writeln!(f)?;
                }
                write!(f, "{}}}", pad)
            }

            ExprKind::Binary(left, op, right) => {
                writeln!(f, "Binary({}) {{", op)?;
                write!(f, "{}left: ", inner_pad)?;
                left.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}right: ", inner_pad)?;
                right.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::Unary(op, right) => {
                writeln!(f, "Unary({}) {{", op)?;
                write!(f, "{}", inner_pad)?;
                right.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::Signature(args, ret, body) => {
                writeln!(f, "Signature {{")?;
                if args.is_empty() {
                    writeln!(f, "{}args: []", inner_pad)?;
                } else {
                    writeln!(f, "{}args: [", inner_pad)?;
                    for a in args {
                        write!(f, "{}  ", inner_pad)?;
                        a.print_indented(f, indent + 2)?;
                        writeln!(f)?;
                    }
                    writeln!(f, "{}]", inner_pad)?;
                }

                write!(f, "{}ret: ", inner_pad)?;
                if let Some(r) = ret {
                    r.print_indented(f, indent + 1)?;
                } else {
                    write!(f, "()")?;
                }
                writeln!(f)?;

                write!(f, "{}body: ", inner_pad)?;
                body.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::Call(callee, args) => {
                writeln!(f, "Call {{")?;
                write!(f, "{}callee: ", inner_pad)?;
                callee.print_indented(f, indent + 1)?;
                writeln!(f)?;

                if args.is_empty() {
                    writeln!(f, "{}args: []", inner_pad)?;
                } else {
                    writeln!(f, "{}args: [", inner_pad)?;
                    for a in args {
                        write!(f, "{}  ", inner_pad)?;
                        a.print_indented(f, indent + 2)?;
                        writeln!(f)?;
                    }
                    writeln!(f, "{}]", inner_pad)?;
                }
                write!(f, "{}}}", pad)
            }

            ExprKind::Member(sub, name) => {
                writeln!(f, "Member(.{}) {{", name)?;
                write!(f, "{}", inner_pad)?;
                sub.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::Ret(val) => {
                if let Some(v) = val {
                    writeln!(f, "Ret {{")?;
                    write!(f, "{}", inner_pad)?;
                    v.print_indented(f, indent + 1)?;
                    writeln!(f)?;
                    write!(f, "{}}}", pad)
                } else {
                    write!(f, "Ret")
                }
            }

            ExprKind::Mod(name, body) => {
                writeln!(f, "Mod({}) {{", name)?;
                write!(f, "{}", inner_pad)?;
                body.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::Sequence(exprs, opt) => {
                writeln!(f, "Sequence {{")?;
                if exprs.is_empty() {
                    writeln!(f, "{}items: []", inner_pad)?;
                } else {
                    writeln!(f, "{}items: [", inner_pad)?;
                    for e in exprs {
                        write!(f, "{}  ", inner_pad)?;
                        e.print_indented(f, indent + 2)?;
                        writeln!(f)?;
                    }
                    writeln!(f, "{}]", inner_pad)?;
                }
                if let Some(o) = opt {
                    write!(f, "{}len: ", inner_pad)?;
                    o.print_indented(f, indent + 1)?;
                    writeln!(f)?;
                }
                write!(f, "{}}}", pad)
            }

            ExprKind::If(cond, then, els) => {
                writeln!(f, "If {{")?;
                write!(f, "{}cond: ", inner_pad)?;
                cond.print_indented(f, indent + 1)?;
                writeln!(f)?;

                write!(f, "{}then: ", inner_pad)?;
                then.print_indented(f, indent + 1)?;
                writeln!(f)?;

                if let Some(e) = els {
                    write!(f, "{}else: ", inner_pad)?;
                    e.print_indented(f, indent + 1)?;
                    writeln!(f)?;
                }
                write!(f, "{}}}", pad)
            }

            ExprKind::While(cond, body) => {
                writeln!(f, "While {{")?;
                write!(f, "{}cond: ", inner_pad)?;
                cond.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}body: ", inner_pad)?;
                body.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::Index(sub, idx) => {
                writeln!(f, "Index {{")?;
                write!(f, "{}subject: ", inner_pad)?;
                sub.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}index: ", inner_pad)?;
                idx.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::Cast(expr, ty) => {
                writeln!(f, "Cast {{")?;
                write!(f, "{}expr: ", inner_pad)?;
                expr.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}type: ", inner_pad)?;
                if let Some(t) = ty {
                    t.print_indented(f, indent + 1)?;
                } else {
                    write!(f, "_")?;
                }
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::Is(expr, ty) => {
                writeln!(f, "Is {{")?;
                write!(f, "{}expr: ", inner_pad)?;
                expr.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}type: ", inner_pad)?;
                if let Some(t) = ty {
                    t.print_indented(f, indent + 1)?;
                } else {
                    write!(f, "_")?;
                }
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::Match { subject, arms } => {
                writeln!(f, "Match {{")?;
                write!(f, "{}subject: ", inner_pad)?;
                subject.print_indented(f, indent + 1)?;
                writeln!(f)?;
                writeln!(f, "{}arms: [", inner_pad)?;
                for arm in arms {
                    writeln!(f, "{}  Arm {{", inner_pad)?;
                    write!(f, "{}    pattern: ", inner_pad)?;
                    arm.pattern.print_indented(f, indent + 3)?;
                    writeln!(f)?;
                    write!(f, "{}    body: ", inner_pad)?;
                    arm.body.print_indented(f, indent + 3)?;
                    writeln!(f)?;
                    writeln!(f, "{}  }}", inner_pad)?;
                }
                writeln!(f, "{}]", inner_pad)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::TypeOf(expr) => {
                writeln!(f, "TypeOf {{")?;
                write!(f, "{}", inner_pad)?;
                expr.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::Defer(expr) => {
                writeln!(f, "Defer {{")?;
                write!(f, "{}", inner_pad)?;
                expr.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::Then(first, second) => {
                writeln!(f, "Then {{")?;
                write!(f, "{}first: ", inner_pad)?;
                first.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}second: ", inner_pad)?;
                second.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::Refinement(cond, expr) => {
                writeln!(f, "Refinement {{")?;
                write!(f, "{}cond: ", inner_pad)?;
                if let Some(c) = cond {
                    c.print_indented(f, indent + 1)?;
                } else {
                    write!(f, "_")?;
                }
                writeln!(f)?;
                write!(f, "{}expr: ", inner_pad)?;
                expr.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::Attributed(attrs, expr) => {
                writeln!(f, "Attributed {{")?;
                writeln!(f, "{}attributes: [", inner_pad)?;
                for a in attrs {
                    writeln!(f, "{}  {}", inner_pad, a)?;
                }
                writeln!(f, "{}]", inner_pad)?;
                write!(f, "{}expr: ", inner_pad)?;
                expr.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::For(pat, range, body) => {
                writeln!(f, "For {{")?;
                write!(f, "{}pattern: ", inner_pad)?;
                pat.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}range: ", inner_pad)?;
                range.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}body: ", inner_pad)?;
                body.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::Range {
                start,
                end,
                step,
                inclusive,
            } => {
                writeln!(f, "Range {{")?;
                write!(f, "{}start: ", inner_pad)?;
                if let Some(s) = start {
                    s.print_indented(f, indent + 1)?;
                } else {
                    write!(f, "_")?;
                }
                writeln!(f)?;
                write!(f, "{}end: ", inner_pad)?;
                if let Some(e) = end {
                    e.print_indented(f, indent + 1)?;
                } else {
                    write!(f, "_")?;
                }
                writeln!(f)?;
                write!(f, "{}step: ", inner_pad)?;
                if let Some(st) = step {
                    st.print_indented(f, indent + 1)?;
                } else {
                    write!(f, "_")?;
                }
                writeln!(f)?;
                writeln!(f, "{}inclusive: {}", inner_pad, inclusive)?;
                write!(f, "{}}}", pad)
            }

            ExprKind::Forever(body) => {
                writeln!(f, "Forever {{")?;
                write!(f, "{}body: ", inner_pad)?;
                body.print_indented(f, indent + 1)?;
                writeln!(f)?;
                write!(f, "{}}}", pad)
            }
        }
    }
}
