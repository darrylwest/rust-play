
use anyhow::{anyhow, Result};
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileOpts {
    pub home: String,
}

impl FileOpts {
    pub fn new(home: &str) -> Result<FileOpts> {
        println!("verify the {} is an existing folder", home);
        let path = Path::new(home);
        if path.exists() && path.is_dir() {
            Ok(FileOpts { home: String::from(home) })
        } else {
            Err(anyhow!("not a valid folder for home"))
        }
    }

    pub fn pattern_find(&self, pattern: &str) -> Vec<PathBuf> {
        let mut files: Vec<PathBuf> = vec![];

        let path = Path::new(&self.home);

        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry)  = entry {
                let pb: PathBuf = entry.path().clone();
                let ext = pb.extension();
                if ext.is_none() {
                    continue;
                }

                let ext = ext.unwrap();
                
                if pb.is_file() && ext == pattern {
                    println!("{:?}, ext: {:?}", &pb, pb.clone().extension());
                    files.push(pb.clone())
                }
            }
        }

        files
    }
}

fn main() {
    println!("vers 0.0.1");
    let args: Vec<String> = env::args().skip(1).collect();
    let home: String;
    match args.len() {
        0 => home = "./tests".to_string(),
        _ => home = args[0].clone(),
    }

    let ops = FileOpts::new(home.as_ref()).unwrap();

    let list = ops.pattern_find("eb64");
    println!("{:?}", list);
}