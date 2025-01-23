use std::collections::HashMap;

use crate::base::module::ModuleId;

struct RootModuleInfo {
    packages: HashMap<String, PackageInfo>,
    modules_by_id: HashMap<ModuleId, SoloCanModule>,
}

struct SoloCanModule {
    src: String,
    module_id: ModuleId,
}

struct PackageInfo {
    // content_hash:
}

// #[derive(Debug)]
// enum Msg<'a> {
//     Header(ModuleHeader<'a>),
//     Parsed(ParsedModule<'a>),
//     SoloCanonicalized(ModuleId, CanSolo<'a>),
//     CanonicalizedAndConstrained(CanAndCon),
//     SolvedTypes {
//         module_id: ModuleId,
//         ident_ids: IdentIds,
//         solved_module: SolvedModule,
//         solved_subs: Solved<Subs>,
//         decls: Declarations,
//         dep_idents: IdentIdsByModule,
//         module_timing: ModuleTiming,
//         abilities_store: AbilitiesStore,
//         loc_expects: LocExpects,
//         has_dbgs: bool,

//         #[cfg(debug_assertions)]
//         checkmate: Option<roc_checkmate::Collector>,
//     },
//     FinishedAllTypeChecking {
//         solved_subs: Solved<Subs>,
//         exposed_vars_by_symbol: Vec<(Symbol, Variable)>,
//         exposed_aliases_by_symbol: MutMap<Symbol, (bool, Alias)>,
//         exposed_types_storage: ExposedTypesStorageSubs,
//         resolved_implementations: ResolvedImplementations,
//         dep_idents: IdentIdsByModule,
//         documentation: VecMap<ModuleId, ModuleDocumentation>,
//         abilities_store: AbilitiesStore,

//         #[cfg(debug_assertions)]
//         checkmate: Option<roc_checkmate::Collector>,
//     },
// }

