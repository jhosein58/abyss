use abyss_parser::ast::{Expr, FunctionDef, StaticDef, Stmt, StructDef, TypeAlias, UnionDef};

use crate::new_type_checker::context::TypeContext;

pub trait AstVisitor {
    fn visit_stmt(&mut self, _stmt: &mut Stmt, _ctx: &mut TypeContext) {}
    fn visit_expr(&mut self, _expr: &mut Expr, _ctx: &mut TypeContext) {}
    fn visit_function_def(&mut self, _func: &mut FunctionDef, _ctx: &mut TypeContext) {}
    fn visit_struct_def(&mut self, _def: &mut StructDef, _ctx: &mut TypeContext) {}
    fn visit_static_def(&mut self, _def: &mut StaticDef, _ctx: &mut TypeContext) {}
    fn visit_union_def(&mut self, _def: &mut UnionDef, _ctx: &mut TypeContext) {}
    fn visit_type_alias(&mut self, _def: &mut TypeAlias, _ctx: &mut TypeContext) {}
    fn init(&mut self, _ctx: &mut TypeContext) {}
}
