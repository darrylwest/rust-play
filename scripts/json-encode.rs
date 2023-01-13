#!/usr/bin/env rust-script
//! Use serde json to encode a HashMap
//! 
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! serde_json = "1"
//! hashbrown = { version = "0.13.1", features = ["serde"] }
//!

use anyhow::Result;
use std::{
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

fn main() -> Result<()> {
    
    let user = create_user();
    let json = serde_json::to_string_pretty(&user)?;

    println!("{}", json);

    let filename = "/tmp/user.json";
    let mut buf = File::create(filename)?;

    buf.write_all(json.as_bytes())?;

    Ok(())
}

