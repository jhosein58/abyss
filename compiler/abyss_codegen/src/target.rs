pub enum Type {
    I32,
    U32,
    F32,
    Void,
    Ptr(Box<Type>),
}

pub trait Target {
    fn generate_code(&mut self) -> String;

    fn start_program(&mut self);
    fn end_program(&mut self);

    fn start_function(&mut self, name: &str, ret_type: Type);
    fn add_function_arg(&mut self, name: &str, ty: Type);
    fn start_function_body(&mut self);
    fn end_function(&mut self);
}
