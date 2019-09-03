

trait Queue {
    fn new() -> Self;
    fn is_empty(&self) -> bool;
    fn put(&self, a: i32);
    fn poll(&self) -> i32;
    fn clear(&self);
    fn len(&self) -> i32;
    fn print();
}