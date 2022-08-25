#!/usr/bin/env rust-script
// cargo-deps: rand

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  let low = 1;
  let high = 100;

  println!("Guess the number between {low} and {high}...");


  let secret_number = rand::thread_rng().gen_range(low..=high);

  // println!("Secret number is {secret_number}.");

  loop {
      println!("Please enter a number: ");

      let mut guess = String::new();

      io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

      let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
      };

      println!("You guessed: {guess}");

      match guess.cmp(&secret_number) {
          Ordering::Less => println!("Too small!"),
          Ordering::Greater => println!("Too big!"),
          Ordering::Equal => {
              println!("You win!");
              break;
          }
      }
  }

}
