use core::cmp::Ordering;
use mem_eq::*;
use mem_ord::*;

/// A type that implements comparison traits via [`MemEq`](trait.MemEq.html)
/// and [`MemOrd`](trait.MemOrd.html).
///
/// # Examples
///
/// ```
/// use mem_cmp::MemOrdered;
///
/// let mut values = [-1, 4, 7, 3];
///
/// MemOrdered::from_slice_mut(&mut values).sort();
///
/// assert_eq!(values, [3, 4, 7, -1]);
/// ```
pub struct MemOrdered<T: ?Sized>(pub T);

/// Slice conversions.
///
/// # Safety
///
/// All types used **_must_** adhere to [these safety rules](index.html#safety).
impl<T> MemOrdered<T> {
    /// Creates a slice of memory-ordered elements.
    pub fn from_slice(slice: &[T]) -> &[MemOrdered<T>] {
        unsafe { &*(slice as *const _ as *const _) }
    }

    /// Creates a mutable slice of memory-ordered elements.
    pub fn from_slice_mut(slice: &mut [T]) -> &mut [MemOrdered<T>] {
        unsafe { &mut *(slice as *mut _ as *mut _) }
    }
}

impl<T> From<T> for MemOrdered<T> {
    fn from(inner: T) -> Self { MemOrdered(inner) }
}

impl<'a, T: ?Sized> From<&'a T> for &'a MemOrdered<T> {
    fn from(inner: &T) -> Self {
        unsafe { &*(inner as *const T as *const _) }
    }
}

impl<'a, T: ?Sized> From<&'a mut T> for &'a mut MemOrdered<T> {
    fn from(inner: &mut T) -> Self {
        unsafe { &mut *(inner as *mut T as *mut _) }
    }
}

impl<T: ?Sized> AsRef<T> for MemOrdered<T> {
    fn as_ref(&self) -> &T { &self.0 }
}

impl<T: ?Sized> AsMut<T> for MemOrdered<T> {
    fn as_mut(&mut self) -> &mut T { &mut self.0 }
}

impl<T: ?Sized, U: ?Sized> PartialEq<MemOrdered<U>> for MemOrdered<T>
    where T: MemEq<U>
{
    fn eq(&self, other: &MemOrdered<U>) -> bool {
        self.0.mem_eq(&other.0)
    }
}

impl<T> Eq for MemOrdered<T> {}

impl<T: ?Sized + MemOrd> PartialOrd for MemOrdered<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.mem_cmp(&other.0))
    }
}

impl<T: MemOrd> Ord for MemOrdered<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.mem_cmp(&other.0)
    }
}
