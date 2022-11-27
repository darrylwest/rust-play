//: -s

use json;

let parsed = json::parse(r#"{"code":200,"success":true,"payload":{"features":["awsome","easyAPI","lowcurve"]}}"#)?;

println!("{}", parsed);

// a runner script: https://crates.io/crates/runner
