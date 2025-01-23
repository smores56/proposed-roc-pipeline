pub mod expr;
pub mod pattern;
pub mod type_;

use expr::FunctionLiftExpr;
use pattern::FunctionLiftPattern;
use type_::FunctionLiftType;

use crate::base::region::Region;

#[derive(Default)]
pub struct FunctionLiftData {
    exprs: Vec<FunctionLiftExpr>,
    expr_regions: Vec<Region>,
    patterns: Vec<FunctionLiftPattern>,
    // pattern_regions: Vec<Region>,
    types: Vec<FunctionLiftType>,
    // type_regions: Vec<Region>,
}
