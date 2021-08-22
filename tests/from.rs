use num_bytes::FromBytes;

#[test]
fn generic() {
    let a = [8,0];
    let b: i16 = from_generic(a);
    assert_eq!(b,8);

    let a = [8,0,0,0];
    let b: i32 = from_generic(a);
    assert_eq!(b,8);

    fn from_generic<T: FromBytes<N>, const N: usize>(x: [u8;N]) -> T {
        T::from_le_bytes(x)
    }
}

#[test]
fn from_u8() {
    let a = [8];
    let b = u8::from_le_bytes(a);
    assert_eq!(b, 8);
}
#[test]
fn from_u16() {
    let a = [8, 0];
    let b = u16::from_le_bytes(a);
    assert_eq!(b, 8);
}
#[test]
fn from_u32() {
    let a = [8, 0, 0, 0];
    let b = u32::from_le_bytes(a);
    assert_eq!(b, 8);
}
#[test]
fn from_u64() {
    let a = [8, 0, 0, 0, 0, 0, 0, 0];
    let b = u64::from_le_bytes(a);
    assert_eq!(b, 8);
}
#[test]
fn from_i8() {
    let a = [8];
    let b = i8::from_le_bytes(a);
    assert_eq!(b, 8);
}
#[test]
fn from_i16() {
    let a = [8, 0];
    let b = i16::from_le_bytes(a);
    assert_eq!(b, 8);
}
#[test]
fn from_i32() {
    let a = [8, 0, 0, 0];
    let b = i32::from_le_bytes(a);
    assert_eq!(b, 8);
}
#[test]
fn from_i64() {
    let a = [8, 0, 0, 0, 0, 0, 0, 0];
    let b = i64::from_le_bytes(a);
    assert_eq!(b, 8);
}

#[cfg(target_pointer_width = "16")]
#[test]
fn from_usize() {
    let a = [8, 0];
    let b = usize::from_le_bytes(a);
    assert_eq!(b, 8);
}
#[cfg(target_pointer_width = "16")]
#[test]
fn from_isize() {
    let a = [8, 0];
    let b = isize::from_le_bytes(a);
    assert_eq!(b, 8);
}
#[cfg(target_pointer_width = "32")]
#[test]
fn from_usize() {
    let a = [8, 0, 0, 0];
    let b = usize::from_le_bytes(a);
    assert_eq!(b, 8);
}
#[cfg(target_pointer_width = "32")]
#[test]
fn from_isize() {
    let a = [8, 0, 0, 0];
    let b = isize::from_le_bytes(a);
    assert_eq!(b, 8);
}

#[cfg(target_pointer_width = "64")]
#[test]
fn from_usize() {
    let a = [8, 0, 0, 0, 0, 0, 0, 0];
    let b = usize::from_le_bytes(a);
    assert_eq!(b, 8);
}
#[cfg(target_pointer_width = "64")]
#[test]
fn from_isize() {
    let a = [8, 0, 0, 0, 0, 0, 0, 0];
    let b = isize::from_le_bytes(a);
    assert_eq!(b, 8);
}