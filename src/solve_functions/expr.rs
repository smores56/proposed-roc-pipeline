use core::mem::MaybeUninit;

use crate::base::foreign_symbol::ForeignSymbolId;
use crate::base::low_level::LowLevel;
use crate::base::problem::FunctionSolveProblem;
use crate::base::symbol::{IdentId, Symbol};
use crate::base::Number;
use crate::env::{FieldNameId, StringLiteralId};
use crate::soa::{Index, NonEmptySlice, Slice, Slice2};

use super::pattern::FunctionSolvePatternId;
use super::type_::FunctionSolveTypeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FunctionSolveExprId {
    index: Index<FunctionSolveExpr>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FunctionSolveExpr {
    Str(StringLiteralId),
    Number(Number),
    List {
        elem_type: FunctionSolveTypeId,
        elems: Slice<FunctionSolveExprId>,
    },
    Lookup(IdentId, FunctionSolveTypeId),

    /// This is *only* for calling functions, not for tag application.
    /// The Tag variant contains any applied values inside it.
    Call {
        fn_type: FunctionSolveTypeId,
        fn_expr: FunctionSolveExprId,
        args: Slice2<FunctionSolveTypeId, FunctionSolveExprId>,
    },
    RunLowLevel {
        op: LowLevel,
        args: Slice<(FunctionSolveTypeId, FunctionSolveExprId)>,
        ret_type: FunctionSolveTypeId,
    },
    ForeignCall {
        foreign_symbol: ForeignSymbolId,
        args: Slice<(FunctionSolveTypeId, FunctionSolveExprId)>,
        ret_type: FunctionSolveTypeId,
    },

    FunctionPack {
        fn_symbol: Symbol,
        captures: Slice<(FunctionSolveTypeId, FunctionSolvePatternId)>,
    },

    Unit,

    Struct(NonEmptySlice<FunctionSolveExpr>),

    /// Look up exactly one field on a record, tuple, or tag payload.
    /// At this point we've already unified those concepts and have
    /// converted (for example) record field names to indices, and have
    /// also dropped all fields that have no runtime representation (e.g. empty records).
    ///
    /// In a later compilation phase, these indices will be re-sorted
    /// by alignment and converted to byte offsets, but we in this
    /// phase we aren't concerned with alignment or sizes, just indices.
    StructAccess {
        record_expr: FunctionSolveExprId,
        record_type: FunctionSolveTypeId,
        field_type: FunctionSolveTypeId,
        field_id: FieldNameId,
    },

    RecordUpdate {
        record_type: FunctionSolveTypeId,
        record_name: IdentId,
        updates: Slice2<FieldNameId, FunctionSolveExprId>,
    },

    Tag {
        discriminant: u16,
        tag_union_type: FunctionSolveTypeId,
        args: Slice2<FunctionSolveTypeId, FunctionSolveExprId>,
    },

    When {
        /// The value being matched on
        value: FunctionSolveExprId,
        /// The type of the value being matched on
        value_type: FunctionSolveTypeId,
        /// The return type of all branches and thus the whole when expression
        branch_type: FunctionSolveTypeId,
        /// The branches of the when expression
        branches: NonEmptySlice<WhenBranch>,
    },

    CompilerBug(FunctionSolveProblem),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WhenBranch {
    /// The pattern(s) to match the value against
    pub patterns: NonEmptySlice<FunctionSolvePatternId>,
    /// A boolean expression that must be true for this branch to be taken
    pub guard: Option<FunctionSolveExprId>,
    /// The expression to produce if the pattern matches
    pub value: FunctionSolveExprId,
}

#[derive(Debug, Default)]
pub struct WhenBranches {
    branches: Vec<MaybeUninit<WhenBranch>>,
}
