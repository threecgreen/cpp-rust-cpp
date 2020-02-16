use crate::algo::Algorithm;

#[repr(C)]
pub struct Demo {
    // TODO: use cpp logger
    is_running: bool,
    name: String,
}

impl Demo {
    #[no_mangle]
    pub extern "C" fn name(&self) -> &String {
        &self.name
    }
}

// To call from another language, need a pointer to the impl struct, not a
// reference. Need to have some wrapper class that then calls into each
// rust trait implementation.
impl Algorithm for Demo {
    #[no_mangle]
    extern "C" fn create() -> Self {
        Self {
            is_running: false,
            name: "Demo".to_owned(),
        }
    }

    #[no_mangle]
    extern "C" fn on_register(&mut self) {
        println!("Registering {}...", self.name);
    }

    #[no_mangle]
    extern "C" fn on_system_start(&mut self) {
        if self.is_running {
            panic!("Tried to start, but already running.");
        }
        println!("Starting {}...", self.name);
        self.is_running = true;
    }

    #[no_mangle]
    extern "C" fn main_thread_event(&mut self) {
        println!("Processing main event");
    }

    #[no_mangle]
    extern "C" fn background_thread_event(&mut self) {
        println!("Processing background event");
    }

    #[no_mangle]
    extern "C" fn on_system_stop(&mut self) {
        println!("Stopping {}...", self.name);
        self.is_running = false;
    }

    #[no_mangle]
    extern "C" fn on_unregister(&mut self) {
        println!("Unregistering {}...", self.name);
    }
}
