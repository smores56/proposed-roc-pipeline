use core::mem::MaybeUninit;

use crate::base::problem::SolveFunctionsProblem;
use crate::base::symbol::{IdentId, Symbol};
use crate::base::Number;
use crate::env::{FieldNameId, StringLiteralId};
use crate::soa::{Index, NonEmptySlice, Slice, Slice2};

use super::pattern::FuncSolvePatternId;
use super::type_::FuncSolveTypeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FuncSolveExprId(pub(crate) Index<FuncSolveExpr>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FuncSolveExpr {
    Str(StringLiteralId),
    Number(Number),
    List {
        elem_type: FuncSolveTypeId,
        elems: Slice<FuncSolveExprId>,
    },
    Lookup(IdentId, FuncSolveTypeId),

    /// This is *only* for calling functions, not for tag application.
    /// The Tag variant contains any applied values inside it.
    Call {
        fn_type: FuncSolveTypeId,
        fn_expr: FuncSolveExprId,
        args: Slice2<FuncSolveTypeId, FuncSolveExprId>,
    },

    FunctionPack {
        fn_symbol: Symbol,
        captures: Slice<(FuncSolveTypeId, FuncSolvePatternId)>,
    },

    Unit,

    Struct(NonEmptySlice<FuncSolveExpr>),

    /// Look up exactly one field on a record, tuple, or tag payload.
    /// At this point we've already unified those concepts and have
    /// converted (for example) record field names to indices, and have
    /// also dropped all fields that have no runtime representation (e.g. empty records).
    ///
    /// In a later compilation phase, these indices will be re-sorted
    /// by alignment and converted to byte offsets, but we in this
    /// phase we aren't concerned with alignment or sizes, just indices.
    StructAccess {
        record_expr: FuncSolveExprId,
        record_type: FuncSolveTypeId,
        field_type: FuncSolveTypeId,
        field_id: FieldNameId,
    },

    Tag {
        discriminant: u16,
        tag_union_type: FuncSolveTypeId,
        args: Slice2<FuncSolveTypeId, FuncSolveExprId>,
    },

    When {
        /// The value being matched on
        value: FuncSolveExprId,
        /// The type of the value being matched on
        value_type: FuncSolveTypeId,
        /// The return type of all branches and thus the whole when expression
        branch_type: FuncSolveTypeId,
        /// The branches of the when expression
        branches: NonEmptySlice<FuncSolveWhenBranch>,
    },

    CompilerBug(SolveFunctionsProblem),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FuncSolveWhenBranch {
    /// The pattern(s) to match the value against
    pub patterns: NonEmptySlice<FuncSolvePatternId>,
    /// A boolean expression that must be true for this branch to be taken
    pub guard: Option<FuncSolveExprId>,
    /// The expression to produce if the pattern matches
    pub value: FuncSolveExprId,
}

#[derive(Debug, Default)]
pub struct FuncSolveWhenBranches {
    branches: Vec<MaybeUninit<FuncSolveWhenBranch>>,
}
