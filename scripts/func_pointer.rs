#!/usr/bin/env rust-script

fn create_adder() -> Box<dyn FnOnce(i32, i32) -> i32> {
    Box::new(|x, y| x + y)
}

fn create_stepper(increment: i32) -> Box<dyn Fn(&i32) -> i32> {
    Box::new(move |x| *x + increment)
}

fn main() {
    let x = 5;
    let y = 3;
    
    let adder = create_adder();
    
    println!("{} + {} = {}", x, y, adder(x, y));

    let stepper = create_stepper(2);
    let mut z = 20;

    for _ in 0..5 {
        z = stepper(&z);

        println!("next step -> {}", z);
    }
    
}
