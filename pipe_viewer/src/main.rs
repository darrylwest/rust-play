use crossbeam::channel::{bounded, unbounded};
use pipe_viewer::{args::Config, read, stats, write};
use std::io::Result;
use std::thread;
use log::info;

fn main() -> Result<()> {
    env_logger::init();
    let config = Config::new();

    let silent: bool = config.silent;
    let infile: String = config.input_file();
    let outfile: String = config.output_file();

    let (stats_tx, stats_rx) = unbounded();
    let (write_tx, write_rx) = bounded(1024);

    let read_handle = thread::spawn(move || {
        read::read_loop(&infile, stats_tx, write_tx)
    });

    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx));

    let write_handle = thread::spawn(move || {
        write::write_loop(&outfile, write_rx)
    });

    // crash if any threads have crashed
    // `.join()` returns a `thread::Result<io::Result<()>>`
    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    // Return an error if any threads returned an error
    read_io_result?;
    stats_io_result?;
    write_io_result?;

    info!("PipeViewer completed...");

    Ok(())
}
