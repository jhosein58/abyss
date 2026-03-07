use abyss_diagnostics::{DiagnosticEngine, Span};
use abyss_parser::ast::{Expr, ExprKind, Program};

use crate::type_checker::rules::binary::check_binary;
use crate::type_checker::rules::block::check_block;
use crate::type_checker::rules::call::check_call;
use crate::type_checker::rules::control_flow::check_if;
use crate::type_checker::rules::ident::check_ident;
use crate::type_checker::rules::literals::check_literal;
use crate::type_checker::rules::prefix::check_ret;
use crate::type_checker::rules::sequence::check_sequence;
use crate::type_checker::rules::signature::check_signature;
use crate::type_checker::rules::unary::check_unary;
use crate::type_checker::tast::TypedProgram;

use super::context::TypeContext;
use super::tast::{TypedExpr, TypedExprKind};
use super::types::Type;

pub struct TypeChecker<'a> {
    pub ctx: TypeContext,
    pub diagnostics: &'a mut DiagnosticEngine,
    pub anon_func_counter: usize,
    pub hoisted_functions: Vec<TypedExpr>,
}

impl<'a> TypeChecker<'a> {
    pub fn new(diagnostics: &'a mut DiagnosticEngine) -> Self {
        Self {
            ctx: TypeContext::new(),
            diagnostics,
            anon_func_counter: 0,
            hoisted_functions: Vec::new(),
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

            ExprKind::Unary(op, inner_expr) => {
                check_unary(self, *op, inner_expr, expr.span_expr(), expr.id)
            }

            ExprKind::Ident(name) => check_ident(self, name.clone(), expr.span_expr(), expr.id),
            ExprKind::Call(calle, args) => check_call(self, calle, args, expr.span_expr(), expr.id),

            ExprKind::Signature(args, ret_ty, body) => {
                check_signature(self, args, ret_ty, body, None, expr.span_expr(), expr.id)
            }

            ExprKind::Sequence(items, count) => {
                check_sequence(self, items, count, expr.span_expr(), expr.id)
            }

            ExprKind::Ret(val) => check_ret(self, val, expr.span_expr(), expr.id),

            ExprKind::If(cond, then_b, else_b) => {
                check_if(self, cond, then_b, else_b, expr.span_expr(), expr.id)
            }

            _ => error_expr(expr.span.clone(), expr.id),
        }
    }

    pub fn check_program(&mut self, prog: Program) -> TypedProgram {
        let body = self.check_expr(&prog.body);
        TypedProgram {
            body,
            hoisted_functions: self.hoisted_functions.clone(),
        }
    }
}

pub fn error_expr(span: Span, id: u32) -> TypedExpr {
    TypedExpr {
        kind: TypedExprKind::ErrorPlaceholder,
        ty: Type::Error,
        span,
        id,
    }
}
