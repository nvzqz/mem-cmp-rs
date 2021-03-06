//! Safe memory comparison between types.
//!
//! Types found in this crate serve as safe wrappers around `memcmp` operations.
//!
//! # Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! mem_cmp = "0.1.4"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! extern crate mem_cmp;
//! # fn main() {}
//! ```
//!
//! # Comparing Memory
//!
//! The [`MemOrd`] trait provides convenience around the result of a `memcmp`
//! call by returning an `Ordering`.
//!
//! If the only thing that matters is equality, [`MemEq`] is also available.
//!
//! ```
//! use mem_cmp::*;
//!
//! let a = [0u8; 256];
//! let b = [0u32; 64];
//! let c = [4u32; 64];
//!
//! assert!(a.mem_eq(&b));
//! assert!(a.mem_neq(&c));
//!
//! // Also works with types of different sizes:
//! assert!(a.mem_neq(&42));
//! ```
//!
//! # Safety
//!
//! Generally, comparing memory directly is safe. However, there are cases where
//! undefined behavior may be encountered.
//!
//! - **Types with padding:**
//!
//!   One such case is with types where the inner values are padded for
//!   alignment. Such a type would be `(u8, u32)`. It has an alignment of 4
//!   bytes, resulting in 3 bytes of padding between the first value and the
//!   second. It's up to the compiler to decide what the byte values are for
//!   the padding.
//!
//! [crate]: https://crates.io/crates/mem_cmp
//! [`MemEq`]: trait.MemEq.html
//! [`MemOrd`]: trait.MemOrd.html

#![no_std]
#![cfg_attr(feature = "simd", feature(repr_simd))]
#![cfg_attr(feature = "specialization", feature(specialization))]

#[cfg(feature = "simd")]
extern crate simd;

mod ext;
mod mem_eq;
mod mem_ord;
mod mem_ordered;

pub use mem_eq::*;
pub use mem_ord::*;
pub use mem_ordered::*;
