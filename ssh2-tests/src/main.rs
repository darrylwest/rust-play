
use anyhow::Result;
use std::net::TcpStream;
use ssh2::Session;
// use std::path::Path;

fn main() -> Result<()> {
    let sess = Session::new().expect("should create a session");
    let mut agent = sess.agent()?;

    println!("connect to the agent");
    agent.connect()?;
    agent.list_identities()?;

    for identity in agent.identities().unwrap() {
        println!("{}", identity.comment());
        println!("{:?}", identity.blob());

        // let r = agent.userauth("dpw", &identity);
        // println!("{:?}", r);
    }

    Ok(())
}

pub fn connect() -> Result<()> {
    let addr = "10.0.1.105:22";
    // let addr = "tiburon.local:22";
    let tcp = TcpStream::connect(addr)?;
    println!("connected to {}", addr);

    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    // println!("session ok");

    sess.userauth_agent("dpw")?;
    println!("auth via agent dpw");

    assert!(sess.authenticated());

    Ok(())
}
