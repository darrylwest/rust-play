
// use anyhow::Result;
use tokio::net::UdpSocket;
use std::io;

// test with echo "ping" | nc -w 1 -u 127.0.0.1 34254

async fn handle_request(msg: &str) -> String {
    println!("request: {}", msg);

    "PONG".to_string()
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // let client_socket = UdpSocket::bind("127.0.0.1:34255").await?;
    let addr = "127.0.0.1:34254";
    let server_socket = UdpSocket::bind(addr).await?;
    println!("server listening on udp port: {}", addr);

    let server_task = tokio::spawn(async move {
        // Server socket code goes here
        loop {
            let mut buf = [0; 64];

            match server_socket.recv_from(&mut buf).await {
                Ok((len, _addr)) => {
                    let msg = String::from_utf8_lossy(&buf[..len]);
                    let msg = msg.trim();
                    println!("server recv: {}", msg);

                    // shutdown
                    if msg == "shutdown" {
                        println!("out...");
                        break;
                    }

                    // handle the message
                    let response = handle_request(msg).await;
                    println!("{}", response);
                },
                Err(e) => {
                    println!("ERROR: {}", e);
                    break;
                }
            }

        }
    });

    server_task.await?;

    Ok(())
}

