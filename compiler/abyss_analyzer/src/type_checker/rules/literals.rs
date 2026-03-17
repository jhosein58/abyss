use abyss_diagnostics::Span;
use abyss_parser::ast::{Lit, UnaryOp};

use abyss_types::{
    tast::{SequenceElement, TypedExpr, TypedExprKind},
    types::Type,
};

use crate::type_checker::{
    context::SymbolInfo,
    engine::TypeChecker,
    resolver::{GlobalMetadata, InlinePolicy},
};
fn unescape_string(raw: &str) -> String {
    let mut result = String::with_capacity(raw.len());
    let mut chars = raw.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next_c) = chars.next() {
                match next_c {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '0' => result.push('\0'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    '\'' => result.push('\''),
                    _ => {
                        result.push('\\');
                        result.push(next_c);
                    }
                }
            } else {
                result.push('\\');
            }
        } else {
            result.push(c);
        }
    }
    result
}

pub fn check_literal(tc: &mut TypeChecker, lit: &Lit, span: Span, id: u32) -> TypedExpr {
    tc.side_table.mark_const(id, true);

    match lit {
        Lit::Str(s) | Lit::Cstr(s) => {
            let unescaped_s = unescape_string(s);

            let str_id = tc.next_id();
            let global_name = format!("__str_lit_{}", str_id);

            let mut elements = Vec::new();

            for c in unescaped_s.chars() {
                let char_expr = TypedExpr {
                    kind: TypedExprKind::Lit(Lit::Int(c as i64)),
                    ty: Type::I32,
                    span: span.clone(),
                    id: tc.next_id(),
                };
                elements.push(SequenceElement {
                    label: None,
                    expr: char_expr,
                });
            }

            let null_expr = TypedExpr {
                kind: TypedExprKind::Lit(Lit::Int('\0' as i64)),
                ty: Type::I32,
                span: span.clone(),
                id: tc.next_id(),
            };
            elements.push(SequenceElement {
                label: None,
                expr: null_expr,
            });

            let array_len = elements.len();
            let array_type = Type::Array(Box::new(Type::I32), array_len);

            let array_expr = TypedExpr {
                kind: TypedExprKind::SequenceInit(elements),
                ty: array_type.clone(),
                span: span.clone(),
                id: tc.next_id(),
            };

            let metadata = GlobalMetadata {
                inline_policy: InlinePolicy::Never,
                is_foldable: true,
            };

            tc.complete_and_register_global(
                global_name.clone(),
                array_type.clone(),
                array_expr,
                false,
                metadata,
            );

            tc.ctx.define_global(
                global_name.clone(),
                SymbolInfo::constant(array_type.clone(), true),
            );

            let ident_expr = TypedExpr {
                kind: TypedExprKind::Ident(global_name),
                ty: array_type.clone(),
                span: span.clone(),
                id: tc.next_id(),
            };

            let index_zero = TypedExpr {
                kind: TypedExprKind::Lit(Lit::Int(0)),
                ty: Type::I32,
                span: span.clone(),
                id: tc.next_id(),
            };

            let array_access = TypedExpr {
                kind: TypedExprKind::Index(Box::new(ident_expr), Box::new(index_zero)),
                ty: Type::I32,
                span: span.clone(),
                id: tc.next_id(),
            };

            TypedExpr {
                kind: TypedExprKind::Unary(UnaryOp::AddrOf, Box::new(array_access)),
                ty: Type::Ptr(Box::new(Type::I32)),
                span,
                id,
            }
        }

        _ => {
            let ty = match lit {
                Lit::Int(_) => Type::I32,
                Lit::Float(_) => Type::F32,
                Lit::Bool(_) => Type::Bool,
                Lit::Char(_) => Type::Char,
                _ => unreachable!(),
            };

            TypedExpr {
                kind: TypedExprKind::Lit(lit.clone()),
                ty,
                span,
                id,
            }
        }
    }
}
