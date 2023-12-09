use anyhow::Result;
use std::net::UdpSocket;

fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.set_write_timeout(Some(std::time::Duration::new(5, 0)))?;
    socket.set_read_timeout(Some(std::time::Duration::new(5, 0)))?;

    let server_address = "127.0.0.1:22200";
    let message = b"/get 123456";
    socket.send_to(message, server_address)?;

    let mut buffer = [0; 1024];
    let (amt, _) = socket.recv_from(&mut buffer)?;
    println!(
        "Received message: {}",
        String::from_utf8_lossy(&buffer[..amt])
    );

    Ok(())
}
