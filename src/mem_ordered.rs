use core::cmp::Ordering;
use mem_eq::*;
use mem_ord::*;

/// A type that implements comparison traits via [`MemEq`](trait.MemEq.html)
/// and [`MemOrd`](trait.MemOrd.html).
pub struct MemOrdered<T: ?Sized>(pub T);

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
