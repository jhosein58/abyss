use abyss_lexer::token::TokenKind as Tk;

use crate::{
    ast::{Expr, ExprKind},
    error::ParseError,
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_call(eng: &mut PrattEngine, left: Expr) -> Result<Expr, ParseError> {
    eng.advance();

    let mut args = Vec::new();

    while eng.current_token().kind != Tk::CParen {
        args.push(eng.parse_expression_bp(Precedence::None)?);

        if eng.current_token().kind == Tk::Comma {
            eng.advance();
        } else {
            break;
        }
    }

    eng.expect(Tk::CParen)?;

    Ok(eng.new_expr(ExprKind::Call(Box::new(left), args)))
}

pub fn parse_member(eng: &mut PrattEngine, left: Expr) -> Result<Expr, ParseError> {
    eng.advance();
    let token = eng.current_token();
    if token.kind != Tk::Ident {
        return Err(ParseError::msg("Identifier after '.'"));
    }

    let member_name = token.text.to_string();
    eng.advance();

    Ok(eng.new_expr(ExprKind::Member(Box::new(left), member_name)))
}
