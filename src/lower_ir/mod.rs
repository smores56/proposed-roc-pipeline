pub mod expr;
pub mod layout;
pub mod stmt;

use std::collections::HashMap;

use layout::{LowerLayout, LowerLayoutId};

use crate::{base::symbol::Symbol, env::Env, soa::Slice, specialize_functions::FuncSpecIR};

// TODO: explain what this stage should do and a bit of how
pub fn lower_ir(_func_spec_ir: &FuncSpecIR, _env: &mut Env) -> LowerIR {
    todo!()
}

pub struct LowerIR {
    procs: HashMap<Symbol, LowerFunctionData>,
    // values: Vec<ConcreteExpr>,
    layouts: Vec<LowerLayout>,
}

impl core::ops::Index<LowerLayoutId> for LowerIR {
    type Output = LowerLayout;

    fn index(&self, index: LowerLayoutId) -> &Self::Output {
        &self.layouts[index.0.index()]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LowerFunctionData {
    // TODO: convert to Slice
    pub argument_symbols: Vec<Symbol>,
    pub argument_layouts: Slice<LowerLayout>,
    pub return_layout: LowerLayoutId,
}
