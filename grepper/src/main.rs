use std::env;
use std::process;

use grepper::Config;

fn main() {
    env_logger::init();

    log::info!("grepper startup");

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        log::error!("something went wrong! {}", err);
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = grepper::run(config) {
        log::error!("something went wrong! {}", e);
        eprintln!("Application error: {}", e);

        process::exit(1);
    }

    process::exit(0);
}
