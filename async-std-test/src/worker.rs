use crate::JsonString;
/// general purpose worker
// use anyhow::Result;
use anyhow::Result;
use async_channel::bounded;
use async_channel::{Receiver, Sender};
use domain_keys::keys::RouteKey;
use log::*;
use serde::{Deserialize, Serialize};
use service_uptime::Uptime;

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
    uptime: Uptime,
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
        let uptime = Uptime::new();
        let id = RouteKey::create();

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
            uptime,
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
    pub fn get_uptime(&self) -> String {
        self.uptime.to_string()
    }

    pub fn get_update_seconds(&self) -> u64 {
        self.uptime.get_uptime_seconds()
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
    let uptime = Uptime::new();
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
                    uptime: uptime.to_string(),
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

            assert_eq!(worker.id().len(), 16);
            assert_eq!(worker.get_update_seconds(), 0);
            println!("{}", worker.get_uptime());

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

    #[test]
    fn bounded_tests() {
        async_std::task::block_on(async move {
            let (s, r) = async_channel::bounded(2);
            assert_eq!(r.is_empty(), true);
            assert_eq!(s.send(10).await, Ok(()));
            assert_eq!(s.send(12).await, Ok(()));

            assert_eq!(r.is_full(), true);
            assert_eq!(r.recv().await, Ok(10));
            assert_eq!(r.recv().await, Ok(12));
            assert_eq!(r.is_empty(), true);

            // second test
            println!("r empty? {}", r.is_empty());
            assert_eq!(s.send(14).await, Ok(()));
            assert_eq!(s.send(16).await, Ok(()));

            // if you try sending more than the buffer allows here, the
            // process will just wait until there is room in the queue

            println!("r empty? {}", r.is_empty());
            assert_eq!(r.recv().await, Ok(14));
            assert_eq!(r.recv().await, Ok(16));

            assert_eq!(s.close(), true);
            assert_eq!(s.is_closed(), true);

            // closing the sender shuts down the receiver as well
            assert_eq!(r.is_closed(), true);

            match r.recv().await {
                Ok(_) => assert!(false, "should not work here"),
                Err(e) => {
                    println!("error: {:?}", e);
                    assert!(true);
                }
            }
        });
    }
}
