use abyss_lexer::token::TokenKind as Tk;

use super::precedence::Precedence;
use crate::ast::Expr;
use crate::parser::PrattEngine;
use crate::parser::handlers::*;

pub type PrefixFn = fn(&mut PrattEngine) -> Result<Expr, super::ParseError>;
pub type InfixFn = fn(&mut PrattEngine, Expr) -> Result<Expr, super::ParseError>;

pub struct ParseRule {
    pub prefix: Option<PrefixFn>,
    pub infix: Option<InfixFn>,
    pub precedence: Precedence,
    pub is_soft: bool,
}

impl ParseRule {
    fn new(prefix: Option<PrefixFn>, infix: Option<InfixFn>, prec: Precedence) -> Self {
        Self {
            prefix,
            infix,
            precedence: prec,
            is_soft: false,
        }
    }

    pub fn soft(mut self) -> Self {
        self.is_soft = true;
        self
    }
}

pub fn get_rule(kind: Tk) -> ParseRule {
    match kind {
        Tk::IntLit | Tk::StrLit | Tk::CStrLit | Tk::FloatLit | Tk::BinIntLit | Tk::HexIntLit => {
            ParseRule::new(Some(parse_literal), None, Precedence::None)
        }

        Tk::Ident => ParseRule::new(Some(parse_ident), None, Precedence::None),

        _ => ParseRule::new(None, None, Precedence::None),
    }
}
