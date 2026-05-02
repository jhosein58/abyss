#[repr(u8)]
pub enum HirExprKind {
    // Literals
    LitInt,
    LitFloat,
    LitBool,
    LitStr,
    LitCstr,
    LitChar,

    Ident,

    // Binary Operations
    BinaryAssign,       // =
    BinaryAssignAdd,    // +=
    BinaryAssignSub,    // -=
    BinaryAssignMul,    // *=
    BinaryAssignDiv,    // /=
    BinaryAssignMod,    // %=
    BinaryAssignBitAnd, // &=
    BinaryAssignBitOr,  // |=
    BinaryAssignBitXor, // ^=
    BinaryAssignShl,    // <<=
    BinaryAssignShr,    // >>=
    BinaryAdd,          // +
    BinarySub,          // -
    BinaryMul,          // *
    BinaryDiv,          // /
    BinaryMod,          // %
    BinaryEq,           // ==
    BinaryNeq,          // !=
    BinaryLt,           // <
    BinaryGt,           // >
    BinaryLte,          // <=
    BinaryGte,          // >=
    BinaryAnd,          // and
    BinaryOr,           // or
    BinaryBitAnd,       // &
    BinaryPipe,         // |
    BinaryBitXor,       // ^
    BinaryShl,          // <<
    BinaryShr,          // >>
    BinaryCollon,       // :

    // Unary Operations
    UnaryNeg,    // -x
    UnaryNot,    // not x
    UnaryBitNot, // ~x
    UnaryDeref,  // *x
    UnaryAddrOf, // &x
}

pub struct HirProgram {
    pub kinds: Vec<HirExprKind>,
    pub lhs: Vec<u32>,
    pub rhs: Vec<u32>,
}
