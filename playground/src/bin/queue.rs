///
/// a thread safe queue
/// 
use anyhow::Result;
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use tokio::time::{sleep, Duration, Instant};

#[derive(Debug, Default, Clone)]
pub struct Queue<T> {
    pub queue: Arc<RwLock<VecDeque<T>>>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            queue: Arc::new(RwLock::new(VecDeque::new())),
        }
    }

    /// push an element to the back of the queue
    pub fn push(&mut self, value: T) {
        let mut q = self.queue.write().unwrap();
        q.push_back(value);
    }

    /// pop an element from the front of the queue
    pub fn pop(&mut self) -> Option<T> {
        let mut q = self.queue.write().unwrap();
        q.pop_front()
    }

    /// return the number of elements in the queue
    pub fn len(&self) -> usize {
        let q = self.queue.read().unwrap();
        q.len()
    }

    /// return true if the queue is empty, else false
    pub fn is_empty(&self) -> bool {
        let q = self.queue.read().unwrap();
        q.is_empty()
    }

}

#[derive(Debug, Clone)]
struct Job {
    name: String,
    delay: u64,
}

impl Job {
    pub fn create(name: &str, delay: u64) -> Job {
        Job {
            name: name.to_string(),
            delay,
        }
    }

    /// run the job...
    pub async fn run(&self) -> Result<()> {
        sleep(Duration::from_millis(self.delay)).await;
        println!("Job '{}' completed after {} milliseconds.", self.name, self.delay);

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let jobs = vec![
        Job::create("first job", 1_500),
        Job::create("second job", 350),
        Job::create("job 3", 750),
        Job::create("job 4", 650),
        Job::create("job 5", 850),
    ];

    let count = jobs.len();
    let now = Instant::now();
    for job in jobs {
        job.run().await?;
    }

    println!("It took {:?} to complete {} jobs.", now.elapsed(), count);

    Ok(())
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

        let value = queue.pop().unwrap();
        assert_eq!(value, 13);
    }
}