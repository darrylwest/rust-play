/// general purpose worker
// use anyhow::Result;
use async_channel::bounded;
use async_channel::Sender;
use log::{error, info};
use std::iter::repeat_with;
use std::time::Instant;

use crate::JsonString;

#[derive(Debug, Clone)]
pub enum Command {
    Status(Sender<JsonString>), // request the worker's status
    Shutdown,
}

#[derive(Debug, Default, Clone)]
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

impl Worker {
    /// create and start a new worker.
    pub async fn new() -> Worker {
        let started_at = Instant::now();
        let id: String = repeat_with(fastrand::alphanumeric).take(10).collect();

        // this is for the worker struct
        let wid = id.clone();

        info!("starting up worker, id: {}", id);

        let (tx, request_receiver) = bounded(250);

        async_std::task::spawn(async move {
            let mut state = WorkerState::Idle;
            let mut error_count = 0;
            // now read and respond to requests
            while let Ok(cmd) = request_receiver.recv().await {
                info!("recv cmd: {:?}", cmd);
                match cmd {
                    Command::Status(tx) => {
                        let msg = format!(
                            r#"{}"status":"{}","state":"{:?}","error_count":{}{}"#,
                            "{",
                            "Ok",
                            state.clone(),
                            error_count,
                            "}\n",
                        );

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

            request_receiver.close();
        });

        // define here, before the async loop to ensure it does not get moved
        let worker = Worker {
            id: wid,
            started_at,
            request_tx: tx,
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
