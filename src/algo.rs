pub trait Algorithm {
    extern "C" fn create() -> Self;
    extern "C" fn on_register(&mut self);
    extern "C" fn on_system_start(&mut self);
    extern "C" fn main_thread_event(&mut self);
    extern "C" fn background_thread_event(&mut self);
    extern "C" fn on_system_stop(&mut self);
    extern "C" fn on_unregister(&mut self);
}
