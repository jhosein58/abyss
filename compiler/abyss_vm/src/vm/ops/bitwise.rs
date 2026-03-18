use crate::vm::opcode::Instruction;

#[inline(always)]
pub fn bit_and(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize);
        let right = *reg_ptr.add(bp + inst.c as usize);
        *reg_ptr.add(bp + inst.a as usize) = left & right;
    }
}

#[inline(always)]
pub fn bit_or(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize);
        let right = *reg_ptr.add(bp + inst.c as usize);
        *reg_ptr.add(bp + inst.a as usize) = left | right;
    }
}

#[inline(always)]
pub fn bit_xor(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize);
        let right = *reg_ptr.add(bp + inst.c as usize);
        *reg_ptr.add(bp + inst.a as usize) = left ^ right;
    }
}

#[inline(always)]
pub fn shl(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize);
        let right = *reg_ptr.add(bp + inst.c as usize);
        *reg_ptr.add(bp + inst.a as usize) = left.wrapping_shl(right as u32);
    }
}

#[inline(always)]
pub fn shr_i(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *reg_ptr.add(bp + inst.c as usize);
        *reg_ptr.add(bp + inst.a as usize) = (left.wrapping_shr(right as u32)) as u64;
    }
}

#[inline(always)]
pub fn shr_u(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize);
        let right = *reg_ptr.add(bp + inst.c as usize);
        *reg_ptr.add(bp + inst.a as usize) = left.wrapping_shr(right as u32);
    }
}

#[inline(always)]
pub fn bit_not(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let val = *reg_ptr.add(bp + inst.b as usize);
        *reg_ptr.add(bp + inst.a as usize) = !val;
    }
}
