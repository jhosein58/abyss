use crate::vm::{
    opcode::{Instruction, OpCode::*},
    ops::{
        basic::{load_const, move_reg, not},
        bitwise::{bit_and, bit_not, bit_or, bit_xor, shl, shr_i, shr_u},
        cast::{cast_f2b, cast_f2i, cast_i2b, cast_i2f},
        control::{call, call_native, jmp, jmp_if, jmp_imm, jmp_z_imm},
        math_float::{
            add_f, add_fc, cmp_eq_f, cmp_eq_fc, cmp_ge_f, cmp_ge_fc, cmp_gt_f, cmp_gt_fc, cmp_le_f,
            cmp_le_fc, cmp_lt_f, cmp_lt_fc, cmp_neq_f, cmp_neq_fc, div_f, div_fc, mul_f, mul_fc,
            sub_f, sub_fc,
        },
        math_int::{
            add_i, add_ic, cmp_eq_i, cmp_eq_ic, cmp_ge_i, cmp_ge_ic, cmp_gt_i, cmp_gt_ic, cmp_le_i,
            cmp_le_ic, cmp_lt_i, cmp_lt_ic, cmp_neq_i, cmp_neq_ic, div_i, div_ic, mod_i, mod_ic,
            mul_i, mul_ic, sub_i, sub_ic,
        },
        memory::{
            alloc, load_ptr, load_ptr_offset, mem_copy, ref_reg, store_ptr, store_ptr_offset,
        },
    },
    types::{CallFrame, NativeFunction, RegisteredNative},
};

pub struct AbyssVm {
    // Stack & Execution
    pub registers: Vec<u64>,
    pub bp: usize,
    pub call_stack: Vec<CallFrame>,
    pub ip: usize,

    // Data
    pub program: Vec<Instruction>,
    pub constants: Vec<u64>,

    pub heap: Vec<u8>,
    pub globals: Vec<u64>,
    pub free_blocks: Vec<(usize, usize)>,
    pub native_funcs: Vec<RegisteredNative>,

    pub out: String,
}

