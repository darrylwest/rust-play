use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Args {
    #[clap(short, long, value_parser)]
    input_file: Option<String>,

    #[clap(short, long, value_parser)]
    output_file: Option<String>,
}

pub struct Config {
    pub infile: Option<String>,
    pub outfile: Option<String>,
    pub silent: bool,
}

impl Config {
    pub fn parse() -> Self {
        let args = Args::parse();

        Config {
            infile: args.input_file,
            outfile: args.output_file,
            silent: false,
        }
    }
}
