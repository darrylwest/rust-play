#!/usr/bin/env rust-script

fn main() {
    let slice: &str = &ransom_note;
    let mut note: Vec<char> = slice.chars().collect();

    note.sort_by(|a, b| a.cmp(b));

    for v in note {
        print!("{}", v);
    }

    println!("");

    let slice: &str = &magazine;
    let mut mag: Vec<char> = slice.chars().collect();
}
