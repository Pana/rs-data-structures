use crate::Queue;
use std::marker;

// TODO make the container size dynamic, maybe can use macro realize it
pub struct LoopQueue<T> {
    front: usize,
    rear: usize,
    queue: [T; 100],
}

impl<T: marker::Copy> LoopQueue<T> {
    #[allow(dead_code)]
    fn new(v: T) -> Self {
        LoopQueue{
            front: 0,
            rear: 0,
            queue: [v; 100],
        }
    }
}

impl<T: marker::Copy> Queue<T> for LoopQueue<T> {
    fn is_empty(&self) -> bool {
        self.front == self.rear
    }

    fn put(&mut self, val: T) -> bool {
        let next = (self.rear + 1) % self.queue.len();
        // if queue is full
        if next == self.front {
            return false;
        }
        self.queue[self.rear] = val;
        self.rear = next;
        true
    }

    fn poll(&mut self) -> Option<T> {
        if self.front == self.rear { // check queue is empty
            None
        } else {
            let val = self.queue[self.front];
            self.front = (self.front + 1) % self.queue.len();
            Some(val)
        }
    }

    fn clear(&self) {

    }

    fn len(&self) -> usize {
        let len = self.queue.len();
        (self.rear - self.front + len) % len
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
        let mut q: super::LoopQueue<i32> = super::LoopQueue::new(1);
        assert_eq!(q.len(), 0);
        q.put(1);
    }
}