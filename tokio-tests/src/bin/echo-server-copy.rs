use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let host = "127.0.0.1:6142";
    let socket = TcpStream::connect(host).await?;
    let (mut rd, mut wr) = io::split(socket);

    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;

        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        // println!("got {:?}", &buf[..n]);

        let s = match std::str::from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => panic!("invalid utf-8: {}", e),
        };

        println!("{}", s);
    }

    Ok(())
}
