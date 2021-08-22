use num_bytes::IntoBytes;

#[test]
fn generic() {
    let a = 8i16;
    let b = into_generic(a);
    assert_eq!(b,[8,0]);

    let a = 8i32;
    let b = into_generic(a);
    assert_eq!(b,[8,0,0,0]);

    fn into_generic<T: IntoBytes<N>, const N: usize>(x: T) -> [u8;N] {
        x.into_le_bytes()
    }
}

#[test]
fn into_u8() {
    let a = 8u8;
    let b = a.into_le_bytes();
    assert_eq!(b, [8]);
}
#[test]
fn into_u16() {
    let a = 8u16;
    let b = a.into_le_bytes();
    assert_eq!(b, [8, 0]);
}
#[test]
fn into_u32() {
    let a = 8u32;
    let b = a.into_le_bytes();
    assert_eq!(b, [8, 0, 0, 0]);
}
#[test]
fn into_u64() {
    let a = 8u64;
    let b = a.into_le_bytes();
    assert_eq!(b, [8, 0, 0, 0, 0, 0, 0, 0]);
}
#[test]
fn into_i8() {
    let a = 8i8;
    let b = a.into_le_bytes();
    assert_eq!(b, [8]);
}
#[test]
fn into_i16() {
    let a = 8i16;
    let b = a.into_le_bytes();
    assert_eq!(b, [8, 0]);
}
#[test]
fn into_i32() {
    let a = 8i32;
    let b = a.into_le_bytes();
    assert_eq!(b, [8, 0, 0, 0]);
}
#[test]
fn into_i64() {
    let a = 8i64;
    let b = a.into_le_bytes();
    assert_eq!(b, [8, 0, 0, 0, 0, 0, 0, 0]);
}
