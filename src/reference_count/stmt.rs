use crate::{
    base::symbol::Symbol,
    lower_ir::{
        expr::LowerExprId,
        layout::LowerLayoutId,
        stmt::{CrashTag, JoinPointId, LowerBranchInfo, LowerParam},
    },
    soa::{Index, Slice},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefCountStmtId(pub(crate) Index<RefCountStmt>);

// TODO: should `LowerExpr` or `RefCountStmt` hold the CompilerBug(ReferenceCountProblem)?

// the same as `LowerStmt`, but with an added `RefCount` variant
#[derive(Debug, Clone, PartialEq)]
pub enum RefCountStmt {
    Let {
        symbol: Symbol,
        expr: LowerExprId,
        layout: LowerExprId,
        continuation: RefCountStmtId,
    },
    Switch {
        /// This *must* stand for an integer, because Switch potentially compiles to a jump table.
        cond_symbol: Symbol,
        // TODO: can we make this a number type?
        cond_layout: LowerLayoutId,
        /// The u64 in the tuple will be compared directly to the condition Expr.
        /// If they are equal, this branch will be taken.
        branches: Slice<(u64, LowerBranchInfo, RefCountStmtId)>,
        /// If no other branches pass, this default branch will be taken.
        default_branch: (LowerBranchInfo, RefCountStmtId),
        /// Each branch must return a value of this type.
        ret_layout: LowerLayoutId,
    },
    Ret(Symbol),
    RefCount {
        symbol: Symbol,
        change: ModifyRefCount,
    },
    /// a join point `join f <params> = <continuation> in remainder`
    Join {
        id: JoinPointId,
        parameters: Slice<LowerParam>,
        /// body of the join point
        /// what happens after _jumping to_ the join point
        body: RefCountStmtId,
        /// what happens after _defining_ the join point
        remainder: RefCountStmtId,
    },
    Jump(JoinPointId, Slice<Symbol>),
    Crash(Symbol, CrashTag),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModifyRefCount {
    /// Increment a reference count
    Inc(Symbol, u64),
    /// Decrement a reference count
    Dec(Symbol),
    /// A DecRef is a non-recursive reference count decrement
    /// e.g. If we Dec a list of lists, then if the reference count of the outer list is one,
    /// a Dec will recursively decrement all elements, then free the memory of the outer list.
    /// A DecRef would just free the outer list.
    /// That is dangerous because you may not free the elements, but in our Zig builtins,
    /// sometimes we know we already dealt with the elements (e.g. by copying them all over
    /// to a new list) and so we can just do a DecRef, which is much cheaper in such a case.
    DecRef(Symbol),
    /// Unconditionally deallocate the memory. For tag union that do pointer tagging (store the tag
    /// id in the pointer) the backend has to clear the tag id!
    Free(Symbol),
}
