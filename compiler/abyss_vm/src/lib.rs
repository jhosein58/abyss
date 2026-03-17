use abyss_ir::ir::{IrLit, IrProgram, IrType};

use crate::codegen::IrCompiler;

pub mod codegen;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OpCode {
    Halt = 0,
    LoadConst,
    Move, // r[a] = r[b]

    // Casting
    CastI2F, // Integer to Float
    CastF2I, // Float to Integer
    CastI2B, // Integer to Boolean
    CastF2B, // Float to Boolean

    // Integer Math
    AddI,
    SubI,
    MulI,
    DivI,
    ModI,
    PrintI,

    // Integer Math with Constant
    AddIC,
    SubIC,
    MulIC,
    DivIC,
    ModIC,

    // Integer Comparisons
    CmpEqI,  // ==
    CmpNeqI, // !=
    CmpLtI,  // <
    CmpLeI,  // <=
    CmpGtI,  // >
    CmpGeI,  // >=

    // Integer Comparisons with Constant
    CmpEqIC,
    CmpNeqIC,
    CmpLtIC,
    CmpLeIC,
    CmpGtIC,
    CmpGeIC,

    // Float Math
    AddF,
    SubF,
    MulF,
    DivF,
    PrintF,

    // Float Math with Constant
    AddFC,
    SubFC,
    MulFC,
    DivFC,

    // Float Comparisons
    CmpEqF,  // ==
    CmpNeqF, // !=
    CmpLtF,  // <
    CmpLeF,  // <=
    CmpGtF,  // >
    CmpGeF,  // >=

    // Float Comparisons with Constant
    CmpEqFC,
    CmpNeqFC,
    CmpLtFC,
    CmpLeFC,
    CmpGtFC,
    CmpGeFC,

    // Logical
    Not,

    // Memory & Pointers
    Alloc,          // a = alloc(b)
    LoadPtr,        // a = *b
    StorePtr,       // *a = b
    LoadPtrOffset,  // a = *(b + c * 8)
    StorePtrOffset, // *(a + c * 8) = b
    RefReg,
    MemCopy,

    Call,
    CallNative,
    Ret,
    Jmp,
    JmpIf,

    JmpImm,
    JmpZImm,
}

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    pub op: OpCode,
    pub a: u8,
    pub b: u8,
    pub c: u8,
}

pub struct CallFrame {
    pub ret_ip: usize,
    pub ret_reg: u8,
    pub bp: usize,
}

pub type NativeFunction = fn(vm: &mut AbyssVm, args: &[u64]) -> u64;

pub struct RegisteredNative {
    pub function: NativeFunction,
    pub arity: u8,
}

const REG_PTR_TAG: u64 = 1 << 63;

pub struct AbyssVm {
    // Stack & Execution
    registers: Vec<u64>,
    bp: usize,
    call_stack: Vec<CallFrame>,
    ip: usize,

    // Data
    program: Vec<Instruction>,
    constants: Vec<u64>,

    heap: Vec<u8>,
    native_funcs: Vec<RegisteredNative>,

    pub out: String,
}

impl AbyssVm {
    pub fn new(program: Vec<Instruction>, constants: Vec<u64>) -> Self {
        Self {
            registers: vec![0; 65536],
            bp: 0,
            call_stack: Vec::with_capacity(1024),
            program,
            constants,
            ip: 0,
            heap: Vec::new(),
            native_funcs: Vec::new(),
            out: String::new(),
        }
    }

    pub fn new_empty() -> Self {
        Self {
            registers: vec![0; 65536],
            bp: 0,
            call_stack: Vec::with_capacity(1024),
            program: Vec::new(),
            constants: Vec::new(),
            ip: 0,
            heap: Vec::new(),
            native_funcs: Vec::new(),
            out: String::new(),
        }
    }

    pub fn inject_constants(&mut self, new_constants: &[u64]) -> usize {
        let offset = self.constants.len();
        self.constants.extend_from_slice(new_constants);
        offset
    }

