use std::u32;

use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind::{self as Tk, CBrace};

use crate::{parser::Parser, precedence::Precedence};

impl Parser<'_> {
    #[inline]
    pub fn parse_struct(&mut self) -> HirId {
        self.bump();
        self.expect(Tk::OBrace);

        let mut names: Vec<u32> = vec![];
        let mut types: Vec<u32> = vec![];

        loop {
            if self.peek() == Some(CBrace) {
                break;
            }

            let name = self.parse_expr(0);
            let ty = self.parse_expr(0);

            names.push(name.0);
            types.push(ty.0);

            if self.peek() == Some(CBrace) {
                break;
            }

            if !self.peek_preceded_by_newline() {
                self.expect(Tk::Comma);
            }
        }

        self.expect(Tk::CBrace);

        let names_id = self.db.add_list_flat(&names);
        let types_id = self.db.add_list_flat(&types);

        let id = self.db.hir.alloc_struct(names_id, types_id);
        id
    }

    #[inline]
    pub fn parse_struct_init(&mut self) -> HirId {
        self.bump(); // .
        self.expect(Tk::OBrace);

        let mut fields = vec![];
        let mut values = vec![];

        loop {
            if self.peek() == Some(Tk::CBrace) {
                break;
            }

            let f = self.parse_expr(Precedence::VarDef.value() + 1);
            self.expect(Tk::Colon);
            let v = self.parse_expr(0);

            fields.push(f.0);
            values.push(v.0);

            if self.peek() == Some(CBrace) {
                break;
            }
            if !self.peek_preceded_by_newline() {
                self.expect(Tk::Comma);
            }
        }

        self.expect(Tk::CBrace);

        let fids = self.db.add_list_flat(&fields);
        let vids = self.db.add_list_flat(&values);

        self.db.hir.alloc_struct_init(fids, vids)
    }
}
