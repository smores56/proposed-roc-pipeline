use std::collections::HashMap;

use expr::MonoExprId;
use pattern::MonoPatternId;
use type_::MonoTypeId;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MonoFieldId {
    inner: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Def {
    pub pattern: MonoPatternId,
    /// Named variables in the pattern, e.g. `a` in `Ok a ->`
    pub pattern_vars: Slice2<IdentId, MonoTypeId>,
    pub expr: MonoExprId,
    pub expr_type: MonoTypeId,
}

/// For MonoTypes that are records, store their field indices.
pub type RecordFieldIds = HashMap<MonoTypeId, HashMap<FieldNameId, MonoFieldId>>;

/// For MonoTypes that are tuples, store their element indices.
/// (These are not necessarily the same as their position in the monomorphized tuple,
/// because we may have deleted some zero-sized types in the middle - yet expressions
/// will still refer to e.g. `tuple.1`, so we still need to know which element `.1`
/// referred to originally before we deleted things.
pub type TupleElemIds = HashMap<MonoTypeId, HashMap<u16, MonoFieldId>>;

/// Variables that have already been monomorphized.
pub struct MonoTypeCache {
    inner: HashMap<Variable, MonoTypeId>,
}

impl MonoTypeCache {
    // pub fn from_solved_subs(subs: &Solved<Subs>) -> Self {
    //     Self {
    //         inner: HashMap::with_capacity(subs.inner().len()),
    //     }
    // }

    /// Returns None if it monomorphizes to a type that should be eliminated
    /// (e.g. a zero-sized type like empty record, empty tuple, a record of just those, etc.)
    pub fn monomorphize_var(
        &mut self,
        // mono_types: &mut MonoTypes,
        var: TypeVar,
        env: &mut Env,
    ) -> MonoTypeId {
        lower_var(var, env)
    }
}

// struct Env<'a, 'c, 'd, 'e, 'f, 'm, 'p, P> {
//     arena: &'a Bump,
//     cache: &'c mut MonoTypeCache,
//     mono_types: &'m mut MonoTypes,
//     field_ids: &'f mut RecordFieldIds,
//     elem_ids: &'e mut TupleElemIds,
//     problems: &'p mut P,
//     debug_info: &'d mut Option<DebugInfo>,
// }

fn lower_builtin(_symbol: Symbol, _args: Slice<TypeVar>, _env: &mut Env) -> MonoTypeId {
    todo!()
}

/// Exposed separately because sometimes we already looked up the Content and know it's a function,
/// and want to continue from there without redoing the lookup.
pub fn monomorphize_fn<'a, 'e>(
    _arg_vars: Slice<Variable>,
    _ret_var: TypeVar,
    _env: &mut Env,
) -> MonoTypeId {
    todo!()
}

fn lower_var(_var: TypeVar, _env: &mut Env) -> MonoTypeId {
    // let root_var = subs.get_root_key_without_compacting(var);
    // if let Some(mono_id) = self.cache.inner.get(&root_var) {
    //     return *mono_id;
    // }

    todo!()
}
// }
