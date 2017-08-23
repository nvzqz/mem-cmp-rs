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

#[derive(Copy, Clone)]
#[cfg_attr(not(feature = "simd"), derive(PartialEq))]
#[cfg_attr(feature = "simd", repr(simd))]
struct U128(u64, u64);

#[cfg(feature = "simd")]
impl PartialEq for U128 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        use core::mem::transmute;
        use simd::u32x4;
        unsafe {
            let x: u32x4 = transmute(*self);
            let y: u32x4 = transmute(*other);
            x.eq(y).all()
        }
    }
}

#[cfg(feature = "avx")]
#[derive(Copy, Clone)]
#[repr(simd)]
struct U256(u64, u64, u64, u64);

#[cfg(feature = "avx")]
impl PartialEq for U256 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        use core::mem::transmute;
        use simd::x86::avx::u8x32;
        unsafe {
            let x: u8x32 = transmute(*self);
            let y: u8x32 = transmute(*other);
            x.eq(y).all()
        }
    }
}

#[cfg(not(feature = "avx"))]
type U256 = (U128, U128);

type U512 = (U256, U256);

impl<T, U> MemEq<U> for T {
    #[inline]
    fn mem_eq(&self, other: &U) -> bool {
        let size = size_of::<T>();
        if size != size_of::<U>() { return false; }

        macro_rules! impl_eq {
            ($($t:ty),+) => {
                $(if size == size_of::<$t>() {
                    unsafe {
                        let x: $t = transmute_copy(self);
                        let y: $t = transmute_copy(other);
                        x == y
                    }
                } else)+ {
                    unsafe { _memcmp(self, other, 1) == 0 }
                }
            }
        }
        impl_eq! {
            u8, u16, u32, u64,
            U128, (U128, u64),
            U256, (U256, u64), (U256, U128), (U256, U128, u64),
            U512, (U512, u64), (U512, U256), (U512, U256, u64)
        }
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
