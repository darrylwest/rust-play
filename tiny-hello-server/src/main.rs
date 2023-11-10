use std::sync::Arc;
use std::thread;
use std::time::SystemTime;
use tiny_http::{Response, Server};

fn main() {
    let host = "0.0.0.0:14090";
    let server = Arc::new(
        Server::https(
            host,
            tiny_http::SslConfig {
                certificate: include_bytes!("../cert.pem").to_vec(),
                private_key: include_bytes!("../key.pem").to_vec(),
            },
        )
        .unwrap(),
    );

    println!("now listening on {}", &host);

    let mut handles = Vec::new();

    for _ in 0..4 {
        let server = server.clone();

        handles.push(thread::spawn(move || {
            for rq in server.incoming_requests() {
                let url = rq.url().to_string();
                // let path = Path::new(&url);

                println!("req {:?} {:?} {:?}", rq.method(), &url, rq.headers());

                match url.as_str() {
                    "/" => {
                        let response = Response::from_string("hello tiny world".to_string());
                        let _ = rq.respond(response);
                    }

                    "/hello" => {
                        let response = Response::from_string("hello world".to_string());
                        let _ = rq.respond(response);
                    }

                    "/goodbye" => {
                        let response = Response::from_string("good bye".to_string());
                        let _ = rq.respond(response);
                    }

                    "/v1/ping" => {
                        let p = ping();
                        let response = Response::from_string(p);
                        let _ = rq.respond(response);
                    }

                    "/v1/status" => {
                        let ss = status();
                        let response = Response::from_string(ss);
                        let _ = rq.respond(response);
                    }

                    _ => {
                        let response = Response::from_string("what?".to_string());
                        let _ = rq.respond(response);
                    }
                }
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}

fn ping() -> String {
    "PONG".to_string()
}

fn status() -> String {
    let now = SystemTime::now();
    format!("i'm ok, at lease at {:?}", now)
}
