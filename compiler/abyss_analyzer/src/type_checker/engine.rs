use abyss_diagnostics::{DiagnosticEngine, Span};
use abyss_parser::ast::{Expr, ExprKind};

use crate::type_checker::rules::binary::check_binary;
use crate::type_checker::rules::block::check_block;
use crate::type_checker::rules::literals::check_literal;

use super::context::TypeContext;

use super::tast::{TypedExpr, TypedExprKind};
use super::types::Type;

pub struct TypeChecker<'a> {
    pub ctx: TypeContext,
    pub diagnostics: &'a mut DiagnosticEngine,
}

impl<'a> TypeChecker<'a> {
    pub fn new(diagnostics: &'a mut DiagnosticEngine) -> Self {
        Self {
            ctx: TypeContext::new(),
            diagnostics,
        }
    }

    pub fn report_error(&mut self, span: Span, message: String) {
        self.diagnostics.report_error(span, message);
    }

    pub fn report_error_with_hint(&mut self, span: Span, message: String, hint: String) {
        self.diagnostics.report_error_with_hint(span, message, hint);
    }

    pub fn check_expr(&mut self, expr: &Expr) -> TypedExpr {
        match &expr.kind {
            ExprKind::Lit(lit) => check_literal(lit, expr.span_expr(), expr.id),
            ExprKind::Block(stmts) => check_block(self, stmts, expr.span_expr(), expr.id),
            ExprKind::Binary(l, op, r) => check_binary(self, l, *op, r, expr.span_expr(), expr.id),
            ExprKind::Ident() => {}

            _ => TypedExpr {
                kind: TypedExprKind::ErrorPlaceholder,
                ty: Type::Error,
                span: expr.span.clone(),
                id: 0,
            },
        }
    }
}