#[allow(clippy::unnecessary_wraps)]
fn canonicalize_and_constrain<'a>(
    qualified_module_ids: &'a PackageModuleIds<'a>,
    dep_idents: IdentIdsByModule,
    exposed_symbols: VecSet<Symbol>,
    aliases: MutMap<Symbol, Alias>,
    parsed: ParsedModule<'a>,
    exposed_module_ids: &[ModuleId],
    exec_mode: ExecutionMode,
    imported_module_params: HashMap<ModuleId, ModuleParams>,
    solo_can_output: SoloCanOutput<'a>,
) -> CanAndCon {
    let canonicalize_start = Instant::now();

    let ParsedModule {
        module_id,
        module_path,
        header_type,
        parsed_defs,
        initial_scope,
        available_modules,
        mut module_timing,
        symbols_from_requires,
        opt_shorthand,
        exposed_ident_ids,
        ..
    } = parsed;

    let mut var_store = VarStore::default();

    let env = Env::from_solo_can(
        arena,
        &module_path,
        module_id,
        &dep_idents,
        qualified_module_ids,
        solo_can_output.problems,
        opt_shorthand,
        solo_can_output.src,
        solo_can_output.lazy_line_info,
    );

    let mut scope = Scope::new(
        module_id,
        qualified_module_ids
            .get_name(module_id)
            .expect("home module not found")
            .as_inner()
            .to_owned(),
        exposed_ident_ids,
        imported_abilities_state,
    );

    for (name, alias) in aliases.into_iter() {
        scope.add_alias(
            name,
            alias.region,
            alias.type_variables,
            alias.infer_ext_in_output_variables,
            alias.typ,
            alias.kind,
        );
    }

    let mut module_output = canonicalize_module_defs(
        arena,
        &header_type,
        module_id,
        initial_scope,
        exposed_symbols,
        &symbols_from_requires,
        &mut var_store,
        scope,
        env,
        solo_can_output.loc_defs,
        solo_can_output.module_params,
    );

    let mut types = Types::new();

    // _after has an underscore because it's unused in --release builds
    let _after = roc_types::types::get_type_clone_count();

    log!(
        "canonicalize of {:?} cloned Type {} times ({} -> {})",
        module_id,
        _after - _before,
        _before,
        _after
    );

    let canonicalize_end = Instant::now();

    module_timing.canonicalize = canonicalize_end.duration_since(canonicalize_start);

    // Generate documentation information
    // TODO: store timing information?
    let module_docs = {
        let module_name = header_type.get_name();
        module_name.map(|module_name| {
            let mut scope = module_output.scope.clone();
            scope.add_docs_imports();
            crate::docs::generate_module_docs(
                scope,
                module_id,
                arena.alloc(qualified_module_ids.clone().into_module_ids()),
                module_name.into(),
                &parsed_defs_for_docs,
                exposed_module_ids,
                module_output.exposed_symbols.clone(),
                parsed.header_comments,
            )
        })
    };

    // _before has an underscore because it's unused in --release builds
    let _before = roc_types::types::get_type_clone_count();

    match exec_mode {
        ExecutionMode::Check => {
            // No need to lower params for `roc check` and lang server
            // If we did, we'd have to update the language server to exclude the extra arguments
        }
        ExecutionMode::Executable | ExecutionMode::ExecutableIfCheck | ExecutionMode::Test => {
            // We need to lower params only if the current module has any or imports at least one with params
            if module_output.module_params.is_some() || !imported_module_params.is_empty() {
                roc_lower_params::lower::lower(
                    module_id,
                    &module_output.module_params,
                    imported_module_params,
                    &mut module_output.declarations,
                    &mut module_output.scope.locals.ident_ids,
                    &mut var_store,
                );
            }
        }
    }

    let mut constraints = Constraints::new();

    let constraint = if skip_constraint_gen {
        roc_can::constraint::Constraint::True
    } else {
        constrain_module(
            &mut types,
            &mut constraints,
            module_output.symbols_from_requires,
            &module_output.scope.abilities_store,
            &module_output.declarations,
            &module_output.module_params,
            module_id,
        )
    };

    // _after has an underscore because it's unused in --release builds
    let _after = roc_types::types::get_type_clone_count();

    log!(
        "constraint gen of {:?} cloned Type {} times ({} -> {})",
        module_id,
        _after - _before,
        _before,
        _after
    );

    // scope has imported aliases, but misses aliases from inner scopes
    // module_output.aliases does have those aliases, so we combine them
    let mut aliases: MutMap<Symbol, (bool, Alias)> = module_output
        .aliases
        .into_iter()
        .map(|(k, v)| (k, (true, v)))
        .collect();

    for (name, alias) in module_output.scope.aliases {
        match aliases.entry(name) {
            Occupied(_) => {
                // do nothing
            }
            Vacant(vacant) => {
                let should_include_builtin = matches!(
                    name.module_id(),
                    ModuleId::ENCODE
                        | ModuleId::DECODE
                        | ModuleId::DICT
                        | ModuleId::SET
                        | ModuleId::HASH
                        | ModuleId::INSPECT
                );

                if !name.is_builtin() || should_include_builtin {
                    vacant.insert((false, alias));
                }
            }
        }
    }

    let module = Module {
        module_id,
        exposed_imports: module_output.exposed_imports,
        exposed_symbols: module_output.exposed_symbols,
        referenced_values: module_output.referenced_values,
        aliases,
        rigid_variables: module_output.rigid_variables,
        abilities_store: module_output.scope.abilities_store,
        loc_expects: module_output.loc_expects,
        has_dbgs: module_output.has_dbgs,
        module_params: module_output.module_params,
    };

    let constrained_module = ConstrainedModule {
        module,
        declarations: module_output.declarations,
        available_modules,
        var_store,
        constraints,
        constraint,
        ident_ids: module_output.scope.locals.ident_ids,
        dep_idents,
        module_timing,
        types,
        pending_derives: module_output.pending_derives,
    };

    CanAndCon {
        constrained_module,
        canonicalization_problems: module_output.problems,
        module_docs,
    }
}
