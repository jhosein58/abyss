use abyss_token::kind::TokenKind;

use crate::precedence::Precedence;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BindingPower {
    pub left: u8,
    pub right: u8,
}

impl BindingPower {
    pub fn from_infix(kind: TokenKind) -> Option<BindingPower> {
        let bp = match kind {
            TokenKind::Colon | TokenKind::ColonEq => Precedence::VarDef.right_assoc(),

            TokenKind::ColonColon => Precedence::ConstDef.right_assoc(),

            TokenKind::Eq => Precedence::Assignment.right_assoc(),

            TokenKind::Plus | TokenKind::Minus => Precedence::Term.left_assoc(),

            TokenKind::Star | TokenKind::Slash => Precedence::Factor.left_assoc(),

            _ => return None,
        };

        Some(bp)
    }
}

#[inline]
pub fn is_soft(kind: TokenKind) -> bool {
    matches!(
        kind,
        // arithmetic
        TokenKind::Plus
        | TokenKind::Slash
        | TokenKind::Percent

        // key-value / const-def
        | TokenKind::Colon
        | TokenKind::ColonColon

        // equality / comparison
        | TokenKind::EqEq
        | TokenKind::BangEq
        | TokenKind::Lt | TokenKind::Gt | TokenKind::LtEq | TokenKind::GtEq

        // logic
        | TokenKind::And | TokenKind::Or

        // bitwise
        | TokenKind::Pipe | TokenKind::Caret
        | TokenKind::LeftShift | TokenKind::RightShift

        // assignments
        | TokenKind::Eq
        | TokenKind::PlusEq | TokenKind::MinusEq
        | TokenKind::StarEq | TokenKind::SlashEq | TokenKind::PercentEq
        | TokenKind::AmpEq | TokenKind::PipeEq | TokenKind::CaretEq
        | TokenKind::LeftShiftEq | TokenKind::RightShiftEq

        // cast
        | TokenKind::As
    )
}
