use std::mem::size_of;

// External C functions are declared here to avoid depending on libc.
extern "C" {
    pub fn memcmp(s1: *const i8, s2: *const i8, n: usize) -> i32;
}

pub unsafe fn _memcmp<T>(x: *const T, y: *const T) -> i32 {
    memcmp(x as _, y as _, size_of::<T>())
}
