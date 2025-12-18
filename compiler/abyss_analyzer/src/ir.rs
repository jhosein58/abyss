use crate::{
    hir::FlatProgram,
    lir::{LirExpr, LirFunctionDef, LirLiteral, LirProgram, LirStmt, LirType},
};
use abyss_parser::ast::{Expr, FunctionBody, FunctionDef, Lit, Stmt, Type};

pub struct Ir;

impl Ir {
    pub fn build(flat_ast: &FlatProgram) -> LirProgram {
        let mut lir = LirProgram::default();

        for func in &flat_ast.functions {
            lir.functions.push(Self::transpile_function(func));
        }

        lir
    }

    fn transpile_function(func: &FunctionDef) -> LirFunctionDef {
        let (body, is_extern) = match &func.body {
            FunctionBody::UserDefined(stmts) => (Self::transpile_block(stmts), false),
            FunctionBody::Extern => (vec![], true),
        };

        let params: Vec<(String, LirType)> = func
            .params
            .iter()
            .map(|(n, t)| (n.clone(), Self::transpile_type(t)))
            .collect();

        LirFunctionDef {
            name: func.name.clone(),
            params,
            return_type: Self::transpile_type(&func.return_type),
            body,
            is_extern,
        }
    }

    fn transpile_type(ty: &Type) -> LirType {
        match ty {
            Type::U8 => LirType::U8,
            Type::I64 => LirType::I64,
            Type::F64 => LirType::F64,
            Type::Bool => LirType::Bool,
            Type::Void => LirType::Void,
            Type::Pointer(inner) => LirType::Pointer(Box::new(Self::transpile_type(inner))),
            Type::Array(inner, size) => {
                LirType::Array(Box::new(Self::transpile_type(inner)), *size)
            }

            Type::Struct(path, _) => LirType::Struct(path.join("__")),
            Type::Enum(path, _) => LirType::Enum(path.join("__")),

            Type::Function(args, ret, _) => {
                let lir_args = args.iter().map(Self::transpile_type).collect();
                let lir_ret = Self::transpile_type(ret);
                LirType::FunctionPtr(lir_args, Box::new(lir_ret))
            }

            Type::Generic(_) => LirType::Void,
        }
    }

    fn transpile_block(stmts: &[Stmt]) -> Vec<LirStmt> {
        let mut result = Vec::new();
        for stmt in stmts {
            result.extend(Self::transpile_stmt(stmt));
        }
        result
    }

    fn transpile_stmt(stmt: &Stmt) -> Vec<LirStmt> {
        match stmt {
            Stmt::Let(name, ty_opt, expr_opt) => {
                let lir_ty = if let Some(t) = ty_opt {
                    Self::transpile_type(t)
                } else {
                    LirType::I64
                };

                let lir_init = expr_opt.as_ref().map(|e| Self::transpile_expr(e));
                vec![LirStmt::Let(name.clone(), lir_ty, lir_init)]
            }

            Stmt::Const(name, ty_opt, expr_opt) => {
                let lir_ty = ty_opt.as_ref().map_or(LirType::I64, Self::transpile_type);
                let lir_init = expr_opt.as_ref().map(|e| Self::transpile_expr(e));
                vec![LirStmt::Let(name.clone(), lir_ty, lir_init)]
            }

            Stmt::Assign(lhs, rhs) => {
                vec![LirStmt::Assign(
                    Self::transpile_expr(lhs),
                    Self::transpile_expr(rhs),
                )]
            }

            Stmt::Expr(e) => vec![LirStmt::ExprStmt(Self::transpile_expr(e))],

            Stmt::Ret(e) => vec![LirStmt::Return(Some(Self::transpile_expr(e)))],

            Stmt::Break => vec![LirStmt::Break],
            Stmt::Continue => vec![LirStmt::Continue],

            Stmt::Block(inner_stmts) => {
                vec![LirStmt::Block(Self::transpile_block(inner_stmts))]
            }

            Stmt::If(cond, then_box, else_box) => {
                let lir_cond = Self::transpile_expr(cond);

                let then_branch = Self::transpile_stmt(then_box);

                let else_branch = if let Some(else_stmt) = else_box {
                    Self::transpile_stmt(else_stmt)
                } else {
                    vec![]
                };

                vec![LirStmt::If {
                    cond: lir_cond,
                    then_branch,
                    else_branch,
                }]
            }

            Stmt::While(cond, body) => {
                vec![LirStmt::While {
                    cond: Self::transpile_expr(cond),
                    body: Self::transpile_stmt(body),
                }]
            }

            Stmt::Import(_)
            | Stmt::FromImport(_, _)
            | Stmt::StructDef(_)
            | Stmt::EnumDef(_)
            | Stmt::FunctionDef(_) => vec![],
        }
    }

