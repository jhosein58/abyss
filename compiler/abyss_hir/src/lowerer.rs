use abyss_diagnostics::Span;
use abyss_nexus::nexus::{FileId, HirAttribute, Nexus};
use abyss_parser::ast::{BinaryOp, Expr, ExprKind, Lit, Program, UnaryOp};

use crate::hir::{HirExprKind, HirProgram};

pub struct HirLowerer<'a> {
    pub hir: HirProgram,
    pub nexus: &'a mut Nexus,
    pub current_file: FileId,
}

impl<'a> HirLowerer<'a> {
    pub fn new(nexus: &'a mut Nexus, current_file: FileId) -> Self {
        Self {
            hir: HirProgram::default(),
            nexus,
            current_file,
        }
    }

    pub fn lower_program(mut self, prog: &Program) -> HirProgram {
        let root = self.lower_expr(&prog.body);
        self.hir.root = root;
        self.hir
    }

    pub fn finish(self) -> HirProgram {
        self.hir
    }

    fn push_node(&mut self, kind: HirExprKind, lhs: u32, rhs: u32, extra: u32, span: Span) -> u32 {
        let node_id = self.hir.kinds.len() as u32;
        self.hir.kinds.push(kind);
        self.hir.lhs.push(lhs);
        self.hir.rhs.push(rhs);
        self.hir.extra.push(extra);
        self.nexus.add_node_meta(span, self.current_file);
        node_id
    }

