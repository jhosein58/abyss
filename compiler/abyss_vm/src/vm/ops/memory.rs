use crate::vm::{opcode::Instruction, types::REG_PTR_TAG};

#[inline(always)]
pub fn alloc(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &mut Vec<u8>) {
    unsafe {
        let size_u64 = *registers_ptr.add(bp + inst.b as usize);
        let size = size_u64 as usize;

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
                panic!(
                    "Runtime error: Segmentation fault in LoadPtr (ptr: {}, heap_len: {})",
                    ptr,
                    heap.len()
                );
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
                panic!(
                    "Runtime error: Segmentation fault in StorePtr (ptr: {}, heap_len: {})",
                    ptr,
                    heap.len()
                );
            }
        }
    }
}

#[inline(always)]
pub fn load_ptr_offset(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &[u8]) {
    unsafe {
        let base_ptr_val = *registers_ptr.add(bp + inst.b as usize);
        let index = (*registers_ptr.add(bp + inst.c as usize) & 0xFFFFFFFF) as usize;

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
                panic!(
                    "Runtime error: Out of bounds in LoadPtrOffset (base: {}, idx: {}, len: {})",
                    base_ptr,
                    index,
                    heap.len()
                );
            }
        }
    }
}

#[inline(always)]
pub fn store_ptr_offset(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &mut [u8]) {
    unsafe {
        let base_ptr_val = *registers_ptr.add(bp + inst.a as usize);
        let val = *registers_ptr.add(bp + inst.b as usize);
        let index = (*registers_ptr.add(bp + inst.c as usize) & 0xFFFFFFFF) as usize;

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
                panic!(
                    "Runtime error: Out of bounds in StorePtrOffset (base: {}, idx: {}, len: {})",
                    base_ptr,
                    index,
                    heap.len()
                );
            }
        }
    }
}

#[inline(always)]
pub fn mem_copy(inst: &Instruction, bp: usize, registers_ptr: *const u64, heap: &mut [u8]) {
    unsafe {
        let dest_ptr_val = *registers_ptr.add(bp + inst.a as usize);
        let src_ptr_val = *registers_ptr.add(bp + inst.b as usize);
        let count = (*registers_ptr.add(bp + inst.c as usize) & 0xFFFFFFFF) as usize;

        let bytes_to_copy = count * 8;

        let dest_ptr = (dest_ptr_val & !REG_PTR_TAG) as usize;
        let src_ptr = (src_ptr_val & !REG_PTR_TAG) as usize;

        if dest_ptr + bytes_to_copy <= heap.len() && src_ptr + bytes_to_copy <= heap.len() {
            std::ptr::copy(
                heap.as_ptr().add(src_ptr),
                heap.as_mut_ptr().add(dest_ptr),
                bytes_to_copy,
            );
        } else {
            panic!(
                "Runtime error: MemCopy out of bounds. dest: {}, src: {}, bytes: {}, heap_len: {}",
                dest_ptr,
                src_ptr,
                bytes_to_copy,
                heap.len()
            );
        }
    }
}

// 8-bit

#[inline(always)]
pub fn load_ptr_8(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &[u8]) {
    unsafe {
        let ptr_val = *registers_ptr.add(bp + inst.b as usize);
        if (ptr_val & REG_PTR_TAG) != 0 {
            let abs_reg_idx = (ptr_val & !REG_PTR_TAG) as usize;
            let val = *registers_ptr.add(abs_reg_idx);
            *registers_ptr.add(bp + inst.a as usize) = val & 0xFF;
        } else {
            let ptr = ptr_val as usize;
            if ptr < heap.len() {
                *registers_ptr.add(bp + inst.a as usize) = heap[ptr] as u64;
            } else {
                panic!("Runtime error: Segmentation fault in LoadPtr8");
            }
        }
    }
}

