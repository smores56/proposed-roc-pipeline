use crate::{
    base::Primitive,
    soa::{Index, NonEmptySlice},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FunctionSpecializeTypeId {
    index: Index<FunctionSpecializeType>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FunctionSpecializeType {
    Primitive(Primitive),
    Box(FunctionSpecializeTypeId),
    List(FunctionSpecializeTypeId),
    Struct(NonEmptySlice<FunctionSpecializeTypeId>),
    TagUnion(NonEmptySlice<FunctionSpecializeTypeId>),
}
