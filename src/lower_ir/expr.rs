use crate::{
    base::{foreign_symbol::ForeignSymbolId, low_level::LowLevelId, symbol::Symbol, Number},
    env::StringLiteralId,
    soa::{NonEmptySlice, Slice},
};

use super::{layout::LoweredLayoutId, stmt::TagIdIntType};

#[derive(Clone, Debug, PartialEq)]
pub enum LoweredExpr {
    Str(StringLiteralId),
    Number(Number),

    // Functions
    Call(LoweredCall),

    Tag {
        tag_layout: UnionLayout<'a>,
        tag_id: TagIdIntType,
        arguments: Slice<Symbol>,
    },
    Struct(NonEmptySlice<Symbol>),
    NullPointer,

    StructAtIndex {
        index: u64,
        field_layouts: &'a [InLayout<'a>],
        structure: Symbol,
    },

    GetTagId {
        structure: Symbol,
        union_layout: UnionLayout<'a>,
    },

    UnionAtIndex {
        structure: Symbol,
        tag_id: TagIdIntType,
        union_layout: UnionLayout<'a>,
        index: u64,
    },
    GetElementPointer {
        structure: Symbol,
        union_layout: UnionLayout<'a>,
        indices: &'a [u64],
    },

    Array {
        elem_layout: InLayout<'a>,
        elems: &'a [ListLiteralElement<'a>],
    },
    EmptyArray,

    /// Returns a pointer to the given function.
    FunctionPointer {
        lambda_name: LambdaName<'a>,
    },

    Alloca {
        element_layout: InLayout<'a>,
        initializer: Option<Symbol>,
    },

    Reset {
        symbol: Symbol,
        update_mode: UpdateModeId,
    },

    // Just like Reset, but does not recursively decrement the children.
    // Used in reuse analysis to replace a decref with a resetRef to avoid decrementing when the dec ref didn't.
    ResetRef {
        symbol: Symbol,
        update_mode: UpdateModeId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoweredCallType {
    ByName {
        name: LambdaName<'a>,
        ret_layout: LoweredLayoutId,
        // TODO: figure out if we should be using Slice<Index> or Slice<Item> in general
        arg_layouts: Slice<LoweredLayoutId>,
        specialization_id: CallSpecId,
    },
    ByPointer {
        pointer: Symbol,
        ret_layout: LoweredLayoutId,
        arg_layouts: Slice<LoweredLayoutId>,
    },
    Foreign {
        foreign_symbol: ForeignSymbolId,
        ret_layout: LoweredLayoutId,
    },
    LowLevel {
        op: LowLevelId,
        update_mode: UpdateModeId,
    },
    HigherOrder(&'a HigherOrderLowLevel<'a>),
}

pub struct LoweredCall {
    // TODO: put `call_type`
    pub call_type: LoweredCallType,
    pub arguments: Slice<Symbol>,
}
