use env::Env;
use reference_count::RefCountIR;
use resolve_imports::ResolveIR;

pub mod base;
pub mod env;
pub mod lift_functions;
pub mod lower_ir;
pub mod reference_count;
pub mod resolve_imports;
pub mod soa;
pub mod solve_functions;
pub mod specialize_functions;
pub mod specialize_types;

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
// notes:
// - typecheck_ir is the same IR from the `resolve_imports` stage because typechecking just narrows types on existing IR
// - when we stop smooshing modules together, this will need to do more coordination, but the interfaces to each stage should be pretty similar to what they are here
pub fn pipe_ir_from_typechecking_to_codegen(
    typecheck_ir: ResolveIR,
    mut env: Env,
) -> (RefCountIR, Env) {
    let type_spec_ir = specialize_types::specialize_types(&typecheck_ir, &mut env);
    let func_lift_ir = lift_functions::lift_functions(&type_spec_ir, &mut env);
    let func_solve_ir = solve_functions::solve_functions(&func_lift_ir, &mut env);
    let func_spec_ir = specialize_functions::specialize_functions(&func_solve_ir, &mut env);
    let lower_ir_data = lower_ir::lower_ir(&func_spec_ir, &mut env);
    let ref_count_ir = reference_count::reference_count(&lower_ir_data, &mut env);

    (ref_count_ir, env)
}
