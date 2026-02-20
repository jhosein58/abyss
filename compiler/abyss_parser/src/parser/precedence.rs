#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None = 0,

    Assignment = 10, // =, +=, -=, ...
    Range = 20,      // ->
    LogicOr = 30,    // or
    LogicAnd = 40,   // and
    BitOr = 50,      // |
    BitXor = 60,     // ^
    BitAnd = 70,     // &
    Equality = 80,   //  ==, !=
    Comparison = 90, //  <, >, <=, >=, Is (Type check)
    Shift = 100,     // <<, >>
    Term = 110,      // +, -
    Factor = 120,    //  *, /, %
    Cast = 130,      // as
    Unary = 140,     // -x, !x, ~x, *x, &x
    Call = 150,      //  (), [], ., method calls

    Primary = 160,
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
