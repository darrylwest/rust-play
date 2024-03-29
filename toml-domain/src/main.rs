use chrono::naive::NaiveDateTime;
use chrono::prelude::*;
use domain_keys::keys::RouteKey;
use serde_derive::{Deserialize, Serialize};

pub type TS = NaiveDateTime;

fn get_now() -> TS {
    // let utc: DateTime<Utc> = Utc::now();
    Utc::now().naive_utc()
    // Utc::now().timestamp_micros() as u64
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Version {
    pub created_at: TS,
    pub updated_at: TS,
    pub update_count: u64,
    pub hash: u64,
}

impl Version {
    pub fn new() -> Version {
        let now = get_now();
        Version {
            created_at: now,
            updated_at: now,
            update_count: 0u64,
            hash: 0u64,
        }
    }

    pub fn update(&self, new_hash: u64) -> Version {
        Version {
            created_at: self.created_at,
            updated_at: get_now(),
            update_count: self.update_count + 1,
            hash: new_hash,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "t", content = "c")]
pub enum Status {
    New(u8),
    Pending(u8),
    Active(u8),
    Inactive(u8),
    Processed(u8),
    Blocked(u8),
    Deleted(u8),
}

impl Default for Status {
    fn default() -> Self {
        Status::New(0)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Model<T> {
    pub key: String,
    pub version: Version,
    pub status: Status,
    pub value: T,
}

impl<T: Clone> Model<T> {
    pub fn new(value: T) -> Model<T> {
        Model {
            key: RouteKey::create(),
            version: Version::new(),
            status: Status::New(0),
            value,
        }
    }

    pub fn from_model(model: &Model<T>) -> Model<T> {
        Model {
            key: model.key.clone(),
            version: model.version.clone(),
            status: model.status.clone(),
            value: model.value.clone(),
        }
    }

    pub fn create_model(key: String, version: Version, status: Status, value: T) -> Model<T> {
        Model {
            key,
            version,
            status,
            value,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Person {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}

impl Person {
    pub fn new(email: &str, first_name: &str, last_name: &str, phone: &str) -> Person {
        Person {
            email: email.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            phone: phone.to_string(),
        }
    }
}

fn main() {
    let person = Person::new("dpw@rcs.com", "dpw", "west", "555-222-1212");

    let model = Model::new(person);
    let toml = toml::to_string(&model).unwrap();

    println!("---------------------- toml ----------------------");
    println!("{}", toml);
    println!("{} bytes", toml.len());

    let json = serde_json::to_string(&model).expect("should encode model to json");
    println!("---------------------- json ----------------------");
    println!("{}", json);
    println!("{} bytes", json.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn new_version() {
        let version = Version::new();

        assert_eq!(version.created_at, version.updated_at);
        assert_eq!(version.update_count, 0);
        assert_eq!(version.hash, 0);
    }

    #[test]
    fn update_version() {
        let v1 = Version::new();

        assert_eq!(v1.created_at, v1.updated_at);
        assert_eq!(v1.update_count, 0);
        assert_eq!(v1.hash, 0);

        thread::sleep(Duration::from_micros(1));

        let v2 = v1.update(10u64);

        assert_eq!(
            v1.created_at.timestamp_micros(),
            v2.created_at.timestamp_micros()
        );

        assert_ne!(v2.created_at, v2.updated_at);
        assert!(v2.created_at < v2.updated_at);
        assert_eq!(v1.update_count, 0);
        assert_eq!(v2.update_count, 1);
        assert_eq!(v1.hash, 0);
        assert_eq!(v2.hash, 10);
    }

    #[test]
    fn new_model() {
        let value = String::from("my test");
        let model = Model::new(value.clone());

        assert_eq!(model.key.len(), 16);
        assert_eq!(model.version.update_count, 0);
        assert_eq!(model.value, value);
    }

    #[test]
    fn create_model() {
        let key = RouteKey::create();
        let version = Version::new();
        let value = "me".to_string();
        let status = Status::Active(128);

        let model =
            Model::create_model(key.clone(), version.clone(), status.clone(), value.clone());

        assert_eq!(model.key, key);
        assert_eq!(model.version, version);
        assert_eq!(model.status, status);
        assert_eq!(model.value, value);
    }
}