    pub fn lower_expr(&mut self, expr: &Expr) -> u32 {
        let span = expr.span_expr();
        match &expr.kind {
            ExprKind::Mod(name, body) => {
                let l = self.lower_expr(name);
                let r = self.lower_expr(body);
                self.push_node(HirExprKind::Mod, l, r, 0, span)
            }
            ExprKind::Use(module) => {
                let l = self.lower_expr(module);
                self.push_node(HirExprKind::Use, l, 0, 0, span)
            }
            ExprKind::Sequence(exprs, opt) => {
                let mut ids = Vec::with_capacity(exprs.len());
                for e in exprs {
                    ids.push(self.lower_expr(e));
                }
                let l = self.nexus.add_list_flat(&ids);
                let r = self.lower_opt(opt);
                self.push_node(HirExprKind::Sequence, l, r, 0, span)
            }
            ExprKind::Signature(args, ret, body) => {
                let mut ids = Vec::with_capacity(args.len());
                for a in args {
                    ids.push(self.lower_expr(a));
                }
                let l = self.nexus.add_list_flat(&ids);
                let r = self.lower_opt(ret);
                let e = self.lower_expr(body);
                self.push_node(HirExprKind::Signature, l, r, e, span)
            }
            ExprKind::Def(name, body) => {
                let l = self.lower_expr(name);
                let r = self.lower_expr(body);
                self.push_node(HirExprKind::Def, l, r, 0, span)
            }
            ExprKind::Ret(opt) => {
                let l = self.lower_opt(opt);
                self.push_node(HirExprKind::Ret, l, 0, 0, span)
            }
            ExprKind::Out(opt) => {
                let l = self.lower_opt(opt);
                self.push_node(HirExprKind::Out, l, 0, 0, span)
            }
            ExprKind::Continue => self.push_node(HirExprKind::Continue, 0, 0, 0, span),
            ExprKind::Block(stmts) => {
                let mut ids = Vec::with_capacity(stmts.len());
                for s in stmts {
                    ids.push(self.lower_expr(s));
                }
                let l = self.nexus.add_list_flat(&ids);
                self.push_node(HirExprKind::Block, l, 0, 0, span)
            }
            ExprKind::If(cond, then, else_opt) => {
                let l = self.lower_expr(cond);
                let r = self.lower_expr(then);
                let e = self.lower_opt(else_opt);
                self.push_node(HirExprKind::If, l, r, e, span)
            }
            ExprKind::For(pattern, range, body) => {
                let l = self.lower_expr(pattern);
                let r = self.lower_expr(range);
                let e = self.lower_expr(body);
                self.push_node(HirExprKind::For, l, r, e, span)
            }
            ExprKind::Range {
                start,
                end,
                step,
                inclusive,
            } => {
                let s = self.lower_opt(start);
                let en = self.lower_opt(end);
                let st = self.lower_opt(step);
                let inc = if *inclusive { 1 } else { 0 };
                let id = self.nexus.add_range(s, en, st, inc);
                self.push_node(HirExprKind::Range, id, 0, 0, span)
            }
            ExprKind::While(cond, body, else_opt) => {
                let l = self.lower_expr(cond);
                let r = self.lower_expr(body);
                let e = self.lower_opt(else_opt);
                self.push_node(HirExprKind::While, l, r, e, span)
            }
            ExprKind::Forever(body) => {
                let l = self.lower_expr(body);
                self.push_node(HirExprKind::Forever, l, 0, 0, span)
            }
            ExprKind::Defer(expr) => {
                let l = self.lower_expr(expr);
                self.push_node(HirExprKind::Defer, l, 0, 0, span)
            }
            ExprKind::Lit(lit) => self.lower_lit(lit, span),
            ExprKind::Ident(name) => {
                let id = self.nexus.intern_string(name);
                self.push_node(HirExprKind::Ident, id.0, 0, 0, span)
            }
            ExprKind::Binary(left, op, right) => {
                let l = self.lower_expr(left);
                let r = self.lower_expr(right);
                let kind = self.lower_binary_op(*op);
                self.push_node(kind, l, r, 0, span)
            }
            ExprKind::Unary(op, right) => {
                let r = self.lower_expr(right);
                let kind = self.lower_unary_op(*op);
                self.push_node(kind, r, 0, 0, span)
            }
            ExprKind::Call(callee, args) => {
                let l = self.lower_expr(callee);
                let mut ids = Vec::with_capacity(args.len());
                for a in args {
                    ids.push(self.lower_expr(a));
                }
                let r = self.nexus.add_list_flat(&ids);
                self.push_node(HirExprKind::Call, l, r, 0, span)
            }
            ExprKind::Index(base, index) => {
                let l = self.lower_expr(base);
                let r = self.lower_expr(index);
                self.push_node(HirExprKind::Index, l, r, 0, span)
            }
            ExprKind::Cast(expr, ty) => {
                let l = self.lower_expr(expr);
                let r = self.lower_expr(ty);
                self.push_node(HirExprKind::Cast, l, r, 0, span)
            }
            ExprKind::Is(expr, opt_ty) => {
                let l = self.lower_expr(expr);
                let r = self.lower_opt(opt_ty);
                self.push_node(HirExprKind::Is, l, r, 0, span)
            }
            ExprKind::Member(expr, name) => {
                let l = self.lower_expr(expr);
                let r = self.nexus.intern_string(name).0;
                self.push_node(HirExprKind::Member, l, r, 0, span)
            }
            ExprKind::SizeOf(opt) => {
                let l = self.lower_opt(opt);
                self.push_node(HirExprKind::SizeOf, l, 0, 0, span)
            }
            ExprKind::Match { subject, arms } => {
                let l = self.lower_expr(subject);
                let mut ids = Vec::with_capacity(arms.len());
                for arm in arms {
                    let p = self.lower_expr(&arm.pattern);
                    let b = self.lower_expr(&arm.body);
                    let arm_id = self.nexus.add_match_arm(p, b);
                    ids.push(arm_id);
                }
                let r = self.nexus.add_list_flat(&ids);
                self.push_node(HirExprKind::Match, l, r, 0, span)
            }
            ExprKind::Then(first, second) => {
                let l = self.lower_expr(first);
                let r = self.lower_expr(second);
                self.push_node(HirExprKind::Then, l, r, 0, span)
            }
            ExprKind::TypeOf(expr) => {
                let l = self.lower_expr(expr);
                self.push_node(HirExprKind::TypeOf, l, 0, 0, span)
            }
            ExprKind::Refinement(opt_cond, body) => {
                let l = self.lower_opt(opt_cond);
                let r = self.lower_expr(body);
                self.push_node(HirExprKind::Refinement, l, r, 0, span)
            }
            ExprKind::Attributed(attrs, expr) => {
                let mut attr_ids = Vec::with_capacity(attrs.len());
                for attr in attrs {
                    let name_id = self.nexus.intern_string(&attr.name);
                    let mut arg_ids = Vec::with_capacity(attr.args.len());
                    for arg in &attr.args {
                        arg_ids.push(self.nexus.intern_string(arg).0);
                    }
                    let args_start = self.nexus.add_list_flat(&arg_ids);
                    let hir_attr = HirAttribute {
                        name: name_id,
                        args_start,
                        span: attr.span.clone(),
                    };
                    attr_ids.push(self.nexus.add_attribute(hir_attr));
                }
                let l = self.nexus.add_list_flat(&attr_ids);
                let r = self.lower_expr(expr);
                self.push_node(HirExprKind::Attributed, l, r, 0, span)
            }
            ExprKind::Comptime(expr) => {
                let l = self.lower_expr(expr);
                self.push_node(HirExprKind::Comptime, l, 0, 0, span)
            }
            ExprKind::Wildcard => self.push_node(HirExprKind::Wildcard, 0, 0, 0, span),
        }
    }

