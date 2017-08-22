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

impl<T, U> MemOrd<U> for T {
    #[inline]
    fn mem_cmp(&self, other: &U) -> Ordering {
        use self::mem::{size_of, transmute_copy};
        macro_rules! impl_match {
            ($($s:expr, $t:ty);+) => {
                match (size_of::<T>(), size_of::<U>()) {
                    $(($s, $s) => unsafe {
                        let x: $t = transmute_copy(self);
                        let y: $t = transmute_copy(other);
                        x.cmp(&y)
                    },)+
                    _ => match unsafe { _memcmp(self, other, 1) } {
                        x if x < 0 => Ordering::Less,
                        x if x > 0 => Ordering::Greater,
                        _ => size_of::<T>().cmp(&size_of::<U>())
                    }
                }
            }
        }
        impl_match!(1, u8; 2, u16; 4, u32; 8, u64)
    }
}

impl<T, U> MemOrd<[U]> for [T] {
    fn mem_cmp(&self, other: &[U]) -> Ordering {
        let size_a = mem::size_of_val(self);
        let size_b = mem::size_of_val(other);
        let cmp = unsafe {
            let size = cmp::min(size_a, size_b);
            memcmp(self.as_ptr() as _, other.as_ptr() as _, size)
        };
        convert(cmp, size_a, size_b)
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
