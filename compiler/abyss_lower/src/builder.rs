pub trait TypeBuilder {
    type Type: Copy;

    fn type_int(&mut self, bits: u16) -> Self::Type;
    fn type_uint(&mut self, bits: u16) -> Self::Type;
    fn type_float(&mut self, bits: u16) -> Self::Type;
    fn type_bool(&mut self) -> Self::Type;
    fn type_never(&mut self) -> Self::Type;
    fn type_ptr(&mut self, pointee: Option<Self::Type>) -> Self::Type;
    fn type_unit(&mut self) -> Self::Type;
    fn type_func(&mut self, params: &[Self::Type], ret: Self::Type) -> Self::Type;
}

pub trait ModuleBuilder: TypeBuilder {
    type FuncId: Copy;
    type FuncBuilder<'a>: FunctionBuilder<Type = Self::Type>
    where
        Self: 'a;

    fn declare_func(&mut self, name: &str, params: &[Self::Type], ret: Self::Type) -> Self::FuncId;

    fn define_func<'a>(&'a mut self, func: Self::FuncId) -> Self::FuncBuilder<'a>;
}

pub trait FunctionBuilder: TypeBuilder {
    type Value: Copy;
    type BasicBlock: Copy;
    type Var: Copy;

    fn create_block(&mut self) -> Self::BasicBlock;
    fn switch_to_block(&mut self, block: Self::BasicBlock);
    fn seal_block(&mut self, block: Self::BasicBlock);
    fn append_block_params_for_function_params(&mut self, block: Self::BasicBlock);

    // Var
    fn declare_var(&mut self, ty: Self::Type) -> Self::Var;
    fn def_var(&mut self, var: Self::Var, value: Self::Value);
    fn use_var(&mut self, var: Self::Var) -> Self::Value;

    // Const
    fn ins_iconst(&mut self, ty: Self::Type, val: i64) -> Self::Value;
    fn ins_fconst(&mut self, ty: Self::Type, val: f64) -> Self::Value;

    // Math -> Integers
    fn ins_iadd(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    fn ins_isub(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    fn ins_imul(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    fn ins_sdiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    fn ins_udiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    // Math -> Floats
    fn ins_fadd(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    fn ins_fsub(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    fn ins_fmul(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    fn ins_fdiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;

    fn ins_ret(&mut self, value: Option<Self::Value>);

    fn get_param(&self, index: usize) -> Self::Value;
    fn finish(self);
}
