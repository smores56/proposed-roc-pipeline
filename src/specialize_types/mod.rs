use std::collections::HashMap;
use std::num::NonZeroU16;

use expr::{TypeSpecExpr, TypeSpecExprId};
use pattern::TypeSpecPatternId;
use type_::{TypeSpecType, TypeSpecTypeId};

use crate::base::region::Region;
use crate::base::symbol::{IdentId, Symbol};
use crate::base::type_var::TypeVar;
use crate::base::Variable;
use crate::env::{Env, FieldNameId};
use crate::soa::{Slice, Slice2};

pub mod expr;
pub mod pattern;
pub mod type_;

//
// SPECIALIZE_TYPES
//
// Create a concretely-typed copy of every generic definition in the program. We do this
// by walking the program starting from the program's entry point and make a copy of every
// definition based on each usage found.
//

#[derive(Debug, Default)]
pub struct TypeSpecIR {
    entries: Vec<TypeSpecType>,
    ids: Vec<TypeSpecTypeId>,
    slices: Vec<(NonZeroU16, TypeSpecTypeId)>, // TODO make this a Vec2
    // TODO convert to Vec2
    exprs: Vec<TypeSpecExpr>,
    expr_regions: Vec<Region>,
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
    inner: HashMap<Variable, TypeSpecTypeId>,
}

impl TypeSpecTypeCache {
    pub fn monomorphize_var(
        &mut self,
        // mono_types: &mut TypeSpecTypes,
        var: TypeVar,
        env: &mut Env,
    ) -> TypeSpecTypeId {
        lower_var(var, env)
    }
}

// struct Env<'a, 'c, 'd, 'e, 'f, 'm, 'p, P> {
//     arena: &'a Bump,
//     cache: &'c mut TypeSpecTypeCache,
//     mono_types: &'m mut TypeSpecTypes,
//     field_ids: &'f mut RecordFieldIds,
//     elem_ids: &'e mut TupleElemIds,
// }

fn lower_builtin(_symbol: Symbol, _args: Slice<TypeVar>, _env: &mut Env) -> TypeSpecTypeId {
    todo!()
}

/// Exposed separately because sometimes we already looked up the Content and know it's a function,
/// and want to continue from there without redoing the lookup.
pub fn monomorphize_fn<'a, 'e>(
    _arg_vars: Slice<Variable>,
    _ret_var: TypeVar,
    _env: &mut Env,
) -> TypeSpecTypeId {
    todo!()
}

fn lower_var(_var: TypeVar, _env: &mut Env) -> TypeSpecTypeId {
    // let root_var = subs.get_root_key_without_compacting(var);
    // if let Some(mono_id) = self.cache.inner.get(&root_var) {
    //     return *mono_id;
    // }

    todo!()
}
// }
