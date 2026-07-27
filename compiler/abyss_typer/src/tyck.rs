use std::vec;

use abyss_hir::{
    hir::{HirExprKind, HirTable},
    visitor::HirVisitor,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Types {
    I32,
    Unknown,
}

pub fn check(hir: &HirTable) -> Vec<Types> {
    let visitor = HirVisitor::new(hir);

    let mut types = vec![Types::Unknown; hir.kinds.len()];

    for (id, node) in hir.kinds.iter().enumerate() {
        types[id] = match node {
            // Literals
            HirExprKind::LitInt => Types::I32,

            // Binary operators
            HirExprKind::BinaryAdd => {
                let lhs_ty = types[visitor.lhs(id)];
                let rhs_ty = types[visitor.rhs(id)];

                assert_eq!(lhs_ty, rhs_ty);

                lhs_ty
            }

            _ => Types::Unknown,
        };
    }

    types
}
