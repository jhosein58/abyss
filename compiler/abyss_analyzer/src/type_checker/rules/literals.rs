use abyss_diagnostics::Span;
use abyss_parser::ast::Lit;

use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

use crate::type_checker::engine::TypeChecker;

pub fn check_literal(tc: &mut TypeChecker, lit: &Lit, span: Span, id: u32) -> TypedExpr {
    tc.side_table.mark_const(id, true);

    let ty = match lit {
        Lit::Int(_) => Type::I32,
        Lit::Float(_) => Type::F32,
        Lit::Bool(_) => Type::Bool,
        Lit::Str(_) => Type::Str,
        Lit::Cstr(_) => Type::Cstr,
        Lit::Char(_) => Type::Char,
    };

    TypedExpr {
        kind: TypedExprKind::Lit(lit.clone()),
        ty,
        span,
        id,
    }
}
