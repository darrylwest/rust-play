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
}

/// runner is not an impl of Job but a stand-alone to solve ownership errors.
async fn run(job: Job) -> Result<Job> {
    sleep(Duration::from_millis(job.delay)).await;
    println!("Job '{}' completed after {} milliseconds.", job.name, job.delay);

    Ok(job.clone())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut jobs = Queue::new();
    jobs.push(Job::create("first job", 500));
    jobs.push(Job::create("second job", 1_150));
    jobs.push(Job::create("job 3", 750));
    jobs.push(Job::create("job 4", 650));
    jobs.push(Job::create("job 5", 850));

    let count = jobs.len();
    let mut handles = vec![];
    let now = Instant::now();

    loop {
        if let Some(job) = jobs.pop() {
            handles.push(tokio::spawn({
                run(job.clone())
            }));
        } else {
            break;
        }
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await?);
    }

    println!("\nIt took {:?} to complete {} jobs.", now.elapsed(), count);

    println!("\nJob results, available to re-run if necessary...");
    for job in results {
        println!("job: {:?}", job);
    }

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