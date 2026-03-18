use crate::vm::opcode::Instruction;

// --- Integer Math ---

#[inline(always)]
pub fn add_i(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *reg_ptr.add(bp + inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = left.wrapping_add(right) as u64;
    }
}

#[inline(always)]
pub fn sub_i(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *reg_ptr.add(bp + inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = left.wrapping_sub(right) as u64;
    }
}

#[inline(always)]
pub fn mul_i(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *reg_ptr.add(bp + inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = left.wrapping_mul(right) as u64;
    }
}

#[inline(always)]
pub fn div_i(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *reg_ptr.add(bp + inst.c as usize) as i64;
        if right == 0 {
            panic!("Runtime error: Division by zero");
        }
        *reg_ptr.add(bp + inst.a as usize) = left.wrapping_div(right) as u64;
    }
}

#[inline(always)]
pub fn mod_i(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *reg_ptr.add(bp + inst.c as usize) as i64;
        if right == 0 {
            panic!("Runtime error: Division by zero");
        }
        *reg_ptr.add(bp + inst.a as usize) = left.wrapping_rem(right) as u64;
    }
}

// --- Integer Math with Constants ---

#[inline(always)]
pub fn add_ic(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *const_ptr.add(inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = left.wrapping_add(right) as u64;
    }
}

#[inline(always)]
pub fn sub_ic(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *const_ptr.add(inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = left.wrapping_sub(right) as u64;
    }
}

#[inline(always)]
pub fn mul_ic(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *const_ptr.add(inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = left.wrapping_mul(right) as u64;
    }
}
#[inline(always)]
pub fn div_ic(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *const_ptr.add(inst.c as usize) as i64;
        if right == 0 {
            panic!("Runtime error: Division by zero");
        }
        *reg_ptr.add(bp + inst.a as usize) = left.wrapping_div(right) as u64;
    }
}

#[inline(always)]
pub fn mod_ic(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *const_ptr.add(inst.c as usize) as i64;
        if right == 0 {
            panic!("Runtime error: Division by zero");
        }
        *reg_ptr.add(bp + inst.a as usize) = left.wrapping_rem(right) as u64;
    }
}

// --- Integer Comparisons ---

#[inline(always)]
pub fn cmp_eq_i(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *reg_ptr.add(bp + inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = if left == right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_neq_i(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *reg_ptr.add(bp + inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = if left != right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_lt_i(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *reg_ptr.add(bp + inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = if left < right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_le_i(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *reg_ptr.add(bp + inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = if left <= right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_gt_i(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *reg_ptr.add(bp + inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = if left > right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_ge_i(inst: &Instruction, bp: usize, reg_ptr: *mut u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *reg_ptr.add(bp + inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = if left >= right { 1 } else { 0 };
    }
}

// --- Integer Comparisons with Constants ---

#[inline(always)]
pub fn cmp_eq_ic(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *const_ptr.add(inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = if left == right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_neq_ic(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *const_ptr.add(inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = if left != right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_lt_ic(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *const_ptr.add(inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = if left < right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_le_ic(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *const_ptr.add(inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = if left <= right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_gt_ic(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *const_ptr.add(inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = if left > right { 1 } else { 0 };
    }
}

#[inline(always)]
pub fn cmp_ge_ic(inst: &Instruction, bp: usize, reg_ptr: *mut u64, const_ptr: *const u64) {
    unsafe {
        let left = *reg_ptr.add(bp + inst.b as usize) as i64;
        let right = *const_ptr.add(inst.c as usize) as i64;
        *reg_ptr.add(bp + inst.a as usize) = if left >= right { 1 } else { 0 };
    }
}
