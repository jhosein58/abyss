use super::AbyssCompiler;
use abyss_ir::ir::{IrExpr, IrStmt, IrType};
use inkwell::values::FunctionValue;

impl<'ctx> AbyssCompiler<'ctx> {
    pub(crate) fn compile_stmt(&mut self, stmt: &IrStmt, current_func: FunctionValue<'ctx>) {
        match stmt {
            IrStmt::VarDec { name, ty, init } => self.compile_var_dec(name, ty, init),
            IrStmt::ConstDef { name, ty, value } => self.compile_const_def(name, ty, value),
            IrStmt::Assign { target, val } => self.compile_assign(target, val),
            IrStmt::WriteIndex { base, index, val } => self.compile_write_index(base, index, val),
            IrStmt::WritePointer { ptr, val } => self.compile_write_pointer(ptr, val),
            IrStmt::Expr(expr) => self.compile_expr_stmt(expr),
            IrStmt::Return(expr) => self.compile_return(expr),
            IrStmt::If(cond, then_body, else_body) => {
                self.compile_if(cond, then_body, else_body, current_func)
            }
            IrStmt::While { cond, body } => self.compile_while(cond, body, current_func),
            IrStmt::Break => self.compile_break(),
            IrStmt::WriteField { base, index, val } => self.compile_write_field(base, index, val),
            IrStmt::WriteUnion { base, index, val } => self.compile_write_union(base, index, val),
        }
    }

    fn compile_var_dec(&mut self, name: &String, ty: &IrType, init: &Option<IrExpr>) {
        let ll_ty = self.compile_type(ty);
        let alloca = self.builder.build_alloca(ll_ty, name).unwrap();
        self.variables.insert(name.clone(), alloca);
        if let Some(expr) = init {
            if let Some(val) = self.compile_expr(expr) {
                self.builder.build_store(alloca, val).unwrap();
            }
        }
    }

    fn compile_const_def(&mut self, name: &String, ty: &IrType, value: &IrExpr) {
        let ll_ty = self.compile_type(ty);
        let alloca = self.builder.build_alloca(ll_ty, name).unwrap();
        self.variables.insert(name.clone(), alloca);
        if let Some(val) = self.compile_expr(value) {
            self.builder.build_store(alloca, val).unwrap();
        }
    }

    fn compile_assign(&mut self, target: &String, val: &IrExpr) {
        let ptr = *self.variables.get(target).unwrap();
        if let Some(value) = self.compile_expr(val) {
            self.builder.build_store(ptr, value).unwrap();
        }
    }

    fn compile_write_index(&mut self, base: &IrExpr, index: &IrExpr, val: &IrExpr) {
        if let Some(base_ptr) = self.get_lvalue_ptr(base) {
            if let Some(index_val) = self.compile_expr(index) {
                if let Some(value) = self.compile_expr(val) {
                    let base_ll_ty = self.compile_type(&base.ty);
                    let gep = unsafe {
                        let zero = self.context.i32_type().const_zero();
                        self.builder
                            .build_gep(
                                base_ll_ty,
                                base_ptr,
                                &[zero, index_val.into_int_value()],
                                "",
                            )
                            .unwrap()
                    };
                    self.builder.build_store(gep, value).unwrap();
                }
            }
        }
    }

    fn compile_write_pointer(&mut self, ptr: &IrExpr, val: &IrExpr) {
        if let Some(ptr_val) = self.compile_expr(ptr) {
            if let Some(value) = self.compile_expr(val) {
                self.builder
                    .build_store(ptr_val.into_pointer_value(), value)
                    .unwrap();
            }
        }
    }

    fn compile_expr_stmt(&mut self, expr: &IrExpr) {
        self.compile_expr(expr);
    }

    fn compile_return(&mut self, expr: &Option<IrExpr>) {
        if let Some(e) = expr {
            if let Some(val) = self.compile_expr(e) {
                self.builder.build_return(Some(&val)).unwrap();
            }
        } else {
            self.builder.build_return(None).unwrap();
        }
    }

