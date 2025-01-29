pub mod expr;
pub mod layout;
pub mod stmt;

use std::collections::HashMap;

use layout::{LowerLayout, LowerLayoutId};

use crate::{base::symbol::Symbol, env::Env, soa::Slice, specialize_functions::FuncSpecIR};

// Convert a purely first-order program to a low level IR mainly comprising
// procedures and statements that, after adding refcounts, can be used directly for codegen.
//
// Design by Ayaz for this stage:
// https://github.com/roc-lang/rfcs/blob/ayaz/compile-with-lambda-sets/0102-compiling-lambda-sets.md#lower_ir
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
