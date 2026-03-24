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

    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    ShrI,
    ShrU,
    BitNot,

    // Logical
    Not,

    // Memory & Pointers
    Alloc,          // a = alloc(b)
    LoadPtr,        // a = *b
    StorePtr,       // *a = b
    LoadPtrOffset,  // a = *(b + c * 8)
    StorePtrOffset, // *(a + c * 8) = b
    LoadGlobal,     // r[a] = globals[b << 8 | c]
    StoreGlobal,    // globals[b << 8 | c] = r[a]
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
