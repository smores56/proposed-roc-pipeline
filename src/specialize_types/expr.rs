use core::mem::MaybeUninit;

use crate::base::foreign_symbol::ForeignSymbolId;
use crate::base::low_level::LowLevel;
use crate::base::problem::SpecializeTypesProblem;
use crate::base::region::Region;
use crate::base::symbol::IdentId;
use crate::base::{Number, Recursive};
use crate::env::{FieldNameId, StringLiteralId};
use crate::soa::{Index, NonEmptySlice, Slice, Slice2};

use super::pattern::MonoPatternId;
use super::type_::MonoTypeId;

#[derive(Debug, Default)]
pub struct MonoExprs {
    // TODO convert to Vec2
    exprs: Vec<MonoExpr>,
    regions: Vec<Region>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MonoExprId {
    inner: Index<MonoExpr>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MonoExpr {
    Str(StringLiteralId),
    Number(Number),
    List {
        elem_type: MonoTypeId,
        elems: Slice<MonoExprId>,
    },
    Lookup(IdentId, MonoTypeId),

    /// This is *only* for calling functions, not for tag application.
    /// The Tag variant contains any applied values inside it.
    Call {
        fn_type: MonoTypeId,
        fn_expr: MonoExprId,
        args: Slice2<MonoTypeId, MonoExprId>,
        /// This is the type of the closure based only on canonical IR info,
        /// not considering what other closures might later influence it.
        /// Lambda set specialization may change this type later!
        capture_type: MonoTypeId,
    },
    RunLowLevel {
        op: LowLevel,
        args: Slice<(MonoTypeId, MonoExprId)>,
        ret_type: MonoTypeId,
    },
    ForeignCall {
        foreign_symbol: ForeignSymbolId,
        args: Slice<(MonoTypeId, MonoExprId)>,
        ret_type: MonoTypeId,
    },

    Lambda {
        fn_type: MonoTypeId,
        arguments: Slice<(MonoTypeId, MonoPatternId)>,
        body: MonoExprId,
        captured_symbols: Slice<(IdentId, MonoTypeId)>,
        recursive: Recursive,
    },

    Unit,

    /// A record literal or a tuple literal.
    /// These have already been sorted alphabetically.
    Struct(NonEmptySlice<MonoExpr>),

    /// Look up exactly one field on a record, tuple, or tag payload.
    /// At this point we've already unified those concepts and have
    /// converted (for example) record field names to indices, and have
    /// also dropped all fields that have no runtime representation (e.g. empty records).
    ///
    /// In a later compilation phase, these indices will be re-sorted
    /// by alignment and converted to byte offsets, but we in this
    /// phase we aren't concerned with alignment or sizes, just indices.
    StructAccess {
        record_expr: MonoExprId,
        record_type: MonoTypeId,
        field_type: MonoTypeId,
        field_id: FieldNameId,
    },

    RecordUpdate {
        record_type: MonoTypeId,
        record_name: IdentId,
        updates: Slice2<FieldNameId, MonoExprId>,
    },

    /// Same as SmallTag but with u16 discriminant instead of u8
    Tag {
        discriminant: u16,
        tag_union_type: MonoTypeId,
        args: Slice2<MonoTypeId, MonoExprId>,
    },

    When {
        /// The value being matched on
        value: MonoExprId,
        /// The type of the value being matched on
        value_type: MonoTypeId,
        /// The return type of all branches and thus the whole when expression
        branch_type: MonoTypeId,
        /// The branches of the when expression
        branches: NonEmptySlice<WhenBranch>,
    },

    CompilerBug(SpecializeTypesProblem),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WhenBranch {
    /// The pattern(s) to match the value against
    pub patterns: NonEmptySlice<MonoPatternId>,
    /// A boolean expression that must be true for this branch to be taken
    pub guard: Option<MonoExprId>,
    /// The expression to produce if the pattern matches
    pub value: MonoExprId,
}

#[derive(Debug, Default)]
pub struct WhenBranches {
    branches: Vec<MaybeUninit<WhenBranch>>,
}
