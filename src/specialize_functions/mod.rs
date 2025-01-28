pub mod expr;
pub mod pattern;
pub mod type_;

use expr::FuncSpecExpr;
use pattern::FuncSpecPattern;
use type_::FuncSpecType;

use crate::base::region::Region;

#[derive(Default)]
pub struct FuncSpecIR {
    exprs: Vec<FuncSpecExpr>,
    expr_regions: Vec<Region>,
    patterns: Vec<FuncSpecPattern>,
    // pattern_regions: Vec<Region>,
    types: Vec<FuncSpecType>,
    // type_regions: Vec<Region>,
}
