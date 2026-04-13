use super::structural::{self, StructuralMatch};
use abyss_types::{tast::TypedExpr, types::Type};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ParamKind {
    Duck { duck_struct: Type, is_ptr: bool },
    MetaType,
}

#[derive(Debug, Clone)]
pub struct TemplateParam {
    pub param_index: usize,
    pub original_type: Type,
    pub kind: ParamKind,
}

#[derive(Debug, Clone)]
pub struct TemplateFunction {
    pub source_name: String,
    pub ir_name: String,
    pub func_type: Type,
    pub typed_def: TypedExpr,
    pub template_params: Vec<TemplateParam>,
}

#[derive(Debug, Clone)]
pub struct MonomorphizedInstance {
    pub ir_name: String,
    pub func_type: Type,
    pub typed_def: TypedExpr,
}

pub struct TemplateRegistry {
    templates: HashMap<String, TemplateFunction>,
    by_source_name: HashMap<String, Vec<String>>,
    instances: HashMap<(String, String), MonomorphizedInstance>,
    instance_order: Vec<(String, String)>,
    mono_counter: usize,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
            by_source_name: HashMap::new(),
            instances: HashMap::new(),
            instance_order: Vec::new(),
            mono_counter: 0,
        }
    }

    pub fn register(&mut self, template: TemplateFunction) {
        self.by_source_name
            .entry(template.source_name.clone())
            .or_default()
            .push(template.ir_name.clone());

        self.templates.insert(template.ir_name.clone(), template);
    }

    pub fn is_template(&self, ir_name: &str) -> bool {
        self.templates.contains_key(ir_name)
    }

    pub fn get(&self, ir_name: &str) -> Option<&TemplateFunction> {
        self.templates.get(ir_name)
    }

    pub fn find_compatible_method(
        &self,
        method_name: &str,
        concrete_type: &Type,
    ) -> Option<(&TemplateFunction, usize, StructuralMatch)> {
        let ir_names = self.by_source_name.get(method_name)?;

        for ir_name in ir_names {
            let template = self.templates.get(ir_name)?;

            for param in &template.template_params {
                if let ParamKind::Duck { duck_struct: _, .. } = &param.kind {
                    if let Some(match_info) =
                        structural::check_structural_compat(concrete_type, &param.original_type)
                    {
                        return Some((template, param.param_index, match_info));
                    }
                }
            }
        }
        None
    }

    pub fn make_concrete_key(concrete_types: &[Type]) -> String {
        concrete_types
            .iter()
            .map(|t| t.mangled_name())
            .collect::<Vec<_>>()
            .join("$")
    }

    pub fn get_cached(
        &self,
        template_ir: &str,
        concrete_key: &str,
    ) -> Option<&MonomorphizedInstance> {
        self.instances
            .get(&(template_ir.to_string(), concrete_key.to_string()))
    }

    pub fn generate_mono_name(&mut self, source_name: &str, concrete_key: &str) -> String {
        self.mono_counter += 1;
        format!(
            "__mono_{}_{}_{}",
            source_name, concrete_key, self.mono_counter
        )
    }

    pub fn cache_instance(
        &mut self,
        template_ir: String,
        concrete_key: String,
        instance: MonomorphizedInstance,
    ) {
        let cache_key = (template_ir, concrete_key);
        if !self.instances.contains_key(&cache_key) {
            self.instance_order.push(cache_key.clone());
        }
        self.instances.insert(cache_key, instance);
    }

    pub fn drain_instances(&mut self) -> Vec<MonomorphizedInstance> {
        let keys: Vec<_> = std::mem::take(&mut self.instance_order);
        let mut result = Vec::with_capacity(keys.len());
        for key in keys {
            if let Some(inst) = self.instances.remove(&key) {
                result.push(inst);
            }
        }
        result
    }
}
