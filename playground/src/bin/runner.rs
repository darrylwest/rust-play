
use std::process::Command;
// use std::io::Write;
// use std::io;

fn main() {
    let mut cmd = Command::new("lsd");
    cmd.arg("-l");
    cmd.arg("-a");

    let output = cmd.output().expect("should work...");
    // io::stdout().write_all(&output.stdout).unwrap();
    // io::stderr().write_all(&output.stderr).unwrap();


    println!("process finished with status: {}", output.status);
}
