use std::collections::HashMap;

use crate::target::Target;
use abyss_parser::ast::{BinaryOp, Expr, Function, Lit, Program, Stmt, Type, UnaryOp};

pub struct Director<'a, T: Target> {
    target: &'a mut T,
    loop_stack: Vec<(T::Block, T::Block)>,
    vars: HashMap<String, Type>,
}

impl<'a, T: Target> Director<'a, T> {
    pub fn new(target: &'a mut T) -> Self {
        Self {
            target,
            loop_stack: Vec::new(),
            vars: HashMap::new(),
        }
    }

    pub fn process_program(&mut self, program: &Program) {
        self.target.start_program();

        for func in &program.functions {
            let ret_ty = func.return_type.clone();
            let mut params = Vec::new();
            for (p_name, p_type) in &func.params {
                params.push((p_name.clone(), p_type.clone()));
            }

            if func.body.is_none() {
                self.target
                    .declare_extern_function(&func.name, &params, ret_ty);
            } else {
                self.target.predefine_function(&func.name, &params, ret_ty);
            }
        }

        for func in &program.functions {
            if func.body.is_some() {
                self.compile_function(func);
            }
        }

        self.target.end_program();
    }

    fn compile_function(&mut self, func: &Function) {
        let ret_ty = func.return_type.clone();

        self.target.start_function(&func.name);
        self.vars.clear();

        for (param_name, param_ty) in &func.params {
            self.target.add_function_param(param_name, param_ty.clone());
            self.vars.insert(param_name.clone(), param_ty.clone());
        }

        self.target.start_function_body();

        if let Some(body) = &func.body {
            self.compile_statements(body);
        }

        if ret_ty == Type::Void {
            self.target.return_void();
        }

        self.target.end_function();
    }

