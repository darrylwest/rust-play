use std::env;
use std::io::{self, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() {
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    let verbose = !env::var("PV_VERBOSE").unwrap_or_default().is_empty();

    let mut total_bytes = 0;

    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let byte_count = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        total_bytes += byte_count;

        if verbose {
            io::stdout().write_all(&buffer[..byte_count]).unwrap();
        }
    }

    if !silent {
        eprintln!("Total Byte Count: {}", total_bytes);
    }
}
