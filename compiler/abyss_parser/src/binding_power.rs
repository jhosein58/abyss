use abyss_token::kind::TokenKind as Tk;

use crate::precedence::Precedence;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BindingPower {
    pub left: u8,
    pub right: u8,
}

impl BindingPower {
    pub fn from_infix(kind: Tk) -> Option<BindingPower> {
        let bp = match kind {
            Tk::Colon => Precedence::VarDef.right_assoc(),
            Tk::ColonColon => Precedence::ConstDef.right_assoc(),

            Tk::Eq => Precedence::Assignment.right_assoc(),
            Tk::Plus | Tk::Minus => Precedence::Term.left_assoc(),
            Tk::Star | Tk::Slash => Precedence::Factor.left_assoc(),

            Tk::OParen => Precedence::Call.left_assoc(),

            Tk::And => Precedence::LogicAnd.left_assoc(),
            Tk::Or => Precedence::LogicOr.left_assoc(),

            _ => return None,
        };

        Some(bp)
    }
}

#[inline]
pub fn is_soft(kind: Tk) -> bool {
    matches!(
        kind,
        // arithmetic
        Tk::Plus
        | Tk::Slash
        | Tk::Percent

        // key-value / const-def
        | Tk::Colon
        | Tk::ColonColon

        // equality / comparison
        | Tk::EqEq
        | Tk::BangEq
        | Tk::Lt | Tk::Gt | Tk::LtEq | Tk::GtEq

        // logic
        | Tk::And | Tk::Or

        // bitwise
        | Tk::Pipe | Tk::Caret
        | Tk::LeftShift | Tk::RightShift

        // assignments
        | Tk::Eq
        | Tk::PlusEq | Tk::MinusEq
        | Tk::StarEq | Tk::SlashEq | Tk::PercentEq
        | Tk::AmpEq | Tk::PipeEq | Tk::CaretEq
        | Tk::LeftShiftEq | Tk::RightShiftEq

        // cast
        | Tk::As
    )
}
