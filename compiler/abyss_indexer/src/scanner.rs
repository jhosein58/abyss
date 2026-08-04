use abyss_nexus::{nexus::Nexus, storages::tokens::storage::TokenId};
use abyss_token::kind::TokenKind as Tk;

pub fn index(db: &mut Nexus) {
    let end = db.tokens.count() as u32;

    let mut i = 1; // skip first token
    while i < end {
        if Tk::ColonColon == db.tokens.kind(TokenId(i)) {
            let start_token = i - 1;
            let prev = db.tokens.kind(TokenId(start_token));

            // ident :: ?
            if prev == Tk::Ident {
                i += 1; // masraf kardan ::
                i = skip_binding(db, i, end); // edaame daadan az aakharesh
            }
        }
        i += 1;
    }
}

// ident :: ???{???}
fn skip_binding(db: &Nexus, mut current: u32, max: u32) -> u32 {
    let mut depth: i32 = 0;
    let mut in_block = false;

    while current < max {
        let kind = db.tokens.kind(TokenId(current));

        match kind {
            Tk::OBrace => {
                depth += 1;
                in_block = true;
            }
            Tk::CBrace => {
                depth -= 1;
                if in_block && depth == 0 {
                    return current;
                }
            }

            _ => {}
        }

        current += 1;
    }

    current.saturating_sub(1)
}
