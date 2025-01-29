pub mod expr;
pub mod layout;
pub mod stmt;

use std::collections::HashMap;

use expr::{LowerExpr, LowerExprId};
use layout::{LowerLayout, LowerLayoutId};
use stmt::{LowerStmt, LowerStmtId};

use crate::{base::symbol::Symbol, env::Env, soa::Slice, specialize_functions::FuncSpecIR};

// Convert a purely first-order program to a low level IR mainly comprising
// procedures and statements that, after adding refcounts, can be used directly for codegen.
//
// Design by Ayaz for this stage:
// https://github.com/roc-lang/rfcs/blob/ayaz/compile-with-lambda-sets/0102-compiling-lambda-sets.md#lower_ir
pub fn lower_ir(_func_spec_ir: &FuncSpecIR, _env: &mut Env) -> LowerIR {
    todo!()
}

pub struct LowerIR {
    procs: HashMap<Symbol, LowerProcedure>,
    exprs: Vec<LowerExpr>,
    layouts: Vec<LowerLayout>,
    stmts: Vec<LowerStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LowerProcedure {
    pub arguments: Slice<(Symbol, LowerLayout)>,
    pub body: LowerStmtId,
    pub return_layout: LowerLayoutId,
}

impl core::ops::Index<LowerExprId> for LowerIR {
    type Output = LowerExpr;

    fn index(&self, index: LowerExprId) -> &Self::Output {
        &self.exprs[index.0.index()]
    }
}

impl core::ops::Index<LowerLayoutId> for LowerIR {
    type Output = LowerLayout;

    fn index(&self, index: LowerLayoutId) -> &Self::Output {
        &self.layouts[index.0.index()]
    }
}

impl core::ops::Index<LowerStmtId> for LowerIR {
    type Output = LowerStmt;

    fn index(&self, index: LowerStmtId) -> &Self::Output {
        &self.stmts[index.0.index()]
    }
}