#[inline(always)]
pub fn store_ptr_8(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &mut [u8]) {
    unsafe {
        let ptr_val = *registers_ptr.add(bp + inst.a as usize);
        let val = *registers_ptr.add(bp + inst.b as usize);
        if (ptr_val & REG_PTR_TAG) != 0 {
            let abs_reg_idx = (ptr_val & !REG_PTR_TAG) as usize;
            let existing = *registers_ptr.add(abs_reg_idx);
            *registers_ptr.add(abs_reg_idx) = (existing & !0xFF) | (val & 0xFF);
        } else {
            let ptr = ptr_val as usize;
            if ptr < heap.len() {
                heap[ptr] = (val & 0xFF) as u8;
            } else {
                panic!("Runtime error: Segmentation fault in StorePtr8");
            }
        }
    }
}

#[inline(always)]
pub fn load_ptr_offset_8(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &[u8]) {
    unsafe {
        let base_ptr_val = *registers_ptr.add(bp + inst.b as usize);
        let index = (*registers_ptr.add(bp + inst.c as usize) & 0xFFFFFFFF) as usize;
        if (base_ptr_val & REG_PTR_TAG) != 0 {
            if index == 0 {
                let abs_reg_idx = (base_ptr_val & !REG_PTR_TAG) as usize;
                let val = *registers_ptr.add(abs_reg_idx);
                *registers_ptr.add(bp + inst.a as usize) = val & 0xFF;
            } else {
                panic!("Runtime error: Cannot use non-zero offset on a direct register reference");
            }
        } else {
            let actual_ptr = (base_ptr_val as usize) + index;
            if actual_ptr < heap.len() {
                *registers_ptr.add(bp + inst.a as usize) = heap[actual_ptr] as u64;
            } else {
                panic!("Runtime error: Memory access out of bounds in LoadPtrOffset8");
            }
        }
    }
}

#[inline(always)]
pub fn store_ptr_offset_8(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &mut [u8]) {
    unsafe {
        let base_ptr_val = *registers_ptr.add(bp + inst.a as usize);
        let val = *registers_ptr.add(bp + inst.b as usize);
        let index = (*registers_ptr.add(bp + inst.c as usize) & 0xFFFFFFFF) as usize;
        if (base_ptr_val & REG_PTR_TAG) != 0 {
            if index == 0 {
                let abs_reg_idx = (base_ptr_val & !REG_PTR_TAG) as usize;
                let existing = *registers_ptr.add(abs_reg_idx);
                *registers_ptr.add(abs_reg_idx) = (existing & !0xFF) | (val & 0xFF);
            } else {
                panic!("Runtime error: Cannot use non-zero offset on a direct register reference");
            }
        } else {
            let actual_ptr = (base_ptr_val as usize) + index;
            if actual_ptr < heap.len() {
                heap[actual_ptr] = (val & 0xFF) as u8;
            } else {
                panic!("Runtime error: Memory access out of bounds in StorePtrOffset8");
            }
        }
    }
}

// 16-bit
#[inline(always)]
pub fn load_ptr_offset_16(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &[u8]) {
    unsafe {
        let base_ptr_val = *registers_ptr.add(bp + inst.b as usize);
        let index = (*registers_ptr.add(bp + inst.c as usize) & 0xFFFFFFFF) as usize;
        if (base_ptr_val & REG_PTR_TAG) != 0 {
            if index == 0 {
                let abs_reg_idx = (base_ptr_val & !REG_PTR_TAG) as usize;
                let val = *registers_ptr.add(abs_reg_idx);
                *registers_ptr.add(bp + inst.a as usize) = val & 0xFFFF;
            } else {
                panic!("Runtime error: Cannot use non-zero offset on a direct register reference");
            }
        } else {
            let actual_ptr = (base_ptr_val as usize) + (index * 2);
            if actual_ptr + 2 <= heap.len() {
                let mut val: u16 = 0;
                std::ptr::copy_nonoverlapping(
                    heap.as_ptr().add(actual_ptr),
                    &mut val as *mut u16 as *mut u8,
                    2,
                );
                *registers_ptr.add(bp + inst.a as usize) = u16::from_le(val) as u64;
            } else {
                panic!(
                    "Out of bounds in LoadPtrOffset16 (actual: {}, len: {})",
                    actual_ptr,
                    heap.len()
                );
            }
        }
    }
}

