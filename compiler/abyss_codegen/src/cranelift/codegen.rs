use abyss_lower::builder::{FunctionBuilder, ModuleBuilder, TypeBuilder};
use cranelift::codegen::ir::immediates::Ieee16;
use cranelift::module::{FuncId, Linkage};
use cranelift::prelude::*;

use cranelift::{
    codegen::Context,
    jit::{JITBuilder, JITModule},
    module::{Module, default_libcall_names},
};

pub struct CraneliftBackend {
    pub module: JITModule,
    pub ctx: Context,
    pub fn_builder_ctx: FunctionBuilderContext,
}

impl CraneliftBackend {
    pub fn new() -> Self {
        let builder = JITBuilder::new(default_libcall_names()).unwrap();
        let module = JITModule::new(builder);
        let ctx = module.make_context();
        let fn_builder_ctx = FunctionBuilderContext::new();

        Self {
            module,
            ctx,
            fn_builder_ctx,
        }
    }

    pub fn compile_and_get_ptr(&mut self, func_id: FuncId) -> *const u8 {
        self.module
            .define_function(func_id, &mut self.ctx)
            .expect("Failed to define function");

        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions().unwrap();

        self.module.get_finalized_function(func_id)
    }
}

impl TypeBuilder for CraneliftBackend {
    type Type = Type;

    fn type_int(&mut self, bits: u16) -> Self::Type {
        match bits {
            8 => types::I8,
            16 => types::I16,
            32 => types::I32,
            64 => types::I64,
            128 => types::I128,
            _ => panic!("unsupported type"),
        }
    }

    fn type_uint(&mut self, bits: u16) -> Self::Type {
        self.type_int(bits)
    }

    fn type_float(&mut self, bits: u16) -> Self::Type {
        match bits {
            16 => types::F16,
            32 => types::F32,
            64 => types::F64,
            128 => types::F128,
            _ => panic!("unsupported type"),
        }
    }

    fn type_bool(&mut self) -> Self::Type {
        types::I8
    }

    fn type_ptr(&mut self, _pointee: Option<Self::Type>) -> Self::Type {
        self.module.target_config().pointer_type()
    }

    fn type_unit(&mut self) -> Self::Type {
        types::INVALID
    }

    fn type_func(&mut self, _params: &[Self::Type], _ret: Self::Type) -> Self::Type {
        self.type_ptr(None)
    }
}

impl ModuleBuilder for CraneliftBackend {
    type FuncId = FuncId;
    type FuncBuilder<'a> = CraneliftFnBuilder<'a>;

    fn declare_func(&mut self, name: &str, params: &[Self::Type], ret: Self::Type) -> Self::FuncId {
        let mut sig = self.module.make_signature();
        for p in params {
            sig.params.push(AbiParam::new(*p));
        }

        if ret != types::INVALID {
            sig.returns.push(AbiParam::new(ret));
        }

        self.module
            .declare_function(name, Linkage::Export, &sig)
            .expect("Failed to declare function")
    }

    fn define_func<'a>(&'a mut self, func: Self::FuncId) -> Self::FuncBuilder<'a> {
        self.ctx.func.clear();

        let sig = self
            .module
            .declarations()
            .get_function_decl(func)
            .signature
            .clone();
        self.ctx.func.signature = sig;

        let builder =
            cranelift::prelude::FunctionBuilder::new(&mut self.ctx.func, &mut self.fn_builder_ctx);

        CraneliftFnBuilder {
            builder,
            module: &mut self.module,
            func_id: func,
        }
    }
}

pub struct CraneliftFnBuilder<'a> {
    pub builder: cranelift::prelude::FunctionBuilder<'a>,
    pub module: &'a mut JITModule,
    pub func_id: FuncId,
}

impl<'a> TypeBuilder for CraneliftFnBuilder<'a> {
    type Type = Type;

    fn type_int(&mut self, bits: u16) -> Self::Type {
        match bits {
            8 => types::I8,
            16 => types::I16,
            32 => types::I32,
            64 => types::I64,
            128 => types::I128,
            _ => panic!("unsupported type"),
        }
    }

