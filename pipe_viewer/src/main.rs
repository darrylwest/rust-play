use std::env;
use std::io::{self, Read, Write, Result};
use clap::Parser;

///
/// Test with `yes | pipe_viewer | head -n 100000`
///

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    input_file: Option<String>,

    #[clap(short, long, value_parser)]
    output_file: Option<String>,
}

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(infile) = args.input_file {
        println!("input: {}", infile );
    }

    if let Some(outfile) = args.output_file {
        println!("output: {}", outfile);
    }

    let verbose = !env::var("PV_VERBOSE").unwrap_or_default().is_empty();

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let byte_count = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        total_bytes += byte_count;

        if verbose {
            io::stdout().write_all(&buffer[..byte_count])?;
        }

        eprint!("\rTotal Byte Count: {}", total_bytes);
    }

    eprintln!("\rTotal Byte Count: {}", total_bytes);

    Ok(())
}
