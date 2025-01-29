//! A simplified mocking of the future `resolve_imports` compiler stage's artifacts,
//! which is roughly the artifacts of today's `roc_can` compiler stage.

use std::collections::HashMap;

use crate::{
    base::{region::Region, symbol::Symbol, TypeVar},
    soa::Index,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeContent {}

#[derive(Clone, Debug)]
pub enum ResolvePattern {}

#[derive(Clone, Debug)]
pub enum ResolveExpr {}

#[derive(Clone, Debug)]
pub enum ResolveDestructureDef {}

#[derive(Clone, Copy, Debug)]
pub enum DeclarationTag {
    Value,
    Function(Index<(FunctionDef, Region)>),
    Recursive(Index<(FunctionDef, Region)>),
    TailRecursive(Index<(FunctionDef, Region)>),
    Destructure(Index<ResolveDestructureDef>),
    MutualRecursion {
        length: u16,
        cycle_mark: IllegalCycleMark,
    },
}

/// Marks whether a recursive let-cycle was determined to be illegal during solving.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IllegalCycleMark(OptTypeVar);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OptTypeVar(Option<TypeVar>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EarlyReturnKind {
    Return,
    Try,
}

#[derive(Clone, Debug)]
pub struct FunctionDef {
    pub closure_type: TypeVar,
    pub return_type: TypeVar,
    pub fx_type: TypeVar,
    pub early_returns: Vec<(TypeVar, Region, EarlyReturnKind)>,
    pub captured_symbols: Vec<(Symbol, TypeVar)>,
    pub arguments: Vec<(TypeVar, ResolvePattern, Region)>,
}

// created from `Declarations`
#[derive(Clone, Debug)]
pub struct ResolveIR {
    pub declarations: Vec<DeclarationTag>,

    // utable: UnificationTable,
    // pub type_var_slices: Vec<TypeVarSubsSlice>,
    pub type_vars: Vec<TypeVar>,
    pub symbols: Vec<Symbol>,
    pub symbol_regions: Vec<Region>,

    pub host_exposed_annotations: HashMap<usize, TypeVar>,

    pub function_bodies: Vec<FunctionDef>,
    pub function_regions: Vec<Region>,
    pub expressions: Vec<ResolveExpr>,
    pub expression_regions: Vec<Region>,
    pub destructs: Vec<ResolveDestructureDef>,
}
