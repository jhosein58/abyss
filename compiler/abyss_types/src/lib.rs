#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TyKind {
    #[default]
    Unknown,

    UntypedInt,
    UntypedFloat,
    Int,
    UInt,
    Float,
    Bool,
    Ptr,
    Type,
    Unit,
    Func,
    Never,
    Struct,
    Nominal,

    Error,
}

#[derive(Default)]
pub struct TyStore {
    pub kinds: Vec<TyKind>,
    pub payload: Vec<u32>,
    pub extra: Vec<u32>,
}

impl TyStore {
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.kinds.len()
    }

    #[inline(always)]
    pub fn reserve(&mut self, additional: usize) {
        self.kinds.reserve(additional);
        self.payload.reserve(additional);
        self.extra.reserve(additional);
    }

    #[inline(always)]
    pub fn push(&mut self, tykind: TyKind, payload: u32) -> usize {
        let idx = self.len();
        self.kinds.push(tykind);
        self.payload.push(payload);
        idx
    }
}
