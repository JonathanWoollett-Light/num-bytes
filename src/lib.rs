//! A crate to allow converting generic primitives into bytes.
//!
//! Defines `TryFromBytes` and `IntoBytes` traits and implements them on all numerical primitives.

use std::{array::TryFromSliceError, convert::TryInto, u8};

/// Defines a type can be converted from a byte slice.
/// ```
/// use num_bytes::TryFromBytes;
/// let a = [8,0,0,0];
/// let b = u32::try_from_le_bytes(&a).unwrap();
/// assert_eq!(b,8);
/// ```
pub trait TryFromBytes: Sized {
    fn try_from_le_bytes(bytes: &[u8]) -> Result<Self, TryFromSliceError>;
    fn try_from_be_bytes(bytes: &[u8]) -> Result<Self, TryFromSliceError>;
}
/// Defines a type can be converted into a byte vector.
/// ```
/// use num_bytes::IntoBytes;
/// let a = 8u32;
/// let b = a.into_le_bytes();
/// assert_eq!(b,[8,0,0,0]);
/// ```
pub trait IntoBytes: Sized {
    fn into_le_bytes(self) -> Vec<u8>;
    fn into_be_bytes(self) -> Vec<u8>;
}
macro_rules! impl_try_from_bytes {
    ($T: ident) => {
        impl TryFromBytes for $T {
            fn try_from_le_bytes(bytes: &[u8]) -> Result<Self, TryFromSliceError> {
                Ok(Self::from_le_bytes(bytes.try_into()?))
            }
            fn try_from_be_bytes(bytes: &[u8]) -> Result<Self, TryFromSliceError> {
                Ok(Self::from_be_bytes(bytes.try_into()?))
            }
        }
    };
}
macro_rules! impl_into_bytes {
    ($T: ident) => {
        impl IntoBytes for $T {
            fn into_le_bytes(self) -> Vec<u8> {
                Vec::from(self.to_le_bytes())
            }
            fn into_be_bytes(self) -> Vec<u8> {
                Vec::from(self.to_be_bytes())
            }
        }
    };
}

// Try from implementations.
impl_try_from_bytes!(i8);
impl_try_from_bytes!(i16);
impl_try_from_bytes!(i32);
impl_try_from_bytes!(i64);
impl_try_from_bytes!(u8);
impl_try_from_bytes!(u16);
impl_try_from_bytes!(u32);
impl_try_from_bytes!(u64);

impl_try_from_bytes!(f32);
impl_try_from_bytes!(f64);

impl_try_from_bytes!(isize);
impl_try_from_bytes!(usize);

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
