

pub const CHUNK_SIZE: usize = 16 * 1024;

use pipe_viewer::{args::Config};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write, Result};

///
/// Test with `yes | pipe_viewer | head -n 100000`
///


fn main() -> Result<()> {

    let config = Config::parse();

    let mut reader: Box<dyn Read> = if let Some(infile) = config.infile {
        Box::new(BufReader::new(File::open(infile.to_string())?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut writer: Box<dyn Write> = if let Some(outfile) = config.outfile {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    let mut total_bytes = 0;
    let mut buffer = [0; crate::CHUNK_SIZE];

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
