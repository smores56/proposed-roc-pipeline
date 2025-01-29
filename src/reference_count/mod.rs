use std::collections::HashMap;

use stmt::{RefCountStmt, RefCountStmtId};

use crate::{
    base::{symbol::Symbol, Primitive},
    env::Env,
    lower_ir::{
        expr::{LowerExpr, LowerExprId},
        layout::{LowerLayout, LowerLayoutId},
        LowerIR,
    },
    soa::Slice,
};

pub mod stmt;

// Add reference counting operations to lowered IR to ensure correct and efficient
// cleanup of memory automatically.
//
// There's no explanation of how this should work from Ayaz, this should be just simpler
// version of what's implemented in the existing `mono` code in these two modules:
// https://github.com/roc-lang/roc/blob/689c58f35e0a39ca59feba549f7fcf375562a7a6/crates/compiler/mono/src/borrow.rs#L1
// https://github.com/roc-lang/roc/blob/689c58f35e0a39ca59feba549f7fcf375562a7a6/crates/compiler/mono/src/inc_dec.rs#L1
pub fn reference_count(_lower_ir: &LowerIR, _env: &mut Env) -> RefCountIR {
    todo!()
}

pub struct RefCountIR {
    procs: HashMap<Symbol, RefCountProcedure>,
    exprs: Vec<LowerExpr>,
    layouts: Vec<LowerLayout>,
    stmts: Vec<RefCountStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefCountProcedure {
    pub arguments: Slice<(Symbol, LowerLayout)>,
    pub body: RefCountStmtId,
    pub return_layout: LowerLayoutId,
}

impl core::ops::Index<LowerExprId> for RefCountIR {
    type Output = LowerExpr;

    fn index(&self, index: LowerExprId) -> &Self::Output {
        &self.exprs[index.0.index()]
    }
}

impl core::ops::Index<LowerLayoutId> for RefCountIR {
    type Output = LowerLayout;

    fn index(&self, index: LowerLayoutId) -> &Self::Output {
        &self.layouts[index.0.index()]
    }
}

impl core::ops::Index<RefCountStmtId> for RefCountIR {
    type Output = RefCountStmt;

    fn index(&self, index: RefCountStmtId) -> &Self::Output {
        &self.stmts[index.0.index()]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ownership {
    Owned,
    Borrowed,
}

fn layout_to_ownership<'a>(layout_id: LowerLayoutId, lower_ir: &LowerIR) -> Ownership {
    match lower_ir[layout_id] {
        LowerLayout::List(_) | LowerLayout::Primitive(Primitive::Str) => Ownership::Borrowed,
        _ => Ownership::Owned,
    }
}

// IDEA: use the mono2 strategy of having a BorrowSignatureCache and calculate said BorrowSignature when needed.
// This avoids the worry that we won't have calculated a borrow signature for something while still avoiding unnecessary work.

// A bitmask approach to saving whether a series of values are owned or borrowed.
//
// https://github.com/roc-lang/roc/blob/689c58f35e0a39ca59feba549f7fcf375562a7a6/crates/compiler/mono/src/borrow.rs#L15
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct BorrowSignature(u64);
