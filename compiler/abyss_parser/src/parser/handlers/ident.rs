use crate::{
    ast::{Expr, ExprKind},
    parser::PrattEngine,
};

pub fn parse_ident(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let tk = eng.current_token();
    let res = Ok(eng.new_expr(ExprKind::Ident(tk.text.to_string())));
    eng.advance();
    res
}
