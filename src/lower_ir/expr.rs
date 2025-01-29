use crate::{
    base::{foreign_symbol::ForeignSymbolId, symbol::Symbol, LowLevel, NumberLiteral},
    env::StringLiteralId,
    soa::{Index, NonEmptySlice, Slice},
};

use super::layout::{LowerLayoutId, TagIdIntType, UnionLayout};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LowerExprId(pub(crate) Index<LowerExpr>);

#[derive(Debug)]
pub enum LowerExpr {
    Str(StringLiteralId),
    Number(NumberLiteral),

    // Functions
    Call(LowerCall),

    Tag {
        // TODO: should this be an index instead?
        tag_layout: UnionLayout,
        tag_id: TagIdIntType,
        arguments: Slice<Symbol>,
    },
    Struct(NonEmptySlice<Symbol>),
    NullPointer,

    StructAtIndex {
        index: u64,
        field_layouts: Slice<LowerLayoutId>,
        structure: Symbol,
    },

    GetTagId {
        structure: Symbol,
        union_layout: UnionLayout,
    },

    UnionAtIndex {
        structure: Symbol,
        tag_id: TagIdIntType,
        union_layout: UnionLayout,
        index: u64,
    },
    GetElementPointer {
        structure: Symbol,
        union_layout: UnionLayout,
        indices: Slice<u64>,
    },

    Array {
        elem_layout: LowerLayoutId,
        elems: Slice<ListLiteralElem>,
    },
    EmptyArray,

    /// Returns a pointer to the given function.
    FunctionPointer {
        symbol: Symbol,
    },

    Alloca {
        element_layout: LowerLayoutId,
        initializer: Option<Symbol>,
    },

    Reset {
        symbol: Symbol,
    },

    // Just like Reset, but does not recursively decrement the children.
    // Used in reuse analysis to replace a decref with a resetRef to avoid decrementing when the dec ref didn't.
    ResetRef {
        symbol: Symbol,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ListLiteralElem {
    Str(StringLiteralId),
    Number(NumberLiteral),
    Symbol(Symbol),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LowerCallType {
    ByName {
        symbol: Symbol,
        ret_layout: LowerLayoutId,
        arg_layouts: Slice<LowerLayoutId>,
    },
    ByPointer {
        pointer: Symbol,
        ret_layout: LowerLayoutId,
        arg_layouts: Slice<LowerLayoutId>,
    },
    Foreign {
        foreign_symbol: ForeignSymbolId,
        ret_layout: LowerLayoutId,
    },
    LowLevel {
        op: LowLevel,
    },
    // TODO: presumably these should be removed in an earlier stage
    // HigherOrder(&'a HigherOrderLowLevel<'a>),
}

#[derive(Debug)]
pub struct LowerCall {
    // TODO: put `call_type`
    pub call_type: LowerCallType,
    pub arguments: Slice<Symbol>,
}
