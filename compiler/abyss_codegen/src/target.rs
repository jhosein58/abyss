use abyss_parser::ast::{BinaryOp, Type, UnaryOp};

pub trait Target {
    type Value: Copy + Clone;
    type Block: Copy + Clone;

    // --- Function Management ---
    fn start_program(&mut self);
    fn end_program(&mut self);

    fn declare_extern_function(&mut self, name: &str, params: &[(String, Type)], return_type: Type);
    fn predefine_function(&mut self, name: &str, params: &[(String, Type)], return_type: Type);

    fn start_function(&mut self, name: &str);
    fn add_function_param(&mut self, name: &str, ty: Type);
    fn start_function_body(&mut self);
    fn end_function(&mut self);

    fn call_function(&mut self, name: &str, args: Vec<Self::Value>) -> Self::Value;
    fn return_value(&mut self, value: Self::Value);
    fn return_void(&mut self);

    // --- Memory & Variables ---
    fn declare_variable(&mut self, name: &str, ty: Type, value: Option<Self::Value>);

    fn get_variable_address(&mut self, name: &str) -> Self::Value;

    fn store(&mut self, address: Self::Value, value: Self::Value);

    fn load(&mut self, address: Self::Value) -> Self::Value;

    // --- Control Flow ---
    fn create_block(&mut self) -> Self::Block;
    fn seal_block(&mut self, block: Self::Block);
    fn switch_to_block(&mut self, block: Self::Block);
    fn jump(&mut self, target_block: Self::Block);
    fn branch(&mut self, condition: Self::Value, then_block: Self::Block, else_block: Self::Block);

    // --- Literals ---
    fn translate_lit_int(&mut self, value: i64) -> Self::Value;
    fn translate_lit_float(&mut self, value: f64) -> Self::Value;
    fn translate_lit_bool(&mut self, value: bool) -> Self::Value;
    fn translate_lit_string(&mut self, value: &str) -> Self::Value;
    fn translate_lit_array(&mut self, values: Vec<Self::Value>, element_ty: Type) -> Self::Value;

    // --- Operations ---
    fn translate_ident(&mut self, name: &str) -> Self::Value;

    fn translate_binary_op(
        &mut self,
        op: BinaryOp,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Self::Value;

    fn translate_unary_op(&mut self, op: UnaryOp, value: Self::Value) -> Self::Value;

    fn translate_cast(&mut self, value: Self::Value, target_type: Type) -> Self::Value;

    fn translate_index(&mut self, ptr: Self::Value, index: Self::Value) -> Self::Value;
    fn emit_expr_stmt(&mut self, value: Self::Value);
    fn emit_value_string(&mut self, value: String) -> Self::Value;
    fn get_address_of_lvalue(&mut self, lvalue: Self::Value) -> Self::Value;
}
