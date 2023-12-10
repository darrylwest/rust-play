//
//
//

use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::UdpSocket;

/// pull out the handler
pub async fn start() -> Result<()> {
    let addr = "0.0.0.0:22200";
    println!("listening on: {}", addr);

    let sock = UdpSocket::bind(addr).await?;
    loop {
        // listen for a message
        let mut buf = [0; 128];
        println!("wait for a connection...");

        let (len, addr) = sock.recv_from(&mut buf).await?;
        let msg = String::from_utf8_lossy(&buf[..len]);
        let msg = msg.trim();

        println!("recv: {} bytes from {:?}, msg: {}", len, addr, msg);
        // split this into [cmd, param, param]
        let params: Vec<&str> = msg.split(' ').collect();
        let response = handle_request(params);

        // return the response
        let len = sock.send_to(response.as_bytes(), addr).await?;
        println!("returned: {:?}, size {}.", response, len);
    }
}

// move this to lib

fn handle_request(request: Vec<&str>) -> String {
    // parse the message to create a response
    match request[0] {
        "/get" => {
            let key = request[1];
            key.to_string()
        }
        "/now" => {
            let tm = get_now();
            tm.to_string()
        }
        "/ms" => {
            let tm = get_now_ms();
            tm.to_string()
        }
        "/ping" => String::from("pong\r\n"),
        _ => String::from("error\r\n"),
    }
}

fn get_now() -> String {
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        _ => 0_u64,
    };

    format!("{}\r\n", now)
}

fn get_now_ms() -> String {
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis(),
        _ => 0_u128,
    };

    format!("{}\r\n", now)
}
