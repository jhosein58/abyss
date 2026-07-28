use std::collections::HashMap;

pub struct SymbolSpan {
    pub start: u32,
    pub end: u32,
}

pub enum SymbolState {
    Unresolved,
    Resolving,
    Resolved,
}

#[derive(Default)]
pub struct SymbolStorage {
    pub table: HashMap<String, (SymbolSpan, SymbolState)>,
}
