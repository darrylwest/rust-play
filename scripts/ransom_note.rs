#!/usr/bin/env rust-script

use std::collections::HashMap;

fn can_construct(ransom_note: String, magazine: String) -> bool {
    println!("{}", ransom_note);
    println!("{}", magazine);

    let mut ransom_hash = HashMap::new();

    for ch in ransom_note.chars() {
        ransom_hash
            .entry(ch)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    show("ransom".to_string(), &ransom_hash);

    let mut text_hash = HashMap::new();

    for ch in magazine.chars() {
        text_hash
            .entry(ch)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    show("text".to_string(), &text_hash);

    let mut can_create = true;

    for (ch, count) in ransom_hash.iter() {
        match text_hash.get(ch) {
            None => {
                can_create = false;
                break;
            }
            Some(n) => {
                if n < count {
                    can_create = false;
                    break;
                }
            }
        }
    }

    can_create
}

fn show(heading: String, hmap: &HashMap<char, i32>) {
    println!("{}", heading);
    for (key, val) in hmap.iter() {
        print!("{}:{}, ", key, val);
    }
    println!("");
}

fn main() {
    let note = "thisisatestmessage";
    let text = "thisisatestmagazinewithlotsofmessages";

    let result = can_construct(note.to_string(), text.to_string());

    println!("result: {}", result);
}
