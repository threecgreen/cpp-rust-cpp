use crate::cpp::{Level, Logger};
use crate::logger_wrapper::LoggerWrapper;

#[repr(C)]
pub struct Demo {
    is_running: bool,
    name: String,
    logger: LoggerWrapper,
}

// This should be `impl Algorithm`, but cbindgen doesn't seem to detect that
// these are public unless we explicitly mark them `pub`
//
// impl Algorithm for Demo {
impl Demo {
    #[no_mangle]
    pub extern "C" fn create(logger_ptr: *mut Logger) -> Self {
        Self {
            is_running: false,
            logger: LoggerWrapper::new(logger_ptr),
            name: "Demo".to_owned(),
        }
    }

    #[no_mangle]
    pub extern "C" fn on_register(&mut self) {
        self.logger.persist(Level::Info, &format!("Registering {}...", self.name));
    }

    #[no_mangle]
    pub extern "C" fn on_system_start(&mut self) {
        if self.is_running {
            let msg = "Tried to start, but already running.";
            self.logger.persist(Level::Error, msg);
            panic!(msg);
        }
        self.logger.persist(Level::Info, &format!("Starting {}...", self.name));
        self.is_running = true;
    }

    #[no_mangle]
    pub extern "C" fn main_thread_event(&mut self) {
        self.logger.persist(Level::Info, &format!("Processing main event"));
    }

    #[no_mangle]
    pub extern "C" fn background_thread_event(&mut self) {
        self.logger.persist(Level::Info, &format!("Processing background event"));
    }

    #[no_mangle]
    pub extern "C" fn on_system_stop(&mut self) {
        self.logger.persist(Level::Info, &format!("Stopping {}...", self.name));
        self.is_running = false;
    }

    #[no_mangle]
    pub extern "C" fn on_unregister(&mut self) {
        self.logger.persist(Level::Info, &format!("Unregistering {}...", self.name));
    }
}
