use abyss_nexus::nexus::HirId;

use crate::{
    binding_power::{BindingPower, is_soft},
    parser::Parser,
};

impl Parser<'_> {
    pub fn parse_expr(&mut self, min_bp: u8) -> HirId {
        let mut lhs = self.dispatch_prefix();

        while let Some(tk) = self.peek() {
            if self.peek_preceded_by_newline() && !is_soft(tk) {
                break;
            }

            let Some(bp) = BindingPower::from_infix(tk) else {
                break;
            };

            if bp.left < min_bp {
                break;
            }

            self.bump();

            lhs = self.dispatch_infix(tk, lhs, bp.right);
        }
        lhs
    }
}
