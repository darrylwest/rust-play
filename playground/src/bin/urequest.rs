
use anyhow::Result;

fn get_readings(url: &str) -> Result<Vec<String>> {
    let body: String = ureq::get(url)
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;

    let lines: Vec<String> = body.split("\n").filter(|x| !x.is_empty()).map(|x| x.to_string()).collect();

    Ok(lines)
}

fn main() -> Result<()> {
    let lines = get_readings("http://10.0.1.197:2020/readings")?;

    println!("{:?}", lines);

    Ok(())
}
