#!/usr/bin/env rust-script
// cargo-deps: rand

fn main() {
    let printr = |x| print!("{}, ", x);

    let mut v = vec![1, 2, 3, 4, 5];

    let first = match v.get(0) {
      None => 1,
      Some(i) => *i,
    };

    println!("First: {:?}", first);

    v.push(6);
    v.insert(0, 7);

    println!("The list: {:?}", v);
    println!("Original First: {:?}", first);

    print!("reverse and filtered evens: ");
    v.into_iter().rev().filter(|x| x % 2 == 0).for_each(|x| printr(x) );
    println!("");
}
