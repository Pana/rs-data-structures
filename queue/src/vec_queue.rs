use crate::Queue;
use std::fmt;

// implement with std vector
#[derive(Debug)]
pub struct VecQueue<T> {
    q: Vec<T>,
}

impl<T> VecQueue<T> {
    #[allow(dead_code)]
    fn new() -> Self {
        VecQueue{
            q: Vec::new(),
        }
    }
}

impl<T: fmt::Debug> Queue<T> for VecQueue<T> {
    fn is_empty(&self) -> bool {
        self.q.len() == 0
    }

    fn put(&mut self, val: T) -> bool {
        self.q.push(val);
        true
    }

    /// will return a Option<T> 
    fn poll(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.q.remove(0))
        }
    }

    fn clear(&self) {
        // TODO
    }

    fn len(&self) -> usize {
        self.q.len()
    }

    fn print(&self) {
        for i in self.q.iter() {
            println!("Element {:?}", i)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Queue;
    
    #[test]
    fn test_new() {
        let mut q: super::VecQueue<i32> = super::VecQueue::new();
        assert_eq!(q.len(), 0);
        q.put(1);
        assert_eq!(q.len(), 1);
        let val = q.poll();
        assert_eq!(val, Some(1));
        let val2 = q.poll();
        assert_eq!(val2, None);
    }
}