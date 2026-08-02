use abyss_nexus::storages::hir::storage::HirId;

use crate::{binding_power::BindingPower, dispatch, parser::Parser};

pub fn parse_expr(p: &mut Parser, min_bp: u8) -> HirId {
    let mut lhs = dispatch::prefix(p);

    while let Some(tk) = p.peek() {
        let Some(bp) = BindingPower::from_infix(tk) else {
            break;
        };
        if bp.left < min_bp {
            break;
        }
        p.bump();

        lhs = dispatch::infix(p, tk, lhs, bp.right);
    }
    lhs
}
