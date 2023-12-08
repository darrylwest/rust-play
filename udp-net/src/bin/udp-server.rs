//
// test withnet cat: echo "/ping" | nc -w 1 -u <ip> <port>
//
use anyhow::Result;
use udp_net::server::start;

#[tokio::main]
async fn main() -> Result<()> {
    start().await
}