    pub fn inject_instructions(&mut self, new_instructions: &[Instruction]) -> usize {
        let start_ip = self.program.len();
        self.program.extend_from_slice(new_instructions);
        start_ip
    }

    pub fn rewind_state(&mut self, saved_ip: usize, saved_const_len: usize) {
        self.program.truncate(saved_ip);
        self.constants.truncate(saved_const_len);
    }

    pub fn run_thunk(&mut self, start_ip: usize) -> Option<u64> {
        self.ip = start_ip;
        self.bp = 0;

        self.run()
    }

    pub fn register_native(&mut self, arity: u8, func: NativeFunction) -> usize {
        self.native_funcs.push(RegisteredNative {
            function: func,
            arity,
        });
        self.native_funcs.len() - 1
    }

    #[inline(always)]
    fn get_reg(&self, r: u8) -> u64 {
        self.registers[self.bp + r as usize]
    }

    #[inline(always)]
    pub fn set_reg(&mut self, r: u8, val: u64) {
        self.registers[self.bp + r as usize] = val;
    }

    pub fn get_register_as_i64(&self, r: u8) -> i64 {
        self.get_reg(r) as i64
    }

    pub fn get_register_as_f64(&self, r: u8) -> f64 {
        f64::from_bits(self.get_reg(r))
    }

