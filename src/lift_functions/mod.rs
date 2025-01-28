pub mod expr;
pub mod pattern;
pub mod type_;

use expr::FuncLiftExpr;
use pattern::FuncLiftPattern;
use type_::FuncLiftType;

use crate::base::region::Region;

#[derive(Default)]
pub struct FuncLiftIR {
    exprs: Vec<FuncLiftExpr>,
    expr_regions: Vec<Region>,
    patterns: Vec<FuncLiftPattern>,
    // pattern_regions: Vec<Region>,
    types: Vec<FuncLiftType>,
    // type_regions: Vec<Region>,
}
