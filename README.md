# tuple-arity

[![Crates.io](https://img.shields.io/crates/v/tuple-arity)](https://crates.io/crates/tuple-arity)
[![Crates.io](https://img.shields.io/crates/d/tuple-arity)](https://crates.io/crates/tuple-arity)
[![Docs.rs shield](https://docs.rs/tuple-arity/badge.svg)](https://docs.rs/tuple-arity)

A simple crate for getting the arity (number of elements) of tuple types with 0 to 12 elements.

## How to use

You can use the `tuple_arity()` function to get the arity of an existing tuple value.

```rust
use tuple_arity::*;
assert_eq!(0, tuple_arity(&()));
assert_eq!(1, tuple_arity(&("foo",)));
assert_eq!(2, tuple_arity(&("foo", "bar")));
assert_eq!(3, tuple_arity(&("foo", "bar", "baz")));
```

You can also use the `Arity` trait to get the arity of a tuple type directly:

```rust
use tuple_arity::Arity;

assert_eq!(0, <()>::arity());
assert_eq!(1, <(u8,)>::arity());
assert_eq!(2, <(u8, u8)>::arity());
assert_eq!(3, <(u8, u8, u8)>::arity());
assert_eq!(4, <(u8, u8, u8, u8)>::arity());
```