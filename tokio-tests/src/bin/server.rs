// creates a mini kv server using components from mini_redis for connection and frames
// uses arc/mutex to control concurrency

use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

use bytes::Bytes;
use std::collections::HashMap;

// use tracing::instrument;

// TODO consider using parking_log::Mutex ...
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    let _ = tracing::subscriber::set_global_default(subscriber);

    let host = "127.0.0.1:6379";
    let listener = TcpListener::bind(host).await.unwrap();

    println!("listening on {}", host);

    // TODO replace HashMap with dash map
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();

        println!("new connection: {:?}", socket);

        // creates a green thread or a task using 64 bytes of overhead...
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

#[tracing::instrument]
async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                println!("cmd: {:?}", cmd);
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                println!("db: {:?}", db.clone());

                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                println!("cmd: {:?}", cmd);
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented command: {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
