pub mod foreign_symbol;
pub mod ident;
pub mod module;
pub mod problem;
pub mod region;
pub mod string_store;
pub mod symbol;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Primitive {
    Crash,
    Str,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    U128,
    I128,
    F32,
    F64,
    Dec,
    Bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    F32(f32),
    F64(f64),
    // TODO: should this be a u128 instead?
    Dec(f64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recursive {
    NotRecursive = 0,
    Recursive = 1,
    TailRecursive = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LowLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeVar(u32);
