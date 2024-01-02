use std::net::{UdpSocket, SocketAddr};

fn main() {

    let port = 9001;
    println!("starting udp client on port {}", port);
    let server_addr = SocketAddr::from(([10, 0, 1, 237], port));
    let socket = UdpSocket::bind(SocketAddr::from(([0,0,0,0], 0))).expect("connect failed");

    let msg = "hello from client";
    socket.send_to(msg.as_bytes(), server_addr).expect("failed to send");

    let mut buf = [0; 1024];
    let (amt, _src_addr) = socket.recv_from(&mut buf).expect("Failed to receive response");
    let response = String::from_utf8_lossy(&buf[..amt]);

    println!("recvd {}", response);
    
}
