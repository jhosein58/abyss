use abyss_parser::ast::Expr;
use abyss_types::{tast::TypedExpr, types::Type};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlinePolicy {
    Never,
    Always,
    Prefer,
}

#[derive(Debug, Clone)]
pub struct GlobalMetadata {
    pub inline_policy: InlinePolicy,
    pub is_foldable: bool,
}

pub enum GlobalDefState<'a> {
    Unresolved(&'a Expr),
    Resolving,
    ForwardDeclared(Type),
    Resolved {
        ty: Type,
        typed_expr: TypedExpr,
        is_type_def: bool,
        metadata: GlobalMetadata,
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
        metadata: GlobalMetadata,
    ) {
        self.definitions.insert(
            name,
            GlobalDefState::Resolved {
                ty,
                typed_expr,
                is_type_def,
                metadata,
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
            Some(GlobalDefState::ForwardDeclared(ty)) => Some(ty.clone()),
            _ => None,
        }
    }

    pub fn get_metadata(&self, name: &str) -> Option<GlobalMetadata> {
        match self.definitions.get(name) {
            Some(GlobalDefState::Resolved { metadata, .. }) => Some(metadata.clone()),
            _ => None,
        }
    }

    pub fn set_forward_declaration(&mut self, name: String, ty: Type) {
        if matches!(self.definitions.get(&name), Some(GlobalDefState::Resolving)) {
            self.definitions
                .insert(name, GlobalDefState::ForwardDeclared(ty));
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
}

impl<'a> Default for GlobalResolver<'a> {
    fn default() -> Self {
        Self::new()
    }
}
