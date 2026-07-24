use abyss_diagnostics::Span;
use abyss_nexus::nexus::{FileId, Nexus, StringId};
use abyss_parser::ast::OrderedFloat;

use crate::hir::{HirExprKind, HirProgram};

#[derive(Clone, Copy)]
pub struct HirCursor<'a> {
    pub id: u32,
    pub program: &'a HirProgram,
    pub nexus: &'a Nexus,
}

pub struct RangeView<'a> {
    pub start: Option<HirCursor<'a>>,
    pub end: Option<HirCursor<'a>>,
    pub step: Option<HirCursor<'a>>,
    pub inclusive: bool,
}

pub struct MatchArmView<'a> {
    pub pattern: HirCursor<'a>,
    pub body: HirCursor<'a>,
}

pub struct AttributeView<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
    pub span: Span,
}

impl<'a> HirCursor<'a> {
    #[inline(always)]
    pub fn new(id: u32, program: &'a HirProgram, nexus: &'a Nexus) -> Self {
        Self { id, program, nexus }
    }

    #[inline(always)]
    pub fn kind(&self) -> HirExprKind {
        self.program.kinds[self.id as usize]
    }

    #[inline(always)]
    pub fn span(&self) -> Span {
        self.nexus.get_node_span(self.id)
    }

    #[inline(always)]
    pub fn file_id(&self) -> FileId {
        self.nexus.get_node_file(self.id)
    }

    #[inline(always)]
    pub fn lhs_id(&self) -> u32 {
        self.program.lhs[self.id as usize]
    }

    #[inline(always)]
    pub fn rhs_id(&self) -> u32 {
        self.program.rhs[self.id as usize]
    }

    #[inline(always)]
    pub fn extra_id(&self) -> u32 {
        self.program.extra[self.id as usize]
    }

    #[inline(always)]
    pub fn left(&self) -> Option<HirCursor<'a>> {
        let id = self.lhs_id();
        if id != u32::MAX {
            Some(Self::new(id, self.program, self.nexus))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn right(&self) -> Option<HirCursor<'a>> {
        let id = self.rhs_id();
        if id != u32::MAX {
            Some(Self::new(id, self.program, self.nexus))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn extra(&self) -> Option<HirCursor<'a>> {
        let id = self.extra_id();
        if id != u32::MAX {
            Some(Self::new(id, self.program, self.nexus))
        } else {
            None
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        if self.kind() == HirExprKind::LitInt {
            Some(self.nexus.ints[self.lhs_id() as usize])
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<OrderedFloat> {
        if self.kind() == HirExprKind::LitFloat {
            Some(self.nexus.floats[self.lhs_id() as usize])
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if self.kind() == HirExprKind::LitBool {
            Some(self.lhs_id() == 1)
        } else {
            None
        }
    }

    pub fn as_char(&self) -> Option<char> {
        if self.kind() == HirExprKind::LitChar {
            std::char::from_u32(self.lhs_id())
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&'a str> {
        match self.kind() {
            HirExprKind::LitStr
            | HirExprKind::LitCstr
            | HirExprKind::Ident
            | HirExprKind::Member => Some(self.nexus.get_string(StringId(self.lhs_id()))),
            _ => None,
        }
    }

    fn read_list(&self, start: u32) -> impl Iterator<Item = HirCursor<'a>> + 'a {
        let (items, prog, nex) = if start == u32::MAX {
            (&[][..], self.program, self.nexus)
        } else {
            let start = start as usize;
            let len = self.nexus.u32_items[start] as usize;
            (
                &self.nexus.u32_items[(start + 1)..(start + 1 + len)],
                self.program,
                self.nexus,
            )
        };
        items.iter().map(move |&id| HirCursor::new(id, prog, nex))
    }

    pub fn list_items_lhs(&self) -> impl Iterator<Item = HirCursor<'a>> + 'a {
        self.read_list(self.lhs_id())
    }

    pub fn list_items_rhs(&self) -> impl Iterator<Item = HirCursor<'a>> + 'a {
        self.read_list(self.rhs_id())
    }

    pub fn as_range(&self) -> Option<RangeView<'a>> {
        if self.kind() != HirExprKind::Range {
            return None;
        }
        let (start, end, step, inc) = self.nexus.ranges[self.lhs_id() as usize];

        let to_cursor = |id: u32| {
            if id != u32::MAX {
                Some(HirCursor::new(id, self.program, self.nexus))
            } else {
                None
            }
        };

        Some(RangeView {
            start: to_cursor(start),
            end: to_cursor(end),
            step: to_cursor(step),
            inclusive: inc == 1,
        })
    }

    pub fn as_match_arms(&'a self) -> Option<impl Iterator<Item = MatchArmView<'a>> + 'a> {
        if self.kind() != HirExprKind::Match {
            return None;
        }

        let iter = self.list_items_rhs().map(move |arm_idx_cursor| {
            let (pat, body) = self.nexus.match_arms[arm_idx_cursor.id as usize];
            MatchArmView {
                pattern: HirCursor::new(pat, self.program, self.nexus),
                body: HirCursor::new(body, self.program, self.nexus),
            }
        });

        Some(iter)
    }

    pub fn as_attributes(&'a self) -> Option<impl Iterator<Item = AttributeView<'a>> + 'a> {
        if self.kind() != HirExprKind::Attributed {
            return None;
        }

        let iter = self.list_items_lhs().map(move |attr_idx_cursor| {
            let attr = &self.nexus.attributes[attr_idx_cursor.id as usize];
            let name = self.nexus.get_string(attr.name);

            let args_start = attr.args_start;
            let args = if args_start == u32::MAX {
                Vec::new()
            } else {
                let start = args_start as usize;
                let len = self.nexus.u32_items[start] as usize;
                self.nexus.u32_items[(start + 1)..(start + 1 + len)]
                    .iter()
                    .map(|&s_id| self.nexus.get_string(StringId(s_id)))
                    .collect()
            };

            AttributeView {
                name,
                args,
                span: attr.span.clone(),
            }
        });

        Some(iter)
    }

    pub fn binary_operands(&self) -> (HirCursor<'a>, HirCursor<'a>) {
        (self.left().unwrap(), self.right().unwrap())
    }

    pub fn unary_operand(&self) -> HirCursor<'a> {
        self.left().unwrap()
    }

    pub fn if_branches(&self) -> (HirCursor<'a>, HirCursor<'a>, Option<HirCursor<'a>>) {
        (self.left().unwrap(), self.right().unwrap(), self.extra())
    }

    pub fn call_components(&self) -> (HirCursor<'a>, impl Iterator<Item = HirCursor<'a>> + 'a) {
        (self.left().unwrap(), self.list_items_rhs())
    }
}
