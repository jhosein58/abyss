use abyss_diagnostics::Span;
use abyss_parser::ast::{BinaryOp, Expr, ExprKind};

use crate::type_checker::{
    engine::{TypeChecker, error_expr},
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

pub fn check_sequence(
    tc: &mut TypeChecker,
    items: &Vec<Expr>,
    count: &Option<Box<Expr>>,
    span: Span,
    id: u32,
) -> TypedExpr {
    let (kind, seq_items) = if let Some(v) = get_kind(tc, items, count) {
        v
    } else {
        return error_expr(span, id);
    };

    match kind {
        SeqKind::Arr => {
            let mut new_items = Vec::new();

            for i in seq_items.iter() {
                if let SeqItem::Lit(val) = i {
                    new_items.push(val.clone());
                } else {
                    tc.report_error(
                        (if let SeqItem::KeyVal(_, d) = i {
                            d
                        } else {
                            unreachable!()
                        })
                        .span_expr(),
                        format!("dont use Key: Value in Arrays."),
                    );
                    return error_expr(span, id);
                }
            }

            let count = tc.check_expr(&count.clone().unwrap());

            let ty = if let Some(ft) = new_items.first() {
                ft.ty.clone()
            } else {
                Type::Unit
            };

            return TypedExpr {
                kind: TypedExprKind::ArrayInit(new_items, Some(Box::new(count.clone()))),
                ty: Type::Array(Box::new(ty), Box::new(count)),
                span,
                id,
            };
        }

        _ => {}
    }

    error_expr(span, id)
}

enum SeqKind {
    Arr,
    Tup,
    Strc,
}

enum SeqItem {
    KeyVal((), TypedExpr),
    Lit(TypedExpr),
}

fn get_items(tc: &mut TypeChecker, items: &Vec<Expr>) -> Option<Vec<SeqItem>> {
    let mut res = Vec::new();

    for i in items.iter() {
        match i.clone().kind {
            ExprKind::Binary(l, BinaryOp::KeyValue, r) => {
                let _name = if let ExprKind::Ident(n) = l.kind {
                    n
                } else {
                    tc.report_error(l.span_expr(), format!("Feild name must be a ident"));
                    return None;
                };

                let checked_r = tc.check_expr(&r);

                res.push(SeqItem::KeyVal((), checked_r));
            }
            _ => {
                res.push(SeqItem::Lit(tc.check_expr(&i)));
            }
        }
    }

    Some(res)
}

fn get_kind(
    tc: &mut TypeChecker,
    items: &Vec<Expr>,
    count: &Option<Box<Expr>>,
) -> Option<(SeqKind, Vec<SeqItem>)> {
    let seq_item = get_items(tc, items)?;

    if let Some(_) = count.clone() {
        return Some((SeqKind::Arr, seq_item));
    }

    if let Some(f) = seq_item.first() {
        if let SeqItem::Lit(_) = f {
            return Some((SeqKind::Tup, seq_item));
        }
    }

    Some((SeqKind::Strc, seq_item))
}
