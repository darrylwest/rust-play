#!/usr/bin/env rust-script

fn main() {
    println!("{}", number_of_steps(14)); 
    println!("{}", number_of_steps(8)); 
    println!("{}", number_of_steps(123)); 
}

pub fn number_of_steps(num: i32) -> i32 {
    let mut steps: i32 = 0;
    let mut target: i32 = num;

    while target > 0 {
        steps = steps + 1;

        if target % 2 == 0 {
            target = target / 2;
        } else {
            target = target - 1;
        }
    }

    steps
}
