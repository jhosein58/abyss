use abyss_hir::hir::HirExprKind;

use crate::{
    nexus::Nexus,
    storages::{
        hir::storage::HirStorage,
        interner::storage::NameId,
        literals::storage::{FloatId, IntId},
    },
};

impl HirStorage {
    pub fn print_dump(&self, nexus: &Nexus) {
        println!("{:-<110}", "");
        println!(
            "{:<5} | {:<22} | {:<25} | {:<25} | {:<20}",
            "ID", "Kind", "LHS (Left/Data)", "RHS (Right/Data)", "Extra"
        );
        println!("{:-<110}", "");

        for i in 0..self.table.kinds.len() {
            let kind = self.table.kinds[i];
            let lhs = self.table.lhs[i];
            let rhs = self.table.rhs[i];
            let extra = self.table.extra[i];

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
                HirExprKind::LitInt => {
                    lhs_str = format!("Int({})", nexus.literals.get_int(IntId(lhs)))
                }
                HirExprKind::LitFloat => {
                    lhs_str = format!("Float({})", nexus.literals.get_float(FloatId(lhs)))
                }
                HirExprKind::LitBool => lhs_str = format!("Bool({})", lhs == 1),
                HirExprKind::LitChar => {
                    if let Some(c) = std::char::from_u32(lhs) {
                        lhs_str = format!("Char('{}')", c);
                    }
                }

                HirExprKind::LitStr
                | HirExprKind::LitCstr
                | HirExprKind::Ident
                | HirExprKind::Member => {
                    let text = nexus.interner.get(NameId(lhs)).unwrap_or("<unknown>");
                    let display_text: String = if text.len() > 15 {
                        format!("{}...", &text[..15])
                    } else {
                        text.to_string()
                    };
                    lhs_str = format!("\"{}\"", display_text);
                }

                HirExprKind::Sequence
                | HirExprKind::Signature
                | HirExprKind::Block
                | HirExprKind::Attributed => {
                    if lhs != u32::MAX {
                        let start = lhs as usize;
                        let len = nexus.u32_items[start] as usize;
                        let items = &nexus.u32_items[(start + 1)..(start + 1 + len)];
                        lhs_str = format!("Nodes{:?}", items);
                    }
                }

                HirExprKind::Call | HirExprKind::Match => {
                    if rhs != u32::MAX {
                        let start = rhs as usize;
                        let len = nexus.u32_items[start] as usize;
                        let items = &nexus.u32_items[(start + 1)..(start + 1 + len)];
                        rhs_str = format!("Nodes{:?}", items);
                    }
                }

                HirExprKind::Range => {
                    if lhs != u32::MAX {
                        let (start, end, step, inc) = nexus.ranges[lhs as usize];
                        lhs_str = format!(
                            "Range(s:{}, e:{}, st:{}, inc:{})",
                            format_idx(start),
                            format_idx(end),
                            format_idx(step),
                            inc
                        );
                    }
                }

                _ => {}
            }

            println!(
                "{:<5} | {:<22} | {:<25} | {:<25} | {:<20}",
                i,
                format!("{:?}", kind),
                lhs_str,
                rhs_str,
                ext_str
            );
        }
        println!("{:-<110}", "");
    }
}
