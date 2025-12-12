// use abyss_parser::ast::{BinaryOp, Type as MyType, UnaryOp};
// use cranelift::codegen::Context;
// use cranelift::codegen::ir::{InstBuilder, MemFlags, StackSlot, StackSlotData, StackSlotKind};
// use cranelift::prelude::*;
// use cranelift_jit::{JITBuilder, JITModule};
// use cranelift_module::{DataDescription, FuncId, Linkage, Module};
// use std::collections::HashMap;

// use crate::target::Target;

// fn abyss_type_to_clif(t: &MyType, ptr_type: types::Type) -> types::Type {
//     match t {
//         MyType::Int => types::I64,
//         MyType::Float => types::F64,
//         MyType::Bool => types::I8,
//         MyType::Void => types::INVALID,
//         MyType::Pointer(_) | MyType::Array(_, _) => ptr_type,
//     }
// }

// pub struct CraneliftTarget {
//     module: JITModule,
//     ctx: Context,
//     data_description: DataDescription,
//     builder_ctx: FunctionBuilderContext,
//     builder: Option<FunctionBuilder<'static>>,

//     var_map: HashMap<String, StackSlot>,
//     functions_map: HashMap<String, FuncId>,
//     current_func_name: String,

//     param_buffer: Vec<(String, MyType)>,
//     data_counter: usize,
// }

// impl CraneliftTarget {
//     pub fn new(extern_symbols: &[(&str, *const u8)]) -> Self {
//         let mut builder = JITBuilder::new(cranelift_module::default_libcall_names()).unwrap();

//         for (name, ptr) in extern_symbols {
//             builder.symbol(*name, *ptr);
//         }

//         let module = JITModule::new(builder);

//         Self {
//             module,
//             ctx: Context::new(),
//             data_description: DataDescription::new(),
//             builder_ctx: FunctionBuilderContext::new(),
//             builder: None,
//             var_map: HashMap::new(),
//             functions_map: HashMap::new(),
//             current_func_name: String::new(),
//             param_buffer: Vec::new(),
//             data_counter: 0,
//         }
//     }

//     pub fn run_fn(&mut self, name: &str) -> Result<i32, String> {
//         self.module
//             .finalize_definitions()
//             .map_err(|e| format!("{:?}", e))?;

//         let code = self.module.get_name(name).ok_or("Function not found")?;

//         if let cranelift_module::FuncOrDataId::Func(func_id) = code {
//             let ptr = self.module.get_finalized_function(func_id);
//             let code_fn = unsafe { std::mem::transmute::<_, extern "C" fn() -> i32>(ptr) };
//             Ok(code_fn())
//         } else {
//             Err("Not a function".to_string())
//         }
//     }

//     fn to_clif_type(&self, t: &MyType) -> types::Type {
//         abyss_type_to_clif(t, self.module.target_config().pointer_type())
//     }

//     fn _ptr_type(&self) -> types::Type {
//         self.module.target_config().pointer_type()
//     }
// }
