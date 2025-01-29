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

/// Run the `build` phase of compiling Roc code except for codegen.
///
/// For now, we pass the IR from the last `check` stage (e.g. the `resolve_imports`
/// IR that is narrowed by typechecking) as a single, combined module here along
/// with a singleton `Env` that holds all common large data, like interned strings,
/// symbols, and interned tag names.
///
/// In the future, we will not be combining all modules into a single "module" so this
/// will need to do more complicated coordination of the compiler stages. That said,
/// this still represents the long-term planned order of compilation.
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
