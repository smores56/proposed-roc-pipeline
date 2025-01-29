pub mod expr;
pub mod pattern;
pub mod type_;

use expr::{FuncSpecExpr, FuncSpecExprId};
use pattern::{FuncSpecPattern, FuncSpecPatternId};
use type_::{FuncSpecType, FuncSpecTypeId};

use crate::{base::region::Region, env::Env, solve_functions::FuncSolveIR};

// Create copies of generic higher-order functions (HOFs) for concrete usage,
// and then fix-up the call sites to reference the copy Moreover, we re-write
// each function set to be a tag union that we pass to the function.
//
// Design by Ayaz for this stage:
// https://github.com/roc-lang/rfcs/blob/ayaz/compile-with-lambda-sets/0102-compiling-lambda-sets.md#function_specialize
pub fn specialize_functions(_func_solve_ir: &FuncSolveIR, _env: &mut Env) -> FuncSpecIR {
    todo!()
}

#[derive(Default)]
pub struct FuncSpecIR {
    exprs: Vec<FuncSpecExpr>,
    expr_regions: Vec<Region>,
    patterns: Vec<FuncSpecPattern>,
    // pattern_regions: Vec<Region>,
    types: Vec<FuncSpecType>,
    // type_regions: Vec<Region>,
}

impl core::ops::Index<FuncSpecExprId> for FuncSpecIR {
    type Output = FuncSpecExpr;

    fn index(&self, index: FuncSpecExprId) -> &Self::Output {
        &self.exprs[index.0.index()]
    }
}

impl core::ops::Index<FuncSpecPatternId> for FuncSpecIR {
    type Output = FuncSpecPattern;

    fn index(&self, index: FuncSpecPatternId) -> &Self::Output {
        &self.patterns[index.0.index()]
    }
}

impl core::ops::Index<FuncSpecTypeId> for FuncSpecIR {
    type Output = FuncSpecType;

    fn index(&self, index: FuncSpecTypeId) -> &Self::Output {
        &self.types[index.0.index()]
    }
}
