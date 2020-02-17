use crate::algo::Algorithm;
use crate::cpp::{Level, Logger};
use crate::logger_wrapper::LoggerWrapper;

pub struct Demo {
    is_running: bool,
    name: String,
    logger: LoggerWrapper,
}

// TODO: These shim functions should be generated with a macro
// Need to be free functions to be compatible with C ABI
#[no_mangle]
pub extern "C" fn create(logger_ptr: &mut Logger) -> *mut Demo {
    Demo::create(logger_ptr)
}

#[no_mangle]
pub extern "C" fn destroy(demo_ptr: *mut Demo) {
    Demo::destroy(demo_ptr);
}

#[no_mangle]
pub extern "C" fn on_register(demo: &mut Demo) {
    demo.on_register();
}

#[no_mangle]
pub extern "C" fn on_system_start(demo: &mut Demo) {
    demo.on_system_start();
}

#[no_mangle]
pub extern "C" fn main_thread_event(demo: &mut Demo) {
    demo.main_thread_event();
}

#[no_mangle]
pub extern "C" fn background_thread_event(demo: &mut Demo) {
    demo.background_thread_event();
}

#[no_mangle]
pub extern "C" fn on_system_stop(demo: &mut Demo) {
    demo.on_system_stop();
}

#[no_mangle]
pub extern "C" fn on_unregister(demo: &mut Demo) {
    demo.on_unregister();
}

impl Algorithm for Demo {
    fn create(logger_ptr: *mut Logger) -> *mut Self {
        let demo = Self {
            is_running: false,
            logger: LoggerWrapper::new(logger_ptr),
            name: "Demo".to_owned(),
        };
        Box::into_raw(Box::new(demo))
    }

    // Because `Demo` is an opaque type to C++, it can't free it
    fn destroy(algo_ptr: *mut Self) {
        if algo_ptr.is_null() {
            return;
        }
        unsafe {
            Box::from_raw(algo_ptr)
        };
        // Dropped
    }

    fn on_register(&mut self) {
        self.logger.persist(Level::Info, &format!("Registering {}...", self.name));
    }

    fn on_system_start(&mut self) {
        if self.is_running {
            let msg = "Tried to start, but already running.";
            self.logger.persist(Level::Error, msg);
            panic!(msg);
        }
        self.logger.persist(Level::Info, &format!("Starting {}...", self.name));
        self.is_running = true;
    }

    fn main_thread_event(&mut self) {
        self.logger.persist(Level::Info, &format!("Processing main event"));
    }

    fn background_thread_event(&mut self) {
        self.logger.persist(Level::Info, &format!("Processing background event"));
    }

    fn on_system_stop(&mut self) {
        self.logger.persist(Level::Info, &format!("Stopping {}...", self.name));
        self.is_running = false;
    }

    fn on_unregister(&mut self) {
        self.logger.persist(Level::Info, &format!("Unregistering {}...", self.name));
    }
}
