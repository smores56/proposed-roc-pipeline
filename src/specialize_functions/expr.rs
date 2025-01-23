use core::mem::MaybeUninit;

use crate::base::foreign_symbol::ForeignSymbolId;
use crate::base::low_level::LowLevelId;
use crate::base::problem::FunctionSpecializeProblem;
use crate::base::symbol::{IdentId, Symbol};
use crate::base::Number;
use crate::env::{FieldNameId, StringLiteralId};
use crate::soa::{Index, NonEmptySlice, Slice, Slice2};

use super::pattern::FunctionSpecializePatternId;
use super::type_::FunctionSpecializeTypeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FunctionSpecializeExprId {
    index: Index<FunctionSpecializeExpr>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FunctionSpecializeExpr {
    Str(StringLiteralId),
    Number(Number),
    List {
        elem_type: FunctionSpecializeTypeId,
        elems: Slice<FunctionSpecializeExprId>,
    },
    Lookup(IdentId, FunctionSpecializeTypeId),

    /// This is *only* for calling functions, not for tag application.
    /// The Tag variant contains any applied values inside it.
    Call {
        fn_type: FunctionSpecializeTypeId,
        fn_symbol: Symbol,
        args: Slice2<FunctionSpecializeTypeId, FunctionSpecializeExprId>,
    },
    RunLowLevel {
        op: LowLevelId,
        args: Slice<(FunctionSpecializeTypeId, FunctionSpecializeExprId)>,
        ret_type: FunctionSpecializeTypeId,
    },
    ForeignCall {
        foreign_symbol: ForeignSymbolId,
        args: Slice<(FunctionSpecializeTypeId, FunctionSpecializeExprId)>,
        ret_type: FunctionSpecializeTypeId,
    },

    Unit,

    Struct(NonEmptySlice<FunctionSpecializeExpr>),

    /// Look up exactly one field on a record, tuple, or tag payload.
    /// At this point we've already unified those concepts and have
    /// converted (for example) record field names to indices, and have
    /// also dropped all fields that have no runtime representation (e.g. empty records).
    ///
    /// In a later compilation phase, these indices will be re-sorted
    /// by alignment and converted to byte offsets, but we in this
    /// phase we aren't concerned with alignment or sizes, just indices.
    StructAccess {
        record_expr: FunctionSpecializeExprId,
        record_type: FunctionSpecializeTypeId,
        field_type: FunctionSpecializeTypeId,
        field_id: FieldNameId,
    },

    RecordUpdate {
        record_type: FunctionSpecializeTypeId,
        record_name: IdentId,
        updates: Slice2<FieldNameId, FunctionSpecializeExprId>,
    },

    Tag {
        discriminant: u16,
        tag_union_type: FunctionSpecializeTypeId,
        args: Slice2<FunctionSpecializeTypeId, FunctionSpecializeExprId>,
    },

    When {
        /// The value being matched on
        value: FunctionSpecializeExprId,
        /// The type of the value being matched on
        value_type: FunctionSpecializeTypeId,
        /// The return type of all branches and thus the whole when expression
        branch_type: FunctionSpecializeTypeId,
        /// The branches of the when expression
        branches: NonEmptySlice<WhenBranch>,
    },

    CompilerBug(FunctionSpecializeProblem),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WhenBranch {
    /// The pattern(s) to match the value against
    pub patterns: NonEmptySlice<FunctionSpecializePatternId>,
    /// A boolean expression that must be true for this branch to be taken
    pub guard: Option<FunctionSpecializeExprId>,
    /// The expression to produce if the pattern matches
    pub value: FunctionSpecializeExprId,
}

#[derive(Debug, Default)]
pub struct WhenBranches {
    branches: Vec<MaybeUninit<WhenBranch>>,
}
