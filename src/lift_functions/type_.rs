use crate::{
    base::Primitive,
    soa::{Index, NonEmptySlice, Slice},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FuncLiftTypeId(pub(crate) Index<FuncLiftType>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FuncLiftType {
    Primitive(Primitive),
    Box(FuncLiftTypeId),
    List(FuncLiftTypeId),
    Struct(NonEmptySlice<FuncLiftTypeId>),
    TagUnion(NonEmptySlice<FuncLiftTypeId>),
    FunctionPack {
        /// zero fields means no captures
        opt_fields: Slice<FuncLiftTypeId>,
    },
}
