mod queue;
use crate::queue::Queue;

fn main() {
    let mut q: queue::VecQueue<i32> = queue::VecQueue::new();

    q.put(1);
    q.put(2);

    let b = q.poll();
    println!("poped value {:?}", b);
    
    q.print();
}


