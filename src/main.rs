use std::collections::VecDeque;

struct ToQueue<T> {
    queue: VecDeque<T>
}

impl<T> ToQueue<T> {
    fn new() -> Self {
        ToQueue {
            queue: VecDeque::new()
        }
    }

    fn push(&mut self, item: T) {
        self.queue.push_back(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn size(&self) -> usize {
        self.queue.len()
    }
}


fn main() {
    let mut queue = ToQueue::new();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    println!("Queue size: {}", queue.size());
    println!("Queue is empty: {}", queue.is_empty());
    println!("Popped item: {}", queue.pop().unwrap());
    println!("Queue size: {}", queue.size());
    println!("Queue is empty: {}", queue.is_empty());
}
