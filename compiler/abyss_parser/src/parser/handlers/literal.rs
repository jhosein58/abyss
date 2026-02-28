use abyss_lexer::token::{Token, TokenKind as Tk};

use crate::{
    ast::{Expr, ExprKind, Lit, OrderedFloat},
    parser::engine::PrattEngine,
};

pub fn parse_literal(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let tk = eng.current_token();

    let expr = match tk.kind {
        Tk::IntLit => parse_int(eng, &tk)?,
        Tk::FloatLit => parse_float(eng, &tk)?,
        Tk::StrLit => parse_str(eng, &tk)?,
        Tk::CStrLit => parse_cstr(eng, &tk)?,
        Tk::CharLit => parse_char(eng, &tk)?,
        Tk::BinIntLit => parse_bin(eng, &tk)?,
        Tk::HexIntLit => parse_hex(eng, &tk)?,
        Tk::True => Expr {
            kind: ExprKind::Lit(Lit::Bool(true)),
            span: tk.span(eng.file_id),
            id: eng.next_id(),
        },
        Tk::False => Expr {
            kind: ExprKind::Lit(Lit::Bool(false)),
            span: tk.span(eng.file_id),
            id: eng.next_id(),
        },
        _ => {
            eng.report_error(
                tk.span(eng.file_id),
                format!("Unexpected token in literal parsing: {:?}", tk.kind),
            );
            return Err(());
        }
    };

    eng.advance();
    Ok(expr)
}

fn parse_int(eng: &mut PrattEngine, tk: &Token<'_>) -> Result<Expr, ()> {
    match tk.text.replace('_', "").parse() {
        Ok(val) => Ok(Expr {
            kind: ExprKind::Lit(Lit::Int(val)),
            span: tk.span(eng.file_id),
            id: eng.next_id(),
        }),
        Err(_) => {
            eng.report_error(
                tk.span(eng.file_id),
                "Integer literal is too large or invalid".to_string(),
            );
            Err(())
        }
    }
}

fn parse_float(eng: &mut PrattEngine, tk: &Token<'_>) -> Result<Expr, ()> {
    match tk.text.parse() {
        Ok(val) => Ok(Expr {
            kind: ExprKind::Lit(Lit::Float(OrderedFloat(val))),
            span: tk.span(eng.file_id),
            id: eng.next_id(),
        }),
        Err(_) => {
            eng.report_error(tk.span(eng.file_id), "Invalid float literal".to_string());
            Err(())
        }
    }
}

fn parse_bin(eng: &mut PrattEngine, tk: &Token<'_>) -> Result<Expr, ()> {
    let text_without_prefix = &tk.text[2..];
    match i64::from_str_radix(text_without_prefix, 2) {
        Ok(val) => Ok(Expr {
            kind: ExprKind::Lit(Lit::Int(val)),
            span: tk.span(eng.file_id),
            id: eng.next_id(),
        }),
        Err(_) => {
            eng.report_error(tk.span(eng.file_id), "Invalid binary literal".to_string());
            Err(())
        }
    }
}

fn parse_hex(eng: &mut PrattEngine, tk: &Token<'_>) -> Result<Expr, ()> {
    let text_without_prefix = &tk.text[2..];
    match i64::from_str_radix(text_without_prefix, 16) {
        Ok(val) => Ok(Expr {
            kind: ExprKind::Lit(Lit::Int(val)),
            span: tk.span(eng.file_id),
            id: eng.next_id(),
        }),
        Err(_) => {
            eng.report_error(
                tk.span(eng.file_id),
                "Invalid hexadecimal literal".to_string(),
            );
            Err(())
        }
    }
}

fn parse_str(eng: &mut PrattEngine, tk: &Token<'_>) -> Result<Expr, ()> {
    if tk.text.len() < 2 {
        eng.report_error(
            tk.span(eng.file_id),
            "Unterminated string literal".to_string(),
        );
        return Err(());
    }
    let text = &tk.text;
    let val = text[1..text.len() - 1].to_string();
    Ok(Expr {
        kind: ExprKind::Lit(Lit::Str(val)),
        span: tk.span(eng.file_id),
        id: eng.next_id(),
    })
}

fn parse_cstr(eng: &mut PrattEngine, tk: &Token<'_>) -> Result<Expr, ()> {
    if tk.text.len() < 3 {
        eng.report_error(
            tk.span(eng.file_id),
            "Unterminated C-string literal".to_string(),
        );
        return Err(());
    }
    let text = &tk.text;
    let val = text[2..text.len() - 1].to_string();
    Ok(Expr {
        kind: ExprKind::Lit(Lit::Cstr(val)),
        span: tk.span(eng.file_id),
        id: eng.next_id(),
    })
}

fn parse_char(eng: &mut PrattEngine, tk: &Token<'_>) -> Result<Expr, ()> {
    let text = &tk.text;
    if text.len() < 3 {
        eng.report_error(tk.span(eng.file_id), "Invalid char literal".to_string());
        return Err(());
    }
    let inner_text = &text[1..text.len() - 1];

    match inner_text.chars().next() {
        Some(val) => Ok(Expr {
            kind: ExprKind::Lit(Lit::Char(val)),
            span: tk.span(eng.file_id),
            id: eng.next_id(),
        }),
        None => {
            eng.report_error(tk.span(eng.file_id), "Empty char literal".to_string());
            Err(())
        }
    }
}
