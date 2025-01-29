use crate::{
    base::Primitive,
    soa::{Index, NonEmptySlice},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeSpecTypeId(pub(crate) Index<TypeSpecType>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeSpecType {
    Primitive(Primitive),
    Box(TypeSpecTypeId),
    List(TypeSpecTypeId),
    /// Records, tuples, and tag union payloads all end up here. (Empty ones are handled separate.)
    ///
    /// Slice of field types, ordered alphabetically by field name (or by tuple elem index).
    /// The strings for the field names (or tuple indices) are stored out of band in DebugInfo,
    /// which references this TypeSpecTypeId. A later compiler phase will sort these by alignment
    /// (this phase is not aware of alignment), and will sort the DebugInfo structs accordingly.
    Struct(NonEmptySlice<TypeSpecTypeId>),

    /// Slice of payloads, where each payload is a struct or Unit. (Empty tag unions become Unit.)
    ///
    /// These have already been sorted alphabetically by tag name, and the tag name strings
    /// have already been recorded out of band in DebugInfo.
    TagUnion(NonEmptySlice<TypeSpecTypeId>),

    /// A function that has a return value and 0 or more arguments.
    /// To avoid wasting memory, we store the return value first in the nonempty slice,
    /// and then the arguments after it.
    Func {
        ret_then_args: NonEmptySlice<TypeSpecTypeId>,
    },
}
