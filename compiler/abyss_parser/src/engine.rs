use abyss_nexus::nexus::{HirId, ScopeId};

use crate::{
    binding_power::{BindingPower, is_soft},
    dispatch,
    parser::Parser,
};

pub fn parse_expr(p: &mut Parser, min_bp: u8, parent: ScopeId) -> HirId {
    let mut lhs = dispatch::prefix(p, parent);

    while let Some(tk) = p.peek() {
        if p.peek_preceded_by_newline() && !is_soft(tk) {
            break;
        }

        let Some(bp) = BindingPower::from_infix(tk) else {
            break;
        };
        if bp.left < min_bp {
            break;
        }
        p.bump();

        lhs = dispatch::infix(p, parent, tk, lhs, bp.right);
    }
    lhs
}
