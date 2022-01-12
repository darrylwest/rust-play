extern crate tiny_http;

use std::sync::Arc;
use std::thread;


fn main() {
    let host = "0.0.0.0:9975";
    let server = Arc::new(tiny_http::Server::http(&host).unwrap());

    println!("now listening on {}", &host);

    let mut handles = Vec::new();

    for _ in 0..4 {
        let server = server.clone();

        handles.push(thread::spawn(move || {
            for rq in server.incoming_requests() {
                let url = rq.url().to_string();
                // let path = Path::new(&url);
                
                println!("rq {:?} {:?} {:?}", rq.method(), &url, rq.headers());

                match url.as_str() {
                    "/hello" => {
                        let response = tiny_http::Response::from_string("hello world".to_string());
                        let _ = rq.respond(response);
                    }

                    "/goodbye" => {
                        let response = tiny_http::Response::from_string("good bye".to_string());
                        let _ = rq.respond(response);
                    }

                    _ => {
                        let response = tiny_http::Response::from_string("what?".to_string());
                        let _ = rq.respond(response);
                    }
                }

            };
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}

