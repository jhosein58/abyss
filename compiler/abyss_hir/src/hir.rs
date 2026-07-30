#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirExprKind {
    // Literals
    LitInt,
    LitFloat,
    LitBool,
    LitStr,
    LitCstr,
    LitChar,
    Ident,

    // Binary
    BinaryAssign,
    BinaryAssignAdd,
    BinaryAssignSub,
    BinaryAssignMul,
    BinaryAssignDiv,
    BinaryAssignMod,
    BinaryAssignBitAnd,
    BinaryAssignBitOr,
    BinaryAssignBitXor,
    BinaryAssignShl,
    BinaryAssignShr,
    BinaryAdd,
    BinarySub,
    BinaryMul,
    BinaryDiv,
    BinaryMod,
    BinaryEq,
    BinaryNeq,
    BinaryLt,
    BinaryGt,
    BinaryLte,
    BinaryGte,
    BinaryAnd,
    BinaryOr,
    BinaryBitAnd,
    BinaryPipe,
    BinaryBitXor,
    BinaryShl,
    BinaryShr,
    BinaryCollon,
    BinaryConstDef,

    // Unary
    UnaryNeg,
    UnaryNot,
    UnaryBitNot,
    UnaryDeref,
    UnaryAddrOf,

    Mod,
    Use,
    Sequence,
    Signature,
    Def,
    Ret,
    Out,
    Continue,
    Block,
    If,
    For,
    Range,
    While,
    Forever,
    Defer,
    Call,
    Index,
    Cast,
    Is,
    Member,
    SizeOf,
    Match,
    Then,
    TypeOf,
    Refinement,
    Attributed,
    Comptime,
    Wildcard,
}

#[derive(Default)]
pub struct HirTable {
    pub root: u32,
    pub kinds: Vec<HirExprKind>,
    pub lhs: Vec<u32>,
    pub rhs: Vec<u32>,
    pub extra: Vec<u32>,
}

impl HirTable {
    #[inline]
    pub fn len(&self) -> usize {
        self.kinds.len()
    }
}
