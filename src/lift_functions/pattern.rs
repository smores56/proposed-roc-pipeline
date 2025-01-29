use crate::base::problem::SpecializeTypesProblem;
use crate::base::symbol::IdentId;
use crate::base::NumberLiteral;
use crate::env::{FieldNameId, StringLiteralId};
use crate::soa::{Index, Slice, Slice3};

use super::type_::FuncLiftTypeId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FuncLiftPatternId(pub(crate) Index<FuncLiftPattern>);

#[derive(Clone, Debug, PartialEq)]
pub enum FuncLiftPattern {
    Identifier(IdentId),
    As(FuncLiftPatternId, IdentId),
    StrLiteral(StringLiteralId),
    NumberLiteral(NumberLiteral),
    AppliedTag {
        tag_union_type: FuncLiftTypeId,
        tag_name: IdentId,
        args: Slice<FuncLiftPatternId>,
    },
    StructDestructure {
        struct_type: FuncLiftTypeId,
        destructs: Slice3<IdentId, FieldNameId, FuncLiftDestructType>,
        opt_spread: Option<(FuncLiftTypeId, FuncLiftPatternId)>,
    },
    List {
        elem_type: FuncLiftTypeId,
        patterns: Slice<FuncLiftPatternId>,

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
pub enum FuncLiftDestructType {
    Required,
    Guard(FuncLiftTypeId, FuncLiftPatternId),
}
