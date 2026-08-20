use abyss_diagnostics::DiagnosticFormatter;
use abyss_indexer::Indexer;
use abyss_lower::{builder::ComptimeProvider, materialazer};
use abyss_nexus::nexus::{FileId, Nexus, SymbolId, SymbolState, TypeId};
use abyss_parser::parser::Parser;
use abyss_typer::tyck;

pub struct Engine<B: ComptimeProvider> {
    pub db: Nexus,
    pub provider: B,
}

impl<B: ComptimeProvider> Engine<B> {
    pub fn new() -> Self {
        Self {
            db: Nexus::default(),
            provider: B::new(),
        }
    }

    pub fn add_file(&mut self, path: &str, source: String) -> FileId {
        let file_id = self.db.add_file(path, source);
        self.db.lex_file(file_id);
        Indexer::index(&mut self.db, file_id);
        file_id
    }

    pub fn get_symbol_id(&mut self, file_id: FileId, sym_name: &str) -> SymbolId {
        let name_id = self.db.interner.get_id(sym_name).unwrap();
        *self.db.symbol_index.get(&(file_id, name_id)).unwrap()
    }

    pub fn parse(&mut self, sym_id: SymbolId) -> SymbolId {
        Parser::parse_top_level(&mut self.db, sym_id)
    }

    pub fn type_check(&mut self, sym_id: SymbolId) {
        let range = self.db.symbol_hir_range.get_copy(sym_id);
        tyck::type_check(&mut self.db, range);
    }

    pub fn compile(&mut self, sym_id: SymbolId) -> B::FuncId {
        materialazer::lower_function(&mut self.db, &mut self.provider, sym_id)
    }

    pub fn run(&mut self, sym_id: SymbolId) -> u64 {
        let func_id = self.compile(sym_id);
        self.provider.eval_function(func_id, &[])
    }

    pub fn type_of(&mut self, sym_id: SymbolId) -> TypeId {
        let state = self.db.symbol_to_state.get_copy(sym_id);

        if state == SymbolState::Resolving {
            panic!("Error, cycle")
        }

        if state == SymbolState::Unresolved {
            self.parse(sym_id);
            self.type_check(sym_id);
            //self.compile(sym_id);

            self.db.symbol_to_state.set(sym_id, SymbolState::Resolved);
        }

        let hir_id = self.db.symbols.get_copy(sym_id);
        let slot = self.db.unify.get_slot(hir_id);
        self.db.unify.resolve_type(slot)
    }

    pub fn print_err(&self) {
        let formater = DiagnosticFormatter::new(&self.db);
        let diagnostics = formater.format_all();
        println!("{}", diagnostics);
    }
}
