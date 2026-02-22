#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None = 0,
    ConstDef = 5, // ::
    AssignmentRhs = 9,
    Assignment = 10, // =, +=, -=, ...
    KeyValue = 15,   // :
    Range = 20,      // ->
    LogicOr = 30,    // or
    LogicAnd = 40,   // and
    Equality = 50,   //  ==, !=
    Comparison = 60, //  <, >, <=, >=, Is (Type check)
    BitOr = 70,      // |
    BitXor = 80,     // ^
    BitAnd = 90,     // &
    Shift = 100,     // <<, >>
    Term = 110,      // +, -
    Factor = 120,    //  *, /, %
    Cast = 130,      // as
    Unary = 140,     // -x, !x, ~x, *x, &x
    Call = 150,      //  (), []
    Member = 160,    // .
    Primary = 170,
}

impl Precedence {
    pub fn value(&self) -> u8 {
        *self as u8
    }

    pub fn lower(&self) -> u8 {
        let val = *self as u8;
        if val > 0 { val - 1 } else { 0 }
    }
}
