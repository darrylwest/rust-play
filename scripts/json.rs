//: -s

use json;

let parsed = json::parse(r#"{"code":200,"success":true,"payload":{"features":["awsome","easyAPI","lowcurve"]}}"#)?;

println!("{}", parsed);

// a runner script: https://crates.io/crates/runner
// do this first: runner --add "time json regex"
// works on linux, osX not so much
