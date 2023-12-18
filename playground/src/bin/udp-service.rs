
// use anyhow::Result;
use tokio::net::UdpSocket;
use std::io;
use service_uptime::status::ServiceStatus;

// test with echo "ping" | nc -w 1 -u 127.0.0.1 34254

struct Handler {
    status: ServiceStatus,
}

impl Handler {
    fn create() -> Handler {
        Handler{ status: ServiceStatus::create() }
    }

    // replace this with a plugin handler; RPC engine, etc.
    async fn handle_request(&mut self, msg: &str) -> String {
        self.status.access.incr();
        let mut response = String::new();

        match msg {
            "help" => response.push_str("status, random, ping"),
            "status" => response.push_str(&format!("{}", self.status)),
            "random" => response.push_str(&fastrand::u64(10_000_000..100_000_000).to_string()),
            "ping" => response.push_str("PONG"),
            _ => {
                self.status.errors.incr();
                response.push_str("what")
            }
        };

        response
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut handler = Handler::create();
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
                    let response = handler.handle_request(msg).await;
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
