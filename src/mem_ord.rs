use std::cmp::Ordering;
use mem_eq::MemEq;

/// Trait for values whose bytes can be compared directly.
pub trait MemOrd: MemEq {
    /// Returns an ordering between the memory of `self` and `other`.
    fn mem_cmp(&self, other: &Self) -> Ordering;
}
