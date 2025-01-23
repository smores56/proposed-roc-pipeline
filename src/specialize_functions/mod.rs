pub mod expr;
pub mod pattern;
pub mod type_;

use expr::FunctionSpecializeExpr;
use pattern::FunctionSpecializePattern;
use type_::FunctionSpecializeType;

use crate::base::region::Region;

#[derive(Default)]
pub struct FunctionSpecializeData {
    exprs: Vec<FunctionSpecializeExpr>,
    expr_regions: Vec<Region>,
    patterns: Vec<FunctionSpecializePattern>,
    // pattern_regions: Vec<Region>,
    types: Vec<FunctionSpecializeType>,
    // type_regions: Vec<Region>,
}
