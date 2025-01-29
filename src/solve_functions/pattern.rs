use crate::base::problem::SpecializeTypesProblem;
use crate::base::symbol::IdentId;
use crate::base::Number;
use crate::env::{FieldNameId, StringLiteralId};
use crate::soa::{Index, Slice, Slice3};

use super::type_::FuncSolveTypeId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FuncSolvePatternId(pub(crate) Index<FuncSolvePattern>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FuncSolvePattern {
    Identifier(IdentId),
    As(FuncSolvePatternId, IdentId),
    StrLiteral(StringLiteralId),
    NumberLiteral(Number),
    AppliedTag {
        tag_union_type: FuncSolveTypeId,
        tag_name: IdentId,
        args: Slice<FuncSolvePatternId>,
    },
    StructDestructure {
        struct_type: FuncSolveTypeId,
        destructs: Slice3<IdentId, FieldNameId, FuncSolveDestructType>,
        opt_spread: Option<(FuncSolveTypeId, FuncSolvePatternId)>,
    },
    List {
        elem_type: FuncSolveTypeId,
        patterns: Slice<FuncSolvePatternId>,

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
pub enum FuncSolveDestructType {
    Required,
    Guard(FuncSolveTypeId, FuncSolvePatternId),
}
