use std::marker::PhantomData;

pub trait ArenaId: Copy + Clone {
    fn new(v: u32) -> Self;
    fn value(self) -> u32;
}

#[macro_export]
macro_rules! arena_id {
    ($name:ident) => {
        #[repr(transparent)]
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub u32);

        impl crate::arena::ArenaId for $name {
            #[inline(always)]
            fn new(v: u32) -> Self {
                Self(v)
            }

            #[inline(always)]
            fn value(self) -> u32 {
                self.0
            }
        }

        impl From<u32> for $name {
            #[inline(always)]
            fn from(value: u32) -> Self {
                Self(value)
            }
        }

        impl From<$name> for u32 {
            #[inline(always)]
            fn from(id: $name) -> Self {
                id.0
            }
        }
    };
}

pub type DirectArena<ID, T> = Arena<ID, ID, T>;

#[derive(Default, Debug, Clone)]
pub struct Arena<I: ArenaId, O: ArenaId, T> {
    data: Vec<T>,
    _marker: std::marker::PhantomData<(I, O)>,
}

impl<I: ArenaId, O: ArenaId, T> Arena<I, O, T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn alloc(&mut self, item: T) -> O {
        let index = self.data.len() as u32;
        self.data.push(item);
        O::new(index)
    }

    #[inline]
    pub fn get(&self, id: I) -> &T {
        &self.data[id.value() as usize]
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<I: ArenaId, O: ArenaId, T: Default + Clone + Copy> Arena<I, O, T> {
    #[inline]
    pub fn resize(&mut self, new_len: usize) {
        if new_len > self.data.len() {
            self.data.resize(new_len, T::default());
        }
    }
}

impl<I: ArenaId, O: ArenaId, T: Copy> Arena<I, O, T> {
    #[inline]
    pub fn get_copy(&self, id: I) -> T {
        self.data[id.value() as usize]
    }
}
