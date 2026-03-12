use abyss_diagnostics::Span;
use abyss_parser::ast::{BinaryOp, Expr, ExprKind, Lit};
use abyss_types::{
    tast::{SequenceElement, TypedExpr, TypedExprKind},
    types::{StructField, Type},
};

use crate::type_checker::engine::{TypeChecker, error_expr};

pub fn check_sequence<'a>(
    tc: &mut TypeChecker<'a>,
    items: &'a Vec<Expr>,
    count: &'a Option<Box<Expr>>,
    span: Span,
    id: u32,
) -> TypedExpr {
    let mut typed_elements = Vec::new();
    let mut has_labels = false;
    let mut all_same_type = true;
    let mut first_type: Option<Type> = None;

    let mut all_are_metatypes = true;

    for item in items.iter() {
        let (label, expr_to_check) = match &item.kind {
            ExprKind::Binary(left, BinaryOp::KeyValue, right) => {
                has_labels = true;
                let name = if let ExprKind::Ident(ref n) = left.kind {
                    n.clone()
                } else {
                    tc.report_error(
                        left.span_expr(),
                        "Field name must be an identifier.".to_string(),
                    );
                    "__error_field".to_string()
                };
                (Some(name), right.as_ref())
            }
            _ => (None, item),
        };

        let typed_expr = tc.check_expr(expr_to_check);

        if typed_expr.ty != Type::Metatype {
            all_are_metatypes = false;
        }

        if let Some(ref ft) = first_type {
            if *ft != typed_expr.ty {
                all_same_type = false;
            }
        } else {
            first_type = Some(typed_expr.ty.clone());
        }

        typed_elements.push(SequenceElement {
            label,
            expr: typed_expr,
        });
    }

    if let Some(count_expr) = count {
        if typed_elements.len() != 1 {
            tc.report_error(
                span.clone(),
                "Array repetition syntax [expr; count] requires exactly one element.".to_string(),
            );
            return error_expr(span, id);
        }
        if has_labels {
            tc.report_error(
                span.clone(),
                "Array repetition cannot have labeled fields.".to_string(),
            );
        }

        let typed_count = tc.check_expr(count_expr);
        let first_element = typed_elements.remove(0);

        if first_element.expr.ty == Type::Metatype {
            let inner_type = tc.evaluate_as_type(first_element.expr);
            let array_type = Type::Array(Box::new(inner_type), Box::new(typed_count));

            return TypedExpr {
                kind: TypedExprKind::Type(array_type),
                ty: Type::Metatype,
                span,
                id,
            };
        }

        let element_ty = first_element.expr.ty.clone();

        return TypedExpr {
            kind: TypedExprKind::SequenceInit(vec![first_element]),
            ty: Type::Array(Box::new(element_ty), Box::new(typed_count)),
            span,
            id,
        };
    }

    let is_empty = typed_elements.is_empty();

    if all_are_metatypes && !is_empty {
        let mut struct_fields = Vec::new();

        for (index, el) in typed_elements.into_iter().enumerate() {
            let field_name = el.label.unwrap_or_else(|| index.to_string());
            let field_ty = tc.evaluate_as_type(el.expr);

            struct_fields.push(StructField {
                name: field_name,
                ty: field_ty,
            });
        }

        return TypedExpr {
            kind: TypedExprKind::Type(Type::Struct(struct_fields)),
            ty: Type::Metatype,
            span,
            id,
        };
    }

    let is_array = !has_labels && all_same_type;

    if is_array || is_empty {
        let element_ty = first_type.unwrap_or(Type::Unit);

        let length_expr = TypedExpr {
            kind: TypedExprKind::Lit(Lit::Int(typed_elements.len() as i64)),
            ty: Type::I32,
            span: span.clone(),
            id: 0,
        };

        return TypedExpr {
            kind: TypedExprKind::SequenceInit(typed_elements),
            ty: Type::Array(Box::new(element_ty), Box::new(length_expr)),
            span,
            id,
        };
    } else {
        let mut struct_fields = Vec::new();
        let mut final_elements = Vec::new();

        for (index, mut el) in typed_elements.into_iter().enumerate() {
            let field_name = el.label.clone().unwrap_or_else(|| index.to_string());

            el.label = Some(field_name.clone());

            struct_fields.push(StructField {
                name: field_name,
                ty: el.expr.ty.clone(),
            });

            final_elements.push(el);
        }

        return TypedExpr {
            kind: TypedExprKind::SequenceInit(final_elements),
            ty: Type::Struct(struct_fields),
            span,
            id,
        };
    }
}
