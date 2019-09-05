
pub mod vec_queue;
pub mod loop_queue;
pub mod linked_list_queue;

/// The queue data structure: enable put&poll
pub trait Queue<T> {
    fn is_empty(&self) -> bool;
    fn put(&mut self, val: T) -> bool;
    fn poll(&mut self) -> Option<T>;
    fn clear(&self);
    fn len(&self) -> usize;
    fn print(&self);
}




