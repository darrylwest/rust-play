#!/usr/bin/env rust-script

use std::env;
use std::borrow::Cow;


fn get_var(key: &str) -> Cow<'static, str> {
    env::var(key)
        .map(|v| Cow::Owned(v))
        .unwrap_or(Cow::Borrowed("no env var with that name"))
}

fn main() {
    let name = get_var("HOME");
    println!("HOME={:?}", name);

    let name = get_var("xHOME");
    println!("xHOME={:?}", name);
}
