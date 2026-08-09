use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

pub fn handle<const H: bool>(p: &mut Parser<H>) -> HirId {
    p.bump();

    let mut items = vec![];

    loop {
        if let Some(Tk::CBrace) = p.peek() {
            p.bump();
            break;
        }

        if p.peek().is_none() {
            break;
        }

        let item = p.parse_expr(0);

        if !H {
            items.push(item.0);
        }

        if let Some(Tk::CBrace) = p.peek() {
            p.bump();
            break;
        }
    }

    if H {
        return HirId::default();
    }

    let items = p.db.add_list_flat(&items);
    let hir_id = p.db.hir.alloc_block(items);

    hir_id
}
