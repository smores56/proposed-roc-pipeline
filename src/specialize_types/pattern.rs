use crate::base::Number;
use crate::problem::SpecializeTypesProblem;
use crate::soa::{Index, Slice, Slice3};
use crate::string_interner::InternedStringId;
use crate::symbol::IdentId;

use super::type_::MonoTypeId;
use super::MonoFieldId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MonoPatternId {
    inner: Index<MonoPattern>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MonoPattern {
    Identifier(IdentId),
    As(MonoPatternId, IdentId),
    StrLiteral(InternedStringId),
    NumberLiteral(Number),
    AppliedTag {
        tag_union_type: MonoTypeId,
        tag_name: IdentId,
        args: Slice<MonoPatternId>,
    },
    StructDestructure {
        struct_type: MonoTypeId,
        destructs: Slice3<IdentId, MonoFieldId, DestructType>,
        opt_spread: Option<(MonoTypeId, MonoPatternId)>,
    },
    List {
        elem_type: MonoTypeId,
        patterns: Slice<MonoPatternId>,

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
pub enum DestructType {
    Required,
    Guard(MonoTypeId, MonoPatternId),
}
