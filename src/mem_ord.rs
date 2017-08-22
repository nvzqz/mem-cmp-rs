use core::cmp::Ordering;
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

impl_specialized!(u8 u16 u32 u64 usize);
impl_specialized!(i8 i16 i32 i64 isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_bytes() {
        let x = [0u8, 0, 0, 0];
        let y = [0u8, 0, 0, 4];
        assert_eq!(x.cmp(&y), x.mem_cmp(&y));
    }
}
