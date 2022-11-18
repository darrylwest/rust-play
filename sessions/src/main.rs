use domain_keys::keys::RouteKey;
use sessions::sessions::{Command, Sessions};
// use tokio::sync::mpsc;
use tokio::sync::oneshot;
// use tokio::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("query sessions");

    let mut sessions = Arc::new(Sessions::new());

    println!("{:?}", sessions);

    // let (req_tx, mut req_rx): (Sender<Command>, Receiver<Command>) = mpsc::channel(8);
    for idx in 0..9 {
        let ss = format!("{}-{}", idx, RouteKey::create());

        let cmd = match idx {
            0 => Command::Insert(ss),
            1 => Command::Find(ss),
            3 => Command::Remove(ss),
            _ => Command::Insert(ss),
        };

        println!("cmd: {:?}", cmd);

        let (resp_tx, resp_rx) = oneshot::channel::<bool>();

        if let Some(sess) = Arc::get_mut(&mut sessions) {
            sess.process(cmd, resp_tx).await;

            let resp = resp_rx.await;
            println!("resp: {:?}", resp);
            println!("session: {:?}", sessions);
        } else {
            eprintln!("error could not access sessions")
        }
    }
}
