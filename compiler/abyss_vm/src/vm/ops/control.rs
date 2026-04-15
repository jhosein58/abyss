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

            if (raw_val & REG_PTR_TAG) == 0 && raw_val > 0 && (raw_val as usize) < vm.heap.len() {
                arg_values[i] = vm.heap.as_mut_ptr().add(raw_val as usize) as u64;
            } else {
                arg_values[i] = raw_val;
            }
        }

        let mut ffi_args_buffer = [(); 16].map(|_| Arg::new(&0u64));
        for i in 0..arity {
            ffi_args_buffer[i] = Arg::new(&arg_values[i]);
        }

        let code_ptr = CodePtr::from_ptr(extern_func.ptr as *const _);

        let result: u64 = extern_func.cif.call(code_ptr, &ffi_args_buffer[..arity]);

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
