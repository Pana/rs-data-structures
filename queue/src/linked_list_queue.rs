use crate::Queue;
use std::collections::LinkedList;

pub struct LinkedListQueue<T> {
    q: LinkedList<T>,
    len: usize,
}

impl<T> LinkedListQueue<T> {
    #[allow(dead_code)]
    fn new() -> Self {
        let q: LinkedList<T> = LinkedList::new();
        LinkedListQueue{
            q: q,
            len: 0,
        }
    }
}

impl<T> Queue<T> for LinkedListQueue<T> {
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn len(&self) -> usize {
        self.len
    }

    fn put(&mut self, val: T) -> bool {
        self.len = self.len + 1;
        self.q.push_back(val);
        true
    }

    fn poll(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            self.q.pop_front()
        }
    }

    fn print(&self) {
        println!("Print LinkedListQueue")
    }

    fn clear(&self) {

    }
}

#[cfg(test)]
mod tests {
    use crate::Queue;

    #[test]
    fn test_new() {
        let mut q: super::LinkedListQueue<i32> = super::LinkedListQueue::new();
        assert_eq!(q.len(), 0);
        q.put(1);
        assert_eq!(q.len(), 1);
        q.put(2);
        let b = q.poll();
        assert_eq!(b, Some(1));
    }
}