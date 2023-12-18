
// use anyhow::Result;
use tokio::net::UdpSocket;
use std::io;

// test with echo "ping" | nc -w 1 -u 127.0.0.1 34254

// replace this with a plugin handler
async fn handle_request(msg: &str) -> String {
    let resp = match msg {
        "code" => "ok",
        _ => "PONG",
    };

    resp.to_string()
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // let client_socket = UdpSocket::bind("127.0.0.1:34255").await?;
    let addr = "0.0.0.0:34254";
    let sock = UdpSocket::bind(addr).await?;
    println!("server listening on udp port: {}", addr);

    let server_task = tokio::spawn(async move {
        // Server socket code goes here
        let mut count: usize = 0;
        loop {
            let mut buf = [0; 64];

            match sock.recv_from(&mut buf).await {
                Ok((len, addr)) => {
                    let msg = String::from_utf8_lossy(&buf[..len]);
                    let msg = msg.trim();

                    count += 1;
                    print!("{} > server recv: {} -> ", count, msg);

                    // shutdown
                    if msg == "shutdown" {
                        println!("out...");
                        break;
                    }

                    // handle the message
                    let response = handle_request(msg).await;
                    println!("{}", response);

                    let _sz = sock.send_to(format!("{}\r\n", response).as_bytes(), addr).await;
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
