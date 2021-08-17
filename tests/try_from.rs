use num_bytes::TryFromBytes;
#[test]
fn try_from_u8() {
    let a = [8];
    let b = u8::try_from_le_bytes(&a).unwrap();
    assert_eq!(b, 8);
}
#[test]
fn try_from_u16() {
    let a = [8, 0];
    let b = u16::try_from_le_bytes(&a).unwrap();
    assert_eq!(b, 8);
}
#[test]
fn try_from_u32() {
    let a = [8, 0, 0, 0];
    let b = u32::try_from_le_bytes(&a).unwrap();
    assert_eq!(b, 8);
}
#[test]
fn try_from_u64() {
    let a = [8, 0, 0, 0, 0, 0, 0, 0];
    let b = u64::try_from_le_bytes(&a).unwrap();
    assert_eq!(b, 8);
}
#[test]
fn try_from_i8() {
    let a = [8];
    let b = i8::try_from_le_bytes(&a).unwrap();
    assert_eq!(b, 8);
}
#[test]
fn try_from_i16() {
    let a = [8, 0];
    let b = i16::try_from_le_bytes(&a).unwrap();
    assert_eq!(b, 8);
}
#[test]
fn try_from_i32() {
    let a = [8, 0, 0, 0];
    let b = i32::try_from_le_bytes(&a).unwrap();
    assert_eq!(b, 8);
}
#[test]
fn try_from_i64() {
    let a = [8, 0, 0, 0, 0, 0, 0, 0];
    let b = i64::try_from_le_bytes(&a).unwrap();
    assert_eq!(b, 8);
}
