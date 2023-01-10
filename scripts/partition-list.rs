#!/usr/bin/env rust-script

use std::collections::HashSet;

// bit manipulation to determine if n and (n-1) evaluates to zero.  if so, it's a power of two
fn power_of_two(n: i32) {

    let pot = |&n| n & (n-1); 
    let is_power_of_two = |&n| { n & (n-1) == 0 }; 

    let r = pot(&n);
    let p = is_power_of_two(&n);
    println!("{:b} & ({:b}-1) == {:b}, is {} a power of two?: {}", n, n, r, n, p);
}

fn main() {
    power_of_two(4);
    power_of_two(9);

    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>) = squares.iter().partition(|&n| n & (n-1) == 0);

    println!("poftwo: {:?}", powers_of_two);
    println!("impure: {:?}", impure);
}
