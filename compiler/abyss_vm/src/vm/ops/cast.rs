use crate::vm::opcode::Instruction;

#[inline(always)]
pub fn cast_i2f(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let val_int = *reg_ptr.add(bp + inst.b as usize) as i64;
        let val_float = val_int as f64;
        *reg_ptr.add(bp + inst.a as usize) = val_float.to_bits();
    }
}

#[inline(always)]
pub fn cast_f2i(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let val_float = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let val_int = val_float as i64;
        *reg_ptr.add(bp + inst.a as usize) = val_int as u64;
    }
}

#[inline(always)]
pub fn cast_i2b(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let val_int = *reg_ptr.add(bp + inst.b as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = if val_int != 0 { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cast_f2b(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let val_float = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        *reg_ptr.add(bp + inst.a as usize) = if val_float != 0.0 { 1 } else { 0 };
    }
}
