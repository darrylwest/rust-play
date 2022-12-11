#!/usr/bin/env rust-script

use std::net::UdpSocket;

fn main() {
    send("get/ping");
    send("get/now");
    send("get/millis");
    send("get/status");
}

fn send(req: &str) {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("should bind");

    print!("{} -> ", req);
    socket.connect("10.0.1.142:22200").expect("connect to remote");
    socket.send(req.as_bytes()).expect("send to error");
    
    let mut buf = [0; 1024];
    match socket.recv(&mut buf) {
      Ok(r) => {
        let s = String::from_utf8_lossy(&buf[..r]);
        println!("{}", s.trim());
      }
      Err(e) => println!("error: {:?}", e),
    };
}
