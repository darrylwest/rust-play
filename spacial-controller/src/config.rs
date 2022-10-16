use clap::Parser;
use log::*;

#[derive(Debug, Default, Parser, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(short, long, value_parser)]
    flight_plan: String,
}

///
/// Configuration to point to flight plan filename, etc.
///
impl Config {
    pub fn new() -> Config {
        let config = Config::parse();
        info!("Config: {:?}", config);

        config
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_config() {
        let filename = "my-test-plan.json";
        let config = Config {
            flight_plan: filename.to_ascii_lowercase(),
        };
        assert_eq!(config.flight_plan, filename.to_string());
    }
}
