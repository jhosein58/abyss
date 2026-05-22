use abyss_nexus::nexus::{Nexus, StringId};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirExprKind {
    LitInt,
    LitFloat,
    LitBool,
    LitStr,
    LitCstr,
    LitChar,
    Ident,
    BinaryAssign,
    BinaryAssignAdd,
    BinaryAssignSub,
    BinaryAssignMul,
    BinaryAssignDiv,
    BinaryAssignMod,
    BinaryAssignBitAnd,
    BinaryAssignBitOr,
    BinaryAssignBitXor,
    BinaryAssignShl,
    BinaryAssignShr,
    BinaryAdd,
    BinarySub,
    BinaryMul,
    BinaryDiv,
    BinaryMod,
    BinaryEq,
    BinaryNeq,
    BinaryLt,
    BinaryGt,
    BinaryLte,
    BinaryGte,
    BinaryAnd,
    BinaryOr,
    BinaryBitAnd,
    BinaryPipe,
    BinaryBitXor,
    BinaryShl,
    BinaryShr,
    BinaryCollon,
    BinaryConstDef,
    UnaryNeg,
    UnaryNot,
    UnaryBitNot,
    UnaryDeref,
    UnaryAddrOf,
    Mod,
    Use,
    Sequence,
    Signature,
    Def,
    Ret,
    Out,
    Continue,
    Block,
    If,
    For,
    Range,
    While,
    Forever,
    Defer,
    Call,
    Index,
    Cast,
    Is,
    Member,
    SizeOf,
    Match,
    Then,
    TypeOf,
    Refinement,
    Attributed,
    Comptime,
    Wildcard,
}

#[derive(Default)]
pub struct HirProgram {
    pub kinds: Vec<HirExprKind>,
    pub lhs: Vec<u32>,
    pub rhs: Vec<u32>,
    pub extra: Vec<u32>,
}

impl HirProgram {
    pub fn print_dump(&self, nexus: &Nexus) {
        println!("{:-<110}", "");
        println!(
            "{:<5} | {:<22} | {:<25} | {:<25} | {:<20}",
            "ID", "Kind", "LHS (Left/Data)", "RHS (Right/Data)", "Extra"
        );
        println!("{:-<110}", "");

        for i in 0..self.kinds.len() {
            let kind = self.kinds[i];
            let lhs = self.lhs[i];
            let rhs = self.rhs[i];
            let extra = self.extra[i];

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
                HirExprKind::LitInt => lhs_str = format!("Int({})", nexus.ints[lhs as usize]),
                HirExprKind::LitFloat => {
                    lhs_str = format!("Float({})", nexus.floats[lhs as usize].0)
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
                    let text = nexus.get_string(StringId(lhs));
                    let display_text = if text.len() > 15 {
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
