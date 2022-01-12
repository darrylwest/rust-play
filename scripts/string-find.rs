#!/usr/bin/env rust-script

fn main() {
    let sentence = "this fox jumps over the slow doggy.";
    let idx = sentence.find("over");
    
    println!("{}", &sentence);

    if let Some(fox) = idx {
        let words = &sentence[fox..];
        println!("idx={} -> {}", idx.unwrap(), words);
    }
}
