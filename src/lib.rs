//! A crate to allow converting generic primitives into bytes.
//!
//! Defines `TryFromBytes` and `IntoBytes` traits and implements them on all numerical primitives.

use std::{array::TryFromSliceError, convert::TryInto, u8};

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
pub trait FromBytes<const T: usize>: Sized {
    fn from_le_bytes(bytes: [u8; T]) -> Self;
    fn from_be_bytes(bytes: [u8; T]) -> Self;
}

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
/// `isize` and `usize` implementations compile to implement `FromBytes<2>`, `FromBytes<4>` or `FromBytes<8>` depending on your system.
///
/// It simply says `FromBytes<8>` in documentation because the machine which has compiled the documentation is 64 bit.
pub trait IntoBytes<const T: usize>: Sized {
    fn into_le_bytes(self) -> [u8; T];
    fn into_be_bytes(self) -> [u8; T];
}

macro_rules! impl_from_bytes {
    ($T: ty, $N:tt) => {
        impl FromBytes<$N> for $T {
            fn from_le_bytes(bytes: [u8; $N]) -> Self {
                Self::from_le_bytes(bytes)
            }
            fn from_be_bytes(bytes: [u8; $N]) -> Self {
                Self::from_be_bytes(bytes)
            }
        }
    };
}
macro_rules! impl_try_from_bytes {
    ($T: ty) => {
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
    ($T: ty, $N:tt) => {
        impl IntoBytes<$N> for $T {
            fn into_le_bytes(self) -> [u8; $N] {
                self.to_le_bytes()
            }
            fn into_be_bytes(self) -> [u8; $N] {
                self.to_be_bytes()
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
impl_into_bytes!(i8, 1);
impl_into_bytes!(i16, 2);
impl_into_bytes!(i32, 4);
impl_into_bytes!(i64, 8);
impl_into_bytes!(u8, 1);
impl_into_bytes!(u16, 2);
impl_into_bytes!(u32, 4);
impl_into_bytes!(u64, 8);

impl_into_bytes!(f32, 4);
impl_into_bytes!(f64, 8);

#[cfg(target_pointer_width = "16")]
impl_into_bytes!(isize, 2);
#[cfg(target_pointer_width = "32")]
impl_into_bytes!(isize, 4);
#[cfg(target_pointer_width = "64")]
impl_into_bytes!(isize, 8);

#[cfg(target_pointer_width = "16")]
impl_into_bytes!(usize, 2);
#[cfg(target_pointer_width = "32")]
impl_into_bytes!(usize, 4);
#[cfg(target_pointer_width = "64")]
impl_into_bytes!(usize, 8);

// From implementations
impl_from_bytes!(i8, 1);
impl_from_bytes!(i16, 2);
impl_from_bytes!(i32, 4);
impl_from_bytes!(i64, 8);
impl_from_bytes!(u8, 1);
impl_from_bytes!(u16, 2);
impl_from_bytes!(u32, 4);
impl_from_bytes!(u64, 8);

impl_from_bytes!(f32, 4);
impl_from_bytes!(f64, 8);

#[cfg(target_pointer_width = "16")]
impl_from_bytes!(isize, 2);
#[cfg(target_pointer_width = "32")]
impl_from_bytes!(isize, 4);
#[cfg(target_pointer_width = "64")]
impl_from_bytes!(isize, 8);

#[cfg(target_pointer_width = "16")]
impl_from_bytes!(usize, 2);
#[cfg(target_pointer_width = "32")]
impl_from_bytes!(usize, 4);
#[cfg(target_pointer_width = "64")]
impl_from_bytes!(usize, 8);
