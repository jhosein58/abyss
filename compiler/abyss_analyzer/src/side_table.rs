use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct NodeProperties {
    pub is_const: bool,
    pub should_fold: bool,
}

pub struct SideTable {
    properties: HashMap<u32, NodeProperties>,
}

impl SideTable {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    fn get_mut_props(&mut self, id: u32) -> &mut NodeProperties {
        self.properties
            .entry(id)
            .or_insert_with(NodeProperties::default)
    }

    pub fn mark_const(&mut self, id: u32, should_fold: bool) {
        let props = self.get_mut_props(id);
        props.is_const = true;
        props.should_fold = should_fold;
    }

    pub fn set_fold_permission(&mut self, id: u32, should_fold: bool) {
        self.get_mut_props(id).should_fold = should_fold;
    }

    pub fn is_const(&self, id: u32) -> bool {
        self.properties
            .get(&id)
            .map(|p| p.is_const)
            .unwrap_or(false)
    }

    pub fn should_fold(&self, id: u32) -> bool {
        self.properties
            .get(&id)
            .map(|p| p.is_const && p.should_fold)
            .unwrap_or(false)
    }
}
