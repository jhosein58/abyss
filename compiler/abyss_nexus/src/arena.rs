use std::{marker::PhantomData, ops::Index};

pub trait ArenaId: Copy + Clone {
    fn new(v: u32) -> Self;
    fn value(self) -> u32;
}

#[macro_export]
macro_rules! arena_id {
    ($name:ident) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

        impl Default for $name {
            fn default() -> Self {
                Self(u32::MAX)
            }
        }
    };
}

pub type Arena<I, T> = ArenaCore<I, T, true>;
pub type SideTable<K, V> = ArenaCore<K, V, false>;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ArenaCore<I: ArenaId, T, const ALLOC: bool> {
    pub data: Vec<T>,
    _marker: PhantomData<I>,
}

impl<I: ArenaId, T, const ALLOC: bool> ArenaCore<I, T, ALLOC> {
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
    pub fn get(&self, id: I) -> &T {
        &self.data[id.value() as usize]
    }

    #[inline(always)]
    pub fn set(&mut self, id: I, value: T) {
        let index = id.value() as usize;
        self.data[index] = value;
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<I: ArenaId, T, const ALLOC: bool> Index<I> for ArenaCore<I, T, ALLOC> {
    type Output = T;
    #[inline(always)]
    fn index(&self, id: I) -> &Self::Output {
        self.get(id)
    }
}

impl<I: ArenaId, T, const ALLOC: bool> Default for ArenaCore<I, T, ALLOC> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: ArenaId, T: Copy, const ALLOC: bool> ArenaCore<I, T, ALLOC> {
    #[inline]
    pub fn get_copy(&self, id: I) -> T {
        self.data[id.value() as usize]
    }
}

impl<I: ArenaId, T> ArenaCore<I, T, true> {
    #[inline]
    pub fn alloc(&mut self, item: T) -> I {
        let index = self.data.len() as u32;
        self.data.push(item);
        I::new(index)
    }
}

impl<I: ArenaId, T: Default + Clone> ArenaCore<I, T, false> {
    #[inline]
    pub fn grow_to(&mut self, new_len: usize) {
        if new_len > self.data.len() {
            self.data.resize(new_len, T::default());
        }
    }
}
