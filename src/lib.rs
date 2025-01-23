use std::collections::HashMap;

use base::symbol::Symbol;
use env::Env;
use specialize_types::MonoTypeCache;

pub mod base;
pub mod can;
pub mod env;
pub mod lift_functions;
pub mod lower_ir;
pub mod soa;
pub mod solve_functions;
pub mod specialize_functions;
pub mod specialize_types;

// #[derive(Clone, Copy, Debug, PartialEq)]
// pub enum Content {
//     /// A type variable which the user did not name in an annotation,
//     ///
//     /// When we auto-generate a type var name, e.g. the "a" in (a -> a), we
//     /// change the Option in here from None to Some.
//     FlexVar(Option<SubsIndex<Lowercase>>),
//     /// name given in a user-written annotation
//     RigidVar(SubsIndex<Lowercase>),
//     // /// Like a [Self::FlexVar], but is also bound to 1+ abilities.
//     // /// This can only happen when unified with a [Self::RigidAbleVar].
//     // FlexAbleVar(Option<SubsIndex<Lowercase>>, SubsSlice<Symbol>),
//     // /// Like a [Self::RigidVar], but is also bound to 1+ abilities.
//     // /// For example, "a implements Hash".
//     // RigidAbleVar(SubsIndex<Lowercase>, SubsSlice<Symbol>),
//     /// name given to a recursion variable
//     RecursionVar {
//         structure: Variable,
//         opt_name: Option<SubsIndex<Lowercase>>,
//     },
//     Structure(FlatType),
//     Alias(Symbol, AliasVariables, Variable, AliasKind),
//     RangedNumber(crate::num::NumericRange),
//     Error,
//     /// The fx type variable for a given function
//     Pure,
//     Effectful,
// }

//

pub struct CanVariable(u32);

pub struct TypecheckingIr {
    vars: Vec<CanVariable>,
}

fn pipe_ir_from_typechecking_to_codegen(typechecking_ir: TypecheckingIr, env: Env) -> CodegenIr {
    // steps for the whole dang compiler (check and run):
    // in common:
    // - read the header of the passed module:
    //   - if it has packages, find their root modules.
    //     - register all .roc files with their respective root modules and shorthands
    //     - register non .roc files as potential file content imports relative to their root modules
    //   - if it doesn't have packages, it's not the root module, so search up the file tree for a `main.roc` ???
    //     - if not found, treat the passed module as the root
    // - for all modules found, parse and solo canonicalize in parallel
    // - once done, combine all modules into a single module in `roc_can_combine`
    // - once done, constrain, and then solve
    // - if just typechecking, return
    // - after that, run the lambda set compiling stages on the megamodule:
    //   - type specialize
    //   - function lift
    //   - function solve
    //   - function specialize
    //   - refcount
    //   - lower IR
    // - in addition to the discovered modules for each package, register the modules

    //
    // SPECIALIZE_TYPES
    //
    // Create a concretely-typed copy of every generic definition in the program. We do this
    // by walking the program starting from the program's entry point and make a copy of every
    // definition based on each usage found.
    //

    let specialize_types_cache = MonoTypeCache::from_solved_subs(subs);
    for var in typechecking_ir.vars {
        let mono_var = specialize_types_cache.monomorphize_var(subs, mono_types, var, env);
    }

    //
    // FUNCTION_LIFT
    //
    // Lift all nested functions to the top level. We do this by finding all values
    // captured by a function (HERE, NOT IN CAN!)
    //
}

//
// MONO (CURRENT)
//

//  #[derive(Clone, Debug, PartialEq)]
// pub enum Stmt<'a> {
//     Let(Symbol, Expr<'a>, InLayout<'a>, &'a Stmt<'a>),
//     Switch {
//         /// This *must* stand for an integer, because Switch potentially compiles to a jump table.
//         cond_symbol: Symbol,
//         cond_layout: InLayout<'a>,
//         /// The u64 in the tuple will be compared directly to the condition Expr.
//         /// If they are equal, this branch will be taken.
//         branches: &'a [(u64, BranchInfo<'a>, Stmt<'a>)],
//         /// If no other branches pass, this default branch will be taken.
//         default_branch: (BranchInfo<'a>, &'a Stmt<'a>),
//         /// Each branch must return a value of this type.
//         ret_layout: InLayout<'a>,
//     },
//     Ret(Symbol),
//     Refcounting(ModifyRc, &'a Stmt<'a>),
//     Expect {
//         condition: Symbol,
//         region: Region,
//         lookups: &'a [Symbol],
//         variables: &'a [LookupType],
//         /// what happens after the expect
//         remainder: &'a Stmt<'a>,
//     },
//     Dbg {
//         /// The location this dbg is in source as a printable string.
//         source_location: &'a str,
//         /// The source code of the expression being debugged.
//         source: &'a str,
//         /// The expression we're displaying
//         symbol: Symbol,
//         /// The specialized variable of the expression
//         variable: Variable,
//         /// What happens after the dbg
//         remainder: &'a Stmt<'a>,
//     },
//     /// a join point `join f <params> = <continuation> in remainder`
//     Join {
//         id: JoinPointId,
//         parameters: &'a [Param<'a>],
//         /// body of the join point
//         /// what happens after _jumping to_ the join point
//         body: &'a Stmt<'a>,
//         /// what happens after _defining_ the join point
//         remainder: &'a Stmt<'a>,
//     },
//     Jump(JoinPointId, &'a [Symbol]),
//     Crash(Symbol, CrashTag),
// }
