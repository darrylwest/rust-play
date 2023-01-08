/// general purpose worker
// use anyhow::Result;
use async_channel::bounded;
use async_channel::Sender;

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
    pub state: WorkerState,
    request_tx: Sender<Command>,
}

impl Worker {
    pub async fn new() -> Worker {
        let (tx, request_receiver) = bounded(250);

        // now
        async_std::task::spawn(async move {
            request_receiver.close();
        });

        Worker {
            state: WorkerState::Idle,
            request_tx: tx,
        }
    }

    /// This is invoked by the client to enable sending command request to
    /// the worker
    pub fn request_channel(&self) -> Sender<Command> {
        self.request_tx.clone()
    }
}
