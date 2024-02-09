
use anyhow::Result;
use clap::Parser;
use ureq::{Agent, AgentBuilder};
use std::time::Duration;
use std::thread;


fn get_server_list() -> Vec<String> {
    vec![ "http://10.0.1.115:2020".to_string(), "http://10.0.1.177:2020".to_string(), "http://10.0.1.197:2020".to_string() ]
}

#[derive(Debug, Default, Clone, Parser)]
#[command(
    name="urequest",
    author,
    version = "0.9.1",
    about,
    long_about = None
)]
struct Cli {
    /// query a single server by url
    #[clap(short, long)]
    server: Option<String>,
}

fn get_readings(url: &str) -> Result<Vec<String>> {
    let agent: Agent = AgentBuilder::new()
        .timeout_read(Duration::from_secs(1))
        .timeout_write(Duration::from_secs(1))
        .build();

    let body: String = agent.get(url)
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;

    let lines: Vec<String> = body.split("\n").filter(|x| !x.is_empty()).map(|x| x.to_string()).collect();

    Ok(lines)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let servers = match cli.server {
        Some(list) => vec![ list ],
        None => get_server_list(),
    };

    let mut handles = Vec::new();
    for url in servers {
        let handle = thread::spawn(move || {
            let url = format!("{}/readings", url);
            let lines = get_readings(url.as_str());
            println!("{:?}", lines);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
