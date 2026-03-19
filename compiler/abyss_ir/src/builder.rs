use std::collections::HashMap;

use abyss_parser::ast::{BinaryOp, Lit, UnaryOp};
use abyss_types::{
    tast::{TypedExpr, TypedExprKind, TypedProgram},
    type_encoder::TypeEncoder,
    types::Type,
};

use crate::ir::{
    IrBinaryOp, IrExpr, IrExprKind, IrFunction, IrLit, IrProgram, IrStmt, IrType, IrUnaryOp,
};
use abyss_diagnostics::Span;

#[derive(Debug, Clone)]
struct LoopContext {
    result_var: String,
    break_flag_var: String,
    is_void: bool,
}

pub struct IrBuilder {
    temp_counter: usize,
    native_function_map: HashMap<String, usize>,
    loop_contexts: Vec<LoopContext>,
    pub encoder: TypeEncoder,
}

impl IrBuilder {
    pub fn new() -> Self {
        Self {
            temp_counter: 0,
            native_function_map: HashMap::new(),
            loop_contexts: Vec::new(),
            encoder: TypeEncoder::new(),
        }
    }

    fn new_temp(&mut self) -> String {
        let name = format!("$tmp{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    fn unit_expr(&self, span: Span) -> IrExpr {
        IrExpr {
            kind: IrExprKind::Lit(IrLit::Bool(false)),
            ty: IrType::Unit,
            span,
        }
    }

    pub fn register_native(&mut self, name: &str, index: usize) {
        self.native_function_map.insert(name.to_string(), index);
    }

    pub fn build_single_function(&mut self, expr: TypedExpr) -> Option<IrProgram> {
        if let Some(ir_func) = self.build_function(expr) {
            Some(IrProgram {
                functions: vec![ir_func],
            })
        } else {
            None
        }
    }
    pub fn build_thunk_program(
        &mut self,
        expr: TypedExpr,
        globals: &HashMap<String, TypedExpr>,
    ) -> IrProgram {
        let mut init_stmts = Vec::new();

        for (name, global_expr) in globals {
            if !matches!(global_expr.kind, TypedExprKind::FunctionDef { .. }) {
                let (value_stmts, value_ir) = self.lower_expr(global_expr.clone());
                init_stmts.extend(value_stmts);

                init_stmts.push(IrStmt::ConstDef {
                    name: name.clone(),
                    ty: value_ir.ty.clone(),
                    value: value_ir,
                });
            }
        }

        let expected_return_ty = self.lower_type(&expr.ty);
        let prev_counter = self.temp_counter;
        self.temp_counter = 0;

        let (mut stmts, final_expr) = self.lower_expr(expr);

        let mut main_body = init_stmts;
        main_body.append(&mut stmts);

        if expected_return_ty != IrType::Unit {
            main_body.push(IrStmt::Return(Some(final_expr)));
        } else {
            main_body.push(IrStmt::Return(None));
        }

        self.temp_counter = prev_counter;

        IrProgram {
            functions: vec![IrFunction {
                name: "thunk_main".to_string(),
                params: vec![],
                return_ty: expected_return_ty,
                body: main_body,
                is_native: false,
            }],
        }
    }

    pub fn build_comptime_program(
        &mut self,
        expr: TypedExpr,
        globals: &HashMap<String, TypedExpr>,
    ) -> IrProgram {
        let mut functions = Vec::new();
        let mut init_stmts = Vec::new();

        for (name, global_expr) in globals {
            if let Some(ir_func) = self.build_function(global_expr.clone()) {
                functions.push(ir_func);
            } else {
                let (value_stmts, value_ir) = self.lower_expr(global_expr.clone());
                init_stmts.extend(value_stmts);

                init_stmts.push(IrStmt::ConstDef {
                    name: name.clone(),
                    ty: value_ir.ty.clone(),
                    value: value_ir,
                });
            }
        }

        let expected_return_ty = self.lower_type(&expr.ty);

        let prev_counter = self.temp_counter;
        self.temp_counter = 0;

        let (mut stmts, final_expr) = self.lower_expr(expr);

        let mut main_body = init_stmts;
        main_body.append(&mut stmts);

        if expected_return_ty != IrType::Unit {
            main_body.push(IrStmt::Return(Some(final_expr)));
        } else {
            main_body.push(IrStmt::Return(None));
        }

        self.temp_counter = prev_counter;

        functions.push(IrFunction {
            name: "main".to_string(),
            params: vec![],
            return_ty: expected_return_ty,
            body: main_body,
            is_native: false,
        });

        IrProgram { functions }
    }

    pub fn build_standalone_expr(&mut self, expr: TypedExpr) -> (Vec<IrStmt>, IrExpr) {
        let prev_counter = self.temp_counter;
        let result = self.lower_expr(expr);
        self.temp_counter = prev_counter;

        result
    }

    // --- decl.rs ---

    pub fn build_program(&mut self, program: TypedProgram) -> IrProgram {
        let mut functions = Vec::new();
        let mut init_stmts = Vec::new();

        for (name, global_expr) in program.globals.clone() {
            if let Some(ir_func) = self.build_function(global_expr.clone()) {
                functions.push(ir_func);
            } else {
                let (value_stmts, value_ir) = self.lower_expr(global_expr);
                init_stmts.extend(value_stmts);

                init_stmts.push(IrStmt::ConstDef {
                    name: name.clone(),
                    ty: value_ir.ty.clone(),
                    value: value_ir,
                });
            }
        }

        let mut main_body = init_stmts;

        if let TypedExprKind::Block(stmts) = program.body.kind {
            for stmt in stmts {
                main_body.extend(self.lower_stmt(stmt));
            }
        } else {
            main_body.extend(self.lower_stmt(program.body));
        }

        functions.push(IrFunction {
            name: "main".to_string(),
            params: vec![],
            return_ty: IrType::Unit,
            body: main_body,
            is_native: false,
        });

        IrProgram { functions }
    }

    fn build_function(&mut self, expr: TypedExpr) -> Option<IrFunction> {
        if let TypedExprKind::FunctionDef {
            name,
            args,
            ret_ty,
            body,
            is_native,
        } = expr.kind
        {
            let mut ir_params = Vec::new();

            for param_expr in args {
                if let TypedExprKind::VarDec(p_name, p_ty, _) = param_expr.kind {
                    ir_params.push((p_name, self.lower_type(&p_ty)));
                } else {
                    panic!("IR Builder: Function arguments must be VarDec.");
                }
            }

            self.temp_counter = 0;

            let mut ir_body = Vec::new();
            if !is_native {
                let (body_stmts, final_expr) = self.lower_expr(*body);
                ir_body.extend(body_stmts);

                if !matches!(ir_body.last(), Some(IrStmt::Return(_))) {
                    if self.lower_type(&ret_ty) == IrType::Unit {
                        if !matches!(final_expr.kind, IrExprKind::Lit(IrLit::Bool(false))) {
                            ir_body.push(IrStmt::Expr(final_expr));
                        }

                        ir_body.push(IrStmt::Return(None));
                    } else {
                        ir_body.push(IrStmt::Return(Some(final_expr)));
                    }
                }
            }

            return Some(IrFunction {
                name,
                params: ir_params,
                return_ty: self.lower_type(&ret_ty),
                body: ir_body,
                is_native,
            });
        }
        None
    }

    // --- stmt.rs ---

    fn lower_stmt(&mut self, expr: TypedExpr) -> Vec<IrStmt> {
        let mut generated_stmts = Vec::new();

        match expr.kind {
            TypedExprKind::VarDec(name, ty, init) => {
                let ir_init = if let Some(init_expr) = init {
                    let (init_stmts, init_val) = self.lower_expr(*init_expr);
                    generated_stmts.extend(init_stmts);
                    Some(init_val)
                } else {
                    None
                };

                generated_stmts.push(IrStmt::VarDec {
                    name,
                    ty: self.lower_type(&ty),
                    init: ir_init,
                });
            }

            TypedExprKind::Binary(left, BinaryOp::Assign, right) => match left.kind {
                TypedExprKind::Ident(name) => {
                    let (right_stmts, right_val) = self.lower_expr(*right);
                    generated_stmts.extend(right_stmts);

                    generated_stmts.push(IrStmt::Assign {
                        target: name,
                        val: right_val,
                    });
                }
                TypedExprKind::Index(base_expr, index_expr) => {
                    let (base_stmts, base_val) = self.lower_expr(*base_expr);
                    generated_stmts.extend(base_stmts);

                    let (index_stmts, index_val) = self.lower_expr(*index_expr);
                    generated_stmts.extend(index_stmts);

                    let (right_stmts, right_val) = self.lower_expr(*right);
                    generated_stmts.extend(right_stmts);

                    generated_stmts.push(IrStmt::WriteIndex {
                        base: base_val,
                        index: index_val,
                        val: right_val,
                    });
                }

                TypedExprKind::FieldAccess(base_expr, field_name) => {
                    let base_ty = base_expr.ty.underlying_type();

                    let field_index = match &base_ty {
                        Type::Struct(fields) => fields
                            .iter()
                            .position(|f| f.name == field_name)
                            .expect("IR Builder: Struct field not found during assignment"),
                        _ => panic!("IR Builder: Field assignment on non-struct type"),
                    };

                    let (base_stmts, base_val) = self.lower_expr(*base_expr);
                    generated_stmts.extend(base_stmts);

                    let (right_stmts, right_val) = self.lower_expr(*right);
                    generated_stmts.extend(right_stmts);

                    generated_stmts.push(IrStmt::WriteField {
                        base: base_val,
                        index: field_index,
                        val: right_val,
                    });
                }

                TypedExprKind::Unary(UnaryOp::Deref, ptr_expr) => {
                    let (ptr_stmts, ptr_val) = self.lower_expr(*ptr_expr);
                    generated_stmts.extend(ptr_stmts);

                    let (right_stmts, right_val) = self.lower_expr(*right);
                    generated_stmts.extend(right_stmts);

                    generated_stmts.push(IrStmt::WritePointer {
                        ptr: ptr_val,
                        val: right_val,
                    });
                }
                _ => panic!("Complex assignments not supported yet: {:?}", left.kind),
            },

            TypedExprKind::Ret(val) => {
                let ir_val = if let Some(ret_expr) = val {
                    let (ret_stmts, ret_val) = self.lower_expr(*ret_expr);
                    generated_stmts.extend(ret_stmts);
                    Some(ret_val)
                } else {
                    None
                };
                generated_stmts.push(IrStmt::Return(ir_val));
            }

            TypedExprKind::Block(stmts) => {
                for stmt in stmts {
                    generated_stmts.extend(self.lower_stmt(stmt));
                }
            }

            TypedExprKind::Def(_name, value_expr) => {
                // let (value_stmts, value_ir) = self.lower_expr(*value_expr);
                // generated_stmts.extend(value_stmts);

                // let value_ty = value_ir.ty.clone();

                // generated_stmts.push(IrStmt::ConstDef {
                //     name,
                //     ty: value_ty,
                //     value: value_ir,
                // });
                //
                let (value_stmts, _value_ir) = self.lower_expr(*value_expr);
                generated_stmts.extend(value_stmts);
            }

            _ => {
                let (expr_stmts, val) = self.lower_expr(expr);
                generated_stmts.extend(expr_stmts);
                generated_stmts.push(IrStmt::Expr(val));
            }
        }

        generated_stmts
    }

    // --- expr.rs ---

    fn lower_expr(&mut self, expr: TypedExpr) -> (Vec<IrStmt>, IrExpr) {
        let span = expr.span.clone();
        let ir_ty = self.lower_type(&expr.ty);
        let mut generated_stmts = Vec::new();

        let kind = match expr.kind {
            TypedExprKind::Lit(lit) => IrExprKind::Lit(self.lower_lit(lit)),

            TypedExprKind::Ident(name) | TypedExprKind::FuncRef(name) => IrExprKind::VarRef(name),

            TypedExprKind::Unary(op, inner_expr) => {
                if op == UnaryOp::AddrOf {
                    match inner_expr.kind.clone() {
                        TypedExprKind::Index(base_expr, index_expr) => {
                            let (mut stmts, base_val) = self.lower_expr(*base_expr);
                            generated_stmts.append(&mut stmts);

                            let (mut idx_stmts, idx_val) = self.lower_expr(*index_expr);
                            generated_stmts.append(&mut idx_stmts);

                            return (
                                generated_stmts,
                                IrExpr {
                                    kind: IrExprKind::GetIndexPtr {
                                        base: Box::new(base_val),
                                        index: Box::new(idx_val),
                                    },
                                    ty: ir_ty,
                                    span,
                                },
                            );
                        }

                        TypedExprKind::FieldAccess(base_expr, field_name) => {
                            let base_ty = base_expr.ty.underlying_type();
                            let field_index = match &base_ty {
                                Type::Struct(fields) => fields
                                    .iter()
                                    .position(|f| f.name == field_name)
                                    .expect("IR Builder: Struct field not found"),
                                _ => panic!("IR Builder: Field access on non-struct type"),
                            };

                            let (mut stmts, base_val) = self.lower_expr(*base_expr);
                            generated_stmts.append(&mut stmts);

                            return (
                                generated_stmts,
                                IrExpr {
                                    kind: IrExprKind::GetFieldPtr {
                                        base: Box::new(base_val),
                                        index: field_index,
                                    },
                                    ty: ir_ty,
                                    span,
                                },
                            );
                        }

                        _ => {}
                    }
                }

                let ir_op = match op {
                    UnaryOp::Neg => IrUnaryOp::Neg,
                    UnaryOp::Not => IrUnaryOp::Not,
                    UnaryOp::AddrOf => IrUnaryOp::Ref,
                    UnaryOp::Deref => IrUnaryOp::Deref,
                    UnaryOp::BitNot => IrUnaryOp::BitNot,
                };

                let (inner_stmts, inner_val) = self.lower_expr(*inner_expr);
                generated_stmts.extend(inner_stmts);

                IrExprKind::Unary(ir_op, Box::new(inner_val))
            }

            TypedExprKind::Binary(left, BinaryOp::Assign, right) => match left.kind {
                TypedExprKind::Ident(name) => {
                    let (right_stmts, right_val) = self.lower_expr(*right);
                    generated_stmts.extend(right_stmts);

                    let ret_val = right_val.clone();

                    generated_stmts.push(IrStmt::Assign {
                        target: name,
                        val: right_val,
                    });

                    return (generated_stmts, ret_val);
                }
                TypedExprKind::Index(base_expr, index_expr) => {
                    let (base_stmts, base_val) = self.lower_expr(*base_expr);
                    generated_stmts.extend(base_stmts);

                    let (index_stmts, index_val) = self.lower_expr(*index_expr);
                    generated_stmts.extend(index_stmts);

                    let (right_stmts, right_val) = self.lower_expr(*right);
                    generated_stmts.extend(right_stmts);

                    let ret_val = right_val.clone();

                    generated_stmts.push(IrStmt::WriteIndex {
                        base: base_val,
                        index: index_val,
                        val: right_val,
                    });

                    return (generated_stmts, ret_val);
                }

                TypedExprKind::FieldAccess(base_expr, field_name) => {
                    let base_ty = base_expr.ty.underlying_type();

                    let field_index = match &base_ty {
                        Type::Struct(fields) => fields
                            .iter()
                            .position(|f| f.name == field_name)
                            .expect("IR Builder: Struct field not found during assignment"),
                        _ => panic!("IR Builder: Field assignment on non-struct type"),
                    };

                    let (base_stmts, base_val) = self.lower_expr(*base_expr);
                    generated_stmts.extend(base_stmts);

                    let (right_stmts, right_val) = self.lower_expr(*right);
                    generated_stmts.extend(right_stmts);

                    let ret_val = right_val.clone();

                    generated_stmts.push(IrStmt::WriteField {
                        base: base_val,
                        index: field_index,
                        val: right_val,
                    });

                    return (generated_stmts, ret_val);
                }

                TypedExprKind::Unary(UnaryOp::Deref, ptr_expr) => {
                    let (ptr_stmts, ptr_val) = self.lower_expr(*ptr_expr);
                    generated_stmts.extend(ptr_stmts);

                    let (right_stmts, right_val) = self.lower_expr(*right);
                    generated_stmts.extend(right_stmts);

                    let ret_val = right_val.clone();

                    generated_stmts.push(IrStmt::WritePointer {
                        ptr: ptr_val,
                        val: right_val,
                    });

                    return (generated_stmts, ret_val);
                }

                _ => panic!("Complex assignments not supported yet: {:?}", left.kind),
            },

            TypedExprKind::Binary(left, op, right) => {
                let ir_op = match op {
                    BinaryOp::Add => IrBinaryOp::Add,
                    BinaryOp::Sub => IrBinaryOp::Sub,
                    BinaryOp::Mul => IrBinaryOp::Mul,
                    BinaryOp::Div => IrBinaryOp::Div,
                    BinaryOp::Mod => IrBinaryOp::Mod,
                    BinaryOp::Eq => IrBinaryOp::Eq,
                    BinaryOp::Neq => IrBinaryOp::Neq,
                    BinaryOp::Lt => IrBinaryOp::Lt,
                    BinaryOp::Lte => IrBinaryOp::Le,
                    BinaryOp::Gt => IrBinaryOp::Gt,
                    BinaryOp::Gte => IrBinaryOp::Ge,
                    BinaryOp::And => IrBinaryOp::And,
                    BinaryOp::Or => IrBinaryOp::Or,

                    BinaryOp::BitAnd => IrBinaryOp::BitAnd,
                    BinaryOp::Pipe => IrBinaryOp::BitOr,
                    BinaryOp::BitXor => IrBinaryOp::BitXor,
                    BinaryOp::Shl => IrBinaryOp::Shl,
                    BinaryOp::Shr => IrBinaryOp::Shr,

                    _ => panic!("Unsupported binary op in IR Builder: {:?}", op),
                };

                let (left_stmts, left_val) = self.lower_expr(*left);
                let (right_stmts, right_val) = self.lower_expr(*right);

                generated_stmts.extend(left_stmts);
                generated_stmts.extend(right_stmts);

                IrExprKind::Binary(Box::new(left_val), ir_op, Box::new(right_val))
            }

            TypedExprKind::Call(func, args, is_native) => {
                let func_name = match func.kind {
                    TypedExprKind::Ident(name) | TypedExprKind::FuncRef(name) => name,
                    _ => panic!("Dynamic dispatch not supported."),
                };

                let mut ir_args = Vec::new();
                for arg in args {
                    let (arg_stmts, arg_val) = self.lower_expr(arg);
                    generated_stmts.extend(arg_stmts);
                    ir_args.push(arg_val);
                }
                if is_native {
                    let func_index = *self.native_function_map.get(&func_name).expect(
                        &format!("IR Builder: Native function '{}' not found in map. This is a compiler bug.", func_name)
                    );
                    IrExprKind::NativeCall {
                        func_index,
                        args: ir_args,
                    }
                } else {
                    IrExprKind::Call {
                        func_name,
                        args: ir_args,
                    }
                }
            }

            TypedExprKind::If(cond, then_branch, else_branch) => {
                let (cond_stmts, cond_val) = self.lower_expr(*cond);
                generated_stmts.extend(cond_stmts);

                let is_void = expr.ty == Type::Unit;

                if is_void {
                    let then_stmts = self.lower_stmt(*then_branch);
                    let else_stmts = if let Some(else_b) = else_branch {
                        self.lower_stmt(*else_b)
                    } else {
                        vec![]
                    };

                    generated_stmts.push(IrStmt::If(cond_val, then_stmts, else_stmts));

                    return (generated_stmts, self.unit_expr(span));
                } else {
                    let temp_var = self.new_temp();
                    generated_stmts.push(IrStmt::VarDec {
                        name: temp_var.clone(),
                        ty: ir_ty.clone(),
                        init: None,
                    });

                    let (mut then_stmts, then_val) = self.lower_expr(*then_branch);
                    then_stmts.push(IrStmt::Assign {
                        target: temp_var.clone(),
                        val: then_val,
                    });

                    let mut else_stmts = Vec::new();
                    if let Some(else_b) = else_branch {
                        let (e_stmts, e_val) = self.lower_expr(*else_b);
                        else_stmts.extend(e_stmts);
                        else_stmts.push(IrStmt::Assign {
                            target: temp_var.clone(),
                            val: e_val,
                        });
                    }

                    generated_stmts.push(IrStmt::If(cond_val, then_stmts, else_stmts));

                    return (
                        generated_stmts,
                        IrExpr {
                            kind: IrExprKind::VarRef(temp_var),
                            ty: ir_ty,
                            span,
                        },
                    );
                }
            }

            TypedExprKind::Block(stmts) => {
                let mut last_val = self.unit_expr(span.clone());
                let stmts_len = stmts.len();
                for (i, stmt) in stmts.into_iter().enumerate() {
                    let is_last = i == stmts_len - 1;
                    if is_last {
                        let (s, v) = self.lower_expr(stmt);
                        generated_stmts.extend(s);
                        last_val = v;
                    } else {
                        generated_stmts.extend(self.lower_stmt(stmt));
                    }
                }
                return (generated_stmts, last_val);
            }

            TypedExprKind::Ret(val) => {
                let ir_val = if let Some(ret_expr) = val {
                    let (ret_stmts, ret_val) = self.lower_expr(*ret_expr);
                    generated_stmts.extend(ret_stmts);
                    Some(ret_val)
                } else {
                    None
                };
                generated_stmts.push(IrStmt::Return(ir_val));

                return (generated_stmts, self.unit_expr(span));
            }

            TypedExprKind::VarDec(name, ty, init) => {
                let mut ret_val = self.unit_expr(span.clone());

                let ir_init = if let Some(init_expr) = init {
                    let (init_stmts, init_val) = self.lower_expr(*init_expr);
                    generated_stmts.extend(init_stmts);

                    ret_val = init_val.clone();

                    Some(init_val)
                } else {
                    None
                };

                generated_stmts.push(IrStmt::VarDec {
                    name,
                    ty: self.lower_type(&ty),
                    init: ir_init,
                });

                return (generated_stmts, ret_val);
            }

            TypedExprKind::Type(ty) => {
                let type_id = self.encoder.get_or_create_id(&ty);
                IrExprKind::Lit(IrLit::Int(type_id))
            }

            TypedExprKind::While(cond, body, else_branch) => {
                let is_void = expr.ty == abyss_types::types::Type::Unit;

                let result_var = if is_void {
                    String::new()
                } else {
                    self.new_temp()
                };
                let break_flag = self.new_temp();

                if !is_void {
                    generated_stmts.push(IrStmt::VarDec {
                        name: result_var.clone(),
                        ty: ir_ty.clone(),
                        init: None,
                    });
                }

                generated_stmts.push(IrStmt::VarDec {
                    name: break_flag.clone(),
                    ty: IrType::Bool,
                    init: Some(IrExpr {
                        kind: IrExprKind::Lit(IrLit::Bool(false)),
                        ty: IrType::Bool,
                        span: span.clone(),
                    }),
                });

                self.loop_contexts.push(LoopContext {
                    result_var: result_var.clone(),
                    break_flag_var: break_flag.clone(),
                    is_void,
                });

                let (cond_stmts, cond_val) = self.lower_expr(*cond);
                generated_stmts.extend(cond_stmts);

                let (body_stmts, _) = self.lower_expr(*body);

                generated_stmts.push(IrStmt::While {
                    cond: cond_val,
                    body: body_stmts,
                });

                self.loop_contexts.pop();

                if let Some(else_b) = else_branch {
                    let mut else_stmts = Vec::new();
                    let (e_stmts, e_val) = self.lower_expr(*else_b);
                    else_stmts.extend(e_stmts);

                    if !is_void {
                        else_stmts.push(IrStmt::Assign {
                            target: result_var.clone(),
                            val: e_val,
                        });
                    }

                    let not_break_cond = IrExpr {
                        kind: IrExprKind::Unary(
                            IrUnaryOp::Not,
                            Box::new(IrExpr {
                                kind: IrExprKind::VarRef(break_flag.clone()),
                                ty: IrType::Bool,
                                span: span.clone(),
                            }),
                        ),
                        ty: IrType::Bool,
                        span: span.clone(),
                    };

                    generated_stmts.push(IrStmt::If(not_break_cond, else_stmts, vec![]));
                }

                if is_void {
                    return (generated_stmts, self.unit_expr(span));
                } else {
                    return (
                        generated_stmts,
                        IrExpr {
                            kind: IrExprKind::VarRef(result_var),
                            ty: ir_ty,
                            span,
                        },
                    );
                }
            }

            TypedExprKind::Out(val_opt) => {
                let ctx = self
                    .loop_contexts
                    .last()
                    .expect("Compiler Error: 'out' statement found outside of a loop!")
                    .clone();

                if let Some(val_expr) = val_opt {
                    let (val_stmts, val_ir) = self.lower_expr(*val_expr);
                    generated_stmts.extend(val_stmts);

                    if !ctx.is_void {
                        generated_stmts.push(IrStmt::Assign {
                            target: ctx.result_var.clone(),
                            val: val_ir,
                        });
                    }
                }

                generated_stmts.push(IrStmt::Assign {
                    target: ctx.break_flag_var.clone(),
                    val: IrExpr {
                        kind: IrExprKind::Lit(IrLit::Bool(true)),
                        ty: IrType::Bool,
                        span: span.clone(),
                    },
                });

                generated_stmts.push(IrStmt::Break);

                return (generated_stmts, self.unit_expr(span));
            }

            TypedExprKind::Def(name, value_expr) => {
                let (value_stmts, value_ir) = self.lower_expr(*value_expr);
                generated_stmts.extend(value_stmts);

                let value_ty = value_ir.ty.clone();

                generated_stmts.push(IrStmt::ConstDef {
                    name,
                    ty: value_ty,
                    value: value_ir,
                });

                return (generated_stmts, self.unit_expr(span));
            }

            TypedExprKind::SequenceInit(elements) => match &expr.ty {
                Type::Array(_, array_len) => {
                    if elements.len() == 1 && *array_len > 1 {
                        let (elem_stmts, elem_val) = self.lower_expr(elements[0].expr.clone());
                        generated_stmts.extend(elem_stmts);

                        IrExprKind::ArrayRepeat {
                            val: Box::new(elem_val),
                            count: *array_len,
                        }
                    } else {
                        let mut ir_elements = Vec::new();
                        for el in elements {
                            let (el_stmts, el_val) = self.lower_expr(el.expr);
                            generated_stmts.extend(el_stmts);
                            ir_elements.push(el_val);
                        }

                        IrExprKind::ArrayInit(ir_elements)
                    }
                }
                Type::Struct(_) => {
                    let mut ir_elements = Vec::new();
                    for el in elements {
                        let (el_stmts, el_val) = self.lower_expr(el.expr);
                        generated_stmts.extend(el_stmts);
                        ir_elements.push(el_val);
                    }
                    IrExprKind::StructInit(ir_elements)
                }
                _ => panic!("IR Builder: SequenceInit expects an Array or Struct type."),
            },

            TypedExprKind::Index(target_expr, index_expr) => {
                let (target_stmts, target_val) = self.lower_expr(*target_expr);
                generated_stmts.extend(target_stmts);

                let (index_stmts, index_val) = self.lower_expr(*index_expr);
                generated_stmts.extend(index_stmts);

                IrExprKind::Index(Box::new(target_val), Box::new(index_val))
            }

            TypedExprKind::FieldAccess(base_expr, field_name) => {
                let base_ty = base_expr.ty.underlying_type();

                let field_index = match &base_ty {
                    Type::Struct(fields) => fields
                        .iter()
                        .position(|f| f.name == field_name)
                        .expect("IR Builder: Struct field not found"),
                    _ => panic!("IR Builder: Field access on non-struct type"),
                };

                let (base_stmts, base_val) = self.lower_expr(*base_expr);
                generated_stmts.extend(base_stmts);

                IrExprKind::FieldAccess {
                    base: Box::new(base_val),
                    index: field_index,
                }
            }

            TypedExprKind::Cast(source, _target) => {
                let (source_stmts, source_val) = self.lower_expr(*source);
                generated_stmts.extend(source_stmts);
                IrExprKind::Cast(Box::new(source_val), ir_ty.clone())
            }

            _ => panic!("Unexpected expression kind: {:?}", expr.kind),
        };

        (
            generated_stmts,
            IrExpr {
                kind,
                ty: ir_ty,
                span,
            },
        )
    }

    // --- utils.rs ---

    fn lower_type(&mut self, ty: &Type) -> IrType {
        let _type_id = self.encoder.get_or_create_id(ty);

        match ty {
            Type::I32 => IrType::I32,
            Type::F32 => IrType::F32,
            Type::Bool => IrType::Bool,
            Type::Unit => IrType::Unit,
            Type::Ptr(inner) => IrType::Ptr(Box::new(self.lower_type(inner))),
            Type::Signature(_, _, _) => IrType::I32,
            Type::Metatype => IrType::I32,
            Type::Array(inner_ty, count) => {
                let inner_ir_ty = self.lower_type(inner_ty);
                IrType::Array(Box::new(inner_ir_ty), *count)
            }
            Type::Struct(fields) => {
                let mut ir_fields = Vec::new();
                for field in fields {
                    ir_fields.push(self.lower_type(&field.ty));
                }
                IrType::Struct(ir_fields)
            }

            Type::Alias(_, inner_ty) => self.lower_type(inner_ty),

            _ => panic!("Unsupported type {:?} for IR generation.", ty),
        }
    }

    fn lower_lit(&self, lit: Lit) -> IrLit {
        match lit {
            Lit::Int(v) => IrLit::Int(v),
            Lit::Float(v) => IrLit::Float(v.0),
            Lit::Bool(v) => IrLit::Bool(v),
            _ => panic!("Unsupported literal."),
        }
    }
}
