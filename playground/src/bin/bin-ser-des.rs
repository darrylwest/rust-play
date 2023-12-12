use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct User {
    key: String,
    email: String,
    first_name: String,
    last_name: String,
}

impl User {
    fn create() -> User {
        User {
            key: "7nQ7YC7SRibu".to_string(),
            email: "john@gmail.com".to_string(),
            first_name: "john".to_string(),
            last_name: "smith".to_string(),
        }
    }
}

fn main() {
    // The object that we will serialize.
    let target = User::create();

    let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
    println!("enc: {:?}", encoded);
    let decoded: User = bincode::deserialize(&encoded[..]).unwrap();
    println!("dec: {:?}", decoded);

    assert_eq!(target, decoded);
}
