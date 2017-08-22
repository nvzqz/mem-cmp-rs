use core::mem::size_of;
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
    #[cfg(feature = "specialization")]
    default fn mem_eq(&self, other: &U) -> bool { _mem_eq(self, other) }

    #[cfg(not(feature = "specialization"))]
    fn mem_eq(&self, other: &U) -> bool { _mem_eq(self, other) }
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

macro_rules! impl_specialized {
    ($($t:ty)+) => {
        $(#[cfg(feature = "specialization")]
        impl MemEq for $t {
            #[inline]
            fn mem_eq(&self, other: &Self) -> bool { self == other }
        })+
    }
}

impl_specialized!(u8 u16 u32 u64 usize);
impl_specialized!(i8 i16 i32 i64 isize);

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
