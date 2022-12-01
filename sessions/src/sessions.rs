// TODO: refactor this to use request/response channels
use std::collections::HashSet;
use tokio::sync::oneshot;

#[derive(Debug, Clone)]
pub enum Command {
    Find(String), // add response channel
    Insert(String), // add response channel
    Remove(String), // add response channel
}

#[derive(Debug, Default, Clone)]
pub struct Sessions {
    list: HashSet<String>,
}

impl Sessions {
    pub fn new() -> Sessions {
        let list = HashSet::new();

        Sessions { list }
    }

    pub async fn process(&mut self, cmd: Command, responder: oneshot::Sender<bool>) {
        match cmd {
            Command::Find(ss) => {
                // list.contains(&ss)
                println!("find: {}", ss);
                let r = self.list.contains(&ss);

                responder.send(r).unwrap();
            }
            Command::Insert(ss) => {
                // list.insert(ss)
                println!("insert: {}", ss);
                let r = self.list.insert(ss);

                responder.send(r).unwrap();
            }
            Command::Remove(ss) => {
                // list.remove(&ss)
                println!("remove: {}", ss);
                let r = self.list.remove(&ss);

                responder.send(r).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::sync::mpsc;
    // use std::sync::mpsc::{Sender, Receiver};
    // use domain_keys::keys::RouteKey;

    #[test]
    fn new() {
        let sessions = Sessions::new();

        assert!(sessions.list.is_empty());
        println!("{:?}", sessions);

        /*
        let (req_tx, req_rx): (Sender<Command>, Receiver<Command>) = mpsc::channel();
        let (resp_tx, resp_rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();

        let key = RouteKey::create();
        println!("key: {}", key);

        let mut ss = sessions.clone();

        ss.exec(req_rx, resp_tx);
        if req_tx.send(Command::Insert(key)).is_ok() {
            let resp = resp_rx.recv();
            println!("{:?}", resp);
        } else {
            eprintln!("channel down");
            assert!(false);
        }
        */
    }
}
