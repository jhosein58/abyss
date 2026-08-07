use crate::nexus::{HirId, TokenId};

macro_rules! define_range {
    ($name:ident, $id_type:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct $name {
            pub start: $id_type,
            pub end: $id_type,
        }

        impl $name {
            #[inline]
            pub const fn new(start: $id_type, end: $id_type) -> Self {
                Self { start, end }
            }

            #[inline]
            pub fn len(&self) -> usize {
                if self.end.0 >= self.start.0 {
                    (self.end.0 - self.start.0 + 1) as usize
                } else {
                    0
                }
            }

            #[inline]
            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }
        }
    };
}

define_range!(TokenRange, TokenId);
define_range!(HirRange, HirId);
