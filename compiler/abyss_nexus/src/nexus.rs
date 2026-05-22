use std::collections::HashMap;

use abyss_diagnostics::Span;
use abyss_parser::ast::OrderedFloat;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HirAttribute {
    pub name: StringId,
    pub args_start: u32,
    pub span: Span,
}

#[derive(Default)]
pub struct Nexus {
    pub strings: Vec<String>,
    pub string_map: HashMap<String, StringId>,
    pub files: Vec<String>,
    pub node_spans: Vec<Span>,
    pub node_files: Vec<FileId>,
    pub ints: Vec<i64>,
    pub floats: Vec<OrderedFloat>,
    pub u32_items: Vec<u32>,
    pub attributes: Vec<HirAttribute>,
    pub match_arms: Vec<(u32, u32)>,
    pub ranges: Vec<(u32, u32, u32, u32)>,
}

impl Nexus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file(&mut self, path: String) -> FileId {
        let id = self.files.len() as u32;
        self.files.push(path);
        FileId(id)
    }

    pub fn intern_string(&mut self, s: &str) -> StringId {
        if let Some(&id) = self.string_map.get(s) {
            return id;
        }
        let id = StringId(self.strings.len() as u32);
        let owned = s.to_string();
        self.strings.push(owned.clone());
        self.string_map.insert(owned, id);
        id
    }

    pub fn get_string(&self, id: StringId) -> &str {
        &self.strings[id.0 as usize]
    }

    pub fn add_node_meta(&mut self, span: Span, file_id: FileId) {
        self.node_spans.push(span);
        self.node_files.push(file_id);
    }

    pub fn get_node_span(&self, node_id: u32) -> Span {
        self.node_spans[node_id as usize].clone()
    }

    pub fn get_node_file(&self, node_id: u32) -> FileId {
        self.node_files[node_id as usize]
    }

    pub fn add_int(&mut self, val: i64) -> u32 {
        let id = self.ints.len() as u32;
        self.ints.push(val);
        id
    }

    pub fn add_float(&mut self, val: OrderedFloat) -> u32 {
        let id = self.floats.len() as u32;
        self.floats.push(val);
        id
    }

    pub fn add_list_flat(&mut self, items: &[u32]) -> u32 {
        let start = self.u32_items.len() as u32;
        self.u32_items.push(items.len() as u32);
        self.u32_items.extend_from_slice(items);
        start
    }

    pub fn add_attribute(&mut self, attr: HirAttribute) -> u32 {
        let id = self.attributes.len() as u32;
        self.attributes.push(attr);
        id
    }

    pub fn add_match_arm(&mut self, pattern: u32, body: u32) -> u32 {
        let id = self.match_arms.len() as u32;
        self.match_arms.push((pattern, body));
        id
    }

    pub fn add_range(&mut self, start: u32, end: u32, step: u32, inclusive: u32) -> u32 {
        let id = self.ranges.len() as u32;
        self.ranges.push((start, end, step, inclusive));
        id
    }
}
