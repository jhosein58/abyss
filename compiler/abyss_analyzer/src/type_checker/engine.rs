use abyss_diagnostics::{DiagnosticEngine, Span};
use abyss_parser::ast::{Attribute, Expr, ExprKind, Program};
use abyss_types::tast::{TypedExpr, TypedExprKind, TypedProgram};
use abyss_types::type_registry::TypeRegistry;
use abyss_types::types::Type;
use std::collections::HashMap;

use crate::comptime::ComptimeEngine;
use crate::type_checker::context::{SymbolInfo, TypeContext};
use crate::type_checker::resolver::{GlobalResolver, InlinePolicy};
use crate::type_checker::rules::binary::check_binary;
use crate::type_checker::rules::ident::check_ident;
use crate::type_checker::rules::index::check_index;

use crate::type_checker::rules::block::check_block;
use crate::type_checker::rules::call::check_call;
use crate::type_checker::rules::control_flow::{check_if, check_out, check_while};
use crate::type_checker::rules::literals::check_literal;
use crate::type_checker::rules::member::check_member;
use crate::type_checker::rules::prefix::{check_cmpt, check_def, check_ret};
use crate::type_checker::rules::sequence::check_sequence;
use crate::type_checker::rules::signature::check_signature;
use crate::type_checker::rules::unary::check_unary;

pub struct TypeChecker<'a> {
    pub ctx: TypeContext,
    pub type_registry: TypeRegistry,
    pub resolver: GlobalResolver<'a>,
    pub diagnostics: &'a mut DiagnosticEngine,
    pub anon_func_counter: usize,
    resolve_stack: Vec<String>,
    active_attributes: Vec<&'a Attribute>,
    pub comptime: ComptimeEngine,
}

impl<'a> TypeChecker<'a> {
    pub fn new(diagnostics: &'a mut DiagnosticEngine) -> Self {
        Self {
            ctx: TypeContext::new(),
            type_registry: TypeRegistry::new(),
            resolver: GlobalResolver::new(),
            diagnostics,
            anon_func_counter: 0,
            resolve_stack: Vec::new(),
            active_attributes: Vec::new(),
            comptime: ComptimeEngine::new(),
        }
    }

    pub fn report_error(&mut self, span: Span, message: String) {
        self.diagnostics.report_error(span, message);
    }

    pub fn report_error_with_hint(&mut self, span: Span, message: String, hint: String) {
        self.diagnostics.report_error_with_hint(span, message, hint);
    }

    fn gather_declarations(&mut self, expr: &'a Expr) {
        match &expr.kind {
            ExprKind::Block(items) => {
                for item in items {
                    self.gather_declarations(item);
                }
            }
            ExprKind::Def(name, _value) => {
                let name_str = if let ExprKind::Ident(n) = &name.kind {
                    n.clone()
                } else {
                    self.report_error(name.span_expr(), "Only ident can be used.".to_string());
                    return;
                };

                if name_str.is_empty() {
                    return;
                }

                self.resolver.register(name_str.clone(), expr);

                self.ctx
                    .define_global(name_str, SymbolInfo::constant(Type::Infer));
            }
            _ => {}
        }
    }

    pub fn complete_and_register_global(
        &mut self,
        name: String,
        ty: Type,
        expr: TypedExpr,
        is_type_def: bool,
        inline_policy: InlinePolicy,
    ) {
        self.resolver
            .complete_resolve(name.clone(), ty, expr.clone(), is_type_def, inline_policy);

        if !is_type_def {
            self.comptime.register_global(name, expr);
        }
    }

    pub fn resolve_global(&mut self, name: &str, span: Span) -> Option<Type> {
        if let Some(ty) = self.resolver.get_resolved_type(name) {
            return Some(ty);
        }

        if self.resolver.is_resolving(name) {
            let cycle: Vec<String> = self
                .resolve_stack
                .iter()
                .skip_while(|n| *n != name)
                .cloned()
                .collect();
            self.report_error(
                span,
                format!(
                    "Circular dependency detected: {} -> {}",
                    cycle.join(" -> "),
                    name
                ),
            );
            return None;
        }

        let expr = self.resolver.begin_resolve(name)?;
        self.resolve_stack.push(name.to_string());

        let (attributes, inner_expr) = if let ExprKind::Attributed(attrs, inner) = &expr.kind {
            (attrs.as_slice(), &**inner)
        } else {
            (&[][..], expr)
        };

        let mut inline_policy = InlinePolicy::Never;
        if attributes.iter().any(|a| a.name == "inline") {
            inline_policy = InlinePolicy::Always;
        }

        let typed_expr = self.check_expr(inner_expr);
        let ty = typed_expr.ty.clone();

        let is_type_def =
            matches!(ty, Type::Metatype) || matches!(&typed_expr.kind, TypedExprKind::Type(_));

        if is_type_def {
            let actual_type = self.extract_actual_type(&typed_expr);
            self.type_registry
                .register(name.to_string(), actual_type.clone());

            self.ctx.update_type(name, Type::Metatype);
        } else {
            self.ctx.update_type(name, ty.clone());
        }

        self.complete_and_register_global(
            name.to_string(),
            ty.clone(),
            typed_expr.clone(),
            is_type_def,
            inline_policy,
        );

        self.resolve_stack.pop();
        Some(ty)
    }

