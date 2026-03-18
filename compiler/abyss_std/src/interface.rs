use abyss_vm::vm::types::NativeFunction;

pub struct NativeFunctionDef {
    pub name: &'static str,
    pub arity: usize,
    pub func: NativeFunction,
}

pub trait AbyssLibrary {
    fn get_functions() -> &'static [NativeFunctionDef];
}