    fn compile_statements(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.compile_statement(stmt);
        }
    }

    fn compile_statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Ret(expr) => {
                let value = self.compile_expr(expr);
                self.target.return_value(value);
            }
            Stmt::Let(name, ty, expr_opt) => {
                self.vars.insert(name.clone(), ty.clone());

                if let Some(expr) = expr_opt {
                    let value = self.compile_expr(expr);
                    self.target.declare_variable(name, ty.clone(), Some(value));
                } else {
                    self.target.declare_variable(name, ty.clone(), None);
                }
            }
            Stmt::Assign(lhs, rhs) => {
                let rhs_val = self.compile_expr(rhs);
                let lhs_addr = self.compile_address(lhs);
                self.target.store(lhs_addr, rhs_val);
            }
            Stmt::Expr(expr) => {
                let value = self.compile_expr(expr);
                self.target.emit_expr_stmt(value);
            }
            Stmt::Break => {
                if let Some((exit_block, _)) = self.loop_stack.last() {
                    self.target.jump(*exit_block);
                } else {
                    panic!("Break statement used outside of a loop");
                }
            }
            Stmt::Continue => {
                if let Some((_, header_block)) = self.loop_stack.last() {
                    self.target.jump(*header_block);
                } else {
                    panic!("Continue statement used outside of a loop");
                }
            }
            Stmt::If(cond_expr, then_stmts, else_stmts) => {
                let condition = self.compile_expr(cond_expr);

                let then_block = self.target.create_block();
                let merge_block = self.target.create_block();

                let else_block = if else_stmts.is_some() {
                    self.target.create_block()
                } else {
                    merge_block
                };

                self.target.branch(condition, then_block, else_block);

                self.target.switch_to_block(then_block);
                self.target.seal_block(then_block);
                self.compile_statements(then_stmts);
                if !self.ends_with_return(then_stmts) {
                    self.target.jump(merge_block);
                }

                if let Some(else_body) = else_stmts {
                    self.target.switch_to_block(else_block);
                    self.target.seal_block(else_block);
                    self.compile_statements(else_body);
                    if !self.ends_with_return(else_body) {
                        self.target.jump(merge_block);
                    }
                }

                self.target.switch_to_block(merge_block);
                self.target.seal_block(merge_block);
            }
            Stmt::While(cond_expr, body_stmts) => {
                let header_block = self.target.create_block();
                let body_block = self.target.create_block();
                let exit_block = self.target.create_block();

                self.loop_stack.push((exit_block, header_block));

                self.target.jump(header_block);

                self.target.switch_to_block(header_block);
                let condition = self.compile_expr(cond_expr);
                self.target.branch(condition, body_block, exit_block);

                self.target.switch_to_block(body_block);
                self.target.seal_block(body_block);
                self.compile_statements(body_stmts);

                self.target.jump(header_block);
                self.target.seal_block(header_block);

                self.loop_stack.pop();

                self.target.switch_to_block(exit_block);
                self.target.seal_block(exit_block);
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr) -> T::Value {
        match expr {
            Expr::Lit(lit) => match lit {
                Lit::Int(val) => self.target.translate_lit_int(*val),
                Lit::Float(val) => self.target.translate_lit_float(*val),
                Lit::Bool(val) => self.target.translate_lit_bool(*val),
                Lit::Str(s) => {
                    let inner_content = if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                        &s[1..s.len() - 1]
                    } else {
                        s.as_str()
                    };

                    let mut processed_bytes = Vec::new();
                    let mut chars = inner_content.chars();

                    while let Some(c) = chars.next() {
                        if c == '\\' {
                            match chars.next() {
                                Some('n') => processed_bytes.push(b'\n'),
                                Some('r') => processed_bytes.push(b'\r'),
                                Some('t') => processed_bytes.push(b'\t'),
                                Some('\\') => processed_bytes.push(b'\\'),
                                Some('"') => processed_bytes.push(b'"'),
                                Some('0') => processed_bytes.push(0),
                                Some(other) => {
                                    processed_bytes.push(b'\\');
                                    processed_bytes.push(other as u8);
                                }
                                None => processed_bytes.push(b'\\'),
                            }
                        } else {
                            processed_bytes.push(c as u8);
                        }
                    }

                    processed_bytes.push(0);

                    let array_str = format!(
                        "{{ {} }}",
                        processed_bytes
                            .iter()
                            .map(|b| b.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    );

                    self.target.emit_value_string(array_str)
                }
                Lit::Array(vals) => {
                    let mut compiled_vals = Vec::new();

                    let element_type = if let Some(first) = vals.first() {
                        self.infer_expr_type(first)
                    } else {
                        Type::I64
                    };
                    for v in vals {
                        compiled_vals.push(self.compile_expr(v));
                    }
                    self.target.translate_lit_array(compiled_vals, element_type)
                }
            },
            Expr::Ident(name) => self.target.translate_ident(name),
            Expr::Binary(lhs, op, rhs) => {
                let left_val = self.compile_expr(lhs);
                let right_val = self.compile_expr(rhs);
                self.target.translate_binary_op(*op, left_val, right_val)
            }
            Expr::Unary(op, expr) => {
                let val = self.compile_expr(expr);
                self.target.translate_unary_op(*op, val)
            }
            Expr::Call(name, args) => {
                let mut compiled_args = Vec::new();
                for arg in args {
                    compiled_args.push(self.compile_expr(arg));
                }
                self.target.call_function(name, compiled_args)
            }
            Expr::Deref(ptr_expr) => {
                if let Expr::AddrOf(inner_expr) = &**ptr_expr {
                    return self.compile_expr(inner_expr);
                }
                let addr = self.compile_expr(ptr_expr);
                self.target.load(addr)
            }
            Expr::AddrOf(inner_expr) => {
                let inner_ty = self.infer_expr_type(inner_expr);

                if let Type::Array(_, _) = inner_ty {
                    self.compile_expr(inner_expr)
                } else {
                    self.compile_address(inner_expr)
                }
            }
            Expr::Index(base, index) => {
                let base_ptr = self.compile_expr(base);
                let index_val = self.compile_expr(index);
                self.target.translate_index(base_ptr, index_val)
            }
            Expr::Cast(expr, target_ty) => {
                let val = self.compile_expr(expr);
                self.target.translate_cast(val, target_ty.clone())
            }
        }
    }

    fn compile_address(&mut self, expr: &Expr) -> T::Value {
        match expr {
            Expr::Ident(name) => self.target.get_variable_address(name),
            Expr::Deref(ptr_expr) => self.compile_expr(ptr_expr),
            Expr::Index(_, _) => {
                let lvalue = self.compile_expr(expr);

                self.target.get_address_of_lvalue(lvalue)
            }
            _ => panic!("Expression cannot be used as an L-Value (address)"),
        }
    }
    fn infer_expr_type(&self, expr: &Expr) -> Type {
        match expr {
            Expr::Lit(lit) => match lit {
                Lit::Float(_) => Type::F64,
                Lit::Bool(_) => Type::Bool,
                Lit::Int(_) => Type::I64,
                Lit::Str(_) => Type::Pointer(Box::new(Type::U8)),
                Lit::Array(vals) => {
                    if let Some(first) = vals.first() {
                        Type::Array(Box::new(self.infer_expr_type(first)), vals.len())
                    } else {
                        Type::Array(Box::new(Type::I64), 0)
                    }
                }
            },
            Expr::Binary(_, op, _) => match op {
                BinaryOp::Eq
                | BinaryOp::Neq
                | BinaryOp::Lt
                | BinaryOp::Gt
                | BinaryOp::Lte
                | BinaryOp::Gte
                | BinaryOp::And
                | BinaryOp::Or => Type::Bool,
                _ => Type::I64,
            },
            Expr::Ident(name) => self.vars.get(name).cloned().unwrap_or(Type::I64),
            Expr::Unary(op, _) => match op {
                UnaryOp::Not => Type::Bool,
                _ => Type::I64,
            },
            Expr::Call(_, _) => Type::I64,

            Expr::AddrOf(inner) => Type::Pointer(Box::new(self.infer_expr_type(inner))),

            Expr::Deref(inner) => {
                if let Type::Pointer(t) = self.infer_expr_type(inner) {
                    *t
                } else {
                    Type::I64
                }
            }
            Expr::Index(base, _) => {
                if let Type::Array(t, _) = self.infer_expr_type(base) {
                    *t
                } else {
                    Type::I64
                }
            }
            Expr::Cast(_, ty) => ty.clone(),
        }
    }

    fn ends_with_return(&self, stmts: &[Stmt]) -> bool {
        if let Some(last) = stmts.last() {
            matches!(last, Stmt::Ret(_))
        } else {
            false
        }
    }
}
