pub use std::{fs, time::Instant};

use abyss_diagnostics::DiagnosticFormatter;
use abyss_nexus::{
    nexus::{HirId, Nexus},
    storages::diagnostics::{DiagnosticKind, DiagnosticMessage, HintMessage, Severity},
};
use abyss_parser::parser::Parser;
use abyss_typer::tyck;

fn main() {
    let t = Instant::now();

    let mut nexus = Nexus::new();

    let file_id = nexus.add_file("main.a", fs::read_to_string("main.a").unwrap());
    nexus.lex_file(file_id);

    Parser::index(&mut nexus, file_id);

    let main_id = nexus.interner.get_id("main").unwrap();
    //let add_id = nexus.interner.get_id("add").unwrap();

    let main_symbol_id = Parser::parse_top_level(&mut nexus, file_id, main_id);
    //let add_symbol_id = Parser::parse_top_level(&mut nexus, file_id, add_id);

    nexus.hir.print_dump(&nexus);

    let main_range = nexus.symbol_hir_range.get(main_symbol_id).clone();

    tyck::check(&mut nexus, main_range);

    let span = nexus.hir_spans.get_copy(HirId(0));
    let file_id = nexus.hir_files.get_copy(HirId(0));

    let span2 = nexus.hir_spans.get_copy(HirId(6));

    nexus
        .diagnostics
        .add_label(DiagnosticMessage::Dummy, file_id, span2, false);

    nexus.diagnostics.emit(
        DiagnosticKind::UnexpectedToken,
        Severity::Error,
        0,
        0,
        file_id,
        span,
        Some(HintMessage::Dummy),
    );

    let formater = DiagnosticFormatter::new(&nexus);
    let diagnostics = formater.format_all();
    println!("{}", diagnostics);

    println!("Time: {:?}", t.elapsed());
}
