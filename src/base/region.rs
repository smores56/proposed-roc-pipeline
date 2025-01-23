use core::fmt;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default)]
pub struct Region {
    start: Position,
    end: Position,
}

impl fmt::Debug for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.start == Position::zero() && self.end == Position::zero() {
            // In tests, it's super common to set all Located values to 0.
            // Also in tests, we don't want to bother printing the locations
            // because it makes failed assertions much harder to read.
            write!(f, "…")
        } else {
            write!(f, "@{}-{}", self.start.offset, self.end.offset,)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Position {
    pub offset: u32,
}

impl Position {
    pub const fn zero() -> Self {
        Self { offset: 0 }
    }
}
