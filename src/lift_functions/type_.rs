use crate::{
    base::Primitive,
    soa::{Index, NonEmptySlice, Slice},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FunctionLiftTypeId {
    index: Index<FunctionLiftType>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FunctionLiftType {
    Primitive(Primitive),
    Box(FunctionLiftTypeId),
    List(FunctionLiftTypeId),
    Struct(NonEmptySlice<FunctionLiftTypeId>),
    TagUnion(NonEmptySlice<FunctionLiftTypeId>),
    // TODO: can this go somewhere outside of the main function union?
    FunctionPack {
        /// zero fields means no captures
        opt_fields: Slice<FunctionLiftTypeId>,
    },
}
