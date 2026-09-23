#[derive(Debug, Clone, Copy, PartialEq, Default, Eq, Hash)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn empty() -> Self {
        Self { start: 0, end: 0 }
    }

    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    pub fn len(&self) -> u32 {
        self.end - self.start
    }
}
