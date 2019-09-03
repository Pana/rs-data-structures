pub trait Queue<T> {
    fn new() -> Self;
    fn is_empty(&self) -> bool;
    fn put(&mut self, val: T);
    fn poll(&mut self) -> Option<T>;
    fn clear(&self);
    fn len(&self) -> usize;
    fn print(&self);
}

pub struct VecQueue<T> {
    q: Vec<T>,
}

impl<T: std::fmt::Debug> Queue<T> for VecQueue<T> {
    fn new() -> Self {
        VecQueue{
            q: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.q.len() == 0
    }

    fn put(&mut self, val: T) {
        self.q.push(val);
    }

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