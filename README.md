# flags-macro

[<img src="https://docs.rs/flags-macro/badge.svg" alt="docs.rs">](https://docs.rs/flags-macro/)

This crate provides a convenient macro `flags` for constructing bitflags.
It's designed to be compatible with [`bitflags`] and [`enumflags`] but works
with any bitflags-like types.

[`bitflags`]: https://crates.io/crates/bitflags
[`enumflags`]: https://crates.io/crates/enumflags

## Examples

`bitflags`:

```rust
#[macro_use]
extern crate bitflags;
bitflags! {
    struct Test: u32 {
        const A = 0b0001;
        const B = 0b0010;
    }
}

let flags0 = flags![Test::{}];
let flags1 = flags![Test::{A}];
let flags2 = flags![Test::{A | B}];

assert_eq!(flags0, Test::empty());
assert_eq!(flags1, Test::A);
assert_eq!(flags2, Test::A | Test::B);
```

`enumflags`:

```rust
#[macro_use]
extern crate enumflags;
#[derive(EnumFlags, Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Test { A = 0b0001, B = 0b0010 }

let flags0 = flags![Test::{}];
let flags1 = flags![Test::{A}];
let flags2 = flags![Test::{A | B}];

assert_eq!(flags0, enumflags::BitFlags::empty());
assert_eq!(flags1, Test::A);
assert_eq!(flags2, Test::A | Test::B);
```

License: CC0-1.0
