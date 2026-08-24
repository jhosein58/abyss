use crate::binding_power::BindingPower;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    _None = 0,
    ConstDef = 5,    // ::
    VarDef = 15,     // :, :=
    Assignment = 10, // =, +=, -=, ...
    _Range = 20,     // ->
    LogicOr = 30,    // or
    LogicAnd = 40,   // and
    _Equality = 50,  //  ==, !=
    Comparison = 60, //  <, >, <=, >=, Is (Type check)
    _BitOr = 70,     // |
    _BitXor = 80,    // ^
    _BitAnd = 90,    // &
    _Shift = 100,    // <<, >>
    Term = 110,      // +, -
    Factor = 120,    //  *, /, %
    _Cast = 130,     // as
    _Unary = 140,    // -x, !x, ~x, *x, &x
    Call = 150,      //  (), []
    _Member = 160,   // .
    _Primary = 170,
}

impl Precedence {
    #[inline]
    pub fn value(self) -> u8 {
        self as u8
    }

    #[inline]
    pub fn left_assoc(self) -> BindingPower {
        let val = self.value();

        BindingPower {
            left: val,
            right: val + 1,
        }
    }

    #[inline]
    pub fn right_assoc(self) -> BindingPower {
        let val = self.value();

        BindingPower {
            left: val,
            right: if val > 0 { val - 1 } else { 0 },
        }
    }
}
