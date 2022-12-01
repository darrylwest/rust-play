
use anyhow::Result;
use std::net::TcpStream;
use ssh2::Session;
use std::io::Read;
// use std::path::Path;

fn main() -> Result<()> {
    // let addr = "10.0.1.105:22";
    let addr = "piedmont:22";
    let tcp = TcpStream::connect(addr).unwrap();
    println!("connected to {}", addr);

    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    println!("session ok");

    let pw = std::env::var("PW")?;
    sess.userauth_password("dpw", &pw).unwrap();
    println!("auth via dpw");

    assert!(sess.authenticated());

    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls -la").unwrap();

    let mut buf = String::new();
    channel.read_to_string(&mut buf).unwrap();
    println!("{}", buf);

    channel.wait_close()?;
    println!("{}", channel.exit_status()?);

    Ok(())
}

