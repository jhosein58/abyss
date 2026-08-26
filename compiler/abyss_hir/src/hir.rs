#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirExprKind {
    #[default]
    Empty,

    // Literals
    LitInt,
    LitFloat,
    LitBoolTrue,
    LitBoolFalse,
    LitStr,
    LitCstr,
    LitChar,
    Ident,

    // Binary
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
    BinaryEqEq,         // ==
    BinaryNeq,          // !=
    BinaryLt,           // <
    BinaryGt,           // >
    BinaryLtEq,         // <=
    BinaryGtEq,         // >=
    BinaryAnd,          // and
    BinaryOr,           // or
    BinaryBitAnd,       // &
    BinaryPipe,         // |
    BinaryBitXor,       // ^
    BinaryShl,          // <<
    BinaryShr,          // >>
    BinaryCollon,       // :

    // Unary
    UnaryNeg,    // -x
    UnaryNot,    // not x
    UnaryBitNot, // ~x
    UnaryDeref,  // *x
    UnaryAddrOf, // &x

    Mod,
    Use,
    Arg,
    Function, // (args_lhs) ret_type_rhs { block_extra }
    Call,     // expr(expr, expr, ..)
    Binding,  // ident :: expr
    Var,      // pattern := expr, ident: type = expr
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
    Index,
    Cast,
    Is,
    Struct,     // lhs: field names, rhs: field types
    StructInit, // lhs: fileds, rhs: values
    Member,
    SizeOf,
    Match,
    Then,
    TypeOf,
    Refinement,
    Attributed,
    Comptime,
    Wildcard,

    Error,

    // markers
    MarkerFnStart, // FIXME: i need to find a better way :/
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

    #[inline]
    pub fn reserve(&mut self, capacity: usize) {
        self.kinds.reserve(capacity);
        self.lhs.reserve(capacity);
        self.rhs.reserve(capacity);
        self.extra.reserve(capacity);
    }
}
