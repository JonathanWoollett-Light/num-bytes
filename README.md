### num-bytes

[![Crates.io](https://img.shields.io/crates/v/num-bytes)](https://crates.io/crates/num-bytes)
[![lib.rs.io](https://img.shields.io/crates/v/num-bytes?color=blue&label=lib.rs)](https://lib.rs/crates/num-bytes)
[![docs](https://img.shields.io/crates/v/num-bytes?color=yellow&label=docs)](https://docs.rs/num-bytes)

Traits for converting primitives into bytes.

Kinda like a bad extension to [num](https://github.com/rust-num/num).

```rust
use num_bytes::FromBytes;
fn into<T:FromBytes>(x:T) -> &
let a = [8,0,0,0];
let b = i32::from_le_bytes(a);
assert_eq!(b, 8);
```