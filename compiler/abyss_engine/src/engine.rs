use abyss_indexer::Indexer;
use abyss_lower::{builder::ComptimeProvider, materialazer};
use abyss_nexus::nexus::{FileId, Nexus, SymbolId};
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

    pub fn parse(&mut self, file_id: FileId, sym_name: &str) -> SymbolId {
        let name_id = self.db.interner.get_id(sym_name).unwrap();
        Parser::parse_top_level(&mut self.db, file_id, name_id)
    }

    pub fn type_check(&mut self, sym_id: SymbolId) {
        let range = self.db.symbol_hir_range.get_copy(sym_id);
        tyck::type_check(&mut self.db, range);
    }

    pub fn run(&mut self, sym_id: SymbolId) -> u64 {
        let func_id = materialazer::lower_function(&mut self.db, &mut self.provider, sym_id);
        self.provider.eval_function(func_id, &[])
    }
}
