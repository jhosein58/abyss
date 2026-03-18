use crate::vm::{core::AbyssVm, opcode::Instruction, types::CallFrame};

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
pub fn call_native(
    inst: &Instruction,
    vm: &mut AbyssVm,
    ip: &mut usize,
    bp: usize,
    registers_ptr: *mut u64,
) {
    unsafe {
        let func_idx = inst.b as usize;
        let arg_start_reg = inst.c;

        vm.ip = *ip;
        vm.bp = bp;

        let (func, arity) = {
            let native = &vm.native_funcs[func_idx];
            (native.function, native.arity as usize)
        };

        let args_start_abs = bp + arg_start_reg as usize;
        let mut args = Vec::with_capacity(arity);
        for i in 0..arity {
            args.push(*registers_ptr.add(args_start_abs + i));
        }

        let result = func(vm, &args);

        *registers_ptr.add(bp + inst.a as usize) = result;

        *ip = vm.ip;
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
