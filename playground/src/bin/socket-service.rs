use std::net::SocketAddr; // , TcpListener};
use std::thread;
use std::io::{Read, Write};
use std::sync::mpsc::channel; // , Sender};
use std::sync::{Arc, Mutex};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

//
// THIS IS MOSTLY BROKEN
//

const MAX_CONNECTIONS: i32 = 128;

fn main() -> std::io::Result<()> {
    let port = 8080;

    println!("An example of a multi-threaded TCP socket server using the socket2 crate");
    println!("use curl or socat to access this at port {}", port);

    let listener = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    listener.set_reuse_address(true)?;
    listener.bind(&SockAddr::from(SocketAddr::from(([127, 0, 0, 1], port))))?;
    listener.listen(MAX_CONNECTIONS)?;

    let (tx, rx) = channel::<String>();
    let shared_tx = Arc::new(Mutex::new(tx));

    loop {
        let shared_tx = shared_tx.clone();
        let listener = listener.try_clone()?;
        thread::spawn(move || {
            let (mut stream, addr) = listener.accept().unwrap();
            let mut buf = [0; 1024];
            let mut data = String::new();
            loop {
                let bytes_read = stream.read(&mut buf).unwrap();
                if bytes_read == 0 {
                    break;
                }
                data.push_str(&String::from_utf8_lossy(&buf[..bytes_read]));
            }
            let response = format!("Received request from {:?}: {:?}", addr, data);
            shared_tx.lock().unwrap().send(response).unwrap();
            stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();

        });

        for response in rx.iter() {
            println!("{}", response);
        }
    }
}

