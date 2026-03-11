use abyss_ir::ir::{IrLit, IrProgram, IrType};

use crate::codegen::IrCompiler;

pub mod codegen;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OpCode {
    Halt = 0,
    LoadConst,
    Move, // r[a] = r[b]

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
    Alloc,    // a = alloc(b)
    LoadPtr,  // a = *b
    StorePtr, // *a = b

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

pub type NativeFunction = fn(args: &[u64]) -> u64;

pub struct RegisteredNative {
    pub function: NativeFunction,
    pub arity: u8,
}
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
        }
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

                // Integer Math
                OpCode::AddI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    set_reg!(inst.a, (left + right) as u64);
                }
                OpCode::SubI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    set_reg!(inst.a, (left - right) as u64);
                }
                OpCode::MulI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    set_reg!(inst.a, (left * right) as u64);
                }
                OpCode::DivI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    set_reg!(inst.a, (left / right) as u64);
                }
                OpCode::ModI => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_reg!(inst.c) as i64;
                    if right == 0 {
                        self.ip = ip;
                        self.bp = bp;
                        panic!("Runtime error: Division by zero");
                    }
                    set_reg!(inst.a, (left % right) as u64);
                }

                // Integer Math with Constant
                OpCode::AddIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    set_reg!(inst.a, (left + right) as u64);
                }
                OpCode::SubIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    set_reg!(inst.a, (left - right) as u64);
                }
                OpCode::MulIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    set_reg!(inst.a, (left * right) as u64);
                }
                OpCode::DivIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    set_reg!(inst.a, (left / right) as u64);
                }
                OpCode::ModIC => {
                    let left = get_reg!(inst.b) as i64;
                    let right = get_const!(inst.c) as i64;
                    if right == 0 {
                        self.ip = ip;
                        self.bp = bp;
                        panic!("Runtime error: Division by zero");
                    }
                    set_reg!(inst.a, (left % right) as u64);
                }

                OpCode::PrintI => {
                    let val = get_reg!(inst.a) as i64;
                    println!("--> [Int] {}", val);
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

                OpCode::Alloc => {
                    let size = get_reg!(inst.b) as usize;
                    let ptr = self.heap.len();
                    self.heap.resize(ptr + size, 0);
                    set_reg!(inst.a, ptr as u64);
                }

                OpCode::LoadPtr => {
                    let ptr = get_reg!(inst.b) as usize;
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

                OpCode::StorePtr => {
                    let ptr = get_reg!(inst.a) as usize;
                    let val = get_reg!(inst.b);
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

                OpCode::CallNative => {
                    let func_idx = inst.b as usize;
                    let arg_start_reg = inst.c;

                    self.ip = ip;
                    self.bp = bp;

                    if let Some(native) = self.native_funcs.get(func_idx) {
                        let args_start_abs = bp + arg_start_reg as usize;
                        let arg_count = native.arity as usize;

                        let args = unsafe {
                            std::slice::from_raw_parts(registers_ptr.add(args_start_abs), arg_count)
                        };

                        let result = (native.function)(args);
                        set_reg!(inst.a, result);
                    } else {
                        panic!("Native function not found!");
                    }
                }
            }
        }

        self.ip = ip;
        self.bp = bp;

        final_result
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
