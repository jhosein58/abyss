use abyss_parser::ast::{Expr, ExprKind};

use crate::type_checker::rules::binary::check_binary;
use crate::type_checker::rules::block::check_block;
use crate::type_checker::rules::literals::check_literal;

use super::context::TypeContext;

use super::tast::{TypedExpr, TypedExprKind};
use super::types::Type;

pub struct TypeChecker {
    pub ctx: TypeContext,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            ctx: TypeContext::new(),
        }
    }

    pub fn check_expr(&mut self, expr: &Expr) -> TypedExpr {
        match &expr.kind {
            ExprKind::Lit(lit) => check_literal(lit, expr.span.clone(), expr.id),
            ExprKind::Block(stmts) => check_block(self, stmts, expr.span.clone(), expr.id),
            ExprKind::Binary(l, op, r) => check_binary(self, l, *op, r, expr.span.clone(), expr.id),

            _ => TypedExpr {
                kind: TypedExprKind::ErrorPlaceholder,
                ty: Type::Error,
                span: expr.span.clone(),
                id: 0,
            },
        }
    }
}
