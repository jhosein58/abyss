use abyss_diagnostics::Span;
use abyss_parser::ast::Expr;
use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

use crate::type_checker::engine::{TypeChecker, error_expr};

pub fn check_index<'a>(
    tc: &mut TypeChecker<'a>,
    target: &'a Box<Expr>,
    index: &'a Box<Expr>,
    span: Span,
    id: u32,
) -> TypedExpr {
    let typed_target = tc.check_expr(target);

    let typed_index = tc.check_expr(index);

    if typed_target.ty == Type::Error || typed_index.ty == Type::Error {
        return error_expr(span, id);
    }

    match typed_target.ty.clone() {
        Type::Array(inner_ty, _) => {
            if typed_index.ty != Type::I32 {
                tc.report_error(
                    index.span_expr(),
                    format!(
                        "Array index must be of type 'i32', found '{}'.",
                        typed_index.ty.name()
                    ),
                );
                return error_expr(span, id);
            }

            TypedExpr {
                kind: TypedExprKind::Index(Box::new(typed_target), Box::new(typed_index)),
                ty: *inner_ty,
                span,
                id,
            }
        }

        Type::Struct(fields) => {
            if let TypedExprKind::Lit(ref lit) = typed_index.kind {
                match lit {
                    abyss_parser::ast::Lit::Str(field_name) => {
                        if let Some(field) = fields.iter().find(|f| f.name == *field_name) {
                            return TypedExpr {
                                kind: TypedExprKind::Index(
                                    Box::new(typed_target),
                                    Box::new(typed_index),
                                ),
                                ty: field.ty.clone(),
                                span,
                                id,
                            };
                        } else {
                            tc.report_error(
                                index.span_expr(),
                                format!("Struct does not have a field named '{}'.", field_name),
                            );
                            return error_expr(span, id);
                        }
                    }
                    abyss_parser::ast::Lit::Int(field_idx) => {
                        let idx = *field_idx as usize;
                        if let Some(field) = fields.get(idx) {
                            return TypedExpr {
                                kind: TypedExprKind::Index(
                                    Box::new(typed_target),
                                    Box::new(typed_index),
                                ),
                                ty: field.ty.clone(),
                                span,
                                id,
                            };
                        } else {
                            tc.report_error(
                                index.span_expr(),
                                format!(
                                    "Struct index out of bounds. Struct has {} fields, but index is {}.",
                                    fields.len(),
                                    idx
                                ),
                            );
                            return error_expr(span, id);
                        }
                    }
                    _ => {
                        tc.report_error(
                            index.span_expr(),
                            "Struct index must be a string or integer literal.".to_string(),
                        );
                        return error_expr(span, id);
                    }
                }
            } else {
                tc.report_error(
                    index.span_expr(),
                    "Struct fields can only be accessed using compile-time literals (string or int).".to_string(),
                );
                return error_expr(span, id);
            }
        }

        other_ty => {
            tc.report_error(
                target.span_expr(),
                format!(
                    "Cannot index into a value of type '{}'. Only arrays and structs can be indexed.",
                    other_ty.name()
                ),
            );
            error_expr(span, id)
        }
    }
}
