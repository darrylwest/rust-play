
use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;

fn main() {


    let mut io = IoHandler::default();

    io.add_method("api.v1.version", |_| {
        let version = r#"{"version":"0.1.3"}"#;
        Ok(Value::String(version.into()))
    });

    io.add_method("api.v1.status", |_| {
        Ok(Value::String("my status is good, or 'ok'".into()))
    });

    println!("listening on port 3030");

    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec! [AccessControlAllowOrigin::Null]))
        .request_middleware(|request: hyper::Request<hyper::Body>| {
            println!("{:?}", request);
            request.into()
        })
        .start_http(&"0.0.0.0:3030".parse().unwrap())
        .expect("Unable to start RPC Server");


    server.wait();
}
