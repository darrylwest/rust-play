///
/// a thread safe queue
/// 
use std::collections::VecDeque;

#[derive(Debug, Default, Clone)]
pub struct Queue<T> {
    pub queue: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            queue: VecDeque::new(),
        }
    }

    /// push an element to the back of the queue
    pub fn push(&mut self, value: T) {
        self.queue.push_back(value);
    }

    /// pop an element from the front of the queue
    pub fn pop(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    /// return the number of elements in the queue
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// return true if the queue is empty, else false
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

}

fn main() {
    println!("TODO: use Arc and RwLock to create thread safety...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let queue: Queue<u16> = Queue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        let mut queue = queue.clone();
        queue.push(13);
        assert_eq!(queue.len(), 1);
    }
}