use abyss_ir::ir::{IrLit, IrProgram, IrType};
use libffi::middle::Type;

use crate::{
    codegen::IrCompiler,
    vm::{core::AbyssVm, types::REG_PTR_TAG},
};

impl AbyssVm {
    pub fn read_ptr_value(&self, ptr_val: u64) -> u64 {
        if (ptr_val & REG_PTR_TAG) != 0 {
            let abs_reg_idx = (ptr_val & !REG_PTR_TAG) as usize;
            self.registers[abs_reg_idx]
        } else {
            let ptr = ptr_val as usize;
            if ptr + 8 <= self.heap.len() {
                let mut val: u64 = 0;
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        self.heap.as_ptr().add(ptr),
                        &mut val as *mut u64 as *mut u8,
                        8,
                    );
                }
                u64::from_le(val)
            } else {
                panic!(
                    "Native Helper Error: Memory access out of bounds at {}",
                    ptr
                );
            }
        }
    }

    pub fn read_heap_u64(&self, ptr: usize, index: usize) -> u64 {
        let offset = ptr + (index * 8);
        if offset + 8 <= self.heap.len() {
            let bytes: [u8; 8] = self.heap[offset..offset + 8].try_into().unwrap();
            u64::from_le_bytes(bytes)
        } else {
            0
        }
    }

    pub fn read_c_string(&self, base_ptr: u64) -> String {
        let mut s = String::new();
        let mut offset = 0;

        loop {
            let current_ptr = if (base_ptr & REG_PTR_TAG) != 0 {
                if offset == 0 {
                    base_ptr
                } else {
                    panic!("Native Helper Error: Cannot use offset on register pointer");
                }
            } else {
                base_ptr + (offset * 8)
            };

            let val = self.read_ptr_value(current_ptr);

            if val == 0 {
                break;
            }

            if let Some(c) = std::char::from_u32(val as u32) {
                s.push(c);
            } else {
                s.push('0');
            }

            offset += 1;
        }

        s
    }
}

pub fn ir_type_to_ffi(ty: &IrType) -> Type {
    match ty {
        IrType::I32 => Type::i32(),
        IrType::F32 => Type::f32(),
        IrType::Bool => Type::u8(),
        IrType::Unit => Type::void(),
        IrType::Ptr(_) => Type::pointer(),
        IrType::Array(_, _) | IrType::Struct(_) => Type::pointer(),
    }
}

pub fn execute_comptime(ir_prog: IrProgram) -> IrLit {
    let expected_type = ir_prog
        .functions
        .iter()
        .find(|f| f.name == "main")
        .map(|f| f.return_ty.clone())
        .expect("Comptime program must have a main function");

    let compiler = IrCompiler::new();

    let (instructions, constants, _extern_funcs) = compiler.compile(&ir_prog);

    let mut vm = AbyssVm::new(instructions, constants);

    let raw_result = vm.run().unwrap_or(0);

    match expected_type {
        IrType::I32 => IrLit::Int(raw_result as i64),
        IrType::F32 => IrLit::Float(f64::from_bits(raw_result)),
        IrType::Bool => IrLit::Bool(raw_result != 0),
        IrType::Unit => IrLit::Bool(false),
        _ => panic!("Unsupported comptime return type: {:?}", expected_type),
    }
}
