use core::cmp::Ordering;
use core::mem;
use ext::*;
use mem_eq::MemEq;

/// Trait for values whose bytes can be compared directly.
pub trait MemOrd: MemEq {
    /// Returns an ordering between the memory of `self` and `other`.
    #[must_use]
    fn mem_cmp(&self, other: &Self) -> Ordering;
}

#[inline(always)]
fn _mem_cmp<T>(this: &T, other: &T) -> Ordering {
    match unsafe { _memcmp(this, other, 1) } {
        x if x < 0 => Ordering::Less,
        x if x > 0 => Ordering::Greater,
        _ => Ordering::Equal,
    }
}

impl<T> MemOrd for T {
    #[inline]
    #[cfg(feature = "specialization")]
    default fn mem_cmp(&self, other: &Self) -> Ordering { _mem_cmp(self, other) }

    #[cfg(not(feature = "specialization"))]
    fn mem_cmp(&self, other: &Self) -> Ordering { _mem_cmp(self, other) }
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
    ($dep:ty => $($t:ty),+) => {
        $(#[cfg(feature = "specialization")]
        impl MemOrd for $t {
            #[inline]
            fn mem_cmp(&self, other: &Self) -> Ordering {
                unsafe {
                    let x: $dep = mem::transmute(*self);
                    let y: $dep = mem::transmute(*other);
                    x.mem_cmp(&y)
                }
            }
        })+
    }
}

impl_specialized!(u8 u16 u32 u64 usize);
impl_specialized_dep!(u8    => i8);
impl_specialized_dep!(u16   => i16);
impl_specialized_dep!(u32   => i32);
impl_specialized_dep!(u64   => i64);
impl_specialized_dep!(usize => isize);

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
}
