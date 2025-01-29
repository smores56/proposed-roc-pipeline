use crate::{
    base::symbol::Symbol,
    soa::{Index, Slice},
};

use super::{
    expr::LowerExprId,
    layout::{LowerLayoutId, TagIdIntType},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LowerStmtId(pub(crate) Index<RefCountStmt>);

#[derive(Debug, Clone, PartialEq)]
pub enum RefCountStmt {
    Let {
        symbol: Symbol,
        expr: LowerExprId,
        layout: LowerExprId,
        continuation: LowerStmtId,
    },
    Switch {
        /// This *must* stand for an integer, because Switch potentially compiles to a jump table.
        cond_symbol: Symbol,
        // TODO: can we make this layout a number type?
        cond_layout: LowerLayoutId,
        /// The u64 in the tuple will be compared directly to the condition Expr.
        /// If they are equal, this branch will be taken.
        branches: Slice<(u64, LowerBranchInfo, LowerStmtId)>,
        /// If no other branches pass, this default branch will be taken.
        default_branch: (LowerBranchInfo, LowerStmtId),
        /// Each branch must return a value of this type.
        ret_layout: LowerLayoutId,
    },
    Ret(Symbol),
    /// a join point `join f <params> = <continuation> in remainder`
    Join {
        id: JoinPointId,
        parameters: Slice<LowerParam>,
        /// body of the join point
        /// what happens after _jumping to_ the join point
        body: LowerStmtId,
        /// what happens after _defining_ the join point
        remainder: LowerStmtId,
    },
    Jump(JoinPointId, Slice<Symbol>),
    Crash(Symbol, CrashTag),
}

#[derive(Clone, Debug, PartialEq, Copy, Eq, Hash)]
pub struct JoinPointId(pub Symbol);

/// Source of crash, and its runtime representation to roc_panic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum CrashTag {
    /// The crash is due to Roc, either via a builtin or type error.
    Roc = 0,
    /// The crash is user-defined.
    User = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LowerParam {
    pub symbol: Symbol,
    pub layout: LowerLayoutId,
}

/// in the block below, symbol `scrutinee` is assumed be be of shape `tag_id`
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LowerBranchInfo {
    None,
    Constructor {
        scrutinee: Symbol,
        layout: LowerLayoutId,
        tag_id: TagIdIntType,
    },
    List {
        scrutinee: Symbol,
        len: u64,
    },
    Unique {
        scrutinee: Symbol,
        unique: bool,
    },
}
