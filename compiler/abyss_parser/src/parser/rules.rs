use abyss_lexer::token::TokenKind as Tk;

use super::precedence::Precedence;
use crate::ast::Expr;
use crate::parser::PrattEngine;
use crate::parser::handlers::*;

pub type PrefixFn = fn(&mut PrattEngine) -> Result<Expr, ()>;
pub type InfixFn = fn(&mut PrattEngine, Expr) -> Result<Expr, ()>;

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
        // literals
        Tk::IntLit
        | Tk::StrLit
        | Tk::CStrLit
        | Tk::CharLit
        | Tk::FloatLit
        | Tk::BinIntLit
        | Tk::HexIntLit
        | Tk::True
        | Tk::False => ParseRule::new(Some(parse_literal), None, Precedence::None),
        // ident
        Tk::Ident => ParseRule::new(Some(parse_ident), None, Precedence::None),
        // -
        Tk::Minus => ParseRule::new(Some(parse_unary), Some(parse_binary), Precedence::Term),
        // &
        Tk::Amp => ParseRule::new(Some(parse_unary), Some(parse_binary), Precedence::BitAnd).soft(),
        // *
        Tk::Star => {
            ParseRule::new(Some(parse_unary), Some(parse_binary), Precedence::Factor).soft()
        }
        // not ~
        Tk::Not | Tk::Tilde => ParseRule::new(Some(parse_unary), None, Precedence::None),
        // :
        Tk::Colon => ParseRule::new(None, Some(parse_binary), Precedence::KeyValue).soft(),
        // ::
        Tk::ColonColon => ParseRule::new(None, Some(parse_binary), Precedence::ConstDef).soft(),
        // +
        Tk::Plus => ParseRule::new(None, Some(parse_binary), Precedence::Term).soft(),
        // / %
        Tk::Slash | Tk::Percent => {
            ParseRule::new(None, Some(parse_binary), Precedence::Factor).soft()
        }
        // == !=
        Tk::EqEq | Tk::BangEq => {
            ParseRule::new(None, Some(parse_binary), Precedence::Equality).soft()
        }
        // < > <= >=
        Tk::Lt | Tk::Gt | Tk::LtEq | Tk::GtEq => {
            ParseRule::new(None, Some(parse_binary), Precedence::Comparison).soft()
        }
        // and
        Tk::And => ParseRule::new(None, Some(parse_binary), Precedence::LogicAnd).soft(),
        // or
        Tk::Or => ParseRule::new(None, Some(parse_binary), Precedence::LogicOr).soft(),
        // |
        Tk::Pipe => ParseRule::new(None, Some(parse_binary), Precedence::BitOr).soft(),
        // ^
        Tk::Caret => ParseRule::new(None, Some(parse_binary), Precedence::BitXor).soft(),
        // << >>
        Tk::LeftShift | Tk::RightShift => {
            ParseRule::new(None, Some(parse_binary), Precedence::Shift).soft()
        }
        // = := += -= *= /= %= &= |= ^= <<= >>=
        Tk::Assign
        | Tk::ColonEq
        | Tk::PlusAssign
        | Tk::MinusAssign
        | Tk::StarAssign
        | Tk::SlashAssign
        | Tk::PercentAssign
        | Tk::AmpAssign
        | Tk::PipeAssign
        | Tk::CaretAssign
        | Tk::LeftShiftAssign
        | Tk::RightShiftAssign => {
            ParseRule::new(None, Some(parse_binary), Precedence::Assignment).soft()
        }
        // (
        Tk::OParen => ParseRule::new(
            Some(parse_group_or_signature),
            Some(parse_call),
            Precedence::Call,
        ),
        // [
        Tk::OBracket => ParseRule::new(Some(parse_sequence), Some(parse_index), Precedence::Call),
        Tk::OBrace => ParseRule::new(Some(parse_block), None, Precedence::None),
        // .
        Tk::Dot => ParseRule::new(None, Some(parse_member), Precedence::Member),
        // _
        Tk::Underscore => ParseRule::new(Some(parse_wildcard), None, Precedence::None),
        // ret
        Tk::Ret => ParseRule::new(Some(parse_ret), None, Precedence::None),
        // out
        Tk::Out => ParseRule::new(Some(parse_out), None, Precedence::None),
        // next
        Tk::Next => ParseRule::new(Some(parse_continue), None, Precedence::None),
        // if
        Tk::If => ParseRule::new(Some(parse_if), None, Precedence::None),
        // while
        Tk::While => ParseRule::new(Some(parse_while), None, Precedence::None),
        // forever
        Tk::Forever => ParseRule::new(Some(parse_forever), None, Precedence::None),
        // def
        Tk::Def => ParseRule::new(Some(parse_def), None, Precedence::None),
        // cmpt
        Tk::Cmpt => ParseRule::new(Some(parse_comptime), None, Precedence::None),
        // #
        Tk::Hash => ParseRule::new(Some(parse_attributed), None, Precedence::None),
        // as
        Tk::As => ParseRule::new(None, Some(parse_cast), Precedence::Cast).soft(),

        _ => ParseRule::new(None, None, Precedence::None),
    }
}