    fn compile_if(
        &mut self,
        cond: &IrExpr,
        then_body: &[IrStmt],
        else_body: &[IrStmt],
        current_func: FunctionValue<'ctx>,
    ) {
        let cond_val = self.compile_expr(cond).unwrap().into_int_value();
        let then_bb = self.context.append_basic_block(current_func, "then");
        let else_bb = self.context.append_basic_block(current_func, "else");
        let merge_bb = self.context.append_basic_block(current_func, "merge");

        self.builder
            .build_conditional_branch(cond_val, then_bb, else_bb)
            .unwrap();

        self.builder.position_at_end(then_bb);
        for s in then_body {
            self.compile_stmt(s, current_func);
            if self
                .builder
                .get_insert_block()
                .unwrap()
                .get_terminator()
                .is_some()
            {
                break;
            }
        }

        if self
            .builder
            .get_insert_block()
            .unwrap()
            .get_terminator()
            .is_none()
        {
            self.builder.build_unconditional_branch(merge_bb).unwrap();
        }

        self.builder.position_at_end(else_bb);
        for s in else_body {
            self.compile_stmt(s, current_func);
            if self
                .builder
                .get_insert_block()
                .unwrap()
                .get_terminator()
                .is_some()
            {
                break;
            }
        }
        if self
            .builder
            .get_insert_block()
            .unwrap()
            .get_terminator()
            .is_none()
        {
            self.builder.build_unconditional_branch(merge_bb).unwrap();
        }

        self.builder.position_at_end(merge_bb);
    }

    fn compile_while(&mut self, cond: &IrExpr, body: &[IrStmt], current_func: FunctionValue<'ctx>) {
        let cond_bb = self.context.append_basic_block(current_func, "while_cond");
        let body_bb = self.context.append_basic_block(current_func, "while_body");
        let merge_bb = self.context.append_basic_block(current_func, "while_merge");

        self.builder.build_unconditional_branch(cond_bb).unwrap();
        self.builder.position_at_end(cond_bb);

        let cond_val = self.compile_expr(cond).unwrap().into_int_value();
        self.builder
            .build_conditional_branch(cond_val, body_bb, merge_bb)
            .unwrap();

        self.builder.position_at_end(body_bb);
        self.loop_targets.push(merge_bb);

        for s in body {
            self.compile_stmt(s, current_func);
            if self
                .builder
                .get_insert_block()
                .unwrap()
                .get_terminator()
                .is_some()
            {
                break;
            }
        }

        self.loop_targets.pop();

        if self
            .builder
            .get_insert_block()
            .unwrap()
            .get_terminator()
            .is_none()
        {
            self.builder.build_unconditional_branch(cond_bb).unwrap();
        }

        self.builder.position_at_end(merge_bb);
    }

    fn compile_break(&mut self) {
        if let Some(target) = self.loop_targets.last() {
            self.builder.build_unconditional_branch(*target).unwrap();
        }
    }

    fn compile_write_field(&mut self, base: &IrExpr, index: &usize, val: &IrExpr) {
        if let Some(base_ptr) = self.get_lvalue_ptr(base) {
            if let Some(value) = self.compile_expr(val) {
                let base_ll_ty = self.compile_type(&base.ty);
                let gep = self
                    .builder
                    .build_struct_gep(base_ll_ty, base_ptr, *index as u32, "")
                    .unwrap();
                self.builder.build_store(gep, value).unwrap();
            }
        }
    }
    fn compile_write_union(&mut self, base: &IrExpr, _index: &usize, val: &IrExpr) {
        if let Some(base_ptr) = self.get_lvalue_ptr(base) {
            if let Some(value) = self.compile_expr(val) {
                self.builder.build_store(base_ptr, value).unwrap();
            }
        }
    }
}
