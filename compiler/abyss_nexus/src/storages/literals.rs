use crate::{arena::DirectArena, arena_id};

arena_id!(IntId);
arena_id!(FloatId);

#[derive(Default)]
pub struct LiteralStorage {
    ints: DirectArena<IntId, i64>,
    floats: DirectArena<FloatId, f64>,
}

impl LiteralStorage {
    #[inline(always)]
    pub fn intern_int(&mut self, val: i64) -> IntId {
        self.ints.alloc(val)
    }

    #[inline(always)]
    pub fn get_int(&self, id: IntId) -> i64 {
        self.ints.get(id)
    }

    #[inline(always)]
    pub fn intern_float(&mut self, val: f64) -> FloatId {
        self.floats.alloc(val)
    }

    #[inline(always)]
    pub fn get_float(&self, id: FloatId) -> f64 {
        self.floats.get(id)
    }
}
