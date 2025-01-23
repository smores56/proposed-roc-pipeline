use crate::{
    base::Primitive,
    soa::{Index, NonEmptySlice, Slice},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FunctionSolveTypeId {
    index: Index<FunctionSolveType>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FunctionSolveType {
    Primitive(Primitive),
    Box(FunctionSolveTypeId),
    List(FunctionSolveTypeId),
    Struct(NonEmptySlice<FunctionSolveTypeId>),
    TagUnion(NonEmptySlice<FunctionSolveTypeId>),
    // TODO: can this go somewhere outside of the main function union?
    FunctionPack {
        /// zero fields means no captures
        opt_fields: Slice<FunctionSolveTypeId>,
    },
}
