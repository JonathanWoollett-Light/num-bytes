//! A crate to allow converting generic from/into bytes.
//!
//! Defines `FromBytes` and `IntoBytes` traits.
//! 
//! Implements them on all numerical primitives.

/// Defines a type can be converted from a byte array.
/// ```
/// use num_bytes::FromBytes;
/// let a = [8,0,0,0];
/// let b = u32::from_le_bytes(a);
/// assert_eq!(b,8);
/// ```
pub trait FromBytes<const T: usize> {
    fn from_le_bytes(bytes: [u8; T]) -> Self;
    fn from_be_bytes(bytes: [u8; T]) -> Self;
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
macro_rules! impl_from_bytes {
    ($T: ty, $N: tt) => {
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
#[cfg(target_pointer_width = "16")]
impl_from_bytes!(usize, 2);

#[cfg(target_pointer_width = "32")]
impl_from_bytes!(isize, 4);
#[cfg(target_pointer_width = "32")]
impl_from_bytes!(usize, 4);

#[cfg(target_pointer_width = "64")]
impl_from_bytes!(isize, 8);
#[cfg(target_pointer_width = "64")]
impl_from_bytes!(usize, 8);

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