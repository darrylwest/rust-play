use subprocess::{Exec, Redirection};

// create a struct for this
fn run_redis() {
    let result = Exec::cmd("ls")
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture();

    match result {
        Ok(data) => match String::from_utf8(data.stdout) {
            Ok(s) => println!("{}", s),
            Err(e) => println!("{}", e),
        }
        Err(e) => eprintln!("error: {}", e),
    }
    
}

fn main() {
    // read config
    // start the instances with async (begin with simple threads)

    run_redis();

    // add shutdown logic with messaging
}
