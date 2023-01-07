
use anyhow::Result;
use log::info;
use std::time::Duration;
use async_std::task;
use async_std_test::files::read_file;

fn main() -> Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let paths = vec!["data/file-1.txt", "data/file-2.txt", "data/file-3.txt" ];
    let mut tasks = vec![];

    for path in paths.iter() {
        let j = task::spawn(read_file(path.to_string()));

        tasks.push(j);
    };

    task::block_on(task::sleep(Duration::from_millis(100)));

    info!("[m]reading files: {:?}", paths);

    for task in tasks.into_iter() {
        let r = task::block_on(task)?;
        info!("[m]file read complete, {} bytes.", r.len());
    }

    Ok(())
}

