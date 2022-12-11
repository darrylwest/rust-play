#!/usr/bin/env rust-script

fn main() {
    parse("get/ping");
    parse("put/key/eyJrZXkiOiIyMzQ0NDQzMiJ9Cg==");

}

fn parse(message: &str) {
    let mut parts = message.split("/");

    println!("{} -> {:?}", message, parts);

    let method = match parts.next() {
        Some("get") => "get",
        Some("put") => "put",
        _ => "error"
    };

    println!("{:?}", method);

    let path = match parts.next() {
        None => "",
        Some(v) => v,
    };

    println!("{:?}", path);

    let params = match parts.next() {
        None => "",
        Some(v) => v,
    };

    println!("{:?}", params);

}


