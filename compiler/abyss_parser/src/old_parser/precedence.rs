use crate::ast::BinaryOp;
use abyss_lexer::token::TokenKind;

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
#[repr(u8)]
pub(crate) enum Precedence {
    _None = 0,
    Assignment = 1,
    Or = 2,
    And = 3,
    BitwiseOr = 4,
    BitwiseXor = 5,
    BitwiseAnd = 6,
    Equality = 7,
    Comparison = 8,
    Shift = 9,
    Term = 10,
    Factor = 11,
    Unary = 12,
    Call = 13,
}

impl Precedence {
    pub fn next_power(self) -> u8 {
        let val = self as u8;
        match self {
            Self::Assignment => val,
            _ => val + 1,
        }
    }

    pub fn infix_for(kind: TokenKind) -> Option<Self> {
        Some(match kind {
            TokenKind::Assign => Self::Assignment,
            TokenKind::Or => Self::Or,
            TokenKind::And => Self::And,
            TokenKind::Pipe => Self::BitwiseOr,
            TokenKind::Caret => Self::BitwiseXor,
            TokenKind::Amp => Self::BitwiseAnd,
            TokenKind::EqEq | TokenKind::BangEq => Self::Equality,
            TokenKind::Lt | TokenKind::Gt | TokenKind::LtEq | TokenKind::GtEq | TokenKind::Is => {
                Self::Comparison
            }
            TokenKind::LeftShift | TokenKind::RightShift => Self::Shift,
            TokenKind::Plus | TokenKind::Minus => Self::Term,
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Self::Factor,
            _ => return None,
        })
    }

    pub fn postfix_for(kind: TokenKind) -> Option<Self> {
        Some(match kind {
            TokenKind::OParen | TokenKind::OBracket | TokenKind::As | TokenKind::Dot => Self::Call,
            _ => return None,
        })
    }
}

pub(crate) fn token_to_binary_op(kind: TokenKind) -> BinaryOp {
    match kind {
        TokenKind::Assign => BinaryOp::Assign,
        TokenKind::Plus => BinaryOp::Add,
        TokenKind::Minus => BinaryOp::Sub,
        TokenKind::Star => BinaryOp::Mul,
        TokenKind::Slash => BinaryOp::Div,
        TokenKind::Percent => BinaryOp::Mod,
        TokenKind::EqEq => BinaryOp::Eq,
        TokenKind::BangEq => BinaryOp::Neq,
        TokenKind::Lt => BinaryOp::Lt,
        TokenKind::Gt => BinaryOp::Gt,
        TokenKind::LtEq => BinaryOp::Lte,
        TokenKind::GtEq => BinaryOp::Gte,
        TokenKind::And => BinaryOp::And,
        TokenKind::Or => BinaryOp::Or,
        TokenKind::Amp => BinaryOp::BitAnd,
        TokenKind::Pipe => BinaryOp::BitOr,
        TokenKind::Caret => BinaryOp::BitXor,
        TokenKind::LeftShift => BinaryOp::Shl,
        TokenKind::RightShift => BinaryOp::Shr,
        _ => panic!("Not a binary operator: {:?}", kind),
    }
}
