use crate::target::{Target, Type};
pub use abyss_parser::ast::Type as AstType;
use abyss_parser::ast::{Function, Program};

pub struct Director<'a, T: Target> {
    target: &'a mut T,
}

impl<'a, T: Target> Director<'a, T> {
    pub fn new(target: &'a mut T) -> Self {
        Self { target }
    }

    pub fn generate_code(&mut self) -> String {
        self.target.generate_code()
    }

    pub fn process_program(&mut self, program: &Program) {
        self.target.start_program();

        for func in &program.functions {
            self.compile_function(func);
        }

        self.target.end_program();
    }

    fn compile_function(&mut self, func: &Function) {
        let ret_ty = if let Some(ref ast_ty) = func.return_type {
            self.map_type(ast_ty)
        } else {
            Type::Void
        };

        self.target.start_function(&func.name.name, ret_ty);

        for param in &func.params {
            let param_ty = self.map_type(&param.ty);
            self.target.add_function_arg(&param.name.name, param_ty);
        }

        self.target.start_function_body();

        self.target.end_function();
    }

    fn map_type(&self, ast_ty: &AstType) -> Type {
        match ast_ty {
            AstType::Named(ident) => match ident.name.as_str() {
                "i32" => Type::I32,
                "u32" => Type::U32,
                "f32" => Type::F32,
                "void" => Type::Void,
                _ => Type::Void,
            },
            AstType::Pointer(inner) => Type::Ptr(Box::new(self.map_type(inner))),
        }
    }
}
