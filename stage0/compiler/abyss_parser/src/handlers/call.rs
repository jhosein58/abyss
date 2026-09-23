use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_call(&mut self, lhs: HirId, _: u8) -> HirId {
        let mut args = Vec::with_capacity(8); // FIXME: remove allocation

        if self.optional(Tk::CParen) {
            let arg_list = self.db.add_list_flat(&[]);
            return self.db.hir.alloc_call(lhs, arg_list);
        }

        loop {
            if self.peek() == Some(Tk::CParen) || self.peek().is_none() {
                break;
            }

            let arg = self.parse_expr(0);

            args.push(arg.0);

            if self.peek() == Some(Tk::CParen) {
                break;
            }

            self.expect(Tk::Comma);
        }

        self.expect(Tk::CParen);

        let arg_list = self.db.add_list_flat(&args);
        self.db.hir.alloc_call(lhs, arg_list)
    }
}
