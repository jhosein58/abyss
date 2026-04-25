use abyss_ir::ir::IrType;

#[cfg(feature = "ffi")]
use libffi::middle::Cif;
#[cfg(feature = "ffi")]
use std::os::raw::c_void;
use std::rc::Rc;

pub struct CallFrame {
    pub ret_ip: usize,
    pub ret_reg: u8,
    pub bp: usize,
}

pub type HostFn = Rc<dyn Fn(&[u64], &mut [u8]) -> u64>;
pub struct ExternFunction {
    pub name: String,
    pub arity: usize,
    pub is_pointer_args: Vec<bool>,
    pub ret_size: usize,

    #[cfg(feature = "ffi")]
    pub ptr: *mut c_void,
    #[cfg(feature = "ffi")]
    pub cif: Cif,

    pub host_fn: Option<HostFn>,
}
#[derive(Debug, Clone)]
pub struct ExternDef {
    pub name: String,
    pub arg_types: Vec<IrType>,
    pub ret_type: IrType,
}

pub const REG_PTR_TAG: u64 = 1 << 63;
pub const EXTERN_FUNC_TAG: u64 = 0x4000_0000_0000_0000;
