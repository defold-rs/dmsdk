//! Time functions.

use dmsdk_ffi::dmTime;

/// Gets the current time in microseconds since Jan. 1, 1970.
pub fn get_time() -> u64 {
    unsafe { dmTime::GetTime() }
}

/// Pauses thread with low precision (~10 milliseconds).
pub fn sleep(useconds: u32) {
    unsafe {
        dmTime::Sleep(useconds);
    }
}
