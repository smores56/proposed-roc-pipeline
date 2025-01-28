use crate::{
    base::Primitive,
    soa::{Index, NonEmptySlice, Slice},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FuncSolveTypeId {
    index: Index<FuncSolveType>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FuncSolveType {
    Primitive(Primitive),
    Box(FuncSolveTypeId),
    List(FuncSolveTypeId),
    Struct(NonEmptySlice<FuncSolveTypeId>),
    TagUnion(NonEmptySlice<FuncSolveTypeId>),
    // TODO: can this go somewhere outside of the main function union?
    FunctionPack {
        /// zero fields means no captures
        opt_fields: Slice<FuncSolveTypeId>,
    },
}
