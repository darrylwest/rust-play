
use anyhow::Result;
use ureq::{Agent, AgentBuilder};
use std::time::Duration;
use std::thread;

fn get_readings(url: &str) -> Result<Vec<String>> {
    let agent: Agent = AgentBuilder::new()
        .timeout_read(Duration::from_secs(2))
        .timeout_write(Duration::from_secs(2))
        .build();

    let body: String = agent.get(url)
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;

    let lines: Vec<String> = body.split("\n").filter(|x| !x.is_empty()).map(|x| x.to_string()).collect();

    Ok(lines)
}

fn main() -> Result<()> {
    println!("Version 1.0.0");
    let urls = vec![ "http://10.0.1.115:2020", "http://10.0.1.177:2020", "http://10.0.1.197:2020" ];
    let mut handles = Vec::new();
    for url in urls {
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
