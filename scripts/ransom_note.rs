#!/usr/bin/env rust-script
// cargo-deps: rand

fn can_construct(ransom_note: String, magazine: String) -> bool {
  println!("{}", ransom_note);
  println!("{}", magazine);


  false
}

fn main() {
  let note = "thisisatestmessage";
  let text = "thisisatestmagazinewithlotsofmessages";

  let result = can_construct(note.to_string(), text.to_string());


  println!("result: {}", result);
}

