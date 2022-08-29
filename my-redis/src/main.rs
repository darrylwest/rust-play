use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> { 
    println!("connect to mini-redis...");
    let mut client = client::connect("127.0.0.1:6379").await?:

    println!("set key hello to value world...");
    client.set("hello", "world".into()).await?r;

    println("read it back...");
    let result = client.get("hello").await?;

    println("got value, result: {:?}", result);

    Ok(())
}
