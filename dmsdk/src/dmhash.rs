use std::ffi::CString;

/// Returns a 32-bit hash of the given string slice.
pub fn hash32(s: &str) -> u32 {
    let s = CString::new(s).unwrap();
    unsafe { dmsdk_ffi::dmHashString32(s.as_ptr()) }
}

/// Returns a 64-bit hash of the given string slice.
pub fn hash64(s: &str) -> u64 {
    let s = CString::new(s).unwrap();
    unsafe { dmsdk_ffi::dmHashString64(s.as_ptr()) }
}
