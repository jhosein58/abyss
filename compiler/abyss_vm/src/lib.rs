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
    PrintI,

    // Integer Comparisons
    CmpEqI,  // ==
    CmpNeqI, // !=
    CmpLtI,  // <
    CmpLeI,  // <=
    CmpGtI,  // >
    CmpGeI,  // >=

    // Float Math
    AddF,
    SubF,
    MulF,
    DivF,
    PrintF,

    // Float Comparisons
    CmpEqF,  // ==
    CmpNeqF, // !=
    CmpLtF,  // <
    CmpLeF,  // <=
    CmpGtF,  // >
    CmpGeF,  // >=

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
    fn set_reg(&mut self, r: u8, val: u64) {
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

        loop {
            let inst = self.program[self.ip];
            self.ip += 1;

            match inst.op {
                OpCode::Halt => {
                    break;
                }

                OpCode::LoadConst => {
                    let val = self.constants[inst.b as usize];
                    self.set_reg(inst.a, val);
                }

                OpCode::Move => {
                    let val = self.get_reg(inst.b);
                    self.set_reg(inst.a, val);
                }

                // Integer Math
                OpCode::AddI => {
                    let left = self.get_register_as_i64(inst.b);
                    let right = self.get_register_as_i64(inst.c);
                    self.set_reg(inst.a, (left + right) as u64);
                }
                OpCode::SubI => {
                    let left = self.get_register_as_i64(inst.b);
                    let right = self.get_register_as_i64(inst.c);
                    self.set_reg(inst.a, (left - right) as u64);
                }
                OpCode::MulI => {
                    let left = self.get_register_as_i64(inst.b);
                    let right = self.get_register_as_i64(inst.c);
                    self.set_reg(inst.a, (left * right) as u64);
                }
                OpCode::DivI => {
                    let left = self.get_register_as_i64(inst.b);
                    let right = self.get_register_as_i64(inst.c);
                    self.set_reg(inst.a, (left / right) as u64);
                }
                OpCode::PrintI => {
                    let val = self.get_register_as_i64(inst.a);
                    println!("--> [Int] {}", val);
                }

                // Integer Comparisons
                OpCode::CmpEqI => {
                    let left = self.get_register_as_i64(inst.b);
                    let right = self.get_register_as_i64(inst.c);
                    self.set_reg(inst.a, if left == right { 1 } else { 0 });
                }
                OpCode::CmpNeqI => {
                    let left = self.get_register_as_i64(inst.b);
                    let right = self.get_register_as_i64(inst.c);
                    self.set_reg(inst.a, if left != right { 1 } else { 0 });
                }
                OpCode::CmpLtI => {
                    let left = self.get_register_as_i64(inst.b);
                    let right = self.get_register_as_i64(inst.c);
                    self.set_reg(inst.a, if left < right { 1 } else { 0 });
                }
                OpCode::CmpLeI => {
                    let left = self.get_register_as_i64(inst.b);
                    let right = self.get_register_as_i64(inst.c);
                    self.set_reg(inst.a, if left <= right { 1 } else { 0 });
                }
                OpCode::CmpGtI => {
                    let left = self.get_register_as_i64(inst.b);
                    let right = self.get_register_as_i64(inst.c);
                    self.set_reg(inst.a, if left > right { 1 } else { 0 });
                }
                OpCode::CmpGeI => {
                    let left = self.get_register_as_i64(inst.b);
                    let right = self.get_register_as_i64(inst.c);
                    self.set_reg(inst.a, if left >= right { 1 } else { 0 });
                }

                // Float Math
                OpCode::AddF => {
                    let left = self.get_register_as_f64(inst.b);
                    let right = self.get_register_as_f64(inst.c);
                    self.set_reg(inst.a, (left + right).to_bits());
                }
                OpCode::SubF => {
                    let left = self.get_register_as_f64(inst.b);
                    let right = self.get_register_as_f64(inst.c);
                    self.set_reg(inst.a, (left - right).to_bits());
                }
                OpCode::MulF => {
                    let left = self.get_register_as_f64(inst.b);
                    let right = self.get_register_as_f64(inst.c);
                    self.set_reg(inst.a, (left * right).to_bits());
                }
                OpCode::DivF => {
                    let left = self.get_register_as_f64(inst.b);
                    let right = self.get_register_as_f64(inst.c);
                    self.set_reg(inst.a, (left / right).to_bits());
                }
                OpCode::PrintF => {
                    let val = self.get_register_as_f64(inst.a);
                    println!("--> [Float] {}", val);
                }

                // Float Comparisons (Example)
                OpCode::CmpEqF => {
                    let left = self.get_register_as_f64(inst.b);
                    let right = self.get_register_as_f64(inst.c);
                    self.set_reg(inst.a, if left == right { 1 } else { 0 });
                }
                OpCode::CmpNeqF => {
                    let left = self.get_register_as_f64(inst.b);
                    let right = self.get_register_as_f64(inst.c);
                    self.set_reg(inst.a, if left != right { 1 } else { 0 });
                }

                OpCode::CmpLtF => {
                    let left = self.get_register_as_f64(inst.b);
                    let right = self.get_register_as_f64(inst.c);
                    self.set_reg(inst.a, if left < right { 1 } else { 0 });
                }

                OpCode::CmpLeF => {
                    let left = self.get_register_as_f64(inst.b);
                    let right = self.get_register_as_f64(inst.c);
                    self.set_reg(inst.a, if left <= right { 1 } else { 0 });
                }
                OpCode::CmpGtF => {
                    let left = self.get_register_as_f64(inst.b);
                    let right = self.get_register_as_f64(inst.c);
                    self.set_reg(inst.a, if left > right { 1 } else { 0 });
                }
                OpCode::CmpGeF => {
                    let left = self.get_register_as_f64(inst.b);
                    let right = self.get_register_as_f64(inst.c);
                    self.set_reg(inst.a, if left >= right { 1 } else { 0 });
                }

                // Logical Operations
                OpCode::Not => {
                    let val = self.get_reg(inst.b);
                    self.set_reg(inst.a, if val == 0 { 1 } else { 0 });
                }

                OpCode::Call => {
                    let target_ip = self.get_reg(inst.b) as usize;

                    self.call_stack.push(CallFrame {
                        ret_ip: self.ip,
                        ret_reg: inst.a,
                        bp: self.bp,
                    });

                    self.bp += inst.c as usize;
                    self.ip = target_ip;
                }

                OpCode::Ret => {
                    let ret_val = self.get_reg(inst.a);

                    if let Some(frame) = self.call_stack.pop() {
                        self.ip = frame.ret_ip;
                        self.bp = frame.bp;
                        self.set_reg(frame.ret_reg, ret_val);
                    } else {
                        final_result = Some(ret_val);
                        break;
                    }
                }

                OpCode::Jmp => {
                    self.ip = self.get_reg(inst.a) as usize;
                }

                OpCode::JmpIf => {
                    let condition = self.get_reg(inst.b);
                    if condition != 0 {
                        self.ip = self.get_reg(inst.a) as usize;
                    }
                }

                OpCode::JmpImm => {
                    let target_ip = ((inst.b as u16) << 8) | (inst.c as u16);
                    self.ip = target_ip as usize;
                }

                OpCode::JmpZImm => {
                    let condition = self.get_reg(inst.a);

                    if condition == 0 {
                        let target_ip = ((inst.b as u16) << 8) | (inst.c as u16);
                        self.ip = target_ip as usize;
                    }
                }

                OpCode::Alloc => {
                    let size = self.get_reg(inst.b) as usize;
                    let ptr = self.heap.len();

                    self.heap.resize(ptr + size, 0);

                    self.set_reg(inst.a, ptr as u64);
                }

                OpCode::LoadPtr => {
                    let ptr = self.get_reg(inst.b) as usize;

                    if ptr + 8 <= self.heap.len() {
                        let bytes: [u8; 8] = self.heap[ptr..ptr + 8].try_into().unwrap();
                        let val = u64::from_le_bytes(bytes);
                        self.set_reg(inst.a, val);
                    } else {
                        panic!("Segmentation fault (core dumped): Invalid Read at {}", ptr);
                    }
                }

                OpCode::StorePtr => {
                    let ptr = self.get_reg(inst.a) as usize;
                    let val = self.get_reg(inst.b);

                    if ptr + 8 <= self.heap.len() {
                        let bytes = val.to_le_bytes();
                        self.heap[ptr..ptr + 8].copy_from_slice(&bytes);
                    } else {
                        panic!("Segmentation fault (core dumped): Invalid Write at {}", ptr);
                    }
                }

                OpCode::CallNative => {
                    let func_idx = inst.b as usize;
                    let arg_start_reg = inst.c;

                    if let Some(native) = self.native_funcs.get(func_idx) {
                        let args_start_abs = self.bp + arg_start_reg as usize;
                        let arg_count = native.arity as usize;
                        let args = &self.registers[args_start_abs..args_start_abs + arg_count];

                        let result = (native.function)(args);

                        self.set_reg(inst.a, result);
                    } else {
                        panic!("Native function index {} not found!", func_idx);
                    }
                }
            }
        }
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
