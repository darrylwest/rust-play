use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Like above, but use `output` which returns a future instead of
    // immediately returning the `Child`.
    let output = Command::new("echo").arg("hello").arg("world")
                        .output();

    let output = output.await?;

    assert!(output.status.success());

    let s = match std::str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("output error: {}", e),
    };

    println!("{}", s);

    Ok(())
}
