use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

pub fn handle(p: &mut Parser) -> HirId {
    p.bump();

    let mut items = vec![];

    loop {
        if let Some(Tk::CBrace) = p.peek() {
            p.bump();
            break;
        }

        let item = p.parse_expr(0);
        items.push(item.0);
    }

    let items = p.db.add_list_flat(&items);
    let hir_id = p.db.hir.alloc_block(items);

    hir_id
}

pub fn consume_cbrace(p: &mut Parser) -> HirId {
    p.bump();
    HirId(0)
}
