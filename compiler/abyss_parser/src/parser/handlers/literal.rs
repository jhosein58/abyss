use abyss_lexer::token::{Token, TokenKind as Tk};

use crate::{
    ast::{Expr, ExprKind, Lit, OrderedFloat},
    error::ParseError,
    parser::engine::PrattEngine,
};

pub fn parse_literal(eng: &mut PrattEngine) -> Result<Expr, ParseError> {
    let tk = eng.current_token();

    let expr = match tk.kind {
        Tk::IntLit => parse_int(eng, &tk),
        Tk::FloatLit => parse_float(eng, &tk),
        Tk::StrLit => parse_str(eng, &tk),
        Tk::CStrLit => parse_cstr(eng, &tk),
        Tk::CharLit => parse_char(eng, &tk),
        Tk::BinIntLit => parse_bin(eng, &tk),
        Tk::HexIntLit => parse_hex(eng, &tk),
        Tk::True => eng.new_expr(ExprKind::Lit(Lit::Bool(true))),
        Tk::False => eng.new_expr(ExprKind::Lit(Lit::Bool(false))),
        _ => panic!("Expected literal token, found: {:?}", tk.kind),
    };

    eng.advance();
    Ok(expr)
}

fn parse_int(eng: &mut PrattEngine, tk: &Token<'_>) -> Expr {
    eng.new_expr(ExprKind::Lit(Lit::Int(tk.text.parse().unwrap())))
}

fn parse_float(eng: &mut PrattEngine, tk: &Token<'_>) -> Expr {
    eng.new_expr(ExprKind::Lit(Lit::Float(OrderedFloat(
        tk.text.parse().unwrap(),
    ))))
}

fn parse_bin(eng: &mut PrattEngine, tk: &Token<'_>) -> Expr {
    let text_without_prefix = &tk.text[2..];
    let val = i64::from_str_radix(text_without_prefix, 2).expect("Invalid binary literal");

    eng.new_expr(ExprKind::Lit(Lit::Int(val)))
}

fn parse_hex(eng: &mut PrattEngine, tk: &Token<'_>) -> Expr {
    let text_without_prefix = &tk.text[2..];
    let val = i64::from_str_radix(text_without_prefix, 16).expect("Invalid hex literal");

    eng.new_expr(ExprKind::Lit(Lit::Int(val)))
}

fn parse_str(eng: &mut PrattEngine, tk: &Token<'_>) -> Expr {
    let text = &tk.text;
    let val = text[1..text.len() - 1].to_string();

    eng.new_expr(ExprKind::Lit(Lit::Str(val)))
}

fn parse_cstr(eng: &mut PrattEngine, tk: &Token<'_>) -> Expr {
    let text = &tk.text;
    let val = text[2..text.len() - 1].to_string();

    eng.new_expr(ExprKind::Lit(Lit::Cstr(val)))
}

fn parse_char(eng: &mut PrattEngine, tk: &Token<'_>) -> Expr {
    let text = &tk.text;
    let inner_text = &text[1..text.len() - 1];

    let val = inner_text.chars().next().expect("Empty char literal");

    eng.new_expr(ExprKind::Lit(Lit::Char(val)))
}
