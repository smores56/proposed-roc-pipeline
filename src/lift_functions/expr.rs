use core::mem::MaybeUninit;

use crate::base::foreign_symbol::ForeignSymbolId;
use crate::base::low_level::LowLevelId;
use crate::base::symbol::{IdentId, Symbol};
use crate::base::Number;
use crate::env::{FieldNameId, StringLiteralId};
use crate::soa::{Index, NonEmptySlice, Slice, Slice2};

use super::pattern::FunctionLiftPatternId;
use super::type_::FunctionLiftTypeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FunctionLiftExprId {
    index: Index<FunctionLiftExpr>,
}

#[derive(Debug, Clone, Debug, PartialEq)]
pub enum FunctionLiftExpr {
    Str(StringLiteralId),
    Number(Number),
    List {
        elem_type: FunctionLiftTypeId,
        elems: Slice<FunctionLiftExprId>,
    },
    Lookup(IdentId, FunctionLiftTypeId),

    /// This is *only* for calling functions, not for tag application.
    /// The Tag variant contains any applied values inside it.
    Call {
        fn_type: FunctionLiftTypeId,
        fn_expr: FunctionLiftExprId,
        args: Slice2<FunctionLiftTypeId, FunctionLiftExprId>,
    },
    RunLowLevel {
        op: LowLevelId,
        args: Slice<(FunctionLiftTypeId, FunctionLiftExprId)>,
        ret_type: FunctionLiftTypeId,
    },
    ForeignCall {
        foreign_symbol: ForeignSymbolId,
        args: Slice<(FunctionLiftTypeId, FunctionLiftExprId)>,
        ret_type: FunctionLiftTypeId,
    },

    FunctionPack {
        fn_symbol: Symbol,
        captures: Slice<(FunctionLiftTypeId, FunctionLiftPatternId)>,
    },

    Unit,

    Struct(NonEmptySlice<FunctionLiftExpr>),

    /// Look up exactly one field on a record, tuple, or tag payload.
    /// At this point we've already unified those concepts and have
    /// converted (for example) record field names to indices, and have
    /// also dropped all fields that have no runtime representation (e.g. empty records).
    ///
    /// In a later compilation phase, these indices will be re-sorted
    /// by alignment and converted to byte offsets, but we in this
    /// phase we aren't concerned with alignment or sizes, just indices.
    StructAccess {
        record_expr: FunctionLiftExprId,
        record_type: FunctionLiftTypeId,
        field_type: FunctionLiftTypeId,
        field_id: FieldNameId,
    },

    RecordUpdate {
        record_type: FunctionLiftTypeId,
        record_name: IdentId,
        updates: Slice2<FieldNameId, FunctionLiftExprId>,
    },

    Tag {
        discriminant: u16,
        tag_union_type: FunctionLiftTypeId,
        args: Slice2<FunctionLiftTypeId, FunctionLiftExprId>,
    },

    When {
        /// The value being matched on
        value: FunctionLiftExprId,
        /// The type of the value being matched on
        value_type: FunctionLiftTypeId,
        /// The return type of all branches and thus the whole when expression
        branch_type: FunctionLiftTypeId,
        /// The branches of the when expression
        branches: NonEmptySlice<WhenBranch>,
    },

    CompilerBug(FunctionLiftProblem),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WhenBranch {
    /// The pattern(s) to match the value against
    pub patterns: NonEmptySlice<FunctionLiftPatternId>,
    /// A boolean expression that must be true for this branch to be taken
    pub guard: Option<FunctionLiftExprId>,
    /// The expression to produce if the pattern matches
    pub value: FunctionLiftExprId,
}

#[derive(Debug, Default)]
pub struct WhenBranches {
    branches: Vec<MaybeUninit<WhenBranch>>,
}