    fn lower_opt(&mut self, opt: &Option<Box<Expr>>) -> u32 {
        opt.as_ref().map_or(u32::MAX, |e| self.lower_expr(e))
    }

    fn lower_lit(&mut self, lit: &Lit, span: Span) -> u32 {
        match lit {
            Lit::Str(s) => {
                let id = self.nexus.intern_string(s);
                self.push_node(HirExprKind::LitStr, id.0, 0, 0, span)
            }
            Lit::Cstr(s) => {
                let id = self.nexus.intern_string(s);
                self.push_node(HirExprKind::LitCstr, id.0, 0, 0, span)
            }
            Lit::Bool(b) => {
                let val = if *b { 1 } else { 0 };
                self.push_node(HirExprKind::LitBool, val, 0, 0, span)
            }
            Lit::Char(c) => self.push_node(HirExprKind::LitChar, *c as u32, 0, 0, span),
            Lit::Int(i) => {
                let id = self.nexus.add_int(*i);
                self.push_node(HirExprKind::LitInt, id, 0, 0, span)
            }
            Lit::Float(f) => {
                let id = self.nexus.add_float(*f);
                self.push_node(HirExprKind::LitFloat, id, 0, 0, span)
            }
        }
    }

    fn lower_binary_op(&self, op: BinaryOp) -> HirExprKind {
        match op {
            BinaryOp::Assign => HirExprKind::BinaryAssign,
            BinaryOp::AssignAdd => HirExprKind::BinaryAssignAdd,
            BinaryOp::AssignSub => HirExprKind::BinaryAssignSub,
            BinaryOp::AssignMul => HirExprKind::BinaryAssignMul,
            BinaryOp::AssignDiv => HirExprKind::BinaryAssignDiv,
            BinaryOp::AssignMod => HirExprKind::BinaryAssignMod,
            BinaryOp::AssignBitAnd => HirExprKind::BinaryAssignBitAnd,
            BinaryOp::AssignBitOr => HirExprKind::BinaryAssignBitOr,
            BinaryOp::AssignBitXor => HirExprKind::BinaryAssignBitXor,
            BinaryOp::AssignShl => HirExprKind::BinaryAssignShl,
            BinaryOp::AssignShr => HirExprKind::BinaryAssignShr,
            BinaryOp::Add => HirExprKind::BinaryAdd,
            BinaryOp::Sub => HirExprKind::BinarySub,
            BinaryOp::Mul => HirExprKind::BinaryMul,
            BinaryOp::Div => HirExprKind::BinaryDiv,
            BinaryOp::Mod => HirExprKind::BinaryMod,
            BinaryOp::Eq => HirExprKind::BinaryEq,
            BinaryOp::Neq => HirExprKind::BinaryNeq,
            BinaryOp::Lt => HirExprKind::BinaryLt,
            BinaryOp::Gt => HirExprKind::BinaryGt,
            BinaryOp::Lte => HirExprKind::BinaryLte,
            BinaryOp::Gte => HirExprKind::BinaryGte,
            BinaryOp::And => HirExprKind::BinaryAnd,
            BinaryOp::Or => HirExprKind::BinaryOr,
            BinaryOp::BitAnd => HirExprKind::BinaryBitAnd,
            BinaryOp::Pipe => HirExprKind::BinaryPipe,
            BinaryOp::BitXor => HirExprKind::BinaryBitXor,
            BinaryOp::Shl => HirExprKind::BinaryShl,
            BinaryOp::Shr => HirExprKind::BinaryShr,
            BinaryOp::KeyValue => HirExprKind::BinaryCollon,
            BinaryOp::ConstDef => HirExprKind::BinaryConstDef,
        }
    }

    fn lower_unary_op(&self, op: UnaryOp) -> HirExprKind {
        match op {
            UnaryOp::Neg => HirExprKind::UnaryNeg,
            UnaryOp::Not => HirExprKind::UnaryNot,
            UnaryOp::BitNot => HirExprKind::UnaryBitNot,
            UnaryOp::Deref => HirExprKind::UnaryDeref,
            UnaryOp::AddrOf => HirExprKind::UnaryAddrOf,
        }
    }
}
