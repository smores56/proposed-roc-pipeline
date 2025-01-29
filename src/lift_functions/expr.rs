use core::mem::MaybeUninit;

use crate::base::problem::LiftFunctionsProblem;
use crate::base::symbol::{IdentId, Symbol};
use crate::base::NumberLiteral;
use crate::env::{FieldNameId, StringLiteralId};
use crate::soa::{Index, NonEmptySlice, Slice, Slice2};

use super::pattern::FuncLiftPatternId;
use super::type_::FuncLiftTypeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FuncLiftExprId(pub(crate) Index<FuncLiftExpr>);

#[derive(Debug, Clone, PartialEq)]
pub enum FuncLiftExpr {
    Let(FuncLiftDef),
    Str(StringLiteralId),
    Number(NumberLiteral),
    List {
        elem_type: FuncLiftTypeId,
        elems: Slice<FuncLiftExprId>,
    },
    Lookup(IdentId, FuncLiftTypeId),

    /// This is *only* for calling functions, not for tag application.
    /// The Tag variant contains any applied values inside it.
    Call {
        fn_type: FuncLiftTypeId,
        fn_expr: FuncLiftExprId,
        args: Slice2<FuncLiftTypeId, FuncLiftExprId>,
    },

    FunctionPack {
        fn_symbol: Symbol,
        captures: Slice<(FuncLiftTypeId, FuncLiftPatternId)>,
    },

    Unit,

    Struct(NonEmptySlice<FuncLiftExpr>),

    /// Look up exactly one field on a record, tuple, or tag payload.
    /// At this point we've already unified those concepts and have
    /// converted (for example) record field names to indices, and have
    /// also dropped all fields that have no runtime representation (e.g. empty records).
    ///
    /// In a later compilation phase, these indices will be re-sorted
    /// by alignment and converted to byte offsets, but we in this
    /// phase we aren't concerned with alignment or sizes, just indices.
    StructAccess {
        record_expr: FuncLiftExprId,
        record_type: FuncLiftTypeId,
        field_type: FuncLiftTypeId,
        field_id: FieldNameId,
    },

    Tag {
        discriminant: u16,
        tag_union_type: FuncLiftTypeId,
        args: Slice2<FuncLiftTypeId, FuncLiftExprId>,
    },

    When {
        /// The value being matched on
        value: FuncLiftExprId,
        /// The type of the value being matched on
        value_type: FuncLiftTypeId,
        /// The return type of all branches and thus the whole when expression
        branch_type: FuncLiftTypeId,
        /// The branches of the when expression
        branches: NonEmptySlice<FuncLiftWhenBranch>,
    },

    CompilerBug(LiftFunctionsProblem),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FuncLiftDef {
    pub pattern: FuncLiftPatternId,
    /// Named variables in the pattern, e.g. `a` in `Ok a ->`
    pub pattern_vars: Slice2<IdentId, FuncLiftTypeId>,
    pub expr: FuncLiftExprId,
    pub expr_type: FuncLiftTypeId,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FuncLiftWhenBranch {
    /// The pattern(s) to match the value against
    pub patterns: NonEmptySlice<FuncLiftPatternId>,
    /// A boolean expression that must be true for this branch to be taken
    pub guard: Option<FuncLiftExprId>,
    /// The expression to produce if the pattern matches
    pub value: FuncLiftExprId,
}

#[derive(Debug, Default)]
pub struct FuncLiftWhenBranches {
    branches: Vec<MaybeUninit<FuncLiftWhenBranch>>,
}
