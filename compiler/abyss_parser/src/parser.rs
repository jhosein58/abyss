use abyss_hir::hir::HirExprKind;
use abyss_nexus::{
    nexus::Nexus,
    storages::{hir::storage::HirId, tokens::storage::TokenId},
};
use abyss_token::kind::TokenKind;

use crate::binding_power::BindingPower;

#[derive(Debug, Clone, Copy)]
pub struct Frame {
    pub lhs: HirId,
    pub op_tk: TokenKind,
    pub right_bp: u8,
}

pub fn parse(db: &mut Nexus, min_bp: u8, mut cursor: u32, end: u32) -> HirId {
    let mut stack: Vec<Frame> = Vec::with_capacity(16);

    let mut current_lhs = parse_prefix(db, &mut cursor);

    loop {
        if cursor >= end {
            break;
        }

        let op_tk = db.tokens.kind(TokenId(cursor));

        let bp = match BindingPower::from_infix(op_tk) {
            Some(bp) => bp,
            None => break,
        };

        if bp.left < min_bp {
            break;
        }
        cursor += 1;

        stack.push(Frame {
            lhs: current_lhs,
            op_tk,
            right_bp: bp.right,
        });

        current_lhs = parse_prefix(db, &mut cursor);

        while let Some(top) = stack.last() {
            if cursor < end {
                let next_tk = db.tokens.kind(TokenId(cursor));
                if let Some(next_bp) = BindingPower::from_infix(next_tk) {
                    if next_bp.left > top.right_bp {
                        break;
                    }
                }
            }

            let frame = stack.pop().unwrap();
            current_lhs = db
                .hir
                .alloc_binary(HirExprKind::BinaryAdd, frame.lhs, current_lhs);
        }
    }

    current_lhs
}

fn parse_prefix(db: &mut Nexus, cursor: &mut u32) -> HirId {
    let tk = db.tokens.kind(TokenId(*cursor));

    match tk {
        TokenKind::IntLit => {
            let id = TokenId(*cursor);
            *cursor += 1;

            let value = db.tokens.text(id).trim().parse().unwrap(); // FIXME: parse error

            db.hir.alloc_int(db.literals.intern_int(value))
        }

        _ => {
            *cursor += 1;
            HirId(0)
        }
    }
}