    fn type_uint(&mut self, bits: u16) -> Self::Type {
        self.type_int(bits)
    }

    fn type_float(&mut self, bits: u16) -> Self::Type {
        match bits {
            16 => types::F16,
            32 => types::F32,
            64 => types::F64,
            128 => types::F128,
            _ => panic!("unsupported type"),
        }
    }

    fn type_bool(&mut self) -> Self::Type {
        types::I8
    }

    fn type_ptr(&mut self, _pointee: Option<Self::Type>) -> Self::Type {
        self.module.target_config().pointer_type()
    }

    fn type_unit(&mut self) -> Self::Type {
        types::INVALID
    }

    fn type_func(&mut self, _params: &[Self::Type], _ret: Self::Type) -> Self::Type {
        self.type_ptr(None)
    }
}

impl<'a> FunctionBuilder for CraneliftFnBuilder<'a> {
    type Value = Value;
    type BasicBlock = Block;
    type Var = Variable;

    fn create_block(&mut self) -> Self::BasicBlock {
        self.builder.create_block()
    }

    fn switch_to_block(&mut self, block: Self::BasicBlock) {
        self.builder.switch_to_block(block);
    }

    fn seal_block(&mut self, block: Self::BasicBlock) {
        self.builder.seal_block(block);
    }

    fn declare_var(&mut self, ty: Self::Type) -> Self::Var {
        self.builder.declare_var(ty)
    }

    fn def_var(&mut self, var: Self::Var, value: Self::Value) {
        self.builder.def_var(var, value);
    }

    fn use_var(&mut self, var: Self::Var) -> Self::Value {
        self.builder.use_var(var)
    }

    // const
    fn ins_iconst(&mut self, ty: Self::Type, val: i64) -> Self::Value {
        self.builder.ins().iconst(ty, val)
    }
    fn ins_fconst(&mut self, ty: Type, val: f64) -> Value {
        match ty {
            types::F32 => self.builder.ins().f32const(val as f32),
            types::F64 => self.builder.ins().f64const(val as f64), // FIXME: remove f64 and i64 type in HIR literals
            _ => unreachable!(),
        }
    }

    // Integers
    fn ins_iadd(&mut self, lhs: Value, rhs: Value) -> Value {
        self.builder.ins().iadd(lhs, rhs)
    }

    fn ins_isub(&mut self, lhs: Value, rhs: Value) -> Value {
        self.builder.ins().isub(lhs, rhs)
    }

    fn ins_imul(&mut self, lhs: Value, rhs: Value) -> Value {
        self.builder.ins().imul(lhs, rhs)
    }

    fn ins_sdiv(&mut self, lhs: Value, rhs: Value) -> Value {
        self.builder.ins().sdiv(lhs, rhs)
    }

    fn ins_udiv(&mut self, lhs: Value, rhs: Value) -> Value {
        self.builder.ins().udiv(lhs, rhs)
    }

    // Floats
    fn ins_fadd(&mut self, lhs: Value, rhs: Value) -> Value {
        self.builder.ins().fadd(lhs, rhs)
    }

    fn ins_fsub(&mut self, lhs: Value, rhs: Value) -> Value {
        self.builder.ins().fsub(lhs, rhs)
    }

    fn ins_fmul(&mut self, lhs: Value, rhs: Value) -> Value {
        self.builder.ins().fmul(lhs, rhs)
    }

    fn ins_fdiv(&mut self, lhs: Value, rhs: Value) -> Value {
        self.builder.ins().fdiv(lhs, rhs)
    }

    fn ins_ret(&mut self, value: Option<Self::Value>) {
        self.builder.ins().return_(value.as_slice());
    }

    fn get_param(&self, index: usize) -> Self::Value {
        self.builder
            .block_params(self.builder.current_block().unwrap())[index]
    }

    fn finish(mut self) {
        self.builder.seal_all_blocks();
        self.builder.finalize();
    }
}
