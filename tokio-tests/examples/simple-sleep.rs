use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut ms = 500;
    loop {
        sleep(Duration::from_millis(500)).await;
        println!("{} ms have elapsed", ms);
        ms += 500;
    }
}
