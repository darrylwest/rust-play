///
/// mock the standard input with a prompter mock
/// 
use std::io::{self, Write};


/// show the prompt then read the line from std in
fn input_reader(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read stdin");

    input.to_string()
}

fn mock_reader(prompt: &str) -> String {
    prompt.to_string()
}

#[derive(Debug, Clone)]
struct Client { 
    prompter: fn(prompt: &str) -> String,
}

impl Client {
    fn create() -> Client {
        Client { prompter: input_reader }
    }

    fn read_input(&self, prompt: &str) -> String {
        (self.prompter)(prompt)
    }
}

fn main() {
    // get input from the real/production source
    let client = Client::create();
    let input = client.read_input("enter data > ");
    println!("{}", input);

    // get input from the mock source
    let mock = Client{ prompter: mock_reader };
    let input = mock.read_input("set key value");
    println!("{}", input);

    let input = mock.read_input("quit");
    println!("{}", input);
}
