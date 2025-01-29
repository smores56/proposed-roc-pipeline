use core::mem::MaybeUninit;

use crate::base::problem::SpecializeTypesProblem;
use crate::base::symbol::IdentId;
use crate::base::{NumberLiteral, Recursive};
use crate::env::{FieldNameId, StringLiteralId};
use crate::soa::{Index, NonEmptySlice, Slice, Slice2};

use super::pattern::TypeSpecPatternId;
use super::type_::TypeSpecTypeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeSpecExprId(pub(crate) Index<TypeSpecExpr>);

#[derive(Clone, Debug, PartialEq)]
pub enum TypeSpecExpr {
    Let(TypeSpecDef),
    Str(StringLiteralId),
    Number(NumberLiteral),
    List {
        elem_type: TypeSpecTypeId,
        elems: Slice<TypeSpecExprId>,
    },
    Lookup(IdentId, TypeSpecTypeId),

    /// This is *only* for calling functions, not for tag application.
    /// The Tag variant contains any applied values inside it.
    Call {
        fn_type: TypeSpecTypeId,
        fn_expr: TypeSpecExprId,
        args: Slice2<TypeSpecTypeId, TypeSpecExprId>,
    },

    Lambda {
        fn_type: TypeSpecTypeId,
        arguments: Slice<(TypeSpecTypeId, TypeSpecPatternId)>,
        body: TypeSpecExprId,
        captured_symbols: Slice<(IdentId, TypeSpecTypeId)>,
        recursive: Recursive,
    },

    Unit,

    /// A record literal or a tuple literal.
    /// These have already been sorted alphabetically.
    Struct(NonEmptySlice<TypeSpecExpr>),

    /// Look up exactly one field on a record, tuple, or tag payload.
    /// At this point we've already unified those concepts and have
    /// converted (for example) record field names to indices, and have
    /// also dropped all fields that have no runtime representation (e.g. empty records).
    ///
    /// In a later compilation phase, these indices will be re-sorted
    /// by alignment and converted to byte offsets, but we in this
    /// phase we aren't concerned with alignment or sizes, just indices.
    StructAccess {
        record_expr: TypeSpecExprId,
        record_type: TypeSpecTypeId,
        field_type: TypeSpecTypeId,
        field_id: FieldNameId,
    },

    /// Same as SmallTag but with u16 discriminant instead of u8
    Tag {
        discriminant: u16,
        tag_union_type: TypeSpecTypeId,
        args: Slice2<TypeSpecTypeId, TypeSpecExprId>,
    },

    When {
        /// The value being matched on
        value: TypeSpecExprId,
        /// The type of the value being matched on
        value_type: TypeSpecTypeId,
        /// The return type of all branches and thus the whole when expression
        branch_type: TypeSpecTypeId,
        /// The branches of the when expression
        branches: NonEmptySlice<TypeSpecWhenBranch>,
    },

    CompilerBug(SpecializeTypesProblem),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeSpecDef {
    pub pattern: TypeSpecPatternId,
    /// Named variables in the pattern, e.g. `a` in `Ok a ->`
    pub pattern_vars: Slice2<IdentId, TypeSpecTypeId>,
    pub expr: TypeSpecExprId,
    pub expr_type: TypeSpecTypeId,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeSpecWhenBranch {
    /// The pattern(s) to match the value against
    pub patterns: NonEmptySlice<TypeSpecPatternId>,
    /// A boolean expression that must be true for this branch to be taken
    pub guard: Option<TypeSpecExprId>,
    /// The expression to produce if the pattern matches
    pub value: TypeSpecExprId,
}

#[derive(Debug, Default)]
pub struct TypeSpecWhenBranches {
    branches: Vec<MaybeUninit<TypeSpecWhenBranch>>,
}
