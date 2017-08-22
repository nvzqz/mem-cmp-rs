use core::cmp::Ordering;
use ext::*;
use mem_eq::MemEq;

/// Trait for values whose bytes can be compared directly.
pub trait MemOrd: MemEq {
    /// Returns an ordering between the memory of `self` and `other`.
    fn mem_cmp(&self, other: &Self) -> Ordering;
}

impl<T> MemOrd for T {
    fn mem_cmp(&self, other: &Self) -> Ordering {
        match unsafe { _memcmp(self, other) } {
            x if x < 0 => Ordering::Less,
            x if x > 0 => Ordering::Greater,
            _ => Ordering::Equal,
        }
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
}
