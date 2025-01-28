pub mod expr;
pub mod pattern;
pub mod type_;

use expr::FuncSolveExpr;
use pattern::FuncSolvePattern;
use type_::FuncSolveType;

use crate::base::region::Region;

#[derive(Default)]
pub struct FuncSolveIR {
    exprs: Vec<FuncSolveExpr>,
    expr_regions: Vec<Region>,
    patterns: Vec<FuncSolvePattern>,
    // pattern_regions: Vec<Region>,
    types: Vec<FuncSolveType>,
    // type_regions: Vec<Region>,
}
