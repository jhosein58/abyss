use crate::vm::opcode::Instruction;

#[inline(always)]
pub fn load_const(
    inst: &Instruction,
    bp: usize,
    registers_ptr: *mut u64,
    constants_ptr: *const u64,
) {
    unsafe {
        let val = *constants_ptr.add(inst.b as usize);
        *registers_ptr.add(bp + inst.a as usize) = val;
    }
}

#[inline(always)]
pub fn move_reg(inst: &Instruction, bp: usize, registers_ptr: *mut u64) {
    unsafe {
        let val = *registers_ptr.add(bp + inst.b as usize);
        *registers_ptr.add(bp + inst.a as usize) = val;
    }
}

#[inline(always)]
pub fn not(inst: &Instruction, bp: usize, registers_ptr: *mut u64) {
    unsafe {
        let val = *registers_ptr.add(bp + inst.b as usize);
        *registers_ptr.add(bp + inst.a as usize) = if val == 0 { 1 } else { 0 };
    }
}
