use crate::{
    ast::{Expr, ExprKind},
    parser::PrattEngine,
};

pub fn parse_ident(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let tk = eng.get_and_bump();
    let span = tk.span(eng.file_id);
    let id = eng.next_id();

    if "_" == tk.text {
        return Ok(Expr {
            kind: ExprKind::Wildcard,
            span,
            id,
        });
    }

    Ok(Expr {
        kind: ExprKind::Ident(tk.text.to_string()),
        span,
        id,
    })
}