impl AbyssVm {
    pub fn new(program: Vec<Instruction>, constants: Vec<u64>) -> Self {
        let mut s = Self::new_empty();
        s.program = program;
        s.constants = constants;
        s
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
            globals: Vec::new(),
            free_blocks: Vec::new(),
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

    pub fn init_globals(&mut self, count: usize) {
        self.globals = vec![0; count];
    }

    pub fn run(&mut self) -> Option<u64> {
        let mut final_result = None;

        let mut ip = self.ip;
        let mut bp = self.bp;

        let program_ptr = self.program.as_ptr();
        let registers_ptr = self.registers.as_mut_ptr();
        let constants_ptr = self.constants.as_ptr();
        let globals_ptr = self.globals.as_mut_ptr();

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

        loop {
            let inst = unsafe { *program_ptr.add(ip) };
            ip += 1;

            match inst.op {
                Halt => break,

                // Basic Operations
                LoadConst => load_const(&inst, bp, registers_ptr, constants_ptr),
                Move => move_reg(&inst, bp, registers_ptr),
                Not => not(&inst, bp, registers_ptr),

                // Casting Operations
                CastI2F => cast_i2f(&inst, bp, registers_ptr),
                CastF2I => cast_f2i(&inst, bp, registers_ptr),
                CastI2B => cast_i2b(&inst, bp, registers_ptr),
                CastF2B => cast_f2b(&inst, bp, registers_ptr),

                // Integer math
                AddI => add_i(&inst, bp, registers_ptr),
                SubI => sub_i(&inst, bp, registers_ptr),
                MulI => mul_i(&inst, bp, registers_ptr),
                DivI => div_i(&inst, bp, registers_ptr),
                ModI => mod_i(&inst, bp, registers_ptr),

                // Integer Math with Constant
                AddIC => add_ic(&inst, bp, registers_ptr, constants_ptr),
                SubIC => sub_ic(&inst, bp, registers_ptr, constants_ptr),
                MulIC => mul_ic(&inst, bp, registers_ptr, constants_ptr),
                DivIC => div_ic(&inst, bp, registers_ptr, constants_ptr),
                ModIC => mod_ic(&inst, bp, registers_ptr, constants_ptr),

                // Integer Comparisons
                CmpEqI => cmp_eq_i(&inst, bp, registers_ptr),
                CmpNeqI => cmp_neq_i(&inst, bp, registers_ptr),
                CmpLtI => cmp_lt_i(&inst, bp, registers_ptr),
                CmpLeI => cmp_le_i(&inst, bp, registers_ptr),
                CmpGtI => cmp_gt_i(&inst, bp, registers_ptr),
                CmpGeI => cmp_ge_i(&inst, bp, registers_ptr),

                // Integer Comparisons with Constant
                CmpEqIC => cmp_eq_ic(&inst, bp, registers_ptr, constants_ptr),
                CmpNeqIC => cmp_neq_ic(&inst, bp, registers_ptr, constants_ptr),
                CmpLtIC => cmp_lt_ic(&inst, bp, registers_ptr, constants_ptr),
                CmpLeIC => cmp_le_ic(&inst, bp, registers_ptr, constants_ptr),
                CmpGtIC => cmp_gt_ic(&inst, bp, registers_ptr, constants_ptr),
                CmpGeIC => cmp_ge_ic(&inst, bp, registers_ptr, constants_ptr),

                // Float Math
                AddF => add_f(&inst, bp, registers_ptr),
                SubF => sub_f(&inst, bp, registers_ptr),
                MulF => mul_f(&inst, bp, registers_ptr),
                DivF => div_f(&inst, bp, registers_ptr),

                // Float Math with Constant
                AddFC => add_fc(&inst, bp, registers_ptr, constants_ptr),
                SubFC => sub_fc(&inst, bp, registers_ptr, constants_ptr),
                MulFC => mul_fc(&inst, bp, registers_ptr, constants_ptr),
                DivFC => div_fc(&inst, bp, registers_ptr, constants_ptr),

                // Float Comparisons
                CmpEqF => cmp_eq_f(&inst, bp, registers_ptr),
                CmpNeqF => cmp_neq_f(&inst, bp, registers_ptr),
                CmpLtF => cmp_lt_f(&inst, bp, registers_ptr),
                CmpLeF => cmp_le_f(&inst, bp, registers_ptr),
                CmpGtF => cmp_gt_f(&inst, bp, registers_ptr),
                CmpGeF => cmp_ge_f(&inst, bp, registers_ptr),

                // Float Comparisons with Constant
                CmpEqFC => cmp_eq_fc(&inst, bp, registers_ptr, constants_ptr),
                CmpNeqFC => cmp_neq_fc(&inst, bp, registers_ptr, constants_ptr),
                CmpLtFC => cmp_lt_fc(&inst, bp, registers_ptr, constants_ptr),
                CmpLeFC => cmp_le_fc(&inst, bp, registers_ptr, constants_ptr),
                CmpGtFC => cmp_gt_fc(&inst, bp, registers_ptr, constants_ptr),
                CmpGeFC => cmp_ge_fc(&inst, bp, registers_ptr, constants_ptr),

                // Bitwise Operations
                BitAnd => bit_and(&inst, bp, registers_ptr),
                BitOr => bit_or(&inst, bp, registers_ptr),
                BitXor => bit_xor(&inst, bp, registers_ptr),
                Shl => shl(&inst, bp, registers_ptr),
                ShrU => shr_u(&inst, bp, registers_ptr),
                ShrI => shr_i(&inst, bp, registers_ptr),
                BitNot => bit_not(&inst, bp, registers_ptr),

                // Control Flow
                Call => call(&inst, &mut ip, &mut bp, registers_ptr, &mut self.call_stack),
                CallNative => call_native(&inst, self, &mut ip, bp, registers_ptr),
                Jmp => jmp(&inst, bp, &mut ip, registers_ptr),
                JmpIf => jmp_if(&inst, bp, &mut ip, registers_ptr),
                JmpImm => jmp_imm(&inst, &mut ip),
                JmpZImm => jmp_z_imm(&inst, bp, &mut ip, registers_ptr),

                Ret => {
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

                // Memory & Pointers
                Alloc => alloc(&inst, bp, registers_ptr, &mut self.heap),
                RefReg => ref_reg(&inst, bp, registers_ptr),
                LoadPtr => load_ptr(&inst, bp, registers_ptr, &self.heap),
                StorePtr => store_ptr(&inst, bp, registers_ptr, &mut self.heap),
                LoadPtrOffset => load_ptr_offset(&inst, bp, registers_ptr, &self.heap),
                StorePtrOffset => store_ptr_offset(&inst, bp, registers_ptr, &mut self.heap),

                LoadGlobal => {
                    let global_idx = ((inst.b as usize) << 8) | (inst.c as usize);
                    let val = unsafe { *globals_ptr.add(global_idx) };
                    set_reg!(inst.a, val);
                }
                StoreGlobal => {
                    let global_idx = ((inst.b as usize) << 8) | (inst.c as usize);
                    let val = get_reg!(inst.a);
                    unsafe { *globals_ptr.add(global_idx) = val };
                }

                MemCopy => mem_copy(&inst, bp, registers_ptr, &mut self.heap),
            }
        }

        self.ip = ip;
        self.bp = bp;

        final_result
    }
}
