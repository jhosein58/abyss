pub trait TypeBuilder {
    type Type: Copy;

    fn type_int(&mut self, bits: u16) -> Self::Type;
    fn type_uint(&mut self, bits: u16) -> Self::Type;
    fn type_float(&mut self, bits: u16) -> Self::Type;
    fn type_bool(&mut self) -> Self::Type;
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

pub trait FunctionBuilder {
    type Type: Copy;
    type Value: Copy;
    type BasicBlock: Copy;

    fn create_block(&mut self) -> Self::BasicBlock;

    fn switch_to_block(&mut self, block: Self::BasicBlock);

    fn ins_iconst(&mut self, ty: Self::Type, val: i64) -> Self::Value;
    fn ins_iadd(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
    fn ins_ret(&mut self, values: Option<Self::Value>);

    fn get_param(&self, index: usize) -> Self::Value;
    fn finish(self);
}
