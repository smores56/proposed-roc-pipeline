use std::collections::HashMap;

use crate::base::{module::ModuleId, region::Region, symbol::Symbol};

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeVarContent {}

// TODO: Rename to TypeVar
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Variable(u32);

#[derive(Clone, Debug)]
pub enum CanPattern {}

#[derive(Clone, Debug)]
pub enum CanExpr {}

#[derive(Clone, Debug)]
pub enum CanDestructureDef {}

#[derive(Clone, Copy, Debug)]
pub enum DeclarationTag {
    // Value,
    // Expectation,
    // Function(Index<Loc<FunctionDef>>),
    // Recursive(Index<Loc<FunctionDef>>),
    // TailRecursive(Index<Loc<FunctionDef>>),
    // Destructure(Index<DestructureDef>),
    // MutualRecursion {
    //     length: u16,
    //     cycle_mark: IllegalCycleMark,
    // },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EarlyReturnKind {
    Return,
    Try,
}

#[derive(Clone, Debug)]
pub struct FunctionDef {
    pub closure_type: Variable,
    pub return_type: Variable,
    pub fx_type: Variable,
    pub early_returns: Vec<(Variable, Region, EarlyReturnKind)>,
    pub captured_symbols: Vec<(Symbol, Variable)>,
    pub arguments: Vec<(Variable, CanPattern, Region)>,
}

// created from `Declarations`
#[derive(Clone, Debug)]
pub struct CanIR {
    pub declarations: Vec<DeclarationTag>,

    /// same lengths as declarations; has a dummy value if not applicable
    pub variables: Vec<Variable>,
    pub symbols: Vec<Symbol>,
    pub symbol_regions: Vec<Region>,

    pub host_exposed_annotations: HashMap<usize, Variable>,

    pub function_bodies: Vec<FunctionDef>,
    pub function_regions: Vec<Region>,
    pub expressions: Vec<CanExpr>,
    pub expression_regions: Vec<Region>,
    pub destructs: Vec<CanDestructureDef>,
}
