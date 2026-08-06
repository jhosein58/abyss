use abyss_nexus::nexus::{HirId, ScopeId};
use abyss_token::kind::TokenKind as Tk;

use crate::{engine::parse_expr, parser::Parser};

pub fn handle(p: &mut Parser, parent: ScopeId) -> HirId {
    p.bump();

    let mut items = vec![];

    let block_scope = p.db.scopes.alloc(Some(parent));

    loop {
        if let Some(Tk::CBrace) = p.peek() {
            p.bump();
            break;
        }

        let item = parse_expr(p, 0, block_scope);
        items.push(item.0);
    }

    let items = p.db.add_list_flat(&items);
    let hir_id = p.db.hir.alloc_block(items);
    p.db.scopes.set(hir_id, block_scope);
    hir_id
}

pub fn consume_cbrace(p: &mut Parser) -> HirId {
    p.bump();
    HirId(0)
}