#[inline(always)]
pub fn store_ptr_offset_16(
    inst: &Instruction,
    bp: usize,
    registers_ptr: *mut u64,
    heap: &mut [u8],
) {
    unsafe {
        let base_ptr_val = *registers_ptr.add(bp + inst.a as usize);
        let val = (*registers_ptr.add(bp + inst.b as usize) & 0xFFFF) as u16;
        let index = (*registers_ptr.add(bp + inst.c as usize) & 0xFFFFFFFF) as usize;
        if (base_ptr_val & REG_PTR_TAG) != 0 {
            if index == 0 {
                let abs_reg_idx = (base_ptr_val & !REG_PTR_TAG) as usize;
                let existing = *registers_ptr.add(abs_reg_idx);
                *registers_ptr.add(abs_reg_idx) = (existing & !0xFFFF) | (val as u64);
            } else {
                panic!("Runtime error: Cannot use non-zero offset on a direct register reference");
            }
        } else {
            let actual_ptr = (base_ptr_val as usize) + (index * 2);
            if actual_ptr + 2 <= heap.len() {
                std::ptr::copy_nonoverlapping(
                    &val as *const u16 as *const u8,
                    heap.as_mut_ptr().add(actual_ptr),
                    2,
                );
            } else {
                panic!(
                    "Out of bounds in StorePtrOffset16 (actual: {}, len: {})",
                    actual_ptr,
                    heap.len()
                );
            }
        }
    }
}

#[inline(always)]
pub fn load_ptr_16(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &[u8]) {
    unsafe {
        let ptr_val = *registers_ptr.add(bp + inst.b as usize);
        if (ptr_val & REG_PTR_TAG) != 0 {
            let abs_reg_idx = (ptr_val & !REG_PTR_TAG) as usize;
            let val = *registers_ptr.add(abs_reg_idx);
            *registers_ptr.add(bp + inst.a as usize) = val & 0xFFFF;
        } else {
            let ptr = ptr_val as usize;
            if ptr + 2 <= heap.len() {
                let mut val: u16 = 0;
                std::ptr::copy_nonoverlapping(
                    heap.as_ptr().add(ptr),
                    &mut val as *mut u16 as *mut u8,
                    2,
                );
                *registers_ptr.add(bp + inst.a as usize) = u16::from_le(val) as u64;
            } else {
                panic!("Runtime error: Segmentation fault in LoadPtr16");
            }
        }
    }
}

#[inline(always)]
pub fn store_ptr_16(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &mut [u8]) {
    unsafe {
        let ptr_val = *registers_ptr.add(bp + inst.a as usize);
        let val = *registers_ptr.add(bp + inst.b as usize);
        if (ptr_val & REG_PTR_TAG) != 0 {
            let abs_reg_idx = (ptr_val & !REG_PTR_TAG) as usize;
            let existing = *registers_ptr.add(abs_reg_idx);
            *registers_ptr.add(abs_reg_idx) = (existing & !0xFFFF) | (val & 0xFFFF);
        } else {
            let ptr = ptr_val as usize;
            if ptr + 2 <= heap.len() {
                let val_16 = (val & 0xFFFF) as u16;
                std::ptr::copy_nonoverlapping(
                    &val_16 as *const u16 as *const u8,
                    heap.as_mut_ptr().add(ptr),
                    2,
                );
            } else {
                panic!("Runtime error: Segmentation fault in StorePtr16");
            }
        }
    }
}

// 32-bit

#[inline(always)]
pub fn load_ptr_32(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &[u8]) {
    unsafe {
        let ptr_val = *registers_ptr.add(bp + inst.b as usize);
        if (ptr_val & REG_PTR_TAG) != 0 {
            let abs_reg_idx = (ptr_val & !REG_PTR_TAG) as usize;
            let val = *registers_ptr.add(abs_reg_idx);
            *registers_ptr.add(bp + inst.a as usize) = val & 0xFFFFFFFF;
        } else {
            let ptr = ptr_val as usize;
            if ptr + 4 <= heap.len() {
                let mut val: u32 = 0;
                std::ptr::copy_nonoverlapping(
                    heap.as_ptr().add(ptr),
                    &mut val as *mut u32 as *mut u8,
                    4,
                );
                *registers_ptr.add(bp + inst.a as usize) = u32::from_le(val) as u64;
            } else {
                panic!("Runtime error: Segmentation fault in LoadPtr32");
            }
        }
    }
}

