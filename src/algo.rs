use crate::cpp;

pub trait Algorithm {
    fn create(logger_ptr: *mut cpp::Logger) -> *mut Self;
    fn destroy(algo_ptr: *mut Self);
    fn on_register(&mut self);
    fn on_system_start(&mut self);
    fn main_thread_event(&mut self);
    fn background_thread_event(&mut self);
    fn on_system_stop(&mut self);
    fn on_unregister(&mut self);
}
