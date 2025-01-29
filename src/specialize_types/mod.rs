use std::collections::HashMap;
use std::num::NonZeroU16;

use expr::{TypeSpecExpr, TypeSpecExprId};
use pattern::{TypeSpecPattern, TypeSpecPatternId};
use type_::{TypeSpecType, TypeSpecTypeId};

use crate::base::region::Region;
use crate::base::symbol::IdentId;
use crate::base::TypeVar;
use crate::env::{Env, FieldNameId};
use crate::resolve_imports::ResolveIR;
use crate::soa::Slice2;

pub mod expr;
pub mod pattern;
pub mod type_;

// Create a concretely-typed copy of every generic definition in the program. We do this
// by walking the program starting from the program's entry point and make a copy of every
// definition based on each usage found.
//
// This is has been partially implemented already by Agus in the compiler:
// https://github.com/roc-lang/roc/tree/main/crates/build/specialize_types
//
// Design by Ayaz for this stage:
// https://github.com/roc-lang/rfcs/blob/ayaz/compile-with-lambda-sets/0102-compiling-lambda-sets.md#type_specialize
pub fn specialize_types(_typecheck_ir: &ResolveIR, _env: &mut Env) -> TypeSpecIR {
    todo!()
}

#[derive(Debug, Default)]
pub struct TypeSpecIR {
    exprs: Vec<TypeSpecExpr>,
    expr_regions: Vec<Region>,
    patterns: Vec<TypeSpecPattern>,
    types: Vec<TypeSpecType>,
    type_ids_for_slicing: Vec<TypeSpecTypeId>,
    // TODO: do we need this yet?
    slices: Vec<(NonZeroU16, TypeSpecTypeId)>,
}

impl core::ops::Index<TypeSpecExprId> for TypeSpecIR {
    type Output = TypeSpecExpr;

    fn index(&self, index: TypeSpecExprId) -> &Self::Output {
        &self.exprs[index.0.index()]
    }
}

impl core::ops::Index<TypeSpecPatternId> for TypeSpecIR {
    type Output = TypeSpecPattern;

    fn index(&self, index: TypeSpecPatternId) -> &Self::Output {
        &self.patterns[index.0.index()]
    }
}

impl core::ops::Index<TypeSpecTypeId> for TypeSpecIR {
    type Output = TypeSpecType;

    fn index(&self, index: TypeSpecTypeId) -> &Self::Output {
        &self.types[index.0.index()]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeSpecFieldId {
    inner: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Def {
    pub pattern: TypeSpecPatternId,
    /// Named variables in the pattern, e.g. `a` in `Ok a ->`
    pub pattern_vars: Slice2<IdentId, TypeSpecTypeId>,
    pub expr: TypeSpecExprId,
    pub expr_type: TypeSpecTypeId,
}

/// For TypeSpecTypes that are records, store their field indices.
pub type RecordFieldIds = HashMap<TypeSpecTypeId, HashMap<FieldNameId, TypeSpecFieldId>>;

/// For TypeSpecTypes that are tuples, store their element indices.
/// (These are not necessarily the same as their position in the monomorphized tuple,
/// because we may have deleted some zero-sized types in the middle - yet expressions
/// will still refer to e.g. `tuple.1`, so we still need to know which element `.1`
/// referred to originally before we deleted things.
pub type TupleElemIds = HashMap<TypeSpecTypeId, HashMap<u16, TypeSpecFieldId>>;

/// Variables that have already been monomorphized.
pub struct TypeSpecTypeCache {
    types_by_var: HashMap<TypeVar, TypeSpecTypeId>,
}
