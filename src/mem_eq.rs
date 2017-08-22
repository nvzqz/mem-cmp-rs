use core::mem::{size_of, transmute_copy};
use ext::*;

/// Trait for equality comparisons performed over bytes directly.
pub trait MemEq<Rhs: ?Sized = Self> {
    /// Tests whether `self` and `other` are equal in memory.
    #[must_use]
    fn mem_eq(&self, other: &Rhs) -> bool;

    /// Tests whether `self` and `other` are not equal in memory.
    #[inline]
    #[must_use]
    fn mem_neq(&self, other: &Rhs) -> bool { !self.mem_eq(other) }
}

#[inline(always)]
fn _mem_eq<T, U>(this: &T, other: &U) -> bool {
    size_of::<T>() == size_of::<U>() && unsafe { _memcmp(this, other, 1) == 0 }
}

impl<T, U> MemEq<U> for T {
    #[inline]
    fn mem_eq(&self, other: &U) -> bool {
        macro_rules! impl_match {
            ($($s:expr, $t:ty);+) => {
                match (size_of::<T>(), size_of::<U>()) {
                    $(($s, $s) => unsafe {
                        let x: $t = transmute_copy(self);
                        let y: $t = transmute_copy(other);
                        x == y
                    },)+
                    _ => _mem_eq(self, other)
                }
            }
        }
        impl_match!(1, u8; 2, u16; 4, u32; 8, u64)
    }
}

impl<T, U> MemEq<[U]> for [T] {
    #[inline]
    fn mem_eq(&self, other: &[U]) -> bool {
        size_of::<T>() == size_of::<U>() && self.len() == other.len() && unsafe {
            let x = self.as_ptr();
            let y = other.as_ptr();
            (x as usize) == (y as usize) || _memcmp(x, y, self.len()) == 0
        }
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
