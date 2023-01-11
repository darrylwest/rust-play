#!/usr/bin/env rust-script
// cargo-deps: rand

use std::any::{Any, TypeId};
use std::collections::HashMap;
use rand::prelude::*;
use std::char::from_digit;

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: i32,
}

fn main() {
    let mut hmap: HashMap<String, Box<dyn Any>> = HashMap::new();
    let p = Person { name: "jonny".to_string(), age: 35 };

    let v1: Box<dyn Any> = Box::new(3_i32);
    let v2: Box<dyn Any> = Box::new("this is &str a test");
    let v3: Box<dyn Any> = Box::new("this is a String test".to_string());
    let v4: Box<dyn Any> = Box::new(vec!["a", "b", "c"]);
    let v5: Box<dyn Any> = Box::new(p);

    let t_i32 = TypeId::of::<i32>();
    let t_str = TypeId::of::<&str>();
    let t_string = TypeId::of::<String>();
    let t_person = TypeId::of::<Person>();
    let t_vec_str = TypeId::of::<Vec<&str>>();

    let t_id = (&*v1).type_id();
    println!("i32: {:?} str: {:?} id: {:?}\n", t_i32, t_str, t_id);

    println!("v1 is i32: {}", (&*v1).type_id() == t_i32);
    println!("v2 is &str: {}", (&*v2).type_id() == t_str);
    println!("v3 is String: {}", (&*v3).type_id() == TypeId::of::<String>());
    println!("v4 is vec: {}", (&*v4).type_id() == TypeId::of::<Vec<&str>>());
    println!("v5 is Person: {}", (&*v5).type_id() == TypeId::of::<Person>());


    hmap.insert(create_key(), v1);
    hmap.insert(create_key(), v2);
    hmap.insert(create_key(), v3);
    hmap.insert(create_key(), v4);
    hmap.insert(create_key(), v5);

    println!("hmap: {:?}\n", hmap);

    for (k, v) in hmap.into_iter() {
        let t_id = (&*v).type_id();

        if t_id == t_i32 {
            if let Ok(n) = v.downcast::<i32>() {
                println!("k: {}, v: {}", k, n);
            }
        } else if t_id == t_str {
            if let Ok(s) = v.downcast::<&str>() {
                println!("k: {}, v: {}", k, s);
            }
        } else if t_id == t_string {
            if let Ok(s) = v.downcast::<String>() {
                println!("k: {}, v: {}", k, s);
            }
        } else if t_id == t_person {
            if let Ok(p) = v.downcast::<Person>() {
                println!("k: {}, v: {:?}", k, p);
            }
        } else if t_id == t_vec_str {
            if let Ok(v) = v.downcast::<Vec<&str>>() {
                println!("k: {}, v: {:?}", k, v);
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


