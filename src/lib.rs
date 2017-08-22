#![cfg_attr(feature = "specialization", feature(specialization))]
#![no_std]

mod ext;
mod mem_eq;
mod mem_ord;
mod mem_ordered;

pub use mem_eq::*;
pub use mem_ord::*;
pub use mem_ordered::*;