    fn extract_actual_type(&mut self, expr: &TypedExpr) -> Type {
        match &expr.kind {
            TypedExprKind::Type(ty) => ty.clone(),
            TypedExprKind::Ident(name) => {
                if let Some(ty) = self.type_registry.get(name) {
                    ty.clone()
                } else {
                    Type::Error
                }
            }
            _ => self.evaluate_as_type(expr.clone()),
        }
    }

    pub fn resolve_type_by_name(&mut self, name: &str, span: Span) -> Type {
        if let Some(ty) = self.type_registry.get(name) {
            return ty.clone();
        }

        match name {
            "i32" => return Type::I32,
            "f32" => return Type::F32,
            "bool" => return Type::Bool,
            "str" => return Type::Str,
            "char" => return Type::Char,
            "unit" | "()" => return Type::Unit,
            _ => {}
        }

        if self.resolver.contains(name) {
            if let Some(_) = self.resolve_global(name, span.clone()) {
                if let Some(ty) = self.type_registry.get(name) {
                    return ty.clone();
                }
            }
        }

        self.report_error(span, format!("Undefined type: '{}'", name));
        Type::Error
    }

    pub fn check_expr(&mut self, expr: &'a Expr) -> TypedExpr {
        match &expr.kind {
            ExprKind::Attributed(attributes, inner_expr) => {
                let original_len = self.active_attributes.len();
                self.active_attributes.extend(attributes.iter());

                let typed_inner_expr = self.check_expr(inner_expr);

                self.active_attributes.truncate(original_len);

                return typed_inner_expr;
            }

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

            ExprKind::Index(arr, idx) => check_index(self, arr, idx, expr.span_expr(), expr.id),

            ExprKind::Member(base, field_name) => {
                check_member(self, base, field_name, expr.span_expr(), expr.id)
            }

            _ => error_expr(expr.span.clone(), expr.id),
        }
    }

    pub fn create_ident_expr(&self, name: String, ty: Type, span: Span, id: u32) -> TypedExpr {
        TypedExpr {
            kind: TypedExprKind::Ident(name),
            ty,
            span,
            id,
        }
    }

    pub fn primitive_type_from_name(&self, name: &str) -> Option<Type> {
        match name {
            "i32" => Some(Type::I32),
            "f32" => Some(Type::F32),
            "bool" => Some(Type::Bool),
            "type" => Some(Type::Metatype),
            "unit" => Some(Type::Unit),
            _ => None,
        }
    }

    pub fn check_program(&mut self, prog: &'a Program) -> TypedProgram {
        self.gather_declarations(&prog.body);

        let body = self.check_expr(&prog.body);

        let resolved = self.resolver.drain_resolved();
        let globals: HashMap<String, TypedExpr> = resolved
            .into_iter()
            .map(|(name, (_, typed_expr))| (name, typed_expr))
            .collect();

        TypedProgram { body, globals }
    }

    pub fn evaluate_as_type(&mut self, expr: TypedExpr) -> Type {
        if let TypedExprKind::Type(ty) = &expr.kind {
            return ty.clone();
        }

        if expr.ty == Type::Metatype {
            if let TypedExprKind::Ident(name) = &expr.kind {
                if let Some(ty) = self.type_registry.get(name) {
                    return ty.clone();
                }
            }

            let evaluated_expr = self.comptime.evaluate_expr(expr);

            if let TypedExprKind::Lit(abyss_parser::ast::Lit::Int(result_value)) =
                evaluated_expr.kind
            {
                return self.comptime.builder.encoder.from_id(result_value);
            } else {
                self.report_error(
                    Span::default(),
                    "Failed to evaluate type expression at compile time".to_string(),
                );
            }
        }

        Type::Unit
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.active_attributes.iter().any(|attr| attr.name == name)
    }

    pub fn find_attribute(&self, name: &str) -> Option<&&'a Attribute> {
        self.active_attributes.iter().find(|attr| attr.name == name)
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
