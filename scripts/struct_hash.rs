#!/usr/bin/env rust-script
// cargo-deps: rand

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Default, Hash)]
pub enum DomainStatus {
    #[default]
    New,
    Active,
    Inactive,
    Deleted,
}

#[derive(Hash)]
pub struct Person {
    id: String,
    first_name: String,
    last_name: String,
    status: DomainStatus,
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    let p1 = Person {
        id: "0100".to_string(),
        first_name: "John".to_string(),
        last_name: "Smith".to_string(),
        status: DomainStatus::New,
    };

    let p2 = Person {
        id: "0101".to_string(),
        first_name: "John".to_string(),
        last_name: "Smith".to_string(),
        status: DomainStatus::New,
    };

    let hash1 = calculate_hash(&p1);
    let hash2 = calculate_hash(&p2);

    println!("hash 1: {}, hash 2: {}", hash1, hash2);

}
