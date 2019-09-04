use crate::Queue;
use std::marker;
// use array_macro::array;

pub struct ArrayQueue<T> {
    front: usize,
    rear: usize,
    container: [T; 100]
}

impl<T: marker::Copy> Queue<T> for ArrayQueue<T> {
    fn new() -> Self {
        ArrayQueue{
            front: 0,
            rear: 0,
            container: array![T; 100],
        }
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn put(&mut self, val: T) {

    }

    fn poll(&mut self) -> Option<T> {
        None
    }

    fn clear(&self) {

    }

    fn len(&self) -> usize {
        0
    }

    fn print(&self) {
        println!("This is a ArrayQueue")
    }
}

#[cfg(test)]
mod tests {
    use crate::Queue;

    #[test]
    fn test_new() {
        let mut q: super::ArrayQueue<i32> = super::ArrayQueue::new();
        assert_eq!(q.len(), 0);
    }
}