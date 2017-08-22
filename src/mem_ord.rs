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

fn convert(cmp: i32, size_a: usize, size_b: usize) -> Ordering {
    match cmp {
        _ if cmp < 0 => Ordering::Less,
        _ if cmp > 0 => Ordering::Greater,
        _ => size_a.cmp(&size_b)
    }
}

#[inline(always)]
fn _mem_cmp<T, U>(this: &T, other: &U) -> Ordering {
    let cmp = unsafe { _memcmp(this, other, 1) };
    convert(cmp, mem::size_of::<T>(), mem::size_of::<U>())
}

impl<T, U> MemOrd<U> for T {
    #[inline]
    #[cfg(feature = "specialization")]
    default fn mem_cmp(&self, other: &U) -> Ordering { _mem_cmp(self, other) }

    #[cfg(not(feature = "specialization"))]
    fn mem_cmp(&self, other: &U) -> Ordering { _mem_cmp(self, other) }
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

macro_rules! impl_specialized {
    ($($t:ty)+) => {
        $(#[cfg(feature = "specialization")]
        impl MemOrd for $t {
            #[inline]
            fn mem_cmp(&self, other: &Self) -> Ordering { self.cmp(other) }
        })+
    }
}

macro_rules! impl_specialized_dep {
    ($($dep:ty => $($t:ty),+;)+) => {
        $($(#[cfg(feature = "specialization")]
        impl MemOrd for $t {
            #[inline]
            fn mem_cmp(&self, other: &Self) -> Ordering {
                unsafe {
                    let x: $dep = mem::transmute(*self);
                    let y: $dep = mem::transmute(*other);
                    x.mem_cmp(&y)
                }
            }
        })+)+
    }
}

impl_specialized!(u8 u16 u32 u64 usize);

impl_specialized_dep! {
    u8    => i8,  [u8; 1], [i8; 1];
    u16   => i16, [u8; 2], [i8; 2], (u8, u8);
    u32   => i32, [u8; 4], [i8; 4], (u8, u8, u8, u8), (u16, u16);
    u64   => i64, [u8; 8], [i8; 8], (u8, u8, u8, u8, u8, u8, u8, u8),
                                    (u16, u16, u16, u16), (u32, u32);
    usize => isize;
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
    fn compare_signed() {
        macro_rules! compare {
            ($x:expr, $y:expr, $($t:ty),+) => {
                $({
                    let x: $t = $x;
                    let y: $t = $y;
                    assert_eq!(x.mem_cmp(&y), _mem_cmp(&x, &y));
                })+
            }
        }
        compare!(-1, 1, i8, i16, i32, i64, isize);
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
