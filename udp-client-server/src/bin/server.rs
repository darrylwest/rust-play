use std::net::{UdpSocket, SocketAddr};
// use std::io::{Error, ErrorKind};

fn main() {
    let port = 9001;
    println!("starting udp server on port {}", port);

    let server_addr = SocketAddr::from(([10, 0, 1, 237], port));
    let socket = UdpSocket::bind(server_addr).expect("failed to connect");

    let mut buf = [0; 1024];
    let (sz, src_addr) = socket.recv_from(&mut buf).expect("failed received message");

    let msg = String::from_utf8_lossy(&buf[..sz]);
    println!("recvd from {}: {}", src_addr, msg);

    let response = "Hello back to ya";
    socket.send_to(response.as_bytes(), src_addr).expect("failed to send response");

}
