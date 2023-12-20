
use std::io::{stdout, Write};
use curl::easy::{Easy, List};

// Print a web page onto stdout
fn main() {
    let mut easy = Easy::new();
    easy.url("https://127.0.0.1:23300/api/v1/ping").unwrap();

    let mut list = List::new();
    list.append("apikey: 38dd8d517578456084b111663033ed3e").unwrap();

    easy.http_headers(list).unwrap();

    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();

    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}
