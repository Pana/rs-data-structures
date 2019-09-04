
pub mod vec_queue;
pub mod array_queue;

/// The queue data structure: enable put&poll
pub trait Queue<T> {
    fn new() -> Self;
    fn is_empty(&self) -> bool;
    fn put(&mut self, val: T);
    fn poll(&mut self) -> Option<T>;
    fn clear(&self);
    fn len(&self) -> usize;
    fn print(&self);
}




