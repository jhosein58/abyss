use abyss_diagnostics::Span;
use abyss_hir::hir::HirTable;

use crate::storages::{
    hir::storage::HirStorage,
    interner::storage::{InternerStorage, NameId},
    literals::storage::LiteralStorage,
    symbols::storage::SymbolStorage,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HirAttribute {
    pub name: NameId,
    pub args_start: u32,
    pub span: Span,
}

#[derive(Default)]

pub struct Nexus {
    // Storages
    pub hir: HirStorage,
    pub interner: InternerStorage,
    pub symbols: SymbolStorage,
    pub literals: LiteralStorage,

    pub files: Vec<String>,
    pub node_spans: Vec<Span>,
    pub node_files: Vec<FileId>,

    pub u32_items: Vec<u32>,
    pub attributes: Vec<HirAttribute>,
    pub match_arms: Vec<(u32, u32)>,
    pub ranges: Vec<(u32, u32, u32, u32)>,
}

impl Nexus {
    pub fn new() -> Self {
        Self::default()
    }

    // --------> hir
    pub fn set_hir(&mut self, table: HirTable) {
        self.hir.set(table);
        self.symbols.init(self.hir.len());
    }

    pub fn add_file(&mut self, path: String) -> FileId {
        let id = self.files.len() as u32;
        self.files.push(path);
        FileId(id)
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

    pub fn add_list_flat(&mut self, items: &[u32]) -> u32 {
        let start = self.u32_items.len() as u32;
        self.u32_items.push(items.len() as u32);
        self.u32_items.extend_from_slice(items);
        start
    }

    pub fn get_list_flat(&self, start: u32) -> &[u32] {
        let start = start as usize;
        let len = self.u32_items[start] as usize;
        &self.u32_items[start + 1..start + 1 + len]
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
