use crate::soa::Index;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LowLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LowLevelId(Index<LowLevel>);

#[derive(Debug, Clone, PartialEq)]
pub struct LowLevels {
    items: Vec<LowLevel>,
}
