use abyss_lower::builder::{FunctionBuilder, ModuleBuilder, TypeBuilder};
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
}

impl TypeBuilder for CraneliftBackend {
    type Type = Type;

    fn type_int(&mut self, bits: u16) -> Self::Type {
        match bits {
            32 => types::I32,
            _ => panic!("unsupported type"),
        }
    }

    fn type_uint(&mut self, bits: u16) -> Self::Type {
        self.type_int(bits)
    }

    fn type_float(&mut self, bits: u16) -> Self::Type {
        match bits {
            32 => types::F32,
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

    fn declare_func(
        &mut self,
        name: &str,
        params: &[Self::Type],
        ret: Option<Self::Type>,
    ) -> Self::FuncId {
        let mut sig = self.module.make_signature();
        for p in params {
            sig.params.push(AbiParam::new(*p));
        }
        if let Some(r) = ret {
            if r != types::INVALID {
                sig.returns.push(AbiParam::new(r));
            }
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

impl<'a> FunctionBuilder for CraneliftFnBuilder<'a> {
    type Type = Type;
    type Value = Value;
    type BasicBlock = Block;

    fn create_block(&mut self) -> Self::BasicBlock {
        self.builder.create_block()
    }

    fn switch_to_block(&mut self, block: Self::BasicBlock) {
        self.builder.switch_to_block(block);
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
