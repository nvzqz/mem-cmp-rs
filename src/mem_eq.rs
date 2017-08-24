use core::mem::{size_of, size_of_val, transmute_copy};
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

macro_rules! from_type {
    ($t:ty, $x:expr, $y:expr) => {
        unsafe {
            let x: $t = transmute_copy($x);
            let y: $t = transmute_copy($y);
            x == y
        }
    }
}

impl<T, U> MemEq<U> for T {
    #[inline]
    fn mem_eq(&self, other: &U) -> bool {
        let size = size_of::<T>();
        if size != size_of::<U>() { return false; }

        macro_rules! impl_eq {
            ($($t:ty),+; simd: $($s:ty),+ $(,)*) => {
                $(if size == size_of::<$t>() {
                    from_type!($t, self, other)
                } else)+ $(if cfg!(feature = "simd") && size == size_of::<$s>() {
                    from_type!($s, self, other)
                } else)+ {
                    unsafe { _memcmp(self, other, 1) == 0 }
                }
            }
        }
        impl_eq! {
            u8, u16, u32, u64,
            U128, (U128, u64),
            U256, (U256, u64), (U256, U128), (U256, U128, u64),
            U512, (U512, u64), (U512, U256), (U512, U256, u64);

            // These types are only used when simd is enabled
            simd: (U512, U512), (U512, U512, U512), (U512, U512, U512, U512)
        }
    }
}

#[inline(always)]
fn _mem_eq<T: ?Sized, U: ?Sized>(a: &T, b: &U) -> bool {
    let size = size_of_val(a);
    size == size_of_val(b) && unsafe {
        let x = a as *const _ as _;
        let y = b as *const _ as _;
        (x as usize) == (y as usize) || memcmp(x, y, size) == 0
    }
}

#[cfg(feature = "specialization")]
impl<T: ?Sized, U: ?Sized> MemEq<U> for T {
    #[inline]
    default fn mem_eq(&self, other: &U) -> bool {
        _mem_eq(self, other)
    }
}

#[cfg(not(feature = "specialization"))]
impl<T, U> MemEq<[U]> for [T] {
    #[inline]
    fn mem_eq(&self, other: &[U]) -> bool {
        _mem_eq(self, other)
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
