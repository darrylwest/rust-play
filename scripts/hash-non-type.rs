#!/usr/bin/env rust-script
// cargo-deps: rand

use std::any::{Any, TypeId};
use std::collections::HashMap;
use rand::prelude::*;
use std::char::from_digit;

pub struct Person {
    pub name: String,
    pub age: i32,
}

fn main() {
    let mut hmap: HashMap<String, Box<dyn Any>> = HashMap::new();
    let p = Person { name: "jonny".to_string(), age: 35 };

    let v1: Box<dyn Any> = Box::new(3_i32);
    let v2: Box<dyn Any> = Box::new("this is a test");
    let v3: Box<dyn Any> = Box::new("this is a test".to_string());
    let v4: Box<dyn Any> = Box::new(vec!["a", "b", "c"]);
    let v5: Box<dyn Any> = Box::new(p);

    let t_i32 = TypeId::of::<i32>();
    let t_str = TypeId::of::<&str>();

    println!("v1 is i32: {}", (&*v1).type_id() == t_i32);
    println!("v2 is &str: {}", (&*v2).type_id() == t_str);
    println!("v3 is String: {}", (&*v3).type_id() == TypeId::of::<String>());
    println!("v4 is vec: {}", (&*v4).type_id() == TypeId::of::<Vec<&str>>());
    println!("v5 is Person: {}", (&*v5).type_id() == TypeId::of::<Person>());


    hmap.insert(create_key(), v1);
    hmap.insert(create_key(), v2);
    hmap.insert(create_key(), v3);
    hmap.insert(create_key(), v4);

    println!("hmap: {:?}", hmap);

    for (k, v) in hmap.into_iter() {
        let t_id = (&*v).type_id();

        if t_id == t_i32 {
            if let Ok(n) = v.downcast::<i32>() {
                println!("k: {}, v: {}", k, n);
            }
        } else {
            println!("what?");
        }
    }

}

fn create_key() -> String {
    let mut rng = rand::thread_rng();
    let mut results: Vec<char> = vec![];

    for _ in 0..12 {
        let n = rng.gen_range(0..36);
        results.push(from_digit(n as u32, 36).unwrap());
    }

    results.into_iter().rev().collect()
}


