use abyss_ir::ir::IrType;
use libffi::middle::Cif;
use std::ffi::c_void;

pub struct CallFrame {
    pub ret_ip: usize,
    pub ret_reg: u8,
    pub bp: usize,
}

pub struct ExternFunction {
    pub name: String,
    pub ptr: *mut c_void,
    pub arity: usize,
    pub cif: Cif,
}

#[derive(Debug, Clone)]
pub struct ExternDef {
    pub name: String,
    pub arg_types: Vec<IrType>,
    pub ret_type: IrType,
}

pub const REG_PTR_TAG: u64 = 1 << 63;
