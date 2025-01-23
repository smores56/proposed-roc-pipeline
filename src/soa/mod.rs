pub mod either_index;
pub mod index;
pub mod slice;
pub mod slice2;
pub mod slice3;

pub use either_index::*;
pub use index::*;
pub use slice::{NonEmptySlice, PairSlice, Slice};
pub use slice2::Slice2;
pub use slice3::Slice3;

pub fn index_push_new<T>(vector: &mut Vec<T>, value: T) -> Index<T> {
    let index = Index::new(vector.len() as u32);

    vector.push(value);

    index
}

/// Extend a std::vec::Vec<T> and then return a slice to the new elements'
/// positions in the Vec.
///
/// This is not a method on soa::Slice because the `soa` is `no_std` by design.
/// Long-term, our arena-allocated vectors should have this as a method!
pub fn slice_extend_new<T>(vector: &mut Vec<T>, values: impl IntoIterator<Item = T>) -> Slice<T> {
    let start = vector.len() as u32;

    vector.extend(values);

    let end = vector.len() as u32;

    Slice::new(start, (end - start) as u16)
}
