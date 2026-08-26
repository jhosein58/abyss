use abyss_hir::hir::HirExprKind;

use crate::{
    arena::ArenaId,
    nexus::{FloatId, HirId, IntId, NameId, Nexus, TypeId},
};

impl Nexus {
    pub fn dump_hir(&mut self) {
        println!("{:-<135}", "");
        println!(
            "{:<5} | {:<22} | {:<25} | {:<25} | {:<20} | {:<20}",
            "ID", "Kind", "LHS (Left/Data)", "RHS (Right/Data)", "Extra", "Type"
        );
        println!("{:-<135}", "");

        for i in 0..self.hir.table.kinds.len() {
            let kind = self.hir.table.kinds[i];
            let lhs = self.hir.table.lhs[i];
            let rhs = self.hir.table.rhs[i];
            let extra = self.hir.table.extra[i];

            let slot = self.unify.get_slot(HirId(i as u32));

            let tyid = if slot.is_some() {
                self.unify.resolve_type(slot)
            } else {
                TypeId::none()
            };

            let type_name = self.types.name(tyid);

            let format_idx = |idx: u32| {
                if idx == u32::MAX {
                    "None".to_string()
                } else {
                    idx.to_string()
                }
            };

            let mut lhs_str = format_idx(lhs);
            let mut rhs_str = format_idx(rhs);
            let ext_str = format_idx(extra);

            match kind {
                HirExprKind::LitInt => lhs_str = format!("Int({})", self.ints.get(IntId(lhs))),
                HirExprKind::LitFloat => {
                    lhs_str = format!("Float({})", self.floats.get(FloatId(lhs)))
                }
                HirExprKind::LitBoolTrue => lhs_str = format!("Bool(True)"),
                HirExprKind::LitBoolFalse => lhs_str = format!("Bool(False)"),
                HirExprKind::LitChar => {
                    if let Some(c) = std::char::from_u32(lhs) {
                        lhs_str = format!("Char('{}')", c);
                    }
                }

                HirExprKind::LitStr
                | HirExprKind::LitCstr
                | HirExprKind::Ident
                | HirExprKind::Member => {
                    let text = self.interner.get(NameId(lhs));
                    let display_text: String = if text.len() > 15 {
                        format!("{}...", &text[..15])
                    } else {
                        text.to_string()
                    };
                    lhs_str = format!("\"{}\"", display_text);
                }

                HirExprKind::Function | HirExprKind::Block | HirExprKind::Attributed => {
                    if lhs != u32::MAX {
                        let start = lhs as usize;
                        let len = self.u32_items[start] as usize;
                        let items = &self.u32_items[(start + 1)..(start + 1 + len)];
                        lhs_str = format!("Nodes{:?}", items);
                    }
                }

                HirExprKind::Struct | HirExprKind::StructInit => {
                    if lhs != u32::MAX {
                        let start = lhs as usize;
                        let len = self.u32_items[start] as usize;
                        let items = &self.u32_items[(start + 1)..(start + 1 + len)];
                        lhs_str = format!("Nodes{:?}", items);
                    }

                    if rhs != u32::MAX {
                        let start = rhs as usize;
                        let len = self.u32_items[start] as usize;
                        let items = &self.u32_items[(start + 1)..(start + 1 + len)];
                        rhs_str = format!("Nodes{:?}", items);
                    }
                }
                HirExprKind::Call | HirExprKind::Match => {
                    if rhs != u32::MAX {
                        let start = rhs as usize;
                        let len = self.u32_items[start] as usize;
                        let items = &self.u32_items[(start + 1)..(start + 1 + len)];
                        rhs_str = format!("Nodes{:?}", items);
                    }
                }

                _ => {}
            }

            println!(
                "{:<5} | {:<22} | {:<25} | {:<25} | {:<20} | {:<20}",
                i,
                format!("{:?}", kind),
                lhs_str,
                rhs_str,
                ext_str,
                type_name
            );
        }
        println!("{:-<135}", "");
    }
}
