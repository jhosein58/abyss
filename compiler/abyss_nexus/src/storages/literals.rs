#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntId(pub u32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FloatId(pub u32);

#[derive(Default)]
pub struct LiteralStorage {
    ints: Vec<i64>,
    floats: Vec<f64>,
}

impl LiteralStorage {
    pub fn intern_int(&mut self, val: i64) -> IntId {
        let id = self.ints.len() as u32;
        self.ints.push(val);
        IntId(id)
    }

    pub fn get_int(&self, id: IntId) -> i64 {
        self.ints[id.0 as usize]
    }

    pub fn intern_float(&mut self, val: f64) -> FloatId {
        let id = self.floats.len() as u32;
        self.floats.push(val);
        FloatId(id)
    }

    pub fn get_float(&self, id: FloatId) -> f64 {
        self.floats[id.0 as usize]
    }
}
