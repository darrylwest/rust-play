use clap::{Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Config {

    #[clap(short, long, value_parser)]
    input_file: Option<String>,

    #[clap(short, long, value_parser)]
    output_file: Option<String>,

    #[clap(short, long, value_parser)]
    silent: bool,

    #[clap(short, long, value_parser)]
    debug: u8,
}

pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub silent: bool,
}
