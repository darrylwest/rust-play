use clap::Parser;
use log::*;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(short, long, value_parser)]
    input_file: Option<String>,

    #[clap(short, long, value_parser)]
    output_file: Option<String>,

    #[clap(short, long, value_parser)]
    pub silent: bool,
}

impl Config {
    pub fn new() -> Self {
        let config = Config::parse();
        info!("Parse the command line args into Config...");
        info!("Config: {:?}", config);
        config
    }

    pub fn input_file(&self) -> String {
        if let Some(s) = self.input_file.as_ref() {
            info!("Read input from {}", s);
            String::from(s)
        } else {
            info!("Read from stdin");
            String::new()
        }
    }

    pub fn output_file(&self) -> String {
        if let Some(s) = self.output_file.as_ref() {
            info!("Write output to {}", s);
            String::from(s)
        } else {
            info!("Write output to stdout");
            String::new()
        }
    }
}
