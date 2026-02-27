use abyss_diagnostics::Span;

use super::types::Type;
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub ty: Type,
    pub is_mutable: bool,
    pub is_initialized: bool,
}

#[derive(Debug, Clone)]
pub struct TypeError {
    pub message: String,
    pub span: Span,
}

pub struct TypeContext {
    scopes: Vec<HashMap<String, SymbolInfo>>,
    pub errors: Vec<TypeError>,
}

impl TypeContext {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            errors: Vec::new(),
        }
    }

    // --- Scope Management ---

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        } else {
            panic!("Compiler Bug: Cannot exit global scope!");
        }
    }

    pub fn define(&mut self, name: String, ty: Type, is_mutable: bool) -> Result<(), String> {
        let current_scope = self.scopes.last_mut().expect("Scope stack is empty");

        if current_scope.contains_key(&name) {
            return Err(format!(
                "Variable '{}' is already defined in this scope.",
                name
            ));
        }

        current_scope.insert(
            name,
            SymbolInfo {
                ty,
                is_mutable,
                is_initialized: true,
            },
        );

        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&SymbolInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }

    pub fn is_defined_in_current_scope(&self, name: &str) -> bool {
        self.scopes
            .last()
            .map(|s| s.contains_key(name))
            .unwrap_or(false)
    }

    // --- Error Handling ---

    pub fn add_error(&mut self, message: String, span: Span) {
        self.errors.push(TypeError { message, span });
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn render_errors(&self) -> String {
        let mut output = String::new();

        if self.errors.is_empty() {
            return output;
        }

        writeln!(&mut output, "Found {} error(s):", self.errors.len()).unwrap();
        writeln!(&mut output, "----------------------------------------").unwrap();

        for (i, err) in self.errors.iter().enumerate() {
            writeln!(
                &mut output,
                "{}. Error: {}\n   at {:?}",
                i + 1,
                err.message,
                err.span
            )
            .unwrap();
            writeln!(&mut output, "----------------------------------------").unwrap();
        }

        output
    }
}
