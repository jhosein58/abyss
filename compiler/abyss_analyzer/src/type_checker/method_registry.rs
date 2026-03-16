use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MethodRegistry {
    registry: HashMap<String, HashMap<String, String>>, // HashMap<TypeName, HashMap<MethodName, MangledGlobalName>>
}

impl MethodRegistry {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }

    pub fn mangle_method_name(type_name: &str, method_name: &str) -> String {
        format!("__method_{}_{}", type_name.to_string(), method_name)
    }

    pub fn register_method(
        &mut self,
        type_name: String,
        method_name: String,
        mangled_name: String,
    ) {
        let type_methods = self.registry.entry(type_name).or_default();
        type_methods.insert(method_name, mangled_name);
    }

    pub fn lookup_method(&self, type_name: &str, method_name: &str) -> Option<String> {
        self.registry.get(type_name)?.get(method_name).cloned()
    }
}
