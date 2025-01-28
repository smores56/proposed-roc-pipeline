use crate::{
    base::Primitive,
    soa::{Index, NonEmptySlice},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FuncSpecTypeId {
    index: Index<FuncSpecType>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FuncSpecType {
    Primitive(Primitive),
    Box(FuncSpecTypeId),
    List(FuncSpecTypeId),
    Struct(NonEmptySlice<FuncSpecTypeId>),
    TagUnion(NonEmptySlice<FuncSpecTypeId>),
}
