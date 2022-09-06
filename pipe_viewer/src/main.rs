use clap::Parser;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write, Result};

///
/// Test with `yes | pipe_viewer | head -n 100000`
///

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Args {
    #[clap(short, long, value_parser)]
    input_file: Option<String>,

    #[clap(short, long, value_parser)]
    output_file: Option<String>,
}

const CSZ: usize = 16 * 1024;

fn main() -> Result<()> {
    let args = Args::parse();

    let mut reader: Box<dyn Read> = match args.input_file {
        Some(infile) => Box::new(BufReader::new(File::open(infile)?)),
        _ => Box::new(BufReader::new(io::stdin())),
    };

    let mut writer: Box<dyn Write> = match args.output_file {
        Some(outfile) => Box::new(BufWriter::new(File::create(outfile)?)),
        _ => Box::new(BufWriter::new(io::stdout())),
    };


    let mut total_bytes = 0;
    let mut buffer = [0; CSZ];

    loop {
        let byte_count = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        total_bytes += byte_count;

        writer.write_all(&buffer[..byte_count])?;

        eprint!("\rTotal Byte Count: {}", total_bytes);
    }

    eprintln!("\rTotal Byte Count: {}", total_bytes);

    Ok(())
}
