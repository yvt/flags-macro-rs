//! This crate provides a convenient macro [`flags`] for constructing bitflags.
//! It's designed to be compatible with [`bitflags`] and [`enumflags`] but works
//! with any bitflags-like types.
//!
//! [`bitflags`]: https://crates.io/crates/bitflags
//! [`enumflags`]: https://crates.io/crates/enumflags
//!
//! # Examples
//!
//! `bitflags`:
//!
//!     # #[macro_use]
//!     # extern crate flags_macro;
//!     #[macro_use]
//!     extern crate bitflags;
//!     # fn main() {
//!     bitflags! {
//!         struct Test: u32 {
//!             const A = 0b0001;
//!             const B = 0b0010;
//!         }
//!     }
//!
//!     let flags0 = flags![Test::{}];
//!     let flags1 = flags![Test::{A}];
//!     let flags2 = flags![Test::{A | B}];
//!
//!     assert_eq!(flags0, Test::empty());
//!     assert_eq!(flags1, Test::A);
//!     assert_eq!(flags2, Test::A | Test::B);
//!     # }
//!
//! `enumflags`:
//!
//!     # #[macro_use]
//!     # extern crate flags_macro;
//!     #[macro_use]
//!     extern crate enumflags;
//!     # #[macro_use]
//!     # extern crate enumflags_derive;
//!     # fn main() {
//!     #[derive(EnumFlags, Copy, Clone, PartialEq, Eq, Debug)]
//!     #[repr(u8)]
//!     pub enum Test { A = 0b0001, B = 0b0010 }
//!
//!     let flags0 = flags![Test::{}];
//!     let flags1 = flags![Test::{A}];
//!     let flags2 = flags![Test::{A | B}];
//!
//!     assert_eq!(flags0, enumflags::BitFlags::empty());
//!     assert_eq!(flags1, Test::A);
//!     assert_eq!(flags2, Test::A | Test::B);
//!     # }
//!
#![no_std]
use core::{iter::FromIterator, ops::BitOr};

/// Emits an expression of type `<E as DefaultSet>::Set` given zero or more
/// values of type `E` defined as associated constants or enumerate items of
/// `E`.
///
/// # Examples
///
/// See the [module-level documentation].
///
/// [module-level documentation]: index.html
///
/// # Syntax
///
/// ```text
/// flags![path::ty::{Item1 | ... | ItemN}]
/// flags![path::ty::{Item1, ..., ItemN}]
/// ```
///
/// `Item1` ... `ItemN` are identifiers. Conceptually, these expressions are
/// expanded into:
///
/// ```text
/// <path::ty as DefaultSet>::Set::from_iter([
///     path::ty::Item1, ..., path::ty::ItemN
/// ].iter().cloned())
/// ```
///
/// Usually, this is equivalent to:
///
/// ```text
/// path::ty::Item1 | ... | path::ty::ItemN
/// ```
///
/// # Invalid usages
///
/// The path prefix (denoted as `path::ty::` in Section "Syntax") must not be
/// empty.
///
#[macro_export(local_inner_macros)]
macro_rules! flags {
    ( $($ns:ident::)* {$($items:tt)*} ) => (
        <__containing_type!($($ns::)*) as $crate::DefaultSet>
            ::set_from_iter(set_array![$($ns::)*{$($items)*}].iter().cloned())
    )
}

/// Gets `A::B` from `A::B::`.
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __containing_type {
    () => {
        compile_error!("The path prefix (`A::` of `flags![A::{...}]`) must not be empty.")
    };
    ($ns:ident::) => {$ns};
    ($ns:ident::$($rest:ident::)*) => {$ns::__containing_type!($($rest::)*)}
}

/// Emits an array expression containing zero or more values defined within
/// the same namespace (or a similar language construct).
///
/// # Syntax
///
/// ```text
/// set_array![path1::path2::{Item1 | ... | ItemN}]
/// set_array![path1::path2::{Item1, ..., ItemN}]
/// ```
///
/// `Item1` ... `ItemN` are identifiers. These expressions are expanded into:
///
/// ```text
/// [path1::path2::Item1, ..., path1::path2::ItemN]
/// ```
///
/// # Examples
///
///     # #[macro_use]
///     # extern crate flags_macro;
///     # fn main() {
///     mod values {
///         pub const A: u32 = 1;
///         pub const B: u32 = 2;
///         pub const C: u32 = 3;
///     }
///
///     let array0: [u32; 0] = set_array![values::{}];
///     let array1 = set_array![values::{A}];
///     let array2a = set_array![values::{A | B}];
///     let array2b = set_array![values::{A, B}]; // alternative syntax
///
///     assert_eq!(array0, []);
///     assert_eq!(array1, [values::A]);
///     assert_eq!(array2a, [values::A, values::B]);
///     assert_eq!(array2b, [values::A, values::B]);
///     # }
#[macro_export(local_inner_macros)]
macro_rules! set_array {
    ( $($ns:ident::)* {$($items:tt)*} ) => (
        __set_array![@[] $($ns::)*{$($items)*}]
    )
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __set_array {
    ( @[$($out:tt)*] $($ns:ident::)* {} ) => (
        [$($out)*]
    );

    ( @[$($out:tt)*] $($ns:ident::)* {$tail:ident} ) => (
        [$($out)* $($ns::)*$tail]
    );

    ( @[$($out:tt)*] $($ns:ident::)* {$head:ident | $($rest:tt)*} ) => (
        __set_array![
            @[$($out)* $($ns::)*$head,]
            $($ns::)*{$($rest)*}
        ]
    );

    ( @[$($out:tt)*] $($ns:ident::)* {$head:ident , $($rest:tt)*} ) => (
        __set_array![
            @[$($out)* $($ns::)*$head,]
            $($ns::)*{$($rest)*}
        ]
    )
}

/// A trait for getting the default "set" type from an "element" type.
///
/// This trait has a blanket implementation for bitflags-like types.
pub trait DefaultSet: Sized {
    type Set: FromIterator<Self>;

    /// Construct a `Set` using `Set::from_iter`.
    fn set_from_iter(iter: impl IntoIterator<Item = Self>) -> Self::Set {
        Self::Set::from_iter(iter)
    }
}

impl<T> DefaultSet for T
where
    T: BitOr,
    <T as BitOr>::Output: FromIterator<Self>,
{
    type Set = <T as BitOr>::Output;
}
