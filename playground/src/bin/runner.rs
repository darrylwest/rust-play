
use std::process::Command;

fn main() {
    Command::new("lsd")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("should work...");

}
