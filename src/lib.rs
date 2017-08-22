#![no_std]

mod ext;
mod mem_eq;
mod mem_ord;

use core::cmp::Ordering;

pub use mem_eq::*;
pub use mem_ord::*;

/// A type that implements comparison traits via [`MemEq`](trait.MemEq.html)
/// and [`MemOrd`](trait.MemOrd.html).
pub struct MemOrdered<T: ?Sized>(pub T);

impl<T: ?Sized, U: ?Sized> PartialEq<MemOrdered<U>> for MemOrdered<T>
    where T: MemEq<U>
{
    fn eq(&self, other: &MemOrdered<U>) -> bool {
        self.0.mem_eq(&other.0)
    }
}

impl<T> Eq for MemOrdered<T> {}

impl<T: ?Sized + MemOrd> PartialOrd for MemOrdered<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.mem_cmp(&other.0))
    }
}

impl<T: MemOrd> Ord for MemOrdered<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.mem_cmp(&other.0)
    }
}
