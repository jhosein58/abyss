use crate::vm::{core::AbyssVm, opcode::OpCode};

#[inline(always)]
fn op(s: &str) -> String {
    format!("\x1b[36m{}\x1b[0m", s)
}

#[inline(always)]
fn reg(r: u8) -> String {
    format!("\x1b[32mr{}\x1b[0m", r)
}

#[inline(always)]
fn num(n: impl std::fmt::Display) -> String {
    format!("\x1b[33m{}\x1b[0m", n)
}

#[inline(always)]
fn addr(n: usize) -> String {
    format!("\x1b[35m0x{:04X}\x1b[0m", n)
}

#[inline(always)]
fn dim(s: &str) -> String {
    format!("\x1b[90m{}\x1b[0m", s)
}

impl AbyssVm {
    pub fn disassemble(&self) -> String {
        let mut out = String::with_capacity(self.program.len() * 64);

        for (ip, inst) in self.program.iter().enumerate() {
            let mnemonic = format!("{:?}", inst.op).to_uppercase();
            let pad_op = format!("{:<16}", mnemonic);
            let f_op = op(&pad_op);

            let ra = reg(inst.a);
            let rb = reg(inst.b);
            let rc = reg(inst.c);
            let c = dim(", ");
            let ob = dim("[");
            let cb = dim("]");
            let p = dim(" + ");

            let args = match inst.op {
                OpCode::Halt | OpCode::Ret => String::new(),

                OpCode::LoadConst => {
                    let idx = ((inst.b as usize) << 8) | (inst.c as usize);
                    let val = self.constants.get(idx).copied().unwrap_or(0);
                    format!("{}{}{}", ra, c, num(val))
                }

                OpCode::AddIC
                | OpCode::SubIC
                | OpCode::MulIC
                | OpCode::DivIC
                | OpCode::ModIC
                | OpCode::CmpEqIC
                | OpCode::CmpNeqIC
                | OpCode::CmpLtIC
                | OpCode::CmpLeIC
                | OpCode::CmpGtIC
                | OpCode::CmpGeIC => {
                    let val = self.constants.get(inst.c as usize).copied().unwrap_or(0);
                    format!("{}{}{}{}{}", ra, c, rb, c, num(val))
                }

                OpCode::AddFC
                | OpCode::SubFC
                | OpCode::MulFC
                | OpCode::DivFC
                | OpCode::CmpEqFC
                | OpCode::CmpNeqFC
                | OpCode::CmpLtFC
                | OpCode::CmpLeFC
                | OpCode::CmpGtFC
                | OpCode::CmpGeFC => {
                    let bits = self.constants.get(inst.c as usize).copied().unwrap_or(0);
                    let val = f64::from_bits(bits);
                    format!("{}{}{}{}{}", ra, c, rb, c, num(val))
                }

                OpCode::LoadGlobal => {
                    let idx = ((inst.b as usize) << 8) | (inst.c as usize);
                    format!("{}{}{}g{}{}", ra, c, ob, num(idx), cb)
                }

                OpCode::StoreGlobal => {
                    let idx = ((inst.b as usize) << 8) | (inst.c as usize);
                    format!("{}g{}{}{}{}", ob, num(idx), cb, c, ra)
                }

                OpCode::LoadPtr | OpCode::LoadPtr8 | OpCode::LoadPtr16 | OpCode::LoadPtr32 => {
                    format!("{}{}{}{}{}", ra, c, ob, rb, cb)
                }

                OpCode::StorePtr | OpCode::StorePtr8 | OpCode::StorePtr16 | OpCode::StorePtr32 => {
                    format!("{}{}{}{}{}", ob, ra, cb, c, rb)
                }

                OpCode::LoadPtrOffset
                | OpCode::LoadPtrOffset8
                | OpCode::LoadPtrOffset16
                | OpCode::LoadPtrOffset32 => format!("{}{}{}{}{}{}{}", ra, c, ob, rb, p, rc, cb),

                OpCode::StorePtrOffset
                | OpCode::StorePtrOffset8
                | OpCode::StorePtrOffset16
                | OpCode::StorePtrOffset32 => {
                    format!("{}{}{}{}{}{}{}", ob, ra, p, rc, cb, c, rb)
                }

                OpCode::JmpImm => {
                    let target =
                        ((inst.a as usize) << 16) | ((inst.b as usize) << 8) | (inst.c as usize);
                    addr(target)
                }

                OpCode::JmpZImm => {
                    let target = ((inst.b as usize) << 8) | (inst.c as usize);
                    format!("{}{}{}", ra, c, addr(target))
                }

                OpCode::Jmp => ra,

                OpCode::JmpIf => format!("{}{}{}", ra, c, rb),

                OpCode::Move
                | OpCode::Not
                | OpCode::Alloc
                | OpCode::RefReg
                | OpCode::CastI2F
                | OpCode::CastF2I
                | OpCode::CastI2B
                | OpCode::CastF2B => format!("{}{}{}", ra, c, rb),

                _ => format!("{}{}{}{}{}", ra, c, rb, c, rc),
            };

            out.push_str(&format!("  {}  {} {}{}\n", addr(ip), dim("|"), f_op, args));
        }

        out
    }
}
