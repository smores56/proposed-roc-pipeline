pub mod expr;
pub mod pattern;
pub mod type_;

use expr::{FuncSolveExpr, FuncSolveExprId};
use pattern::{FuncSolvePattern, FuncSolvePatternId};
use type_::{FuncSolveType, FuncSolveTypeId};

use crate::{base::region::Region, env::Env, lift_functions::FuncLiftIR};

// Make higher-order function arguments to top-level functions "generic" over the values they capture.
//
// This one is a little tricky, so please ready Ayaz' description below for details:
// https://github.com/roc-lang/rfcs/blob/ayaz/compile-with-lambda-sets/0102-compiling-lambda-sets.md#function_solve
pub fn solve_functions(_func_lift_ir: &FuncLiftIR, _env: &mut Env) -> FuncSolveIR {
    todo!()
}

#[derive(Default)]
pub struct FuncSolveIR {
    exprs: Vec<FuncSolveExpr>,
    expr_regions: Vec<Region>,
    patterns: Vec<FuncSolvePattern>,
    // pattern_regions: Vec<Region>,
    types: Vec<FuncSolveType>,
    // type_regions: Vec<Region>,
}

impl core::ops::Index<FuncSolveExprId> for FuncSolveIR {
    type Output = FuncSolveExpr;

    fn index(&self, index: FuncSolveExprId) -> &Self::Output {
        &self.exprs[index.0.index()]
    }
}

impl core::ops::Index<FuncSolvePatternId> for FuncSolveIR {
    type Output = FuncSolvePattern;

    fn index(&self, index: FuncSolvePatternId) -> &Self::Output {
        &self.patterns[index.0.index()]
    }
}

impl core::ops::Index<FuncSolveTypeId> for FuncSolveIR {
    type Output = FuncSolveType;

    fn index(&self, index: FuncSolveTypeId) -> &Self::Output {
        &self.types[index.0.index()]
    }
}
