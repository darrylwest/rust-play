#!/usr/bin/env rust-script
//! Use serde json to encode a HashMap (HashBrown) of <String, String>.  I tried to
//! use Any but serde rejects encoding that, so...
//!
//! Example Use: json-encode.rs id=23434 email=flarb age=44
//! this creates a json string: { "email": "flarb", "age": "44", "id": "rjZ17fEKPcKSoBrG" }
//!
//! A better approach would be to use a json mustash template and single values on the commad
//! line.  That would solve the type issues. But in any case this provides an easy way to create
//! key/value pairs in a json starter format that can be edited or turned into a template faster
//! than creating one from scratch. 
//! 
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! serde_json = "1"
//! hashbrown = { version = "0.13.1", features = ["serde"] }
//!

use anyhow::Result;
use std::{
    env,
    fs::File,
    io::prelude::*,
};
use hashbrown::HashMap;

fn create_user() -> HashMap<String, String> {
    let id = String::from("938f7fEChq9lUiMr");
    let email = String::from("jonny@boxed.com");
    let phone = String::from("1-415-666-2334");
    let age = String::from("42");
    let profile = String::from("");

    let mut user: HashMap<String, String> = HashMap::new();
    user.insert("id".to_string(), id);
    user.insert("email".to_string(), email);
    user.insert("phone".to_string(), phone);
    user.insert("age".to_string(), age);
    user.insert("profile".to_string(), profile);

    user
}

fn parse_args(args: Vec<String>) -> Result<HashMap<String, String>> {
    let mut map: HashMap<String, String> = HashMap::new();

    // now parse through the key=value pairs and insert into hashmap
    for arg in args.iter() {
        let v: Vec<_> = arg.split("=").collect();
        let (key, value) = (v[0].to_string(), v[1].to_string());

        // println!("{}{}", key, value);

        map.insert(key, value);
    }

    Ok(map)
}

fn main() -> Result<()> {

    let args: Vec<String> = env::args().skip(1).collect();
    let map: HashMap<String, String> = if args.is_empty() {
        create_user()
    } else {
        parse_args(args).unwrap()
    };
    
    let json = serde_json::to_string_pretty(&map)?;

    println!("{}", json);

    let filename = "/tmp/user.json";
    let mut buf = File::create(filename)?;

    buf.write_all(json.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args() {
        assert!(true);
    }
}
