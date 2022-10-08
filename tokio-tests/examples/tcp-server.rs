use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("attach with telnet or curl...");

    let host = "0.0.0.0:8080";
    let listener = TcpListener::bind(host).await?;

    println!("listening on host: {}", host);

    let mut id = 0;

    loop {
        id += 1;
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let cid = id.clone();

            // read data from the socket; parse the command; return a response
            loop {
                let mut request = [0; 1024];
                let response = match socket.read(&mut request).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => {
                        let s = match std::str::from_utf8(&request) {
                            Ok(v) => v,
                            Err(e) => panic!("invalid utf-8: {}", e),
                        };
                        println!("session: {} len: {} msg: {}", cid, n, s);

                        if request[..n] == b"ping\r\n"[..] {
                            "pong\r\n".as_bytes()
                        } else {
                            "ERROR\r\n".as_bytes()
                        }
                    }
                    Err(e) => {
                        eprintln!("error: {}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&response[..]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
