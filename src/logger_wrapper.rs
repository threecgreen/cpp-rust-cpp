use crate::cpp::{Level, Logger};

use std::ffi;

// Not a box because this wrapper does not own `Logger`. That is owned by C++
// caller
pub struct LoggerWrapper(*mut Logger);

impl LoggerWrapper {
    pub fn new(logger_ptr: *mut Logger) -> Self {
        if logger_ptr.is_null() {
            // TODO: panicking in FFI results in undefined behavior
            panic!("Demo was passed a null logger pointer");
        }
        Self (logger_ptr)
    }

    /// Persistence will panic if `msg` cannot be converted to a
    /// `std::ffi::CString`. Maybe this should be converted to an error
    pub fn persist(&mut self, level: Level, msg: &str) {
        let c_msg = ffi::CString::new(msg)
            .expect("Failed to create CString");
        let c_msg_ptr = c_msg.as_ptr();
        unsafe {
            (*self.0).Persist(level, c_msg_ptr);
        }
    }
}
