use anyhow::Result;
use hashbrown::HashMap;
use log::{debug, info};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub type Data = Vec<u8>;

pub struct Config {
    pub dbsize: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config { dbsize: 10_000 }
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    db: HashMap<String, Data>,
}

impl Database {
    pub fn init(config: Config) -> Database {
        info!("initialize the database");

        let db = HashMap::with_capacity(config.dbsize);

        // read data from config.filename

        Database { db }
    }

    pub fn put(&mut self, key: &str, value: Data) -> Result<String> {
        debug!("put item for key: {}", key);

        let k = key.to_string();
        let _ = self.db.insert(k, value);

        Ok(key.to_string())
    }

    pub fn get(&self, key: &str) -> Option<&Data> {
        debug!("get item for key: {}", key);

        self.db.get(key)
    }

    pub fn len(&self) -> usize {
        self.db.len()
    }

    pub fn is_empty(&self) -> bool {
        self.db.is_empty()
    }

    pub fn keys(&self) -> Vec<String> {
        let result: Vec<String> = self.db.clone().into_keys().collect();

        result
    }

    pub fn values(&self) -> Vec<Data> {
        let result: Vec<Data> = self.db.clone().into_values().collect();

        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    New(u8),
    Active(u8),
    Inacive(u8),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Person {
    pub id: String,
    pub version: u64,
    pub email: String,       // unique index required
    pub phones: Vec<String>, // possible empty list
    pub status: Status,
}

impl Person {
    pub fn new(id: &str, email: &str) -> Self {
        Self::with_values(id, 0u64, email, vec![], Status::New(0))
    }

    pub fn with_values(
        id: &str,
        version: u64,
        email: &str,
        phones: Vec<String>,
        status: Status,
    ) -> Self {
        Person {
            id: String::from(id),
            version,
            email: String::from(email),
            phones,
            status,
        }
    }

    pub fn copy(&self) -> Person {
        Self {
            id: self.id.clone(),
            version: self.version,
            email: self.email.clone(),
            phones: self.phones.clone(),
            status: self.status.clone(),
        }
    }

    pub fn from_json(json: Data) -> Self {
        let s = String::from_utf8(json).unwrap();
        let person: Self = serde_json::from_str(&s).unwrap();

        person
    }

    pub fn to_json(&self) -> Result<Data> {
        let json = serde_json::to_vec(self)?;

        Ok(json)
    }

    pub fn random() -> Person {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(100_000..=999_999);
        let id = format!("{:x}", n);
        let email = format!("{}@rcs.com", id);

        Person::new(&id, &email)
    }

    pub fn create_models(count: usize) -> Vec<Person> {
        let mut list: Vec<Person> = Vec::with_capacity(count);

        for _idx in 1..=count {
            list.push(Person::random());
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_person() {
        let p = Person::random();

        assert!(p.id.len() > 3);
        assert!(p.email.starts_with(&p.id));
    }

    #[test]
    fn to_json() {
        let p = Person::random();

        let json = p.to_json().expect("should encode to json");

        assert!(json.len() > 5);
    }
}
