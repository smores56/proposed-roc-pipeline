use crate::base::problem::SpecializeTypesProblem;
use crate::base::symbol::IdentId;
use crate::base::Number;
use crate::env::{FieldNameId, StringLiteralId};
use crate::soa::{Index, Slice, Slice3};

use super::type_::FunctionLiftTypeId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FunctionLiftPatternId {
    inner: Index<FunctionLiftPattern>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FunctionLiftPattern {
    Identifier(IdentId),
    As(FunctionLiftPatternId, IdentId),
    StrLiteral(StringLiteralId),
    NumberLiteral(Number),
    AppliedTag {
        tag_union_type: FunctionLiftTypeId,
        tag_name: IdentId,
        args: Slice<FunctionLiftPatternId>,
    },
    StructDestructure {
        struct_type: FunctionLiftTypeId,
        destructs: Slice3<IdentId, FieldNameId, FunctionLiftDestructType>,
        opt_spread: Option<(FunctionLiftTypeId, FunctionLiftPatternId)>,
    },
    List {
        elem_type: FunctionLiftTypeId,
        patterns: Slice<FunctionLiftPatternId>,

        /// Where a rest pattern splits patterns before and after it, if it does at all.
        /// If present, patterns at index >= the rest index appear after the rest pattern.
        /// For example:
        ///   [ .., A, B ] -> patterns = [A, B], rest = 0
        ///   [ A, .., B ] -> patterns = [A, B], rest = 1
        ///   [ A, B, .. ] -> patterns = [A, B], rest = 2
        /// Optionally, the rest pattern can be named - e.g. `[ A, B, ..others ]`
        opt_rest: Option<(u16, Option<IdentId>)>,
    },
    Underscore,
    CompilerBug(SpecializeTypesProblem),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FunctionLiftDestructType {
    Required,
    Guard(FunctionLiftTypeId, FunctionLiftPatternId),
}
