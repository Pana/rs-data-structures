

pub trait Stack<T> {
    fn push(&mut self, val: T) -> bool;
    fn pop(&mut self) -> Option<T>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn print(&self);
}