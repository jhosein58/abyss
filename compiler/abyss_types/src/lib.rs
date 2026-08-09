#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeKind {
    #[default]
    Unknown,

    Int,
    Uint,
    Float,
    Bool,
}

#[derive(Default)]
pub struct TypeStore {
    pub kinds: Vec<TypeKind>,
    pub payload: Vec<u32>,
    pub extra: Vec<u32>,
}

impl TypeStore {
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
}
