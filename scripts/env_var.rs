#!/usr/bin/env rust-script

use std::env;
use std::borrow::Cow;
use std::fmt::Write;

fn get_var(key: &str, title: Option<String>) -> Cow<'static, str> {
    let mut name: Cow<'static, str> = env::var(key)
        .map(|v| Cow::Owned(v))
        .unwrap_or(key.to_string().into());

    if let Some(ttl) = title {
        write!(name.to_mut(), ", {}", ttl).unwrap();
    }

    name
}

fn main() {
    let name = get_var("HOME", Some("Phd".to_string()));
    println!("HOME={:?}", name);

    let name = get_var("my-empty-var", None);
    println!("xHOME={:?}", name);

    let name = get_var("rrr", Some("III".to_string()));
    println!("xHOME={:?}", name);
}
