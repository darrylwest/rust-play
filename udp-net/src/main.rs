
use tokio::net::UdpSocket;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use std::borrow::Cow;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "0.0.0.0:22200";
    println!("listening on: {}", addr);

    let sock = UdpSocket::bind(addr).await?;
    loop {
        // listen for a message
        let mut buf = [0; 128];
        println!("wait for a connection...");

        let (len, addr) = sock.recv_from(&mut buf).await?;
        let msg = String::from_utf8_lossy(&buf[..len]);
        println!("recv: {} bytes from {:?}, msg: {}", len, addr, msg);

        // parse the message to create a response
        let response = match msg {
            Cow::Borrowed("/now\n") => {
                let tm = get_now();
                tm.to_string()
            },
            Cow::Borrowed("/ms\n") => {
                let tm = get_now_ms();
                tm.to_string()
            },
            Cow::Borrowed("/ping\n") => String::from("pong\r\n"),
            _ => String::from("error\r\n"),
        };


        // return the response
        let len = sock.send_to(response.as_bytes(), addr).await?;
        println!("returned: {:?}, size {}.", response, len);
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
