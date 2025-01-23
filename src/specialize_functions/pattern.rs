use crate::base::problem::SpecializeTypesProblem;
use crate::base::symbol::IdentId;
use crate::base::Number;
use crate::env::{FieldNameId, StringLiteralId};
use crate::soa::{Index, Slice, Slice3};

use super::type_::FunctionSpecializeTypeId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FunctionSpecializePatternId {
    inner: Index<FunctionSpecializePattern>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FunctionSpecializePattern {
    Identifier(IdentId),
    As(FunctionSpecializePatternId, IdentId),
    StrLiteral(StringLiteralId),
    NumberLiteral(Number),
    AppliedTag {
        tag_union_type: FunctionSpecializeTypeId,
        tag_name: IdentId,
        args: Slice<FunctionSpecializePatternId>,
    },
    StructDestructure {
        struct_type: FunctionSpecializeTypeId,
        destructs: Slice3<IdentId, FieldNameId, FunctionSpecializeDestructType>,
        opt_spread: Option<(FunctionSpecializeTypeId, FunctionSpecializePatternId)>,
    },
    List {
        elem_type: FunctionSpecializeTypeId,
        patterns: Slice<FunctionSpecializePatternId>,

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
pub enum FunctionSpecializeDestructType {
    Required,
    Guard(FunctionSpecializeTypeId, FunctionSpecializePatternId),
}
