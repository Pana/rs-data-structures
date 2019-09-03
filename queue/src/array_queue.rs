

struct ArrayQueue {
    front: i32,
    rear: i32,
    q: Vec<i32>,
}

impl Queue for ArrayQueue {
    fn new() -> Self {
        ArrayQueue{
            q: Vec::new(),
            front: 0,
            rear: 0,
        }
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn put(&self, val: i32) {
        self.q.push(val);
        self.rear = self.q.len();
    }

    fn poll(&self) -> i32 {
        
    }

    fn clear(&self) {

    }

    fn len(&self) -> i32 {
        100
    }

    fn print() {
        println!("This is a queue")
    }
}