#!/usr/bin/env rust-script


fn takes_fptr(f: fn(i32) -> i32) -> i32 {
    f(5)
}

fn main() {
    // this works only if there are no closed vars...

    let add_two = |n| n + 2;
    let add_five = |n| n + 5;

    let sum = takes_fptr(add_two);
    assert_eq!(sum, 7);
    println!("add two to 5 = {}", sum);

    let sum = takes_fptr(add_five);
    assert_eq!(sum, 10);
    println!("add five to 5 = {}", sum);

    let greeting = |name| println!("hello {}", name);
    greeting("harry");
}
