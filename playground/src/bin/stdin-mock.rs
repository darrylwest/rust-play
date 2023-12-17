///
/// mock the standard input with a prompter mock
/// 

fn read_stdin(prefix: &str) -> String {
    let mut input = String::from(prefix);
    input.push_str("from std-in");
    input.to_string()
}

fn mock_stdin(prefix: &str) -> String {
    prefix.to_string()
}

#[derive(Debug, Clone)]
struct Client { 
    prompter: fn(prefix: &str) -> String,
}

impl Client {
    fn create() -> Client {
        Client { prompter: read_stdin }
    }

    fn read_input(&self, prefix: &str) -> String {
        (self.prompter)(prefix)
    }
}

fn main() {
    let client = Client::create();
    let input = client.read_input("");
    println!("{}", input);

    let mock = Client{ prompter: mock_stdin };
    let input = mock.read_input("set key value");
    println!("{}", input);

    let input = mock.read_input("quit");
    println!("{}", input);
}