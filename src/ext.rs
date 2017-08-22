use core::mem::size_of;
use core::cmp;

// External C functions are declared here to avoid depending on libc.
extern "C" {
    pub fn memcmp(s1: *const i8, s2: *const i8, n: usize) -> i32;
}

pub unsafe fn _memcmp<T, U>(x: *const T, y: *const U) -> i32 {
    let size = cmp::min(size_of::<T>(), size_of::<U>());
    memcmp(x as _, y as _, size)
}
