use std::ffi::CString;

use crate::ffi;

pub fn hash32(s: &str) -> u32 {
    let s = CString::new(s).unwrap();
    unsafe { ffi::dmHashString32(s.as_ptr()) }
}

pub fn hash64(s: &str) -> u64 {
    let s = CString::new(s).unwrap();
    unsafe { ffi::dmHashString64(s.as_ptr()) }
}
