use anyhow::Result;
use tokio::time;

async fn my_task(num: u8) -> Result<u16> {
    let x = num * 10;
    println!("task {} -> {}", num, x);

    time::sleep(time::Duration::from_millis(1)).await;

    Ok(x as u16)
}

#[tokio::main]
async fn main() {
    let mut interval = time::interval(time::Duration::from_millis(1000));

    for idx in 1..=5 {
        interval.tick().await;

        println!("request# {}", idx);

        let r = my_task(idx as u8).await;

        println!("response: {:?}", r);
    }
}