use log::info;

use spacial_controller::config::Config;
// use spacial_controller::drone::Drone;

fn main() {
    log4rs::init_file("config/rolling.yaml", Default::default()).unwrap();

    // parse the args...
    let _config = Config::new();

    // read the flight plan...

    // info!("initial drone: {:?}", Drone::new());
    info!("just getting started...");
}
