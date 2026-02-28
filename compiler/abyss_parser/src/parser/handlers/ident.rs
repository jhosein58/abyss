use crate::{
    ast::{Expr, ExprKind},
    parser::PrattEngine,
};

pub fn parse_ident(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let tk = eng.get_and_bump();
    let res = Ok(Expr {
        kind: ExprKind::Ident(tk.text.to_string()),
        span: tk.span(eng.file_id),
        id: eng.next_id(),
    });

    res
}
