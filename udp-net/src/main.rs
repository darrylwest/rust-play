
use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "0.0.0.0:22200";
    println!("listening on: {}", addr);

    let sock = UdpSocket::bind(addr).await?;
    loop {
        // listen for a message
        let mut buf = [0; 1024];
        println!("wait for a connection...");

        let (len, addr) = sock.recv_from(&mut buf).await?;
        let msg = String::from_utf8_lossy(&buf[..len]);
        println!("recv: {} bytes from {:?}, msg: {}", len, addr, msg);

        // parse the message to create a response
        let response = "ok\r\n";

        // return the response
        let len = sock.send_to(response.as_bytes(), addr).await?;
        println!("returned: {:?}, size {}.", response, len);
    }
}