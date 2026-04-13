use abyss_diagnostics::{DiagnosticEngine, Span};
use abyss_parser::ast::{Attribute, Expr, ExprKind, Program};
use abyss_types::tast::{TypedExpr, TypedExprKind, TypedProgram};
use abyss_types::type_registry::TypeRegistry;
use abyss_types::types::Type;
use abyss_utils::idgen::IdGenerator;

use crate::comptime::ComptimeEngine;
use crate::side_table::SideTable;
use crate::type_checker::context::{SymbolInfo, TypeContext};
use crate::type_checker::method_registry::MethodRegistry;
use crate::type_checker::resolver::{GlobalMetadata, GlobalResolver};
use crate::type_checker::rules::binary::check_binary;
use crate::type_checker::rules::cast::check_cast;
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
use crate::type_checker::template::registry::TemplateRegistry;

pub enum DefTarget {
    Global(String),
    Method {
        type_name: String,
        method_name: String,
    },
}

pub struct TypeChecker<'a> {
    pub ctx: TypeContext,
    pub type_registry: TypeRegistry,
    pub method_registry: MethodRegistry,
    pub template_registry: TemplateRegistry,
    pub resolver: GlobalResolver<'a>,
    pub diagnostics: &'a mut DiagnosticEngine,
    pub anon_func_counter: usize,
    pub resolve_stack: Vec<String>,
    pub comptime: ComptimeEngine,
    pub side_table: SideTable,
    pub idgen: &'a mut IdGenerator,
    pub active_attributes: Vec<&'a Attribute>,
}

impl<'a> TypeChecker<'a> {
    pub fn new(diagnostics: &'a mut DiagnosticEngine, idgen: &'a mut IdGenerator) -> Self {
        Self {
            ctx: TypeContext::new(),
            type_registry: TypeRegistry::new(),
            method_registry: MethodRegistry::new(),
            template_registry: TemplateRegistry::new(),
            resolver: GlobalResolver::new(),
            diagnostics,
            anon_func_counter: 0,
            resolve_stack: Vec::new(),
            comptime: ComptimeEngine::new(),
            side_table: SideTable::new(),
            idgen,
            active_attributes: Vec::new(),
        }
    }

    pub fn report_error(&mut self, span: Span, message: String) {
        self.diagnostics.report_error(span, message);
    }

    pub fn report_error_with_hint(&mut self, span: Span, message: String, hint: String) {
        self.diagnostics.report_error_with_hint(span, message, hint);
    }

    fn gather_globals_pass(&mut self, expr: &'a Expr) {
        match &expr.kind {
            ExprKind::Block(items) => {
                for item in items {
                    self.gather_globals_pass(item);
                }
            }
            ExprKind::Def(target_expr, _) => {
                if let ExprKind::Ident(name_str) = &target_expr.kind {
                    if !name_str.is_empty() {
                        self.resolver.register(name_str.clone(), expr);
                        self.ctx.define_global(
                            name_str.clone(),
                            SymbolInfo::constant(name_str.clone(), Type::Infer, true),
                        );
                    }
                }
            }
            ExprKind::Attributed(_, inner) => self.gather_globals_pass(inner),
            _ => {}
        }
    }

    fn gather_methods_pass(&mut self, expr: &'a Expr) {
        match &expr.kind {
            ExprKind::Block(items) => {
                for item in items {
                    self.gather_methods_pass(item);
                }
            }
            ExprKind::Def(target_expr, _) => {
                if let ExprKind::Member(base, field_name) = &target_expr.kind {
                    let checked_base = self.check_expr(base);
                    let type_name = self.evaluate_as_type(checked_base).mangled_name();

                    let mangled_name = MethodRegistry::mangle_method_name(&type_name, field_name);

                    self.method_registry.register_method(
                        type_name,
                        field_name.clone(),
                        mangled_name.clone(),
                    );

                    self.resolver.register(mangled_name.clone(), expr);

                    self.ctx.define_global(
                        mangled_name.clone(),
                        SymbolInfo::constant(mangled_name.clone(), Type::Infer, true),
                    );
                }
            }
            ExprKind::Attributed(_, inner) => self.gather_methods_pass(inner),
            _ => {}
        }
    }

    pub fn gather_declarations(&mut self, expr: &'a Expr) {
        self.gather_globals_pass(expr);
        self.gather_methods_pass(expr);
    }

    pub fn complete_and_register_global(
        &mut self,
        name: String,
        ty: Type,
        expr: TypedExpr,
        is_type_def: bool,
        metadata: GlobalMetadata,
    ) {
        self.resolver
            .complete_resolve(name.clone(), ty, expr.clone(), is_type_def, metadata);

        if !is_type_def {
            self.comptime.register_global(name, expr);
        }
    }

    pub fn resolve_global(&mut self, name: &str, span: Span) -> Option<Type> {
        if let Some(ty) = self.resolver.get_resolved_type(name) {
            return Some(ty);
        }

        if self.resolver.is_resolving(name) {
            let mut cycle: Vec<String> = self
                .resolve_stack
                .iter()
                .skip_while(|n| *n != name)
                .cloned()
                .collect();

            cycle.push(name.to_string());

            self.report_error(
                span,
                format!("Circular dependency detected: {}", cycle.join(" -> ")),
            );
            return None;
        }

        let expr = self.resolver.begin_resolve(&name)?;
        self.resolve_stack.push(name.to_string());

        let typed_expr = self.check_expr(expr);
        self.resolve_stack.pop();

        Some(typed_expr.ty)
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
                let old_attrs =
                    std::mem::replace(&mut self.active_attributes, attributes.iter().collect());

                let typed_inner_expr = self.check_expr(inner_expr);

                self.active_attributes = old_attrs;
                return typed_inner_expr;
            }
            ExprKind::Lit(lit) => check_literal(self, lit, expr.span_expr(), expr.id),

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

            ExprKind::Cast(l, r) => check_cast(self, l, r, expr.span_expr(), expr.id),

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
        let mut globals: Vec<(String, TypedExpr)> = resolved
            .into_iter()
            .map(|(name, (_, typed_expr))| (name, typed_expr))
            .collect();

        for instance in self.template_registry.drain_instances() {
            globals.push((instance.ir_name.clone(), instance.typed_def));
        }

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

    pub fn next_id(&mut self) -> u32 {
        self.idgen.next()
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.active_attributes.iter().any(|&attr| attr.name == name)
    }

    pub fn get_attribute(&self, name: &str) -> Option<&'a Attribute> {
        self.active_attributes
            .iter()
            .find(|&&attr| attr.name == name)
            .copied()
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
