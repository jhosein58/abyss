use abyss_nexus::nexus::HirId;

use crate::{parser::Parser, precedence::Precedence};

impl Parser<'_> {
    #[inline(always)]
    pub fn parse_not(&mut self) -> HirId {
        self.bump();
        let body = self.parse_expr(Precedence::Unary.value());
        self.db.hir.alloc_not(body)
    }

    #[inline(always)]
    pub fn parse_neg(&mut self) -> HirId {
        self.bump();
        let body = self.parse_expr(Precedence::Unary.value());
        self.db.hir.alloc_neg(body)
    }

    #[inline(always)]
    pub fn parse_bit_not(&mut self) -> HirId {
        self.bump();
        let body = self.parse_expr(Precedence::Unary.value());
        self.db.hir.alloc_bit_not(body)
    }

    // IDEA: combine all methods to a one single methode called "parse_unary"

    #[inline(always)]
    pub fn parse_addrof(&mut self) -> HirId {
        self.bump();
        let inner = self.parse_expr(Precedence::Unary.value());
        self.db.hir.alloc_addrof(inner)
    }

    #[inline(always)]
    pub fn parse_deref(&mut self) -> HirId {
        self.bump();
        let inner = self.parse_expr(Precedence::Unary.value());
        self.db.hir.alloc_deref(inner)
    }

    #[inline(always)]
    pub fn parse_break(&mut self) -> HirId {
        self.bump();
        self.db.hir.alloc_break()
    }

    #[inline(always)]
    pub fn parse_cont(&mut self) -> HirId {
        self.bump();
        self.db.hir.alloc_cont()
    }
}
