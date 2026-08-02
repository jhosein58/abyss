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
            TokenKind::Eq => Precedence::Assignment.right_assoc(),

            TokenKind::Plus | TokenKind::Minus => Precedence::Term.left_assoc(),

            TokenKind::Star | TokenKind::Slash => Precedence::Factor.left_assoc(),

            _ => return None,
        };

        Some(bp)
    }
}
