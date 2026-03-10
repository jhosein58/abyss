use abyss_diagnostics::Span;
use abyss_parser::ast::Lit;

use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

pub fn check_literal(lit: &Lit, span: Span, id: u32) -> TypedExpr {
    let l = match lit {
        Lit::Int(v) => (Lit::Int(*v), Type::I32),
        Lit::Float(v) => (Lit::Float(*v), Type::F32),
        Lit::Bool(v) => (Lit::Bool(*v), Type::Bool),
        Lit::Str(v) => (Lit::Str(v.clone()), Type::Str),
        Lit::Cstr(v) => (Lit::Cstr(v.clone()), Type::Cstr),
        Lit::Char(v) => (Lit::Char(*v), Type::Char),
    };

    TypedExpr {
        kind: TypedExprKind::Lit(l.0),
        ty: l.1,
        span,
        id,
    }
}
