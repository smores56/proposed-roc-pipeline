use expr::{FuncLiftExpr, FuncLiftExprId};
use pattern::{FuncLiftPattern, FuncLiftPatternId};
use type_::{FuncLiftType, FuncLiftTypeId};

use crate::{base::region::Region, env::Env, specialize_types::TypeSpecIR};

pub mod expr;
pub mod pattern;
pub mod type_;

// Lift all nested functions to the top level. We do this by finding all values
// captured by a function (now here instead of during canonicalization!) and putting them
// into a struct. We then make a new top-level function that we call with a
// FuncLiftExpr::FunctionPack, which takes said struct and the newly-lifted function's symbol.
//
// This should be able to take a lot of work from the canonicalization code's capture tracking:
// https://github.com/roc-lang/roc/blob/689c58f35e0a39ca59feba549f7fcf375562a7a6/crates/compiler/can/src/module.rs#L633
//
// Design by Ayaz for this stage:
// https://github.com/roc-lang/rfcs/blob/ayaz/compile-with-lambda-sets/0102-compiling-lambda-sets.md#function_lift
pub fn lift_functions(_type_spec_ir: &TypeSpecIR, _env: &mut Env) -> FuncLiftIR {
    todo!()
}

#[derive(Default)]
pub struct FuncLiftIR {
    exprs: Vec<FuncLiftExpr>,
    expr_regions: Vec<Region>,
    patterns: Vec<FuncLiftPattern>,
    // pattern_regions: Vec<Region>,
    types: Vec<FuncLiftType>,
    // type_regions: Vec<Region>,
}

impl core::ops::Index<FuncLiftExprId> for FuncLiftIR {
    type Output = FuncLiftExpr;

    fn index(&self, index: FuncLiftExprId) -> &Self::Output {
        &self.exprs[index.0.index()]
    }
}

impl core::ops::Index<FuncLiftPatternId> for FuncLiftIR {
    type Output = FuncLiftPattern;

    fn index(&self, index: FuncLiftPatternId) -> &Self::Output {
        &self.patterns[index.0.index()]
    }
}

impl core::ops::Index<FuncLiftTypeId> for FuncLiftIR {
    type Output = FuncLiftType;

    fn index(&self, index: FuncLiftTypeId) -> &Self::Output {
        &self.types[index.0.index()]
    }
}