    pub fn run(&mut self) -> Option<u64> {
        let mut final_result = None;

        let mut ip = self.ip;
        let mut bp = self.bp;

        let program_ptr = self.program.as_ptr();
        let registers_ptr = self.registers.as_mut_ptr();
        let constants_ptr = self.constants.as_ptr();

        macro_rules! get_reg {
            ($r:expr) => {
                unsafe { *registers_ptr.add(bp + $r as usize) }
            };
        }

        macro_rules! set_reg {
            ($r:expr, $val:expr) => {
                unsafe {
                    *registers_ptr.add(bp + $r as usize) = $val;
                }
            };
        }

        macro_rules! get_const {
            ($c:expr) => {
                unsafe { *constants_ptr.add($c as usize) }
            };
        }

        loop {
            let inst = unsafe { *program_ptr.add(ip) };
            ip += 1;

            match inst.op {
                OpCode::Halt => {
                    break;
                }

                OpCode::LoadConst => {
                    let val = unsafe { *self.constants.get_unchecked(inst.b as usize) };
                    set_reg!(inst.a, val);
                }

                OpCode::Move => {
                    let val = get_reg!(inst.b);
                    set_reg!(inst.a, val);
                }

                // Casting Operations
                OpCode::CastI2F => {
                    let val_int = get_reg!(inst.b) as i64;
                    let val_float = val_int as f64;
                    set_reg!(inst.a, val_float.to_bits());
                }
                OpCode::CastF2I => {
                    let val_float = f64::from_bits(get_reg!(inst.b));
                    let val_int = val_float as i64;
                    set_reg!(inst.a, val_int as u64);
                }
                OpCode::CastI2B => {
                    let val_int = get_reg!(inst.b) as i64;
                    set_reg!(inst.a, if val_int != 0 { 1 } else { 0 });
                }
                OpCode::CastF2B => {
                    let val_float = f64::from_bits(get_reg!(inst.b));
                    set_reg!(inst.a, if val_float != 0.0 { 1 } else { 0 });
                }

                // Integer Math
                OpCode::AddI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    set_reg!(inst.a, left.wrapping_add(right) as u64);
                }
                OpCode::SubI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    set_reg!(inst.a, left.wrapping_sub(right) as u64);
                }
                OpCode::MulI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    set_reg!(inst.a, left.wrapping_mul(right) as u64);
                }
                OpCode::DivI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    if right == 0 {
                        self.ip = ip;
                        self.bp = bp;
                        panic!("Runtime error: Division by zero");
                    }
                    set_reg!(inst.a, left.wrapping_div(right) as u64);
                }
                OpCode::ModI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    if right == 0 {
                        self.ip = ip;
                        self.bp = bp;
                        panic!("Runtime error: Division by zero");
                    }
                    set_reg!(inst.a, left.wrapping_rem(right) as u64);
                }

                // Integer Math with Constant
                OpCode::AddIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    set_reg!(inst.a, left.wrapping_add(right) as u64);
                }
                OpCode::SubIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    set_reg!(inst.a, left.wrapping_sub(right) as u64);
                }
                OpCode::MulIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    set_reg!(inst.a, left.wrapping_mul(right) as u64);
                }
                OpCode::DivIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    if right == 0 {
                        self.ip = ip;
                        self.bp = bp;
                        panic!("Runtime error: Division by zero");
                    }
                    set_reg!(inst.a, left.wrapping_div(right) as u64);
                }
                OpCode::ModIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    if right == 0 {
                        self.ip = ip;
                        self.bp = bp;
                        panic!("Runtime error: Division by zero");
                    }
                    set_reg!(inst.a, left.wrapping_rem(right) as u64);
                }

                OpCode::PrintI => {
                    let val = get_reg!(inst.a) as i64;
                    println!("--> [Int] {}", val);

                    self.out.push_str(format!("int:    {}\n", val).as_str());
                }

                // Integer Comparisons
                OpCode::CmpEqI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    set_reg!(inst.a, if left == right { 1 } else { 0 });
                }
                OpCode::CmpNeqI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    set_reg!(inst.a, if left != right { 1 } else { 0 });
                }
                OpCode::CmpLtI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    set_reg!(inst.a, if left < right { 1 } else { 0 });
                }
                OpCode::CmpLeI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    set_reg!(inst.a, if left <= right { 1 } else { 0 });
                }
                OpCode::CmpGtI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    set_reg!(inst.a, if left > right { 1 } else { 0 });
                }
                OpCode::CmpGeI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    set_reg!(inst.a, if left >= right { 1 } else { 0 });
                }

                // Integer Comparisons with Constant
                OpCode::CmpEqIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    set_reg!(inst.a, if left == right { 1 } else { 0 });
                }
                OpCode::CmpNeqIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    set_reg!(inst.a, if left != right { 1 } else { 0 });
                }
                OpCode::CmpLtIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    set_reg!(inst.a, if left < right { 1 } else { 0 });
                }
                OpCode::CmpLeIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    set_reg!(inst.a, if left <= right { 1 } else { 0 });
                }
                OpCode::CmpGtIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    set_reg!(inst.a, if left > right { 1 } else { 0 });
                }
                OpCode::CmpGeIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    set_reg!(inst.a, if left >= right { 1 } else { 0 });
                }

                // Float Math
                OpCode::AddF => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_reg!(inst.c));
                    set_reg!(inst.a, (left + right).to_bits());
                }
                OpCode::SubF => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_reg!(inst.c));
                    set_reg!(inst.a, (left - right).to_bits());
                }
                OpCode::MulF => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_reg!(inst.c));
                    set_reg!(inst.a, (left * right).to_bits());
                }
                OpCode::DivF => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_reg!(inst.c));
                    set_reg!(inst.a, (left / right).to_bits());
                }
                OpCode::PrintF => {
                    let val = f64::from_bits(get_reg!(inst.a));
                    println!("--> [Float] {}", val);

                    self.out.push_str(format!("float:  {}\n", val).as_str());
                }

                // Float Math with Constant
                OpCode::AddFC => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_const!(inst.c));
                    set_reg!(inst.a, (left + right).to_bits());
                }
                OpCode::SubFC => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_const!(inst.c));
                    set_reg!(inst.a, (left - right).to_bits());
                }
                OpCode::MulFC => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_const!(inst.c));
                    set_reg!(inst.a, (left * right).to_bits());
                }
                OpCode::DivFC => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_const!(inst.c));
                    set_reg!(inst.a, (left / right).to_bits());
                }

                // Float Comparisons
                OpCode::CmpEqF => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_reg!(inst.c));
                    set_reg!(inst.a, if left == right { 1 } else { 0 });
                }
                OpCode::CmpNeqF => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_reg!(inst.c));
                    set_reg!(inst.a, if left != right { 1 } else { 0 });
                }
                OpCode::CmpLtF => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_reg!(inst.c));
                    set_reg!(inst.a, if left < right { 1 } else { 0 });
                }
                OpCode::CmpLeF => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_reg!(inst.c));
                    set_reg!(inst.a, if left <= right { 1 } else { 0 });
                }
                OpCode::CmpGtF => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_reg!(inst.c));
                    set_reg!(inst.a, if left > right { 1 } else { 0 });
                }
                OpCode::CmpGeF => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_reg!(inst.c));
                    set_reg!(inst.a, if left >= right { 1 } else { 0 });
                }

                // Float Comparisons with Constant
                OpCode::CmpEqFC => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_const!(inst.c));
                    set_reg!(inst.a, if left == right { 1 } else { 0 });
                }
                OpCode::CmpNeqFC => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_const!(inst.c));
                    set_reg!(inst.a, if left != right { 1 } else { 0 });
                }
                OpCode::CmpLtFC => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_const!(inst.c));
                    set_reg!(inst.a, if left < right { 1 } else { 0 });
                }
                OpCode::CmpLeFC => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_const!(inst.c));
                    set_reg!(inst.a, if left <= right { 1 } else { 0 });
                }
                OpCode::CmpGtFC => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_const!(inst.c));
                    set_reg!(inst.a, if left > right { 1 } else { 0 });
                }
                OpCode::CmpGeFC => {
                    let left = f64::from_bits(get_reg!(inst.b));
                    let right = f64::from_bits(get_const!(inst.c));
                    set_reg!(inst.a, if left >= right { 1 } else { 0 });
                }

                // Logical Operations
                OpCode::Not => {
                    let val = get_reg!(inst.b);
                    set_reg!(inst.a, if val == 0 { 1 } else { 0 });
                }

                // Control Flow
                OpCode::Call => {
                    let target_ip = get_reg!(inst.b) as usize;

                    self.call_stack.push(CallFrame {
                        ret_ip: ip,
                        ret_reg: inst.a,
                        bp: bp,
                    });

                    bp += inst.c as usize;
                    ip = target_ip;
                }

                OpCode::Ret => {
                    let ret_val = get_reg!(inst.a);

                    if let Some(frame) = self.call_stack.pop() {
                        ip = frame.ret_ip;
                        bp = frame.bp;
                        set_reg!(frame.ret_reg, ret_val);
                    } else {
                        final_result = Some(ret_val);
                        break;
                    }
                }

                OpCode::Jmp => {
                    ip = get_reg!(inst.a) as usize;
                }

                OpCode::JmpIf => {
                    let condition = get_reg!(inst.b);
                    if condition != 0 {
                        ip = get_reg!(inst.a) as usize;
                    }
                }

                OpCode::JmpImm => {
                    let target_ip = ((inst.b as u16) << 8) | (inst.c as u16);
                    ip = target_ip as usize;
                }

                OpCode::JmpZImm => {
                    let condition = get_reg!(inst.a);
                    if condition == 0 {
                        let target_ip = ((inst.b as u16) << 8) | (inst.c as u16);
                        ip = target_ip as usize;
                    }
                }

                // Memory & Pointers
                OpCode::Alloc => {
                    let size = get_reg!(inst.b) as usize;
                    let ptr = self.heap.len();
                    self.heap.resize(ptr + size, 0);
                    set_reg!(inst.a, ptr as u64);
                }

                OpCode::RefReg => {
                    let reg_idx = inst.b;
                    let abs_addr = bp + reg_idx as usize;

                    let tagged_ptr = (abs_addr as u64) | REG_PTR_TAG;
                    set_reg!(inst.a, tagged_ptr);
                }

                OpCode::LoadPtr => {
                    let ptr_val = get_reg!(inst.b);

                    if (ptr_val & REG_PTR_TAG) != 0 {
                        let abs_reg_idx = (ptr_val & !REG_PTR_TAG) as usize;
                        let val = unsafe { *registers_ptr.add(abs_reg_idx) };
                        set_reg!(inst.a, val);
                    } else {
                        let ptr = ptr_val as usize;
                        if ptr + 8 <= self.heap.len() {
                            let mut val: u64 = 0;
                            unsafe {
                                std::ptr::copy_nonoverlapping(
                                    self.heap.as_ptr().add(ptr),
                                    &mut val as *mut u64 as *mut u8,
                                    8,
                                );
                            }
                            set_reg!(inst.a, u64::from_le(val));
                        } else {
                            self.ip = ip;
                            self.bp = bp;
                            panic!("Segmentation fault");
                        }
                    }
                }

                OpCode::StorePtr => {
                    let ptr_val = get_reg!(inst.a);
                    let val = get_reg!(inst.b);

                    if (ptr_val & REG_PTR_TAG) != 0 {
                        let abs_reg_idx = (ptr_val & !REG_PTR_TAG) as usize;
                        unsafe {
                            *registers_ptr.add(abs_reg_idx) = val;
                        }
                    } else {
                        let ptr = ptr_val as usize;
                        if ptr + 8 <= self.heap.len() {
                            unsafe {
                                std::ptr::copy_nonoverlapping(
                                    &val as *const u64 as *const u8,
                                    self.heap.as_mut_ptr().add(ptr),
                                    8,
                                );
                            }
                        } else {
                            self.ip = ip;
                            self.bp = bp;
                            panic!("Segmentation fault");
                        }
                    }
                }

                OpCode::LoadPtrOffset => {
                    let base_ptr_val = get_reg!(inst.b);
                    let index = get_reg!(inst.c) as usize;

                    if (base_ptr_val & REG_PTR_TAG) != 0 {
                        if index == 0 {
                            let abs_reg_idx = (base_ptr_val & !REG_PTR_TAG) as usize;
                            let val = unsafe { *registers_ptr.add(abs_reg_idx) };
                            set_reg!(inst.a, val);
                        } else {
                            self.ip = ip;
                            self.bp = bp;
                            panic!(
                                "Runtime error: Cannot use non-zero offset on a direct register reference"
                            );
                        }
                    } else {
                        let base_ptr = base_ptr_val as usize;
                        let actual_ptr = base_ptr + (index * 8);

                        if actual_ptr + 8 <= self.heap.len() {
                            let mut val: u64 = 0;
                            unsafe {
                                std::ptr::copy_nonoverlapping(
                                    self.heap.as_ptr().add(actual_ptr),
                                    &mut val as *mut u64 as *mut u8,
                                    8,
                                );
                            }
                            set_reg!(inst.a, u64::from_le(val));
                        } else {
                            self.ip = ip;
                            self.bp = bp;
                            panic!("Runtime error: Memory access out of bounds");
                        }
                    }
                }

                OpCode::StorePtrOffset => {
                    let base_ptr_val = get_reg!(inst.a);
                    let val = get_reg!(inst.b);
                    let index = get_reg!(inst.c) as usize;

                    if (base_ptr_val & REG_PTR_TAG) != 0 {
                        if index == 0 {
                            let abs_reg_idx = (base_ptr_val & !REG_PTR_TAG) as usize;
                            unsafe {
                                *registers_ptr.add(abs_reg_idx) = val;
                            }
                        } else {
                            self.ip = ip;
                            self.bp = bp;
                            panic!(
                                "Runtime error: Cannot use non-zero offset on a direct register reference"
                            );
                        }
                    } else {
                        let base_ptr = base_ptr_val as usize;
                        let actual_ptr = base_ptr + (index * 8);

                        if actual_ptr + 8 <= self.heap.len() {
                            unsafe {
                                std::ptr::copy_nonoverlapping(
                                    &val as *const u64 as *const u8,
                                    self.heap.as_mut_ptr().add(actual_ptr),
                                    8,
                                );
                            }
                        } else {
                            self.ip = ip;
                            self.bp = bp;
                            panic!("Runtime error: Memory access out of bounds");
                        }
                    }
                }

                OpCode::CallNative => {
                    let func_idx = inst.b as usize;
                    let arg_start_reg = inst.c;

                    self.ip = ip;
                    self.bp = bp;

                    let (func, arity) = {
                        let native = &self.native_funcs[func_idx];
                        (native.function, native.arity as usize)
                    };

                    let args_start_abs = bp + arg_start_reg as usize;
                    let mut args = Vec::with_capacity(arity);
                    for i in 0..arity {
                        args.push(unsafe { *registers_ptr.add(args_start_abs + i) });
                    }

                    let result = func(self, &args);

                    set_reg!(inst.a, result);
                }

                OpCode::MemCopy => {
                    let dest_ptr_val = get_reg!(inst.a);
                    let src_ptr_val = get_reg!(inst.b);
                    let count = get_reg!(inst.c) as usize;
                    let bytes_to_copy = count * 8;

                    let dest_ptr = (dest_ptr_val & !REG_PTR_TAG) as usize;
                    let src_ptr = (src_ptr_val & !REG_PTR_TAG) as usize;

                    if dest_ptr + bytes_to_copy <= self.heap.len()
                        && src_ptr + bytes_to_copy <= self.heap.len()
                    {
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                self.heap.as_ptr().add(src_ptr),
                                self.heap.as_mut_ptr().add(dest_ptr),
                                bytes_to_copy,
                            );
                        }
                    } else {
                        self.ip = ip;
                        self.bp = bp;
                        panic!("Runtime error: MemCopy out of bounds");
                    }
                }
            }
        }

        self.ip = ip;
        self.bp = bp;

        final_result
    }

    pub fn read_ptr_value(&self, ptr_val: u64) -> u64 {
        if (ptr_val & REG_PTR_TAG) != 0 {
            let abs_reg_idx = (ptr_val & !REG_PTR_TAG) as usize;
            self.registers[abs_reg_idx]
        } else {
            let ptr = ptr_val as usize;
            if ptr + 8 <= self.heap.len() {
                let mut val: u64 = 0;
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        self.heap.as_ptr().add(ptr),
                        &mut val as *mut u64 as *mut u8,
                        8,
                    );
                }
                u64::from_le(val)
            } else {
                panic!(
                    "Native Helper Error: Memory access out of bounds at {}",
                    ptr
                );
            }
        }
    }

    pub fn read_c_string(&self, base_ptr: u64) -> String {
        let mut s = String::new();
        let mut offset = 0;

        loop {
            let current_ptr = if (base_ptr & REG_PTR_TAG) != 0 {
                if offset == 0 {
                    base_ptr
                } else {
                    panic!("Native Helper Error: Cannot use offset on register pointer");
                }
            } else {
                base_ptr + (offset * 8)
            };

            let val = self.read_ptr_value(current_ptr);

            if val == 0 {
                break;
            }

            if let Some(c) = std::char::from_u32(val as u32) {
                s.push(c);
            } else {
                s.push('0');
            }

            offset += 1;
        }

        s
    }
}

pub fn execute_comptime(ir_prog: IrProgram) -> IrLit {
    let expected_type = ir_prog
        .functions
        .iter()
        .find(|f| f.name == "main")
        .map(|f| f.return_ty.clone())
        .expect("Comptime program must have a main function");

    let compiler = IrCompiler::new();

    let (instructions, constants) = compiler.compile(&ir_prog);

    let mut vm = AbyssVm::new(instructions, constants);

    let raw_result = vm.run().unwrap_or(0);

    match expected_type {
        IrType::I32 => IrLit::Int(raw_result as i64),
        IrType::F32 => IrLit::Float(f64::from_bits(raw_result)),
        IrType::Bool => IrLit::Bool(raw_result != 0),
        IrType::Unit => IrLit::Bool(false),
        _ => panic!("Unsupported comptime return type: {:?}", expected_type),
    }
}
