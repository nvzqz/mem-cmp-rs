use core::mem::size_of;
use ext::*;

/// Trait for equality comparisons performed over bytes directly.
pub trait MemEq<Rhs: ?Sized = Self> {
    /// Tests whether `self` and `other` are equal in memory.
    fn mem_eq(&self, other: &Rhs) -> bool;

    /// Tests whether `self` and `other` are not equal in memory.
    #[inline]
    fn mem_neq(&self, other: &Rhs) -> bool { !self.mem_eq(other) }
}

impl<T, U> MemEq<U> for T {
    fn mem_eq(&self, other: &U) -> bool {
        size_of::<T>() == size_of::<U>() && unsafe { _memcmp(self, other) == 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_bytes() {
        let buf = [0u8; 128];
        assert!(buf.mem_eq(&buf));

        let x = [0u8; 1];
        let y = 0u8;
        assert!(x.mem_eq(&y));

        assert!(buf.mem_neq(&x));
        assert!(buf.mem_neq(&y));
    }
}
