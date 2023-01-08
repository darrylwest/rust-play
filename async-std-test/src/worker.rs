use async_channel::bounded;
use async_channel::Sender;
/// general purpose worker
// use anyhow::Result;
use log::info;
use std::time::Instant;

#[derive(Debug, Default, Clone)]
pub enum Command {
    #[default]
    Status, // request the worker's status
}

#[derive(Debug, Default, Clone)]
pub enum WorkerState {
    #[default]
    Idle,
    Busy,
}

#[derive(Debug, Clone)]
pub struct Worker {
    id: String,
    started_at: Instant,
    request_tx: Sender<Command>,
}

impl Worker {
    /// create and start a new worker.
    pub async fn new() -> Worker {
        let started_at = Instant::now();
        let id = "worker-1";
        let state = WorkerState::Idle;

        info!("starting up worker, id: {}", id);

        let (tx, request_receiver) = bounded(250);

        let worker = Worker {
            id: id.to_string(),
            started_at,
            request_tx: tx,
        };

        async_std::task::spawn(async move {
            // now read and respond to requests

            request_receiver.close();
        });

        info!("worker created: {:?}, state: {:?}", &worker, state);

        // return a reference to this worker
        worker
    }

    /// return the worker's id
    pub fn worker_id(&self) -> String {
        self.id.to_string()
    }

    /// return the number of seconds this worker has been alive
    pub fn get_uptime(&self) -> u64 {
        let secs = self.started_at.elapsed().as_secs();

        info!("uptime {} seconds", secs);

        secs
    }

    /// This is invoked by the client to enable sending command request to
    /// the worker
    pub fn request_channel(&self) -> Sender<Command> {
        self.request_tx.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        async_std::task::block_on(async move {
            let worker = Worker::new().await;
            println!("worker: {:?}", worker);

            assert_eq!(worker.get_uptime(), 0);
        });
    }
}
