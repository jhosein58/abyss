use crate::vm::core::AbyssVm;

pub struct CallFrame {
    pub ret_ip: usize,
    pub ret_reg: u8,
    pub bp: usize,
}

pub type NativeFunction = fn(vm: &mut AbyssVm, args: &[u64]) -> u64;

pub struct RegisteredNative {
    pub function: NativeFunction,
    pub arity: u8,
}

pub const REG_PTR_TAG: u64 = 1 << 63;
