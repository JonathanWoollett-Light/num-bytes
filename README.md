### num-bytes

[![Crates.io](https://img.shields.io/crates/v/num-bytes)](https://crates.io/crates/num-bytes)
[![lib.rs.io](https://img.shields.io/crates/v/num-bytes?color=blue&label=lib.rs)](https://lib.rs/crates/num-bytes)
[![docs](https://img.shields.io/crates/v/num-bytes?color=yellow&label=docs)](https://docs.rs/num-bytes)

Traits for converting primitives into bytes.

Kinda like a bad extension to [num](https://github.com/rust-num/num).

Usage is a little tricky:
```rust
let a = [8,0];
let b: i16 = from_generic(a);
assert_eq!(b,8);
fn from_generic<T: num_bytes::FromBytes<N>, const N: usize>(x: [u8;N]) -> T {
    T::from_le_bytes(x)
}
```
and:
```rust
let a = 8i16;
let b = into_generic(a);
assert_eq!(b,[8,0]);
fn into_generic<T: num_bytes::IntoBytes<N>, const N: usize>(x: T) -> [u8;N] {
    x.into_le_bytes()
}
```