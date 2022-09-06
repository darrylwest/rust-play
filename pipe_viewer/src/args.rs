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
    pub outfile: Option<String>,
    pub infile: Option<String>,
    pub silent: bool,
}

impl Config {
    pub fn new(outfile: Option<String>, infile: Option<String>, silent: bool) -> Self {
        Self {
            outfile,
            infile,
            silent,
        }
    }

    pub fn parse() -> Self {
        let args = Args::parse();

        Config::new(args.output_file, args.input_file, false)
    }
}
