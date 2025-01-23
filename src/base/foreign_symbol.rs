use crate::soa::Index;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignSymbol;

#[derive(Debug, Default)]
pub struct ForeignSymbols {
    symbols: Vec<ForeignSymbol>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ForeignSymbolId {
    index: Index<ForeignSymbol>,
}
