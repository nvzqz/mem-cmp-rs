use core::cmp::{self, Ordering};
use core::mem;
use ext::*;
use mem_eq::MemEq;

/// Trait for values whose bytes can be compared directly.
pub trait MemOrd<Rhs: ?Sized = Self>: MemEq<Rhs> {
    /// Returns an ordering between the memory of `self` and `other`.
    #[must_use]
    fn mem_cmp(&self, other: &Rhs) -> Ordering;
}

#[inline(always)]
fn convert(cmp: i32, size_a: usize, size_b: usize) -> Ordering {
    match cmp {
        _ if cmp < 0 => Ordering::Less,
        _ if cmp > 0 => Ordering::Greater,
        _ => size_a.cmp(&size_b)
    }
}

impl<T, U> MemOrd<U> for T {
    #[inline]
    fn mem_cmp(&self, other: &U) -> Ordering {
        use self::mem::{size_of, transmute_copy};
        let size_a = size_of::<T>();
        let size_b = size_of::<U>();
        macro_rules! impl_cmp {
            ($($t:ty),+) => {
                $(if size_a == size_b && size_a == size_of::<$t>() {
                    unsafe {
                        let x: $t = transmute_copy(self);
                        let y: $t = transmute_copy(other);
                        x.cmp(&y)
                    }
                } else)+ {
                    let cmp = unsafe { _memcmp(self, other, 1) };
                    convert(cmp, size_a, size_b)
                }
            }
        }
        impl_cmp!(u8, u16, u32, u64)
    }
}

#[inline(always)]
fn _mem_cmp<T: ?Sized, U: ?Sized>(a: &T, b: &U) -> Ordering {
    let size_a = mem::size_of_val(a);
    let size_b = mem::size_of_val(b);
    let cmp = unsafe {
        let size = cmp::min(size_a, size_b);
        memcmp(a as *const _ as _, b as *const _ as _, size)
    };
    convert(cmp, size_a, size_b)
}

#[cfg(feature = "specialization")]
impl<T: ?Sized, U: ?Sized> MemOrd<U> for T {
    #[inline]
    default fn mem_cmp(&self, other: &U) -> Ordering {
        _mem_cmp(self, other)
    }
}

#[cfg(not(feature = "specialization"))]
impl<T, U> MemOrd<[U]> for [T] {
    #[inline]
    fn mem_cmp(&self, other: &[U]) -> Ordering {
        _mem_cmp(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_bytes() {
        let x = [0u8, 0, 0, 0];
        let y = [0u8, 0, 0, 4];
        assert_eq!(x.cmp(&y), x.mem_cmp(&y));
    }

    #[test]
    fn different_sizes() {
        macro_rules! helper {
            ($a:expr, $b:expr, $ord:expr) => {
                assert_eq!($a.mem_cmp(&$b), $ord);
                assert_eq!($a[..].mem_cmp(&$b[..]), $ord);
            }
        }
        let a = [0u8, 0, 0];
        let b = [0u8, 0, 2];
        let c = [0u8; 0];
        helper!(a, b, Ordering::Less);
        helper!(b, b, Ordering::Equal);
        helper!(b, c, Ordering::Greater);
    }
}
