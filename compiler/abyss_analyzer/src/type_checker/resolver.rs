use abyss_parser::ast::Expr;
use abyss_types::{tast::TypedExpr, types::Type};
use std::collections::HashMap;

pub enum InlinePolicy {
    Never,
    Always,
    Prefer,
}

pub enum GlobalDefState<'a> {
    Unresolved(&'a Expr),
    Resolving,
    Resolved {
        ty: Type,
        typed_expr: TypedExpr,
        is_type_def: bool,
        inline_policy: InlinePolicy,
    },
}

pub struct GlobalResolver<'a> {
    definitions: HashMap<String, GlobalDefState<'a>>,
}

impl<'a> GlobalResolver<'a> {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, expr: &'a Expr) {
        self.definitions
            .insert(name, GlobalDefState::Unresolved(expr));
    }

    pub fn begin_resolve(&mut self, name: &str) -> Option<&'a Expr> {
        let state = self.definitions.remove(name)?;

        match state {
            GlobalDefState::Unresolved(expr) => {
                self.definitions
                    .insert(name.to_string(), GlobalDefState::Resolving);
                Some(expr)
            }
            other => {
                self.definitions.insert(name.to_string(), other);
                None
            }
        }
    }

    pub fn complete_resolve(
        &mut self,
        name: String,
        ty: Type,
        typed_expr: TypedExpr,
        is_type_def: bool,
        policy: InlinePolicy,
    ) {
        self.definitions.insert(
            name,
            GlobalDefState::Resolved {
                ty,
                typed_expr,
                is_type_def,
                inline_policy: policy,
            },
        );
    }

    pub fn get_state(&self, name: &str) -> Option<&GlobalDefState<'a>> {
        self.definitions.get(name)
    }

    pub fn is_resolving(&self, name: &str) -> bool {
        matches!(self.definitions.get(name), Some(GlobalDefState::Resolving))
    }

    pub fn is_resolved(&self, name: &str) -> bool {
        matches!(
            self.definitions.get(name),
            Some(GlobalDefState::Resolved { .. })
        )
    }

    pub fn get_resolved_type(&self, name: &str) -> Option<Type> {
        match self.definitions.get(name) {
            Some(GlobalDefState::Resolved { ty, .. }) => Some(ty.clone()),
            _ => None,
        }
    }

    pub fn get_resolved_expr(&self, name: &str) -> Option<&TypedExpr> {
        match self.definitions.get(name) {
            Some(GlobalDefState::Resolved { typed_expr, .. }) => Some(typed_expr),
            _ => None,
        }
    }

    pub fn is_type_definition(&self, name: &str) -> bool {
        matches!(
            self.definitions.get(name),
            Some(GlobalDefState::Resolved {
                is_type_def: true,
                ..
            })
        )
    }

    pub fn contains(&self, name: &str) -> bool {
        self.definitions.contains_key(name)
    }

    pub fn drain_resolved(&mut self) -> HashMap<String, (Type, TypedExpr)> {
        let mut result = HashMap::new();

        let keys: Vec<_> = self.definitions.keys().cloned().collect();
        for name in keys {
            if let Some(GlobalDefState::Resolved { ty, typed_expr, .. }) =
                self.definitions.remove(&name)
            {
                result.insert(name, (ty, typed_expr));
            }
        }

        result
    }

    pub fn get_all_resolved(&self) -> HashMap<String, TypedExpr> {
        let mut result = HashMap::new();

        for (name, state) in &self.definitions {
            if let GlobalDefState::Resolved { typed_expr, .. } = state {
                result.insert(name.clone(), typed_expr.clone());
            }
        }

        result
    }

    pub fn get_all_resolved_with_types(&self) -> HashMap<String, (Type, TypedExpr)> {
        let mut result = HashMap::new();

        for (name, state) in &self.definitions {
            if let GlobalDefState::Resolved { ty, typed_expr, .. } = state {
                result.insert(name.clone(), (ty.clone(), typed_expr.clone()));
            }
        }

        result
    }

    pub fn get_resolved_values(&self) -> HashMap<String, TypedExpr> {
        let mut result = HashMap::new();

        for (name, state) in &self.definitions {
            if let GlobalDefState::Resolved {
                typed_expr,
                is_type_def: false,
                ..
            } = state
            {
                result.insert(name.clone(), typed_expr.clone());
            }
        }

        result
    }
}

impl<'a> Default for GlobalResolver<'a> {
    fn default() -> Self {
        Self::new()
    }
}
