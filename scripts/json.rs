//: -s

use json;
// use serde_json::Serialize;

#[derive(Debug, Clone)]
struct Resp {
    code: u16,
    success: bool,
    payload: String,
}

let parsed = json::parse(r#"{"code":200,"success":true,"payload":{"features":["awsome","easyAPI","lowcurve"]}}"#)?;

println!("{}", parsed);


let obj = Resp {
    code: 200,
    success: true,
    payload: "hello world".to_owned(),
};

println!("obj: {:?}", obj);
// println!("json: {}", serde_json::to_string(obj));

// a runner script: https://crates.io/crates/runner
// do this first: runner --add "time json regex"
// works on linux, osX not so much
