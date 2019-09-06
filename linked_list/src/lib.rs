

pub trait LinkedList<T> {
    fn is_empty(&self) -> bool;
    fn push(&mut self, val: T) -> bool;
    fn pop(&mut self) -> Option<T>;
    fn clear(&self);
    fn len(&self) -> usize;
    fn print(&self);
}