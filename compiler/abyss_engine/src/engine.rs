use abyss_diagnostics::DiagnosticFormatter;
use abyss_indexer::Indexer;
use abyss_lower::{codegen::CCodeGen, lowerer};
use abyss_nexus::{
    arena::ArenaId,
    nexus::{FileId, Nexus, SlotId, SymbolId, TypeId},
};
use abyss_parser::parser::Parser;
use abyss_typer::tyck::{TyCtx, Typer};

pub struct Engine {
    pub db: Nexus,
    pub ccg: CCodeGen,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            db: Nexus::default(),
            ccg: CCodeGen::default(),
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

        let mut tc = Typer::new(self);

        tc.type_check(range);
    }

    pub fn compile(&mut self, sym_id: SymbolId) {
        lowerer::lower_function(&mut self.db, &mut self.ccg, sym_id)
    }

    pub fn print_err(&self) {
        let formater = DiagnosticFormatter::new(&self.db);
        let diagnostics = formater.format_all();
        println!("{}", diagnostics);
    }

    // PERF: create a fast-path
    // TODO: report error
    pub fn ensure_resolved(&mut self, sym_id: SymbolId) {
        let is_resolving = self.db.symbol_is_resolving.get_copy(sym_id);

        if is_resolving == true {
            panic!("Error, cycle")
        }

        let needs_parsing = self.db.symbols.get_copy(sym_id).is_none();

        if needs_parsing {
            self.db.symbol_is_resolving.set(sym_id, true);
            self.parse(sym_id);
        }

        let needs_typecheck = self
            .db
            .unify
            .get_slot(self.db.symbols.get_copy(sym_id))
            .is_none();

        if needs_typecheck {
            self.db.symbol_is_resolving.set(sym_id, true);
            self.type_check(sym_id);
        }

        self.db.symbol_is_resolving.set(sym_id, false);
    }
}

impl TyCtx for Engine {
    fn db(&self) -> &Nexus {
        &self.db
    }

    fn db_mut(&mut self) -> &mut Nexus {
        &mut self.db
    }

    fn slot_of(&mut self, sym_id: SymbolId) -> SlotId {
        self.ensure_resolved(sym_id);

        let hir_id = self.db.symbols.get_copy(sym_id);
        self.db.unify.get_slot(hir_id)
    }

    fn type_of(&mut self, sym_id: SymbolId) -> TypeId {
        let slot = self.slot_of(sym_id);
        self.db.unify.resolve_type(slot)
    }
}
