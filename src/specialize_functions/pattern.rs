use crate::base::problem::SpecializeTypesProblem;
use crate::base::symbol::IdentId;
use crate::base::NumberLiteral;
use crate::env::{FieldNameId, StringLiteralId};
use crate::soa::{Index, Slice, Slice3};

use super::type_::FuncSpecTypeId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FuncSpecPatternId(pub(crate) Index<FuncSpecPattern>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FuncSpecPattern {
    Identifier(IdentId),
    As(FuncSpecPatternId, IdentId),
    StrLiteral(StringLiteralId),
    NumberLiteral(NumberLiteral),
    AppliedTag {
        tag_union_type: FuncSpecTypeId,
        tag_name: IdentId,
        args: Slice<FuncSpecPatternId>,
    },
    StructDestructure {
        struct_type: FuncSpecTypeId,
        destructs: Slice3<IdentId, FieldNameId, FuncSpecDestructType>,
        opt_spread: Option<(FuncSpecTypeId, FuncSpecPatternId)>,
    },
    List {
        elem_type: FuncSpecTypeId,
        patterns: Slice<FuncSpecPatternId>,

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
pub enum FuncSpecDestructType {
    Required,
    Guard(FuncSpecTypeId, FuncSpecPatternId),
}
