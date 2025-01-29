use crate::{
    base::Primitive,
    soa::{Index, NonEmptySlice, Slice},
};

// TODO: is this necessary?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagIdIntType(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LowerLayoutId(pub(crate) Index<LowerLayout>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LowerLayout {
    Primitive(Primitive),
    Box(LowerLayoutId),
    List(LowerLayoutId),
    Struct(NonEmptySlice<LowerLayoutId>),
    TagUnion(NonEmptySlice<LowerLayoutId>),
    // probably necessary for returning empty structs, but would be good to remove this if that's not the case
    Unit,
}

// TODO: should these use `NonEmptySlice`s?
//
// Copied (and adapted) from:
// https://github.com/roc-lang/roc/blob/689c58f35e0a39ca59feba549f7fcf375562a7a6/crates/compiler/mono/src/layout.rs#L733
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnionLayout {
    /// A non-recursive tag union
    /// e.g. `Result a e : [Ok a, Err e]`
    NonRecursive(Slice<Slice<LowerLayout>>),
    /// A recursive tag union (general case)
    /// e.g. `Expr : [Sym Str, Add Expr Expr]`
    Recursive(Slice<Slice<LowerLayout>>),
    /// A recursive tag union with just one constructor
    /// Optimization: No need to store a tag ID (the payload is "unwrapped")
    /// e.g. `RoseTree a : [Tree a (List (RoseTree a))]`
    NonNullableUnwrapped(Slice<LowerLayout>),
    /// A recursive tag union that has an empty variant
    /// Optimization: Represent the empty variant as null pointer => no memory usage & fast comparison
    /// It has more than one other variant, so they need tag IDs (payloads are "wrapped")
    /// e.g. `FingerTree a : [Empty, Single a, More (Some a) (FingerTree (Tuple a)) (Some a)]`
    /// see also: https://youtu.be/ip92VMpf_-A?t=164
    ///
    /// nullable_id refers to the index of the tag that is represented at runtime as NULL.
    /// For example, in `FingerTree a : [Empty, Single a, More (Some a) (FingerTree (Tuple a)) (Some a)]`,
    /// the ids would be Empty = 0, More = 1, Single = 2, because that's how those tags are
    /// ordered alphabetically. Since the Empty tag will be represented at runtime as NULL,
    /// and since Empty's tag id is 0, here nullable_id would be 0.
    NullableWrapped {
        nullable_id: u16,
        other_tags: Slice<Slice<LowerLayout>>,
    },
    /// A recursive tag union with only two variants, where one is empty.
    /// Optimizations: Use null for the empty variant AND don't store a tag ID for the other variant.
    /// e.g. `ConsList a : [Nil, Cons a (ConsList a)]`
    ///
    /// nullable_id is a bool because it's only ever 0 or 1, but (as with the NullableWrapped
    /// variant), it reprsents the index of the tag that will be represented at runtime as NULL.
    ///
    /// So for example, in `ConsList a : [Nil, Cons a (ConsList a)]`, Nil is tag id 1 and
    /// Cons is tag id 0 because Nil comes alphabetically after Cons. Here, Nil will be
    /// represented as NULL at runtime, so nullable_id is 1 - which is to say, `true`, because
    /// `(1 as bool)` is `true`.
    NullableUnwrapped {
        nullable_id: bool,
        other_fields: Slice<LowerLayout>,
    },
}
