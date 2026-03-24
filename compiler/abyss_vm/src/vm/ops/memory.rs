use crate::vm::{opcode::Instruction, types::REG_PTR_TAG};

#[inline(always)]
pub fn alloc(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &mut Vec<u8>) {
    unsafe {
        let size = *registers_ptr.add(bp + inst.b as usize) as usize;
        let ptr = heap.len();
        heap.resize(ptr + size, 0);
        *registers_ptr.add(bp + inst.a as usize) = ptr as u64;
    }
}

#[inline(always)]
pub fn ref_reg(inst: &Instruction, bp: usize, registers_ptr: *mut u64) {
    unsafe {
        let reg_idx = inst.b;
        let abs_addr = bp + reg_idx as usize;
        let tagged_ptr = (abs_addr as u64) | REG_PTR_TAG;
        *registers_ptr.add(bp + inst.a as usize) = tagged_ptr;
    }
}

#[inline(always)]
pub fn load_ptr(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &[u8]) {
    unsafe {
        let ptr_val = *registers_ptr.add(bp + inst.b as usize);

        if (ptr_val & REG_PTR_TAG) != 0 {
            let abs_reg_idx = (ptr_val & !REG_PTR_TAG) as usize;
            let val = *registers_ptr.add(abs_reg_idx);
            *registers_ptr.add(bp + inst.a as usize) = val;
        } else {
            let ptr = ptr_val as usize;
            if ptr + 8 <= heap.len() {
                let mut val: u64 = 0;
                std::ptr::copy_nonoverlapping(
                    heap.as_ptr().add(ptr),
                    &mut val as *mut u64 as *mut u8,
                    8,
                );
                *registers_ptr.add(bp + inst.a as usize) = u64::from_le(val);
            } else {
                panic!("Runtime error: Segmentation fault in LoadPtr");
            }
        }
    }
}

#[inline(always)]
pub fn store_ptr(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &mut [u8]) {
    unsafe {
        let ptr_val = *registers_ptr.add(bp + inst.a as usize);
        let val = *registers_ptr.add(bp + inst.b as usize);

        if (ptr_val & REG_PTR_TAG) != 0 {
            let abs_reg_idx = (ptr_val & !REG_PTR_TAG) as usize;
            *registers_ptr.add(abs_reg_idx) = val;
        } else {
            let ptr = ptr_val as usize;
            if ptr + 8 <= heap.len() {
                std::ptr::copy_nonoverlapping(
                    &val as *const u64 as *const u8,
                    heap.as_mut_ptr().add(ptr),
                    8,
                );
            } else {
                panic!("Runtime error: Segmentation fault in StorePtr");
            }
        }
    }
}

#[inline(always)]
pub fn store_لم(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &mut [u8]) {
    unsafe {
        let ptr_val = *registers_ptr.add(bp + inst.a as usize);
        let val = *registers_ptr.add(bp + inst.b as usize);

        if (ptr_val & REG_PTR_TAG) != 0 {
            let abs_reg_idx = (ptr_val & !REG_PTR_TAG) as usize;
            *registers_ptr.add(abs_reg_idx) = val;
        } else {
            let ptr = ptr_val as usize;
            if ptr + 8 <= heap.len() {
                std::ptr::copy_nonoverlapping(
                    &val as *const u64 as *const u8,
                    heap.as_mut_ptr().add(ptr),
                    8,
                );
            } else {
                panic!("Runtime error: Segmentation fault in StorePtr");
            }
        }
    }
}

#[inline(always)]
pub fn load_ptr_offset(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &[u8]) {
    unsafe {
        let base_ptr_val = *registers_ptr.add(bp + inst.b as usize);
        let index = *registers_ptr.add(bp + inst.c as usize) as usize;

        if (base_ptr_val & REG_PTR_TAG) != 0 {
            if index == 0 {
                let abs_reg_idx = (base_ptr_val & !REG_PTR_TAG) as usize;
                let val = *registers_ptr.add(abs_reg_idx);
                *registers_ptr.add(bp + inst.a as usize) = val;
            } else {
                panic!("Runtime error: Cannot use non-zero offset on a direct register reference");
            }
        } else {
            let base_ptr = base_ptr_val as usize;
            let actual_ptr = base_ptr + (index * 8);

            if actual_ptr + 8 <= heap.len() {
                let mut val: u64 = 0;
                std::ptr::copy_nonoverlapping(
                    heap.as_ptr().add(actual_ptr),
                    &mut val as *mut u64 as *mut u8,
                    8,
                );
                *registers_ptr.add(bp + inst.a as usize) = u64::from_le(val);
            } else {
                panic!("Runtime error: Memory access out of bounds in LoadPtrOffset");
            }
        }
    }
}

#[inline(always)]
pub fn store_ptr_offset(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &mut [u8]) {
    unsafe {
        let base_ptr_val = *registers_ptr.add(bp + inst.a as usize);
        let val = *registers_ptr.add(bp + inst.b as usize);
        let index = *registers_ptr.add(bp + inst.c as usize) as usize;

        if (base_ptr_val & REG_PTR_TAG) != 0 {
            if index == 0 {
                let abs_reg_idx = (base_ptr_val & !REG_PTR_TAG) as usize;
                *registers_ptr.add(abs_reg_idx) = val;
            } else {
                panic!("Runtime error: Cannot use non-zero offset on a direct register reference");
            }
        } else {
            let base_ptr = base_ptr_val as usize;
            let actual_ptr = base_ptr + (index * 8);

            if actual_ptr + 8 <= heap.len() {
                std::ptr::copy_nonoverlapping(
                    &val as *const u64 as *const u8,
                    heap.as_mut_ptr().add(actual_ptr),
                    8,
                );
            } else {
                panic!("Runtime error: Memory access out of bounds in StorePtrOffset");
            }
        }
    }
}

#[inline(always)]
pub fn mem_copy(inst: &Instruction, bp: usize, registers_ptr: *const u64, heap: &mut [u8]) {
    unsafe {
        let dest_ptr_val = *registers_ptr.add(bp + inst.a as usize);
        let src_ptr_val = *registers_ptr.add(bp + inst.b as usize);
        let count = *registers_ptr.add(bp + inst.c as usize) as usize;
        let bytes_to_copy = count * 8;

        let dest_ptr = (dest_ptr_val & !REG_PTR_TAG) as usize;
        let src_ptr = (src_ptr_val & !REG_PTR_TAG) as usize;

        if dest_ptr + bytes_to_copy <= heap.len() && src_ptr + bytes_to_copy <= heap.len() {
            std::ptr::copy_nonoverlapping(
                heap.as_ptr().add(src_ptr),
                heap.as_mut_ptr().add(dest_ptr),
                bytes_to_copy,
            );
        } else {
            panic!("Runtime error: MemCopy out of bounds");
        }
    }
}
