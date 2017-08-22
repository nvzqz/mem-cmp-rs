/// Trait for equality comparisons performed over bytes directly.
pub trait MemEq {
    /// Tests whether `self` and `other` are equal in memory.
    fn mem_eq(&self, other: &Self) -> bool;

    /// Tests whether `self` and `other` are not equal in memory.
    #[inline]
    fn mem_neq(&self, other: &Self) -> bool { !self.mem_eq(other) }
}
