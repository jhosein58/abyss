use super::AbyssCompiler;
use abyss_ir::ir::{IrBinaryOp, IrExpr, IrExprKind, IrLit, IrType, IrUnaryOp};
use inkwell::types::BasicTypeEnum;
use inkwell::values::{AnyValue, BasicValueEnum, FloatValue, IntValue, PointerValue};
use inkwell::{FloatPredicate, IntPredicate};
use std::convert::TryFrom;

impl<'ctx> AbyssCompiler<'ctx> {
    pub(crate) fn compile_expr(&mut self, expr: &IrExpr) -> Option<BasicValueEnum<'ctx>> {
        match &expr.kind {
            IrExprKind::Lit(lit) => self.compile_lit(lit, &expr.ty),

            IrExprKind::VarRef(name) => self.compile_var_ref(name, &expr.ty),
            IrExprKind::Unary(op, rhs) => self.compile_unary(op, rhs, &expr.ty),
            IrExprKind::Binary(lhs, op, rhs) => self.compile_binary(lhs, op, rhs),
            IrExprKind::Cast(inner, target_ty) => self.compile_cast(inner, target_ty),
            IrExprKind::Call { func_name, args } => self.compile_call(func_name, args),

            IrExprKind::ArrayInit(items) => self.compile_array_init(items, &expr.ty),
            IrExprKind::ArrayRepeat { val, count } => {
                self.compile_array_repeat(val, count, &expr.ty)
            }
            IrExprKind::Index(base, index) => self.compile_index(base, index, &expr.ty),
            IrExprKind::StructInit(fields) => self.compile_struct_init(fields, &expr.ty),
            IrExprKind::FieldAccess { base, index } => {
                self.compile_field_access(base, index, &expr.ty)
            }
            IrExprKind::GetIndexPtr { base, index } => self.compile_get_index_ptr(base, index),
            IrExprKind::GetFieldPtr { base, index } => self.compile_get_field_ptr(base, index),

            IrExprKind::FuncAddr(name) => self.compile_func_addr(name),
            IrExprKind::CallIndirect { ptr, args } => {
                self.compile_call_indirect(ptr, args, &expr.ty)
            }
        }
    }

    fn compile_lit(&self, lit: &IrLit, expr_ty: &IrType) -> Option<BasicValueEnum<'ctx>> {
        let ll_ty = self.compile_type(expr_ty);

        match lit {
            IrLit::Int(n) => {
                if ll_ty.is_int_type() {
                    Some(ll_ty.into_int_type().const_int(*n as u64, true).into())
                } else {
                    Some(self.context.i32_type().const_int(*n as u64, true).into())
                }
            }
            IrLit::Float(f) => {
                if ll_ty.is_float_type() {
                    Some(ll_ty.into_float_type().const_float(*f).into())
                } else {
                    Some(self.context.f64_type().const_float(*f).into())
                }
            }
            IrLit::Bool(b) => {
                if ll_ty.is_int_type() {
                    Some(
                        ll_ty
                            .into_int_type()
                            .const_int(if *b { 1 } else { 0 }, false)
                            .into(),
                    )
                } else {
                    Some(
                        self.context
                            .bool_type()
                            .const_int(if *b { 1 } else { 0 }, false)
                            .into(),
                    )
                }
            }
        }
    }

    fn compile_var_ref(&self, name: &String, ty: &IrType) -> Option<BasicValueEnum<'ctx>> {
        let ll_ty = self.compile_type(ty);

        if let Some(ptr) = self.variables.get(name) {
            Some(self.builder.build_load(ll_ty, *ptr, name).unwrap())
        } else if let Some(global_var) = self.module.get_global(name) {
            let ptr = global_var.as_pointer_value();
            Some(self.builder.build_load(ll_ty, ptr, name).unwrap())
        } else if let Some(function) = self.module.get_function(name) {
            Some(function.as_global_value().as_pointer_value().into())
        } else {
            panic!(
                "LLVM Codegen Error: Cannot find variable, global constant, or function named '{}'",
                name
            );
        }
    }

    fn compile_unary(
        &mut self,
        op: &IrUnaryOp,
        rhs: &IrExpr,
        expr_ty: &IrType,
    ) -> Option<BasicValueEnum<'ctx>> {
        let rhs_val = self.compile_expr(rhs)?;
        match op {
            IrUnaryOp::Neg => {
                if rhs_val.is_float_value() {
                    Some(
                        self.builder
                            .build_float_neg(rhs_val.into_float_value(), "")
                            .unwrap()
                            .into(),
                    )
                } else {
                    Some(
                        self.builder
                            .build_int_neg(rhs_val.into_int_value(), "")
                            .unwrap()
                            .into(),
                    )
                }
            }
            IrUnaryOp::Not | IrUnaryOp::BitNot => Some(
                self.builder
                    .build_not(rhs_val.into_int_value(), "")
                    .unwrap()
                    .into(),
            ),
            IrUnaryOp::Ref => match &rhs.kind {
                IrExprKind::VarRef(name) => Some((*self.variables.get(name).unwrap()).into()),
                _ => None,
            },
            IrUnaryOp::Deref => {
                let ptr = rhs_val.into_pointer_value();
                let ll_ty = self.compile_type(expr_ty);
                Some(self.builder.build_load(ll_ty, ptr, "").unwrap())
            }
        }
    }

    fn compile_binary(
        &mut self,
        lhs: &IrExpr,
        op: &IrBinaryOp,
        rhs: &IrExpr,
    ) -> Option<BasicValueEnum<'ctx>> {
        let lhs_val = self.compile_expr(lhs)?;
        let rhs_val = self.compile_expr(rhs)?;

        if lhs_val.is_int_value() {
            self.compile_int_binary(lhs_val.into_int_value(), op, rhs_val.into_int_value())
        } else if lhs_val.is_float_value() {
            self.compile_float_binary(lhs_val.into_float_value(), op, rhs_val.into_float_value())
        } else {
            None
        }
    }

    fn compile_int_binary(
        &self,
        l: IntValue<'ctx>,
        op: &IrBinaryOp,
        r: IntValue<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> {
        match op {
            IrBinaryOp::Add => Some(self.builder.build_int_add(l, r, "").unwrap().into()),
            IrBinaryOp::Sub => Some(self.builder.build_int_sub(l, r, "").unwrap().into()),
            IrBinaryOp::Mul => Some(self.builder.build_int_mul(l, r, "").unwrap().into()),
            IrBinaryOp::Div => Some(self.builder.build_int_signed_div(l, r, "").unwrap().into()),
            IrBinaryOp::Mod => Some(self.builder.build_int_signed_rem(l, r, "").unwrap().into()),
            IrBinaryOp::Eq => Some(
                self.builder
                    .build_int_compare(IntPredicate::EQ, l, r, "")
                    .unwrap()
                    .into(),
            ),
            IrBinaryOp::Neq => Some(
                self.builder
                    .build_int_compare(IntPredicate::NE, l, r, "")
                    .unwrap()
                    .into(),
            ),
            IrBinaryOp::Lt => Some(
                self.builder
                    .build_int_compare(IntPredicate::SLT, l, r, "")
                    .unwrap()
                    .into(),
            ),
            IrBinaryOp::Le => Some(
                self.builder
                    .build_int_compare(IntPredicate::SLE, l, r, "")
                    .unwrap()
                    .into(),
            ),
            IrBinaryOp::Gt => Some(
                self.builder
                    .build_int_compare(IntPredicate::SGT, l, r, "")
                    .unwrap()
                    .into(),
            ),
            IrBinaryOp::Ge => Some(
                self.builder
                    .build_int_compare(IntPredicate::SGE, l, r, "")
                    .unwrap()
                    .into(),
            ),
            IrBinaryOp::And | IrBinaryOp::BitAnd => {
                Some(self.builder.build_and(l, r, "").unwrap().into())
            }
            IrBinaryOp::Or | IrBinaryOp::BitOr => {
                Some(self.builder.build_or(l, r, "").unwrap().into())
            }
            IrBinaryOp::BitXor => Some(self.builder.build_xor(l, r, "").unwrap().into()),
            IrBinaryOp::Shl => Some(self.builder.build_left_shift(l, r, "").unwrap().into()),
            IrBinaryOp::Shr => Some(
                self.builder
                    .build_right_shift(l, r, false, "")
                    .unwrap()
                    .into(),
            ),
        }
    }

    fn compile_float_binary(
        &self,
        l: FloatValue<'ctx>,
        op: &IrBinaryOp,
        r: FloatValue<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> {
        match op {
            IrBinaryOp::Add => Some(self.builder.build_float_add(l, r, "").unwrap().into()),
            IrBinaryOp::Sub => Some(self.builder.build_float_sub(l, r, "").unwrap().into()),
            IrBinaryOp::Mul => Some(self.builder.build_float_mul(l, r, "").unwrap().into()),
            IrBinaryOp::Div => Some(self.builder.build_float_div(l, r, "").unwrap().into()),
            IrBinaryOp::Mod => Some(self.builder.build_float_rem(l, r, "").unwrap().into()),
            IrBinaryOp::Eq => Some(
                self.builder
                    .build_float_compare(FloatPredicate::OEQ, l, r, "")
                    .unwrap()
                    .into(),
            ),
            IrBinaryOp::Neq => Some(
                self.builder
                    .build_float_compare(FloatPredicate::ONE, l, r, "")
                    .unwrap()
                    .into(),
            ),
            IrBinaryOp::Lt => Some(
                self.builder
                    .build_float_compare(FloatPredicate::OLT, l, r, "")
                    .unwrap()
                    .into(),
            ),
            IrBinaryOp::Le => Some(
                self.builder
                    .build_float_compare(FloatPredicate::OLE, l, r, "")
                    .unwrap()
                    .into(),
            ),
            IrBinaryOp::Gt => Some(
                self.builder
                    .build_float_compare(FloatPredicate::OGT, l, r, "")
                    .unwrap()
                    .into(),
            ),
            IrBinaryOp::Ge => Some(
                self.builder
                    .build_float_compare(FloatPredicate::OGE, l, r, "")
                    .unwrap()
                    .into(),
            ),
            _ => None,
        }
    }

    fn compile_cast(&mut self, inner: &IrExpr, target_ty: &IrType) -> Option<BasicValueEnum<'ctx>> {
        let inner_val = self.compile_expr(inner)?;
        let dest_ty = self.compile_type(target_ty);
        if inner_val.is_int_value() && dest_ty.is_int_type() {
            Some(
                self.builder
                    .build_int_cast(inner_val.into_int_value(), dest_ty.into_int_type(), "")
                    .unwrap()
                    .into(),
            )
        } else if inner_val.is_float_value() && dest_ty.is_float_type() {
            Some(
                self.builder
                    .build_float_cast(inner_val.into_float_value(), dest_ty.into_float_type(), "")
                    .unwrap()
                    .into(),
            )
        } else {
            None
        }
    }

    fn compile_call(
        &mut self,
        func_name: &String,
        args: &[IrExpr],
    ) -> Option<BasicValueEnum<'ctx>> {
        let function = self
            .module
            .get_function(func_name)
            .unwrap_or_else(|| panic!("LLVM Codegen Error: Function '{}' not found", func_name));

        let mut compiled_args = Vec::new();
        for arg in args {
            compiled_args.push(self.compile_expr(arg)?.into());
        }

        let call_site = self
            .builder
            .build_call(function, &compiled_args, "")
            .unwrap();

        BasicValueEnum::try_from(call_site.as_any_value_enum()).ok()
    }

    fn compile_array_init(
        &mut self,
        items: &[IrExpr],
        expr_ty: &IrType,
    ) -> Option<BasicValueEnum<'ctx>> {
        let ll_ty = self.compile_type(expr_ty);
        let alloca = self.builder.build_alloca(ll_ty, "array_init").unwrap();
        let zero = self.context.i32_type().const_zero();

        for (i, item) in items.iter().enumerate() {
            let item_val = self.compile_expr(item)?;
            let gep = unsafe {
                self.builder
                    .build_gep(
                        ll_ty,
                        alloca,
                        &[zero, self.context.i32_type().const_int(i as u64, false)],
                        "",
                    )
                    .unwrap()
            };
            self.builder.build_store(gep, item_val).unwrap();
        }
        Some(self.builder.build_load(ll_ty, alloca, "").unwrap())
    }
    fn compile_array_repeat(
        &mut self,
        val: &IrExpr,
        count: &usize,
        expr_ty: &IrType,
    ) -> Option<BasicValueEnum<'ctx>> {
        let ll_ty = self.compile_type(expr_ty);
        let alloca = self.builder.build_alloca(ll_ty, "array_repeat").unwrap();
        let repeat_val = self.compile_expr(val)?;
        let zero = self.context.i32_type().const_zero();

        for i in 0..*count {
            let gep = unsafe {
                self.builder
                    .build_gep(
                        ll_ty,
                        alloca,
                        &[zero, self.context.i32_type().const_int(i as u64, false)],
                        "",
                    )
                    .unwrap()
            };
            self.builder.build_store(gep, repeat_val).unwrap();
        }
        Some(self.builder.build_load(ll_ty, alloca, "").unwrap())
    }

    fn compile_struct_init(
        &mut self,
        fields: &[IrExpr],
        expr_ty: &IrType,
    ) -> Option<BasicValueEnum<'ctx>> {
        let ll_ty = self.compile_type(expr_ty);
        let alloca = self.builder.build_alloca(ll_ty, "struct_init").unwrap();
        for (i, field_expr) in fields.iter().enumerate() {
            let field_val = self.compile_expr(field_expr)?;
            let gep = self
                .builder
                .build_struct_gep(ll_ty, alloca, i as u32, "")
                .unwrap();
            self.builder.build_store(gep, field_val).unwrap();
        }
        Some(self.builder.build_load(ll_ty, alloca, "").unwrap())
    }

    fn compile_index(
        &mut self,
        base: &IrExpr,
        index: &IrExpr,
        expr_ty: &IrType,
    ) -> Option<BasicValueEnum<'ctx>> {
        let base_ptr = self.get_lvalue_ptr(base)?;
        let index_val = self.compile_expr(index)?.into_int_value();
        let base_ll_ty = self.compile_type(&base.ty);
        let zero = self.context.i32_type().const_zero();

        let gep = unsafe {
            self.builder
                .build_gep(base_ll_ty, base_ptr, &[zero, index_val], "")
                .unwrap()
        };
        let res_ty = self.compile_type(expr_ty);
        Some(self.builder.build_load(res_ty, gep, "").unwrap())
    }

    fn compile_field_access(
        &mut self,
        base: &IrExpr,
        index: &usize,
        expr_ty: &IrType,
    ) -> Option<BasicValueEnum<'ctx>> {
        let ptr = self
            .compile_get_field_ptr(base, index)?
            .into_pointer_value();
        let res_ty = self.compile_type(expr_ty);

        Some(self.builder.build_load(res_ty, ptr, "").unwrap())
    }

    fn compile_get_index_ptr(
        &mut self,
        base: &IrExpr,
        index: &IrExpr,
    ) -> Option<BasicValueEnum<'ctx>> {
        let base_ptr = self.get_lvalue_ptr(base)?;
        let index_val = self.compile_expr(index)?.into_int_value();
        let base_ll_ty = self.compile_type(&base.ty);
        let zero = self.context.i32_type().const_zero();

        let gep = unsafe {
            self.builder
                .build_gep(base_ll_ty, base_ptr, &[zero, index_val], "")
                .unwrap()
        };
        Some(gep.into())
    }

    fn compile_get_field_ptr(
        &mut self,
        base: &IrExpr,
        index: &usize,
    ) -> Option<BasicValueEnum<'ctx>> {
        let base_ptr = self.get_lvalue_ptr(base)?;

        if let IrType::Union(_) = base.ty {
            return Some(base_ptr.into());
        }

        let base_ll_ty = self.compile_type(&base.ty);
        let gep = self
            .builder
            .build_struct_gep(base_ll_ty, base_ptr, *index as u32, "")
            .unwrap();
        Some(gep.into())
    }

    pub(crate) fn get_lvalue_ptr(&mut self, expr: &IrExpr) -> Option<PointerValue<'ctx>> {
        match &expr.kind {
            IrExprKind::VarRef(name) => {
                if let Some(ptr) = self.variables.get(name) {
                    Some(*ptr)
                } else if let Some(global_var) = self.module.get_global(name) {
                    Some(global_var.as_pointer_value())
                } else {
                    panic!("Cannot find lvalue for {}", name);
                }
            }
            IrExprKind::Index(base, index) => self
                .compile_get_index_ptr(base, index)
                .map(|v| v.into_pointer_value()),
            IrExprKind::FieldAccess { base, index } => self
                .compile_get_field_ptr(base, index)
                .map(|v| v.into_pointer_value()),
            _ => {
                let val = self.compile_expr(expr)?;
                if val.is_pointer_value() {
                    Some(val.into_pointer_value())
                } else {
                    let ll_ty = self.compile_type(&expr.ty);
                    let alloca = self.builder.build_alloca(ll_ty, "temp_lvalue").unwrap();
                    self.builder.build_store(alloca, val).unwrap();
                    Some(alloca)
                }
            }
        }
    }

    fn compile_func_addr(&self, name: &String) -> Option<BasicValueEnum<'ctx>> {
        let function = self
            .module
            .get_function(name)
            .unwrap_or_else(|| panic!("LLVM Codegen Error: Function '{}' not found", name));

        Some(function.as_global_value().as_pointer_value().into())
    }

    fn compile_call_indirect(
        &mut self,
        ptr_expr: &IrExpr,
        args: &[IrExpr],
        _ret_ty: &IrType,
    ) -> Option<BasicValueEnum<'ctx>> {
        let ptr_val = self.compile_expr(ptr_expr)?.into_pointer_value();

        let mut compiled_args = Vec::new();
        for arg in args {
            compiled_args.push(self.compile_expr(arg)?.into());
        }

        let fn_type = if let IrType::FuncPtr { params, ret } = &ptr_expr.ty {
            let param_ll_tys: Vec<_> = params.iter().map(|p| self.compile_type(p).into()).collect();

            if **ret == IrType::Unit {
                self.context.void_type().fn_type(&param_ll_tys, false)
            } else {
                let ret_ll_ty = self.compile_type(ret);
                match ret_ll_ty {
                    BasicTypeEnum::IntType(t) => t.fn_type(&param_ll_tys, false),
                    BasicTypeEnum::FloatType(t) => t.fn_type(&param_ll_tys, false),
                    BasicTypeEnum::PointerType(t) => t.fn_type(&param_ll_tys, false),
                    BasicTypeEnum::StructType(t) => t.fn_type(&param_ll_tys, false),
                    BasicTypeEnum::ArrayType(t) => t.fn_type(&param_ll_tys, false),
                    BasicTypeEnum::VectorType(t) => t.fn_type(&param_ll_tys, false),
                    BasicTypeEnum::ScalableVectorType(t) => t.fn_type(&param_ll_tys, false),
                }
            }
        } else {
            panic!("LLVM Codegen Error: CallIndirect pointer expression is not of type FuncPtr");
        };

        let call_site = self
            .builder
            .build_indirect_call(fn_type, ptr_val, &compiled_args, "")
            .unwrap();

        BasicValueEnum::try_from(call_site.as_any_value_enum()).ok()
    }
}
