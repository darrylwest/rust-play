use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(short, long, value_parser)]
    input_file: Option<String>,

    #[clap(short, long, value_parser)]
    output_file: Option<String>,

    #[clap(short, long, value_parser)]
    silent: bool,
}

impl Config {
    pub fn new() -> Self {
        Config::parse()
    }
}
