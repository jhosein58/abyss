use abyss_diagnostics::{DiagnosticEngine, Span};
use abyss_ir::builder::IrBuilder;
use abyss_ir::ir::IrLit;
use abyss_parser::ast::{BinaryOp, Expr, ExprKind, Program};
use abyss_types::tast::{TypedExpr, TypedExprKind, TypedProgram};
use abyss_types::types::Type;
use abyss_vm::execute_comptime;

use crate::type_checker::rules::binary::check_binary;
use crate::type_checker::rules::block::check_block;
use crate::type_checker::rules::call::check_call;
use crate::type_checker::rules::control_flow::{check_if, check_out, check_while};
use crate::type_checker::rules::ident::check_ident;
use crate::type_checker::rules::literals::check_literal;
use crate::type_checker::rules::prefix::{check_cmpt, check_def, check_ret};
use crate::type_checker::rules::sequence::check_sequence;
use crate::type_checker::rules::signature::check_signature;
use crate::type_checker::rules::unary::check_unary;

use super::context::TypeContext;

pub struct TypeChecker<'a> {
    pub ctx: TypeContext,
    pub diagnostics: &'a mut DiagnosticEngine,
    pub anon_func_counter: usize,
}
impl<'a> TypeChecker<'a> {
    pub fn new(diagnostics: &'a mut DiagnosticEngine) -> Self {
        Self {
            ctx: TypeContext::new(),
            diagnostics,
            anon_func_counter: 0,
        }
    }

    pub fn report_error(&mut self, span: Span, message: String) {
        self.diagnostics.report_error(span, message);
    }

    pub fn report_error_with_hint(&mut self, span: Span, message: String, hint: String) {
        self.diagnostics.report_error_with_hint(span, message, hint);
    }

    fn gather_declarations(&mut self, expr: &Expr) {
        match &expr.kind {
            ExprKind::Block(items) => {
                for item in items {
                    self.gather_declarations(item);
                }
            }
            ExprKind::Def(name, value) => {
                let name_str = if let ExprKind::Ident(n) = &name.kind {
                    n.clone()
                } else {
                    self.report_error(name.span_expr(), "Only ident can be used.".to_string());
                    "".to_string()
                };

                if name_str.is_empty() {
                    return;
                }

                let ty = self.extract_type_for_pass1(value);
                self.ctx.define_global(name_str, ty);
            }
            _ => {}
        }
    }

    fn extract_type_for_pass1(&mut self, value: &Expr) -> Type {
        match &value.kind {
            ExprKind::Signature(args, ret_ty, body) => {
                let mut arg_types = Vec::new();
                for arg in args {
                    if let ExprKind::Binary(_, BinaryOp::KeyValue, right) = &arg.kind {
                        let typed_ty = self.check_expr(right);
                        arg_types.push(self.evaluate_as_type(typed_ty));
                    }
                }

                let return_type = if let Some(rt) = ret_ty {
                    let typed_ret = self.check_expr(rt);
                    self.evaluate_as_type(typed_ret)
                } else {
                    Type::Unit
                };

                let is_native = if ExprKind::Wildcard == body.kind {
                    true
                } else {
                    false
                };

                Type::Signature(arg_types, Box::new(return_type), is_native)
            }
            ExprKind::Lit(lit) => match lit {
                abyss_parser::ast::Lit::Int(_) => Type::I32,
                abyss_parser::ast::Lit::Float(_) => Type::F32,
                abyss_parser::ast::Lit::Bool(_) => Type::Bool,
                _ => Type::Infer,
            },

            _ => Type::Infer,
        }
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

            ExprKind::Out(val) => check_out(self, val, expr.span_expr(), expr.id),

            ExprKind::If(cond, then_b, else_b) => {
                check_if(self, cond, then_b, else_b, expr.span_expr(), expr.id)
            }

            ExprKind::While(cond, body, else_branch) => {
                check_while(self, cond, body, else_branch, expr.span_expr(), expr.id)
            }

            ExprKind::Def(name, value) => check_def(self, name, value, expr.span_expr(), expr.id),

            ExprKind::Comptime(inner_expr) => {
                check_cmpt(self, inner_expr, expr.span_expr(), expr.id)
            }

            _ => error_expr(expr.span.clone(), expr.id),
        }
    }

    pub fn check_program(&mut self, prog: Program) -> TypedProgram {
        self.gather_declarations(&prog.body);
        let body = self.check_expr(&prog.body);

        TypedProgram {
            body,
            globals: self.ctx.resolved_globals.clone(),
        }
    }

    pub fn evaluate_as_type(&mut self, expr: TypedExpr) -> Type {
        if let TypedExprKind::Type(ty) = &expr.kind {
            return ty.clone();
        }

        if expr.ty == Type::Metatype {
            let mut ir_builder = IrBuilder::new();

            let ir_prog = ir_builder.build_comptime_program(expr, &self.ctx.resolved_globals);

            if let IrLit::Int(result_value) = execute_comptime(ir_prog) {
                return Type::from_id(result_value as i64);
            } else {
                self.report_error(
                    Span::default(),
                    "Failed to evaluate type expression at compile time".to_string(),
                );
            }
        }

        Type::Unit
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
