use abyss_lexer::token::TokenKind as Tk;

use crate::{
    ast::{Expr, ExprKind},
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_group_or_signature(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let tk = eng.get_and_bump();

    let mut args = Vec::new();
    let mut has_trailing_comma = false;

    while eng.current_token().kind != Tk::CParen {
        args.push(eng.parse_expression_bp(Precedence::None)?);

        if eng.current_token().kind == Tk::Comma {
            eng.advance();
            if eng.current_token().kind == Tk::CParen {
                has_trailing_comma = true;
                break;
            }
        } else {
            break;
        }
    }

    eng.expect(Tk::CParen)?;

    let next_tk = eng.current_token().kind;

    let is_signature = args.is_empty()
        || args.len() > 1
        || has_trailing_comma
        || next_tk == Tk::Colon
        || next_tk == Tk::OBrace;

    if is_signature {
        let mut ret_type = None;

        if eng.current_token().kind == Tk::Colon {
            eng.advance();
            ret_type = Some(Box::new(eng.parse_expression_bp(Precedence::None)?));
        }

        let sig_eng_span = eng.current_span();

        let body = eng.parse_expression_bp(Precedence::None)?;

        return Ok(Expr {
            kind: ExprKind::Signature(args, ret_type, Box::new(body)),
            span: tk.span(eng.file_id).merge(sig_eng_span),
            id: eng.next_id(),
        });
    }

    Ok(args.into_iter().next().unwrap())
}
