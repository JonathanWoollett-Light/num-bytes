//! A crate to allow converting generic primitives into bytes.
//!
//! Defines `FromBytes` and `IntoBytes` traits for converting generics into bytes.
//!
//! Also implements them on all numerical primitives.

#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

/// Defines a type can be converted from a bytes array.
/// ```
/// use num_bytes::FromBytes;
/// let a = [8,0,0,0];
/// let b = i32::from_le_bytes(a);
/// assert_eq!(b, 8);
/// ```
/// `isize` and `usize` implementations compile to implement `FromBytes<2>`, `FromBytes<4>` or `FromBytes<8>` depending on your system.
///
/// It simply says `FromBytes<8>` in documentation because the machine which has compiled the documentation is 64 bit.
pub trait FromBytes: Sized {
    fn from_le_bytes(bytes: [u8; std::mem::size_of::<Self>()]) -> Self;
    fn from_be_bytes(bytes: [u8; std::mem::size_of::<Self>()]) -> Self;
}

/// Defines a type can be converted into a byte vector.
/// ```
/// use num_bytes::IntoBytes;
/// let a = 8u32;
/// let b = a.into_le_bytes();
/// assert_eq!(b,[8,0,0,0]);
/// ```
/// `isize` and `usize` implementations compile to implement `FromBytes<2>`, `FromBytes<4>` or `FromBytes<8>` depending on your system.
///
/// It simply says `FromBytes<8>` in documentation because the machine which has compiled the documentation is 64 bit.
pub trait IntoBytes: Sized {
    type Output;
    fn into_le_bytes(self) -> [u8; std::mem::size_of::<Self>()];
    fn into_be_bytes(self) -> [u8; std::mem::size_of::<Self>()];
}

macro_rules! impl_from_bytes {
    ($T: ty) => {
        impl FromBytes for $T {
            fn from_le_bytes(bytes: [u8; std::mem::size_of::<$T>()]) -> Self {
                Self::from_le_bytes(bytes)
            }
            fn from_be_bytes(bytes: [u8; std::mem::size_of::<$T>()]) -> Self {
                Self::from_be_bytes(bytes)
            }
        }
    };
}
macro_rules! impl_into_bytes {
    ($T: ty) => {
        impl IntoBytes for $T where [u8;std::mem::size_of::<$T>()]: Sized {
            type Output = [u8;std::mem::size_of::<$T>()];
            fn into_le_bytes(self) -> Self::Output {
                self.to_le_bytes()
            }
            fn into_be_bytes(self) -> Self::Output {
                self.to_be_bytes()
            }
        }
    };
}

// Into implementations.
impl_into_bytes!(i8);
impl_into_bytes!(i16);
impl_into_bytes!(i32);
impl_into_bytes!(i64);
impl_into_bytes!(u8);
impl_into_bytes!(u16);
impl_into_bytes!(u32);
impl_into_bytes!(u64);

impl_into_bytes!(f32);
impl_into_bytes!(f64);

impl_into_bytes!(isize);

impl_into_bytes!(usize);

// From implementations
impl_from_bytes!(i8);
impl_from_bytes!(i16);
impl_from_bytes!(i32);
impl_from_bytes!(i64);
impl_from_bytes!(u8);
impl_from_bytes!(u16);
impl_from_bytes!(u32);
impl_from_bytes!(u64);

impl_from_bytes!(f32);
impl_from_bytes!(f64);

impl_from_bytes!(isize);
impl_from_bytes!(usize);
