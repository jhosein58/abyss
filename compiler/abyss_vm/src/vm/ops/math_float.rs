use crate::vm::opcode::Instruction;

// --- Float Math ---

#[inline(always)]
pub fn add_f(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*reg_ptr.add(bp + inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = (left + right).to_bits();
    }
}

#[inline(always)]
pub fn sub_f(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*reg_ptr.add(bp + inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = (left - right).to_bits();
    }
}

#[inline(always)]
pub fn mul_f(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*reg_ptr.add(bp + inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = (left * right).to_bits();
    }
}

#[inline(always)]
pub fn div_f(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*reg_ptr.add(bp + inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = (left / right).to_bits();
    }
}

// --- Float Math with Constants ---

#[inline(always)]
pub fn add_fc(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*const_ptr.add(inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = (left + right).to_bits();
    }
}

#[inline(always)]
pub fn sub_fc(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*const_ptr.add(inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = (left - right).to_bits();
    }
}

#[inline(always)]
pub fn mul_fc(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*const_ptr.add(inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = (left * right).to_bits();
    }
}

#[inline(always)]
pub fn div_fc(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*const_ptr.add(inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = (left / right).to_bits();
    }
}

// --- Float Comparisons ---

#[inline(always)]
pub fn cmp_eq_f(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*reg_ptr.add(bp + inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = if left == right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_neq_f(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*reg_ptr.add(bp + inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = if left != right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_lt_f(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*reg_ptr.add(bp + inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = if left < right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_le_f(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*reg_ptr.add(bp + inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = if left <= right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_gt_f(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*reg_ptr.add(bp + inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = if left > right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_ge_f(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*reg_ptr.add(bp + inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = if left >= right { 1 } else { 0 };
    }
}

// --- Float Comparisons with Constants ---

#[inline(always)]
pub fn cmp_eq_fc(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*const_ptr.add(inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = if left == right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_neq_fc(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*const_ptr.add(inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = if left != right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_lt_fc(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*const_ptr.add(inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = if left < right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_le_fc(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*const_ptr.add(inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = if left <= right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_gt_fc(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*const_ptr.add(inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = if left > right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_ge_fc(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = f64::from_bits(*reg_ptr.add(bp + inst.b as usize));
        let right = f64::from_bits(*const_ptr.add(inst.c as usize));
        *reg_ptr.add(bp + inst.a as usize) = if left >= right { 1 } else { 0 };
    }
}
