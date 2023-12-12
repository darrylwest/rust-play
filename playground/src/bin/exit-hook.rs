use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let pid = std::process::id().to_string();
    println!("my pid: {}", pid);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {}

    println!("do some clean up here...");
    println!("Got it! Exiting...");
}
