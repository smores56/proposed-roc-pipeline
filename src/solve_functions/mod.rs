pub mod expr;
pub mod pattern;
pub mod type_;

use expr::FunctionSolveExpr;
use pattern::FunctionSolvePattern;
use type_::FunctionSolveType;

use crate::base::region::Region;

#[derive(Default)]
pub struct FunctionSolveData {
    exprs: Vec<FunctionSolveExpr>,
    expr_regions: Vec<Region>,
    patterns: Vec<FunctionSolvePattern>,
    // pattern_regions: Vec<Region>,
    types: Vec<FunctionSolveType>,
    // type_regions: Vec<Region>,
}
