use tokio::sync::oneshot;

fn do_work() {
    use std::thread;
    use std::time::Duration;

    for n in 1..=9 {
        println!("{} - working...", n);
        thread::sleep(Duration::from_millis(250));
    }
}

async fn sender(tx: oneshot::Sender<String>, msg: String) {
    do_work();

    if let Err(_) = tx.send(msg) {
        eprint!("the recever dropped");
    }
}

async fn manager() {
    let (tx, rx) = oneshot::channel();

    let msg = String::from("this is a test message");

    tokio::spawn(async move {
        sender(tx, msg).await;
    });

    waiter(rx).await;
}

async fn waiter(rx: oneshot::Receiver<String>) {
    match rx.await {
        Ok(v) => println!("got {:?}", v),
        Err(_) => eprint!("the sender dropped"),
    }
}

#[tokio::main]
async fn main() {
    manager().await;
}
