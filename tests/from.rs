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

#[test]
fn from_isize() {
    #[cfg(target_pointer_width = "16")]
    let a = [8, 0];
    #[cfg(target_pointer_width = "32")]
    let a = [8, 0, 0, 0, 0];
    #[cfg(target_pointer_width = "64")]
    let a = [8, 0, 0, 0, 0, 0, 0, 0];

    let b = isize::from_le_bytes(a);
    assert_eq!(b, 8);
}
#[test]
fn from_usize() {
    #[cfg(target_pointer_width = "16")]
    let a = [8, 0];
    #[cfg(target_pointer_width = "32")]
    let a = [8, 0, 0, 0, 0];
    #[cfg(target_pointer_width = "64")]
    let a = [8, 0, 0, 0, 0, 0, 0, 0];

    let b = usize::from_le_bytes(a);
    assert_eq!(b, 8);
}
