use core::mem::MaybeUninit;

use crate::base::problem::SpecializeFunctionsProblem;
use crate::base::symbol::{IdentId, Symbol};
use crate::base::NumberLiteral;
use crate::env::{FieldNameId, StringLiteralId};
use crate::soa::{Index, NonEmptySlice, Slice, Slice2};

use super::pattern::FuncSpecPatternId;
use super::type_::FuncSpecTypeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FuncSpecExprId(pub(crate) Index<FuncSpecExpr>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FuncSpecExpr {
    Let(FuncSpecDef),
    Str(StringLiteralId),
    Number(NumberLiteral),
    List {
        elem_type: FuncSpecTypeId,
        elems: Slice<FuncSpecExprId>,
    },
    Lookup(IdentId, FuncSpecTypeId),

    /// This is *only* for calling functions, not for tag application.
    /// The Tag variant contains any applied values inside it.
    Call {
        fn_type: FuncSpecTypeId,
        fn_symbol: Symbol,
        args: Slice2<FuncSpecTypeId, FuncSpecExprId>,
    },

    Unit,

    Struct(NonEmptySlice<FuncSpecExpr>),

    /// Look up exactly one field on a record, tuple, or tag payload.
    /// At this point we've already unified those concepts and have
    /// converted (for example) record field names to indices, and have
    /// also dropped all fields that have no runtime representation (e.g. empty records).
    ///
    /// In a later compilation phase, these indices will be re-sorted
    /// by alignment and converted to byte offsets, but we in this
    /// phase we aren't concerned with alignment or sizes, just indices.
    StructAccess {
        record_expr: FuncSpecExprId,
        record_type: FuncSpecTypeId,
        field_type: FuncSpecTypeId,
        field_id: FieldNameId,
    },

    Tag {
        discriminant: u16,
        tag_union_type: FuncSpecTypeId,
        args: Slice2<FuncSpecTypeId, FuncSpecExprId>,
    },

    When {
        /// The value being matched on
        value: FuncSpecExprId,
        /// The type of the value being matched on
        value_type: FuncSpecTypeId,
        /// The return type of all branches and thus the whole when expression
        branch_type: FuncSpecTypeId,
        /// The branches of the when expression
        branches: NonEmptySlice<FuncSpecWhenBranch>,
    },

    CompilerBug(SpecializeFunctionsProblem),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FuncSpecDef {
    pub pattern: FuncSpecPatternId,
    /// Named variables in the pattern, e.g. `a` in `Ok a ->`
    pub pattern_vars: Slice2<IdentId, FuncSpecTypeId>,
    pub expr: FuncSpecExprId,
    pub expr_type: FuncSpecTypeId,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FuncSpecWhenBranch {
    /// The pattern(s) to match the value against
    pub patterns: NonEmptySlice<FuncSpecPatternId>,
    /// A boolean expression that must be true for this branch to be taken
    pub guard: Option<FuncSpecExprId>,
    /// The expression to produce if the pattern matches
    pub value: FuncSpecExprId,
}

#[derive(Debug, Default)]
pub struct FuncSpecWhenBranches {
    branches: Vec<MaybeUninit<FuncSpecWhenBranch>>,
}
