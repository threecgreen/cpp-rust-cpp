pub trait Algorithm {
    extern fn create() -> Self;
    extern fn on_register(&mut self);
    extern fn on_system_start(&mut self);
    extern fn main_thread_event(&mut self);
    extern fn background_thread_event(&mut self);
    extern fn on_system_stop(&mut self);
    extern fn on_unregister(&mut self);
}