    fn transpile_expr(expr: &Expr) -> LirExpr {
        match expr {
            Expr::Lit(Lit::Array(items)) => {
                let lir_items = items.iter().map(Self::transpile_expr).collect();
                LirExpr::ArrayInit(lir_items)
            }
            Expr::Lit(l) => LirExpr::Lit(Self::transpile_lit(l)),

            Expr::Ident(path) => LirExpr::Ident(path.join("__")),

            Expr::Binary(left, op, right) => LirExpr::Binary(
                Box::new(Self::transpile_expr(left)),
                *op,
                Box::new(Self::transpile_expr(right)),
            ),

            Expr::Unary(op, operand) => {
                LirExpr::Unary(*op, Box::new(Self::transpile_expr(operand)))
            }

            Expr::Call(callee, args, _) => {
                let lir_args = args.iter().map(Self::transpile_expr).collect();

                match &**callee {
                    Expr::Ident(path) => LirExpr::Call {
                        func_name: path.join("__"),
                        args: lir_args,
                    },
                    _ => LirExpr::CallPtr(Box::new(Self::transpile_expr(callee)), lir_args),
                }
            }

            Expr::Index(arr, idx) => LirExpr::Index(
                Box::new(Self::transpile_expr(arr)),
                Box::new(Self::transpile_expr(idx)),
            ),

            Expr::Deref(inner) => LirExpr::Deref(Box::new(Self::transpile_expr(inner))),
            Expr::AddrOf(inner) => LirExpr::AddrOf(Box::new(Self::transpile_expr(inner))),

            Expr::Cast(inner, ty) => LirExpr::Cast(
                Box::new(Self::transpile_expr(inner)),
                Self::transpile_type(ty),
            ),

            Expr::Member(obj, field) => {
                LirExpr::MemberAccess(Box::new(Self::transpile_expr(obj)), field.clone())
            }

            Expr::MethodCall(obj, method_name, args, _) => {
                let mut new_args = vec![Self::transpile_expr(obj)];
                new_args.extend(args.iter().map(Self::transpile_expr));

                LirExpr::Call {
                    func_name: method_name.clone(),
                    args: new_args,
                }
            }

            Expr::StructInit(path, fields, _) => {
                let lir_fields = fields
                    .iter()
                    .map(|(n, e)| (n.clone(), Self::transpile_expr(e)))
                    .collect();

                LirExpr::StructInit {
                    struct_name: path.join("__"),
                    fields: lir_fields,
                }
            }

            Expr::EnumInit(path, params, _) => {
                let payload = if let Some(first) = params.first() {
                    Some(Box::new(Self::transpile_expr(first)))
                } else {
                    None
                };

                LirExpr::EnumInit {
                    enum_name: path.join("__"),
                    variant_tag: 0,
                    payload,
                }
            }

            Expr::Ternary(cond, true_val, false_val) => LirExpr::Ternary(
                Box::new(Self::transpile_expr(cond)),
                Box::new(Self::transpile_expr(true_val)),
                Box::new(Self::transpile_expr(false_val)),
            ),

            Expr::SizeOf(ty) => LirExpr::SizeOf(Self::transpile_type(ty)),

            Expr::Match(_, _) => {
                panic!("Match expression must be lowered to statements before IR generation")
            }
        }
    }

    fn transpile_lit(lit: &Lit) -> LirLiteral {
        match lit {
            Lit::Int(v) => LirLiteral::Int(*v),
            Lit::Float(v) => LirLiteral::Float(*v),
            Lit::Bool(v) => LirLiteral::Bool(*v),
            Lit::Str(v) => LirLiteral::Str(v.clone()),
            Lit::Null => LirLiteral::Null,
            Lit::Array(_) => LirLiteral::Null,
        }
    }
}
