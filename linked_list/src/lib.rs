pub mod linked_list;

pub trait LinkedList<T> {
    fn is_empty(&self) -> bool;
    fn clear(&self);
    fn len(&self) -> usize;
    fn print(&self);
    fn insert(&mut self, val: T, index: usize) -> bool;
    fn remove(&mut self, val: T, index: usize) -> bool;
    fn search(&self, val: T) -> bool;
}