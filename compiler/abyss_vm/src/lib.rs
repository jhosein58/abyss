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

    // Float Math
    AddF,
    SubF,
    MulF,
    DivF,
    PrintF,

    Call,
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

pub struct AbyssVm {
    registers: Vec<u64>,
    bp: usize,
    call_stack: Vec<CallFrame>,
    constants: Vec<u64>,
    program: Vec<Instruction>,
    ip: usize,
}

impl AbyssVm {
    pub fn new(program: Vec<Instruction>, constants: Vec<u64>) -> Self {
        Self {
            registers: vec![0; 65536],
            bp: 0,
            call_stack: Vec::new(),
            constants,
            program,
            ip: 0,
        }
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

    pub fn run(&mut self) {
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
            }
        }
    }
}
