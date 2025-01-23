use crate::base::problem::SpecializeTypesProblem;
use crate::base::symbol::IdentId;
use crate::base::Number;
use crate::env::{FieldNameId, StringLiteralId};
use crate::soa::{Index, Slice, Slice3};

use super::type_::FunctionSolveTypeId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FunctionSolvePatternId {
    inner: Index<FunctionSolvePattern>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FunctionSolvePattern {
    Identifier(IdentId),
    As(FunctionSolvePatternId, IdentId),
    StrLiteral(StringLiteralId),
    NumberLiteral(Number),
    AppliedTag {
        tag_union_type: FunctionSolveTypeId,
        tag_name: IdentId,
        args: Slice<FunctionSolvePatternId>,
    },
    StructDestructure {
        struct_type: FunctionSolveTypeId,
        destructs: Slice3<IdentId, FieldNameId, FunctionSolveDestructType>,
        opt_spread: Option<(FunctionSolveTypeId, FunctionSolvePatternId)>,
    },
    List {
        elem_type: FunctionSolveTypeId,
        patterns: Slice<FunctionSolvePatternId>,

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
pub enum FunctionSolveDestructType {
    Required,
    Guard(FunctionSolveTypeId, FunctionSolvePatternId),
}
