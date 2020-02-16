use crate::cpp::{Level, Logger};

use std::ffi;

// TODO: does it make sense for `Logger` to be managed?
pub struct LoggerWrapper(Box<Logger>);

impl LoggerWrapper {
    pub fn new(logger_ptr: *mut Logger) -> Self {
        if logger_ptr.is_null() {
            panic!("Demo was passed a null logger pointer");
        }
        let logger = unsafe {
            Box::from_raw(logger_ptr)
        };
        Self (logger)
    }

    /// Persistence will panic if `msg` cannot be converted to a
    /// `std::ffi::CString`. Maybe this should be converted to an error
    pub fn persist(&mut self, level: Level, msg: &str) {
        let c_msg = ffi::CString::new(msg)
            .expect("Failed to create CString");
        let c_msg_ptr = c_msg.as_ptr();
        unsafe {
            self.0.Persist(level, c_msg_ptr);
        }
    }
}
