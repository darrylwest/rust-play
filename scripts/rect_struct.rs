#!/usr/bin/env rust-script
// cargo-deps: rand

#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
        }
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);

    println!("Rectangle area is {:?} square pixels.", rect.area());
}
