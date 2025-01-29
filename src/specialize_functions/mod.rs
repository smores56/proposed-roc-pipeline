pub mod expr;
pub mod pattern;
pub mod type_;

use expr::{FuncSpecExpr, FuncSpecExprId};
use pattern::{FuncSpecPattern, FuncSpecPatternId};
use type_::{FuncSpecType, FuncSpecTypeId};

use crate::{base::region::Region, env::Env, solve_functions::FuncSolveIR};

// TODO: explain what this stage should do and a bit of how
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
