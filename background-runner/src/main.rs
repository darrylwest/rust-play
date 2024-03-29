
extern crate daemonize;

use std::fs::File;
use std::thread;
use std::time::Duration;

use daemonize::Daemonize;

fn main() {
    println!("do this: tail -f tmp/daemon.out");

    let stdout = File::create("./tmp/daemon.out").unwrap();
    let stderr = File::create("./tmp/daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("daemon.pid") // Every method except `new` and `start`
        .chown_pid_file(false)      // is optional, see `Daemonize` documentation
        .working_directory("./tmp") // for default behaviour.
        // .user("dpw")
        // .group("daemon") // Group name
        // .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `./tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `./tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            println!("Success, daemonized");

            // put this in a separate thread...
            for n in 0..=1_000_000 {
                println!("i'm alive: {}", n);
                thread::sleep(Duration::from_secs(1));
            }
            //
            // now, remove the pid file
            //
        }
        Err(e) => {
            eprintln!("Error, {}", e);
            eprintln!("bailing out!");
        }
    }
}
