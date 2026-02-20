use super::utils::GenericsEngine;
use crate::new_type_checker::{Pass, context::TypeContext, visitor::AstVisitor};
use abyss_parser::ast::{
    BinaryOp as BinOp, Expr, ExprKind, FunctionBody, FunctionDef, Stmt, StmtKind, Type, UnaryOp,
};
use std::collections::HashMap;

pub struct SafetyPass;

impl SafetyPass {
    pub fn new() -> Self {
        SafetyPass
    }

    fn check_type_compatibility(&self, expected: &Type, actual: &Type) -> bool {
        if expected == actual {
            return true;
        }
        match (expected, actual) {
            (Type::Const(inner_expected), _) => {
                self.check_type_compatibility(inner_expected, actual)
            }
            (Type::Pointer(inner_expected), Type::Array(inner_actual, _)) => {
                self.check_type_compatibility(inner_expected, inner_actual)
            }
            (Type::Char, Type::U8) | (Type::U8, Type::Char) => true,
            (Type::Pointer(inner), Type::Pointer(_)) if matches!(**inner, Type::Void) => true,
            _ => false,
        }
    }
}

impl AstVisitor for SafetyPass {
    fn visit_function_def(&mut self, func: &mut FunctionDef, ctx: &mut TypeContext) {
        if let FunctionBody::UserDefined(body) = &mut func.body {
            ctx.set_current_function(func.name.clone());
            for stmt in body {
                self.visit_stmt(stmt, ctx);
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt, ctx: &mut TypeContext) {
        match stmt.kind {
            StmtKind::Assign(ref mut l, ref mut r) => {
                self.visit_expr(l, ctx);
                self.visit_expr(r, ctx);
                let l_ty = l.ty.as_ref().expect("LHS has no type");
                let r_ty = r.ty.as_ref().expect("RHS has no type");
                if l_ty != r_ty {
                    panic!("Type mismatch in assignment: {:?} = {:?}", l_ty, r_ty);
                }
            }
            StmtKind::Ret(ref mut expr) => {
                self.visit_expr(expr, ctx);
            }
            StmtKind::If(ref mut cond, ref mut then_branch, ref mut else_branch) => {
                self.visit_expr(cond, ctx);
                if cond.ty != Some(Type::Bool) {
                    panic!("If condition must be bool");
                }
                self.visit_stmt(then_branch, ctx);
                if let Some(e) = else_branch {
                    self.visit_stmt(e, ctx);
                }
            }
            StmtKind::While(ref mut cond, ref mut body) => {
                self.visit_expr(cond, ctx);
                if cond.ty != Some(Type::Bool) {
                    panic!("While condition must be bool");
                }
                self.visit_stmt(body, ctx);
            }
            _ => {}
        }
    }
}

impl SafetyPass {
    pub fn visit_expr(&mut self, expr: &mut Expr, ctx: &mut TypeContext) {
        match expr.kind {
            ExprKind::Binary(ref left, ref op, ref right) => {
                let l_ty = left.ty.as_ref().unwrap();
                let r_ty = right.ty.as_ref().unwrap();

                if l_ty != r_ty {
                    panic!("Binary operation types mismatch: {:?} vs {:?}", l_ty, r_ty);
                }
                match op {
                    BinOp::Add
                    | BinOp::Sub
                    | BinOp::Mul
                    | BinOp::Div
                    | BinOp::Mod
                    | BinOp::BitAnd
                    | BinOp::BitOr
                    | BinOp::BitXor
                    | BinOp::Shl
                    | BinOp::Shr => {}
                    BinOp::Eq
                    | BinOp::Neq
                    | BinOp::Lt
                    | BinOp::Lte
                    | BinOp::Gt
                    | BinOp::Gte
                    | BinOp::And
                    | BinOp::Or => {}
                    _ => panic!("Binary op {:?} not supported", op),
                }
            }
            ExprKind::Unary(ref op, ref operand) => {
                let ty = operand.ty.as_ref().unwrap();
                match op {
                    UnaryOp::Neg => {
                        if !matches!(ty, Type::I32 | Type::I64 | Type::F32 | Type::F64) {
                            panic!("Cannot negate type {:?}", ty);
                        }
                    }
                    UnaryOp::Not => {
                        if !matches!(
                            ty,
                            Type::Bool | Type::I32 | Type::I64 | Type::U32 | Type::U64
                        ) {
                            panic!("Cannot apply NOT to type {:?}", ty);
                        }
                    }
                    _ => {}
                }
            }
            ExprKind::Call(ref callee, ref args, _) => {
                if let Some(Type::Function(param_types, _, generic_params_decl)) = &callee.ty {
                    let mut generic_map = HashMap::new();
                    let engine = GenericsEngine;

                    if !generic_params_decl.is_empty() {
                        for (param_ty, arg) in param_types.iter().zip(args.iter()) {
                            engine.unify_types(
                                param_ty,
                                arg.ty.as_ref().unwrap(),
                                &mut generic_map,
                            );
                        }
                    }

                    if args.len() != param_types.len() {
                        panic!("Argument count mismatch");
                    }

                    for (i, (param_ty, arg)) in param_types.iter().zip(args.iter()).enumerate() {
                        let concrete_param_ty =
                            GenericsEngine::substitute_generics_mut(param_ty, &generic_map, ctx);
                        let arg_ty = arg.ty.as_ref().unwrap();
                        if !self.check_type_compatibility(&concrete_param_ty, arg_ty) {
                            panic!(
                                "Arg {} type mismatch: expected {:?}, found {:?}",
                                i, concrete_param_ty, arg_ty
                            );
                        }
                    }
                }
            }
            ExprKind::StructInit(ref path, ref fields, _) => {
                let struct_name = path.last().unwrap();
                let (_struct_generics_decl, struct_fields_decl) =
                    if let Some(def) = ctx.concrete_structs.get(struct_name) {
                        (def.generics.clone(), def.fields.clone())
                    } else if let Some(def) = ctx.generic_struct_templates.get(struct_name) {
                        (def.generics.clone(), def.fields.clone())
                    } else {
                        panic!("Struct definition not found: {}", struct_name);
                    };

                for (field_name, expr) in fields.iter() {
                    let _expected_base_ty = struct_fields_decl
                        .iter()
                        .find(|(n, _)| n == field_name)
                        .map(|(_, t)| t)
                        .expect("Field not found");

                    let _actual_ty = expr.ty.as_ref().unwrap();
                }
            }
            ExprKind::Index(ref arr, ref idx) => {
                let idx_ty = idx.ty.as_ref().unwrap();
                if !matches!(
                    idx_ty,
                    Type::I32 | Type::I64 | Type::Usize | Type::U32 | Type::U64
                ) {
                    panic!("Index must be integer");
                }
                let arr_ty = arr.ty.as_ref().unwrap();
                if !matches!(arr_ty, Type::Array(..) | Type::Pointer(..)) {
                    panic!("Cannot index non-array type");
                }
            }
            _ => {}
        }
    }
}

impl Pass for SafetyPass {
    fn name(&self) -> &str {
        "SafetyPass"
    }

    fn run(&mut self, ctx: &mut TypeContext) {
        let keys: Vec<_> = ctx.concrete_funcs.keys().cloned().collect();
        for k in keys {
            let mut func = ctx.concrete_funcs.remove(&k).unwrap();
            self.visit_function_def(&mut func, ctx);
            ctx.concrete_funcs.insert(k, func);
        }
    }
}
