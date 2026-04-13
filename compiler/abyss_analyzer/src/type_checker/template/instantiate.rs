use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TypeSubstitution {
    map: HashMap<String, Type>,
}

impl TypeSubstitution {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, duck: &Type, concrete: &Type) {
        self.map.insert(duck.mangled_name(), concrete.clone());

        let ptr_duck = Type::Ptr(Box::new(duck.clone()));
        let ptr_concrete = Type::Ptr(Box::new(concrete.clone()));
        self.map.insert(ptr_duck.mangled_name(), ptr_concrete);
    }

    pub fn apply(&self, ty: &Type) -> Type {
        if let Some(concrete) = self.map.get(&ty.mangled_name()) {
            return concrete.clone();
        }
        match ty {
            Type::Ptr(inner) => Type::Ptr(Box::new(self.apply(inner))),
            Type::Array(inner, len) => Type::Array(Box::new(self.apply(inner)), *len),
            Type::Signature(params, ret, is_native) => {
                let new_params = params.iter().map(|p| self.apply(p)).collect();
                let new_ret = self.apply(ret);
                Type::Signature(new_params, Box::new(new_ret), *is_native)
            }
            Type::Alias(name, inner) => Type::Alias(name.clone(), Box::new(self.apply(inner))),
            Type::Struct(fields) => {
                let new_fields = fields
                    .iter()
                    .map(|f| abyss_types::types::StructField {
                        name: f.name.clone(),
                        ty: self.apply(&f.ty),
                    })
                    .collect();
                Type::Struct(new_fields)
            }

            other => other.clone(),
        }
    }
}

pub fn instantiate_template(
    template_def: &TypedExpr,
    new_name: String,
    subst: &TypeSubstitution,
) -> TypedExpr {
    let mut result = template_def.clone();

    if let TypedExprKind::FunctionDef {
        ref mut name,
        ref mut args,
        ref mut ret_ty,
        ref mut body,
        ..
    } = result.kind
    {
        *name = new_name;
        *ret_ty = subst.apply(ret_ty);

        for arg in args.iter_mut() {
            substitute_expr(arg, subst);
        }

        substitute_expr(body, subst);
    }

    result.ty = subst.apply(&result.ty);
    result
}

fn substitute_expr(expr: &mut TypedExpr, subst: &TypeSubstitution) {
    expr.ty = subst.apply(&expr.ty);

    match &mut expr.kind {
        TypedExprKind::Block(stmts) => {
            for s in stmts.iter_mut() {
                substitute_expr(s, subst);
            }
        }

        TypedExprKind::Binary(lhs, _, rhs) => {
            substitute_expr(lhs, subst);
            substitute_expr(rhs, subst);
        }
        TypedExprKind::Unary(_, inner) => {
            substitute_expr(inner, subst);
        }

        TypedExprKind::Call(callee, args, _) => {
            substitute_expr(callee, subst);
            for a in args.iter_mut() {
                substitute_expr(a, subst);
            }
        }

        TypedExprKind::Def(_, value) => {
            substitute_expr(value, subst);
        }
        TypedExprKind::VarDec(_, ty, init) => {
            *ty = subst.apply(ty);
            if let Some(init_expr) = init {
                substitute_expr(init_expr, subst);
            }
        }

        TypedExprKind::FieldAccess(base, _name) => {
            substitute_expr(base, subst);
        }
        TypedExprKind::Index(base, idx) => {
            substitute_expr(base, subst);
            substitute_expr(idx, subst);
        }

        TypedExprKind::If(cond, then_b, else_b) => {
            substitute_expr(cond, subst);
            substitute_expr(then_b, subst);
            if let Some(e) = else_b.as_mut() {
                substitute_expr(e, subst);
            }
        }
        TypedExprKind::While(cond, body, else_b) => {
            substitute_expr(cond, subst);
            substitute_expr(body, subst);
            if let Some(e) = else_b.as_mut() {
                substitute_expr(e, subst);
            }
        }

        TypedExprKind::Ret(val) => {
            if let Some(v) = val.as_mut() {
                substitute_expr(v, subst);
            }
        }

        TypedExprKind::Cast(inner, target_ty_expr) => {
            substitute_expr(inner, subst);
            substitute_expr(target_ty_expr, subst);
        }

        TypedExprKind::SequenceInit(elements) => {
            for el in elements.iter_mut() {
                substitute_expr(&mut el.expr, subst);
            }
        }

        TypedExprKind::BoundMethod { receiver, .. } => {
            substitute_expr(receiver, subst);
        }

        TypedExprKind::FunctionDef {
            args, ret_ty, body, ..
        } => {
            *ret_ty = subst.apply(ret_ty);
            for a in args.iter_mut() {
                substitute_expr(a, subst);
            }
            substitute_expr(body, subst);
        }

        TypedExprKind::Type(ty) => {
            *ty = subst.apply(ty);
        }

        TypedExprKind::Lit(_)
        | TypedExprKind::Ident(_)
        | TypedExprKind::FuncRef(_)
        | TypedExprKind::Wildcard
        | TypedExprKind::ErrorPlaceholder => {}

        _ => {}
    }
}
