#[cfg(feature = "ffi")]
use libffi::{low::CodePtr, middle::Arg};

use crate::vm::{
    core::AbyssVm,
    opcode::Instruction,
    types::{CallFrame, REG_PTR_TAG},
};

#[inline(always)]
pub fn call(
    inst: &Instruction,
    ip: &mut usize,
    bp: &mut usize,
    registers_ptr: *const u64,
    call_stack: &mut Vec<CallFrame>,
) {
    let target_ip = unsafe { *registers_ptr.add(*bp + inst.b as usize) } as usize;

    call_stack.push(CallFrame {
        ret_ip: *ip,
        ret_reg: inst.a,
        bp: *bp,
    });

    *bp += inst.c as usize;
    *ip = target_ip;
}

#[inline(always)]
pub fn call_extern(inst: &Instruction, vm: &mut AbyssVm, bp: usize, registers_ptr: *mut u64) {
    unsafe {
        let func_idx = (*registers_ptr.add(bp + inst.b as usize)) as usize;
        let arg_start_reg = inst.c;

        let extern_func = &vm.extern_funcs[func_idx];
        let arity = extern_func.arity;

        let args_start_abs = bp + arg_start_reg as usize;
        let mut arg_values = [0u64; 16];

        for i in 0..arity {
            let raw_val = *registers_ptr.add(args_start_abs + i);

            if extern_func.is_pointer_args[i] {
                if (raw_val & REG_PTR_TAG) != 0 {
                    arg_values[i] = raw_val;
                } else if (raw_val as usize) < vm.heap.len() {
                    let ptr = vm.heap.as_mut_ptr().add(raw_val as usize);
                    arg_values[i] = ptr as u64;
                } else if (raw_val as usize) < (vm.globals.len() * 8) {
                    let ptr = (vm.globals.as_mut_ptr() as *mut u8).add(raw_val as usize);
                    arg_values[i] = ptr as u64;
                } else {
                    panic!("FFI Error: Invalid pointer {}", raw_val);
                }
            } else {
                arg_values[i] = raw_val;
            }
        }

        let result: u64;

        if let Some(ref host_fn) = extern_func.host_fn {
            let heap_slice = std::slice::from_raw_parts_mut(vm.heap.as_mut_ptr(), vm.heap.len());

            result = host_fn(&arg_values[..arity], heap_slice);
        } else {
            #[cfg(feature = "ffi")]
            {
                let mut ffi_args_buffer = [(); 16].map(|_| Arg::new(&0u64));
                for i in 0..arity {
                    ffi_args_buffer[i] = Arg::new(&arg_values[i]);
                }

                let code_ptr = CodePtr::from_ptr(extern_func.ptr as *const _);
                let ret_size = extern_func.ret_size;

                result = match ret_size {
                    0 => {
                        extern_func
                            .cif
                            .call::<()>(code_ptr, &ffi_args_buffer[..arity]);
                        0
                    }
                    1 => extern_func
                        .cif
                        .call::<u8>(code_ptr, &ffi_args_buffer[..arity])
                        as u64,
                    2 => extern_func
                        .cif
                        .call::<u16>(code_ptr, &ffi_args_buffer[..arity])
                        as u64,
                    4 => extern_func
                        .cif
                        .call::<u32>(code_ptr, &ffi_args_buffer[..arity])
                        as u64,
                    8 => extern_func
                        .cif
                        .call::<u64>(code_ptr, &ffi_args_buffer[..arity]),
                    _ => {
                        extern_func
                            .cif
                            .call::<()>(code_ptr, &ffi_args_buffer[..arity]);
                        0
                    }
                };
            }

            #[cfg(not(feature = "ffi"))]
            {
                panic!(
                    "Execution Error: FFI is disabled, but function '{}' was not registered manually via host functions.",
                    extern_func.name
                );
            }
        }

        *registers_ptr.add(bp + inst.a as usize) = result;
    }
}

#[inline(always)]
pub fn jmp(inst: &Instruction, bp: usize, ip: &mut usize, registers_ptr: *const u64) {
    unsafe {
        *ip = *registers_ptr.add(bp + inst.a as usize) as usize;
    }
}

#[inline(always)]
pub fn jmp_if(inst: &Instruction, bp: usize, ip: &mut usize, registers_ptr: *const u64) {
    unsafe {
        let condition = *registers_ptr.add(bp + inst.b as usize);
        if condition != 0 {
            *ip = *registers_ptr.add(bp + inst.a as usize) as usize;
        }
    }
}

#[inline(always)]
pub fn jmp_imm(inst: &Instruction, ip: &mut usize) {
    let target_ip = ((inst.b as u16) << 8) | (inst.c as u16);
    *ip = target_ip as usize;
}

#[inline(always)]
pub fn jmp_z_imm(inst: &Instruction, bp: usize, ip: &mut usize, registers_ptr: *const u64) {
    let condition = unsafe { *registers_ptr.add(bp + inst.a as usize) };
    if condition == 0 {
        let target_ip = ((inst.b as u16) << 8) | (inst.c as u16);
        *ip = target_ip as usize;
    }
}