#[inline(always)]
pub fn store_ptr_32(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &mut [u8]) {
    unsafe {
        let ptr_val = *registers_ptr.add(bp + inst.a as usize);
        let val = *registers_ptr.add(bp + inst.b as usize);
        if (ptr_val & REG_PTR_TAG) != 0 {
            let abs_reg_idx = (ptr_val & !REG_PTR_TAG) as usize;
            let existing = *registers_ptr.add(abs_reg_idx);
            *registers_ptr.add(abs_reg_idx) = (existing & !0xFFFFFFFF) | (val & 0xFFFFFFFF);
        } else {
            let ptr = ptr_val as usize;
            if ptr + 4 <= heap.len() {
                let val_32 = (val & 0xFFFFFFFF) as u32;
                std::ptr::copy_nonoverlapping(
                    &val_32 as *const u32 as *const u8,
                    heap.as_mut_ptr().add(ptr),
                    4,
                );
            } else {
                panic!("Runtime error: Segmentation fault in StorePtr32");
            }
        }
    }
}

#[inline(always)]
pub fn load_ptr_offset_32(inst: &Instruction, bp: usize, registers_ptr: *mut u64, heap: &[u8]) {
    unsafe {
        let base_ptr_val = *registers_ptr.add(bp + inst.b as usize);
        let index = (*registers_ptr.add(bp + inst.c as usize) & 0xFFFFFFFF) as usize;
        if (base_ptr_val & REG_PTR_TAG) != 0 {
            if index == 0 {
                let abs_reg_idx = (base_ptr_val & !REG_PTR_TAG) as usize;
                let val = *registers_ptr.add(abs_reg_idx);
                *registers_ptr.add(bp + inst.a as usize) = val & 0xFFFFFFFF;
            } else {
                panic!("Runtime error: Cannot use non-zero offset on a direct register reference");
            }
        } else {
            let actual_ptr = (base_ptr_val as usize) + (index * 4);
            if actual_ptr + 4 <= heap.len() {
                let mut val: u32 = 0;
                std::ptr::copy_nonoverlapping(
                    heap.as_ptr().add(actual_ptr),
                    &mut val as *mut u32 as *mut u8,
                    4,
                );
                *registers_ptr.add(bp + inst.a as usize) = u32::from_le(val) as u64;
            } else {
                panic!(
                    "Out of bounds in LoadPtrOffset32 (actual: {}, len: {})",
                    actual_ptr,
                    heap.len()
                );
            }
        }
    }
}

#[inline(always)]
pub fn store_ptr_offset_32(
    inst: &Instruction,
    bp: usize,
    registers_ptr: *mut u64,
    heap: &mut [u8],
) {
    unsafe {
        let base_ptr_val = *registers_ptr.add(bp + inst.a as usize);
        let val = (*registers_ptr.add(bp + inst.b as usize) & 0xFFFFFFFF) as u32;
        let index = (*registers_ptr.add(bp + inst.c as usize) & 0xFFFFFFFF) as usize;

        if (base_ptr_val & REG_PTR_TAG) != 0 {
            if index == 0 {
                let abs_reg_idx = (base_ptr_val & !REG_PTR_TAG) as usize;
                let existing = *registers_ptr.add(abs_reg_idx);
                *registers_ptr.add(abs_reg_idx) = (existing & !0xFFFFFFFF) | (val as u64);
            } else {
                panic!("Runtime error: Cannot use non-zero offset on a direct register reference");
            }
        } else {
            let actual_ptr = (base_ptr_val as usize) + (index * 4);
            if actual_ptr + 4 <= heap.len() {
                std::ptr::copy_nonoverlapping(
                    &val as *const u32 as *const u8,
                    heap.as_mut_ptr().add(actual_ptr),
                    4,
                );
            } else {
                panic!(
                    "Out of bounds in StorePtrOffset32 (actual: {}, len: {})",
                    actual_ptr,
                    heap.len()
                );
            }
        }
    }
}
