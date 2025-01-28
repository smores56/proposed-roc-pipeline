//
// MONO (CURRENT)
//

use std::collections::HashMap;

use layout::{LoweredLayout, LoweredLayoutId};

use crate::{base::symbol::Symbol, soa::Slice};

pub mod expr;
pub mod layout;
pub mod stmt;

pub struct LoweredIrData {
    procs: HashMap<Symbol, LoweredFunctionData>,
    // values: Vec<ConcreteExpr>,
    layouts: Vec<LoweredLayout>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredFunctionData {
    // TODO: convert to Slice
    pub argument_symbols: Vec<Symbol>,
    pub argument_layouts: Slice<LoweredLayout>,
    pub return_layout: LoweredLayoutId,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoweredExpr {}

pub enum LoweredType {}
