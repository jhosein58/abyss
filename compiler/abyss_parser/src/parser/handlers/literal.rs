use crate::{
    ast::{Expr, ExprKind},
    error::ParseError,
    parser::engine::PrattEngine,
};

pub fn parse_literal(eng: &mut PrattEngine) -> Result<Expr, ParseError> {
    let tk = eng.current();
    let res = Ok(eng.new_expr(ExprKind::Ident(vec![tk.text.to_string()])));
    eng.advance();
    res
}
