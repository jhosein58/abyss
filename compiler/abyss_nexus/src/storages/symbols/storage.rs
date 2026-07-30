#[derive(Debug, Default, Clone, Copy)]
pub struct SymbolSpan {
    pub start: u32,
    pub end: u32,
}

#[repr(u8)]
#[derive(Clone, Copy, Default)]
pub enum SymbolState {
    #[default]
    Unresolved,
    Resolving,
    Resolved,
}

#[derive(Default)]
pub struct SymbolStorage {
    span_arena: Vec<SymbolSpan>,
    state_arena: Vec<SymbolState>,
}

impl SymbolStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(&mut self, ca: usize) {
        self.span_arena.resize(ca, SymbolSpan::default());
        self.state_arena.resize(ca, SymbolState::default());
    }

    pub fn set_span(&mut self, idx: u32, span: SymbolSpan) {
        self.span_arena[idx as usize] = span;
    }

    pub fn set_state(&mut self, idx: u32, state: SymbolState) {
        self.state_arena[idx as usize] = state;
    }

    pub fn get_state(&self, idx: u32) -> SymbolState {
        self.state_arena[idx as usize]
    }

    pub fn get_span(&self, idx: u32) -> SymbolSpan {
        self.span_arena[idx as usize]
    }

    pub fn define(&mut self, idx: u32, span: SymbolSpan) {
        self.set_span(idx, span);
        self.set_state(idx, SymbolState::Unresolved);
    }
}
