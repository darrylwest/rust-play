use tokio::time::{timeout, Duration, sleep};

async fn long_future() {
    println!("working...");
    let fut = sleep(Duration::from_secs(10)).await;
    println!("complete.");

    fut
}

#[tokio::main]
async fn main() {
    let res = timeout(Duration::from_secs(5), long_future()).await;

    if res.is_err() {
        println!("op timed out...");
    }
}
