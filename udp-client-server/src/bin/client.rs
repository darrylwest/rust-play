use std::net::{UdpSocket, SocketAddr};
use std::time::Duration;

fn main() {

    let port = 9001;
    let server_host = [10, 0, 1, 237];
    println!("starting udp client on host: {:?}, port {}", server_host, port);

    let server_addr = SocketAddr::from((server_host, port));
    let socket = UdpSocket::bind(SocketAddr::from(([0,0,0,0], 0))).expect("connect failed");

    socket.set_read_timeout(Some(Duration::from_secs(2))).expect("set timeout failed");
    socket.set_write_timeout(Some(Duration::from_secs(2))).expect("set timeout failed");

    let msg = "hello from client";
    socket.send_to(msg.as_bytes(), server_addr).expect("failed to send");

    let mut buf = [0; 1024];
    let (amt, _src_addr) = socket.recv_from(&mut buf).expect("Failed to receive response");
    let response = String::from_utf8_lossy(&buf[..amt]);

    println!("recvd {}", response);
    
}
