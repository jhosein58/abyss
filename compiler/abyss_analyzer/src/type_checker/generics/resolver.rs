use abyss_parser::ast::{Expr, FunctionBody, FunctionDef, Lit, Stmt, StructDef, Type};

pub struct GenericResolver;

impl GenericResolver {
    pub fn resolve_func(&self, func: &mut FunctionDef) {
        let generics = func.generics.clone();

        for (_, param_ty) in &mut func.params {
            self.convert_struct_to_generic(param_ty, &generics);
        }

        self.convert_struct_to_generic(&mut func.return_type, &generics);

        if let FunctionBody::UserDefined(stmts) = &mut func.body {
            for stmt in stmts {
                self.resolve_generics_in_stmt(stmt, &generics);
            }
        }
    }

    fn convert_struct_to_generic(&self, ty: &mut Type, generic_names: &[String]) {
        match ty {
            Type::Struct(path, args) => {
                if path.len() == 1 && args.is_empty() {
                    if generic_names.contains(&path[0]) {
                        *ty = Type::Generic(path[0].clone());
                        return;
                    }
                }
                for arg in args {
                    self.convert_struct_to_generic(arg, generic_names);
                }
            }
            Type::Pointer(inner) | Type::Array(inner, _) => {
                self.convert_struct_to_generic(inner, generic_names);
            }
            Type::Function(args, ret, _) => {
                for arg in args {
                    self.convert_struct_to_generic(arg, generic_names);
                }
                self.convert_struct_to_generic(ret, generic_names);
            }

            Type::Union(variants) => {
                for variant in variants {
                    self.convert_struct_to_generic(variant, generic_names);
                }
            }
            _ => {}
        }
    }

    fn resolve_generics_in_stmt(&self, stmt: &mut Stmt, generic_names: &[String]) {
        match stmt {
            Stmt::Let(_, ty_opt, expr_opt) => {
                if let Some(ty) = ty_opt {
                    self.convert_struct_to_generic(ty, generic_names);
                }
                if let Some(expr) = expr_opt {
                    self.resolve_generics_in_expr(expr, generic_names);
                }
            }
            Stmt::Assign(lhs, rhs) => {
                self.resolve_generics_in_expr(lhs, generic_names);
                self.resolve_generics_in_expr(rhs, generic_names);
            }
            Stmt::Expr(expr) | Stmt::Ret(expr) => {
                self.resolve_generics_in_expr(expr, generic_names);
            }
            Stmt::If(cond, then_b, else_b) => {
                self.resolve_generics_in_expr(cond, generic_names);
                self.resolve_generics_in_stmt(then_b, generic_names);
                if let Some(e) = else_b {
                    self.resolve_generics_in_stmt(e, generic_names);
                }
            }
            Stmt::While(cond, body) => {
                self.resolve_generics_in_expr(cond, generic_names);
                self.resolve_generics_in_stmt(body, generic_names);
            }
            Stmt::Block(stmts) => {
                for s in stmts {
                    self.resolve_generics_in_stmt(s, generic_names);
                }
            }
            _ => {}
        }
    }

    fn resolve_generics_in_expr(&self, expr: &mut Expr, generic_names: &[String]) {
        match expr {
            Expr::Cast(inner, ty) => {
                self.resolve_generics_in_expr(inner, generic_names);
                self.convert_struct_to_generic(ty, generic_names);
            }

            Expr::Binary(lhs, _, rhs) => {
                self.resolve_generics_in_expr(lhs, generic_names);
                self.resolve_generics_in_expr(rhs, generic_names);
            }
            Expr::Call(callee, args, generics) => {
                self.resolve_generics_in_expr(callee, generic_names);
                for arg in args {
                    self.resolve_generics_in_expr(arg, generic_names);
                }
                for g in generics {
                    self.convert_struct_to_generic(g, generic_names);
                }
            }
            Expr::StructInit(_, fields, generics) => {
                for (_, val) in fields {
                    self.resolve_generics_in_expr(val, generic_names);
                }
                for g in generics {
                    self.convert_struct_to_generic(g, generic_names);
                }
            }
            Expr::Unary(_, inner)
            | Expr::Deref(inner)
            | Expr::AddrOf(inner)
            | Expr::Member(inner, _) => {
                self.resolve_generics_in_expr(inner, generic_names);
            }
            Expr::SizeOf(ty) => {
                self.convert_struct_to_generic(ty, generic_names);
            }
            Expr::Index(arr, idx) => {
                self.resolve_generics_in_expr(arr, generic_names);
                self.resolve_generics_in_expr(idx, generic_names);
            }
            Expr::Lit(Lit::Array(exprs)) => {
                for e in exprs {
                    self.resolve_generics_in_expr(e, generic_names);
                }
            }
            _ => {}
        }
    }
    pub fn resolve_struct(&self, struct_def: &mut StructDef) {
        let generics = struct_def.generics.clone();
        for (_, field_ty) in &mut struct_def.fields {
            self.convert_struct_to_generic(field_ty, &generics);
        }
    }
}
