use crate::base::problem::SpecializeTypesProblem;
use crate::base::symbol::IdentId;
use crate::base::Number;
use crate::env::StringLiteralId;
use crate::soa::{Index, Slice, Slice3};

use super::type_::TypeSpecTypeId;
use super::TypeSpecFieldId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeSpecPatternId(pub(crate) Index<TypeSpecPattern>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeSpecPattern {
    Identifier(IdentId),
    As(TypeSpecPatternId, IdentId),
    StrLiteral(StringLiteralId),
    NumberLiteral(Number),
    AppliedTag {
        tag_union_type: TypeSpecTypeId,
        tag_name: IdentId,
        args: Slice<TypeSpecPatternId>,
    },
    StructDestructure {
        struct_type: TypeSpecTypeId,
        destructs: Slice3<IdentId, TypeSpecFieldId, TypeSpecDestructType>,
        opt_spread: Option<(TypeSpecTypeId, TypeSpecPatternId)>,
    },
    List {
        elem_type: TypeSpecTypeId,
        patterns: Slice<TypeSpecPatternId>,

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
pub enum TypeSpecDestructType {
    Required,
    Guard(TypeSpecTypeId, TypeSpecPatternId),
}
