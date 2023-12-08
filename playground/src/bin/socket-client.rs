use std::net::{SocketAddr, TcpStream};
use std::io::{Read, Write};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

fn main() -> std::io::Result<()> {
    let port = 8080;
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    let saddr = format!("127.0.0.1:{}", port);
    let address: SocketAddr = saddr.parse().unwrap();
    let sock_addr = SockAddr::from(address);
    socket.connect(&sock_addr)?;

    let mut stream = TcpStream::from(socket);
    stream.write_all(b"Hello, world!")?;

    println!("written");

    let mut buf = [0; 1024];
    let bytes_read = stream.read(&mut buf)?;
    let response = String::from_utf8_lossy(&buf[..bytes_read]);
    println!("{}", response);

    Ok(())
}

