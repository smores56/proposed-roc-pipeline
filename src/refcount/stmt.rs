use crate::{base::symbol::Symbol, soa::Index};

pub type TagIdIntType = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoweredStmtId {
    index: Index<LoweredStmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoweredStmt {
    Let {
        symbol: Symbol,
        expr: LoweredExpr,
        layout: LoweredTypeId,
        continuation: LoweredStmtId,
    },
    Switch {
        /// This *must* stand for an integer, because Switch potentially compiles to a jump table.
        cond_symbol: Symbol,
        cond_layout: InLayout<'a>,
        /// The u64 in the tuple will be compared directly to the condition Expr.
        /// If they are equal, this branch will be taken.
        branches: &'a [(u64, BranchInfo<'a>, Stmt<'a>)],
        /// If no other branches pass, this default branch will be taken.
        default_branch: (BranchInfo<'a>, &'a Stmt<'a>),
        /// Each branch must return a value of this type.
        ret_layout: InLayout<'a>,
    },
    Ret(Symbol),
    Refcounting(ModifyRc, LoweredStmtId),
    Expect {
        condition: Symbol,
        region: Region,
        lookups: &'a [Symbol],
        variables: &'a [LookupType],
        /// what happens after the expect
        remainder: &'a Stmt<'a>,
    },
    Dbg {
        /// The location this dbg is in source as a printable string.
        source_location: &'a str,
        /// The source code of the expression being debugged.
        source: &'a str,
        /// The expression we're displaying
        symbol: Symbol,
        /// The specialized variable of the expression
        variable: Variable,
        /// What happens after the dbg
        remainder: &'a Stmt<'a>,
    },
    /// a join point `join f <params> = <continuation> in remainder`
    Join {
        id: JoinPointId,
        parameters: &'a [Param<'a>],
        /// body of the join point
        /// what happens after _jumping to_ the join point
        body: &'a Stmt<'a>,
        /// what happens after _defining_ the join point
        remainder: &'a Stmt<'a>,
    },
    Jump(JoinPointId, &'a [Symbol]),
    Crash(Symbol, CrashTag),
}

/// in the block below, symbol `scrutinee` is assumed be be of shape `tag_id`
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BranchInfo<'a> {
    None,
    Constructor {
        scrutinee: Symbol,
        layout: InLayout<'a>,
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
