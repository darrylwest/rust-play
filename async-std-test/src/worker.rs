/// general purpose worker
// use anyhow::Result;
use anyhow::Result;
use async_channel::bounded;
use async_channel::{Receiver, Sender};
use log::*;
use serde::{Deserialize, Serialize};
use std::iter::repeat_with;
use std::time::Instant;

use crate::JsonString;

#[derive(Debug, Clone)]
pub enum Command {
    Status(Sender<JsonString>), // request the worker's status
    Shutdown,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum WorkerState {
    #[default]
    Idle,
    Busy,
    Shutdown,
}

#[derive(Debug, Clone)]
pub struct Worker {
    id: String,
    started_at: Instant,
    request_tx: Sender<Command>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerStatus {
    status: String,
    state: WorkerState,
    uptime: String,
    error_count: u16,
}

//
impl Worker {
    /// create and start a new worker.
    pub async fn new() -> Worker {
        let started_at = Instant::now();
        let id: String = repeat_with(fastrand::alphanumeric).take(10).collect();

        // this is for the worker struct
        let wid = id.clone();

        info!("starting up worker, id: {}", id);

        let (request_tx, request_receiver) = bounded(250);

        // run the handler loop as a background task
        async_std::task::spawn(async move {
            match handler(id.clone(), request_receiver).await {
                Ok(()) => info!("worker handler exit for worker id: {}", id),
                Err(e) => error!("worker exex with error: {:?}", e),
            }
        });

        // define here, before the async loop to ensure it does not get moved
        let worker = Worker {
            id: wid,
            started_at,
            request_tx,
        };

        info!("worker created: {:?}", &worker);

        // return this worker
        worker
    }

    /// return the worker's id
    pub fn id(&self) -> String {
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

// the handler loop
async fn handler(id: String, rx: Receiver<Command>) -> Result<()> {
    // let mut meta: Vec<Pairs>
    let mut state = WorkerState::Idle;
    let mut error_count = 0;
    // now read and respond to requests
    while let Ok(cmd) = rx.recv().await {
        info!("recv cmd: {:?}", cmd);
        match cmd {
            Command::Status(tx) => {
                let status = WorkerStatus {
                    status: "ok".to_string(),
                    state: state.clone(),
                    uptime: String::new(),
                    error_count,
                    // meta,
                };

                let msg = match serde_json::to_string(&status) {
                    Ok(js) => js,
                    Err(e) => {
                        format!(r#"{}"status":"json parse error: {:?}"{}"#, "{", e, "}\n")
                    }
                };

                info!("status response: {}", msg);
                if tx.send(msg).await.is_err() {
                    error_count += 1;
                    error!("error returning status to channel: {:?}", tx);
                }
            }
            Command::Shutdown => {
                state = WorkerState::Shutdown;
                info!("worker id: {}, state: {:?}", id, state);
                break;
            }
        }
    }

    rx.close();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        async_std::task::block_on(async move {
            let worker = Worker::new().await;
            println!("worker: {:?}", worker);

            assert_eq!(worker.id().len(), 10);
            assert_eq!(worker.get_uptime(), 0);

            let (send, response_channel) = async_channel::unbounded();

            let request_channel = worker.request_channel();

            let cmd = Command::Status(send);
            let ok = request_channel.send(cmd).await.is_ok();
            assert_eq!(ok, true);
            let resp = response_channel
                .recv()
                .await
                .expect("should respond to status request");
            println!("[t] status response: {}", resp);
            assert!(resp.len() > 6);

            let cmd = Command::Shutdown;
            let ok = request_channel.send(cmd).await.is_ok();
            assert_eq!(ok, true);
        });
    }
}
