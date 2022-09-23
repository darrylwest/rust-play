

use log::{debug,info};
use serde::{Serialize, Deserialize};
use hashbrown::HashMap;
use anyhow::Result;
use rand::Rng;

pub struct Database {
    db: HashMap<String, Vec<u8>>,
}

impl Database {
    pub fn init() -> Database {
        info!("initialize the database");

        let db = HashMap::with_capacity(10_000);

        Database { db }
    }

    pub fn put(&mut self, key: &str, value: &Vec<u8>) -> Result<String> {
        debug!("put item for key: {}", key);

        let k = key.to_string();
        let v = value.clone();
        let _ = self.db.insert(k, v);

        Ok(key.to_string())
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        debug!("get item for key: {}", key);

        if let Some(v) = self.db.get(key) {
            Some(v.to_vec())
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.db.len()
    }

    pub fn keys(&self) -> Vec<String> {
        let mut result: Vec::<String> = Vec::with_capacity(self.db.len());

        for key in self.db.keys() {
            result.push(String::from(key));
        }

        result
    }

    pub fn values(&self) -> Vec<Vec<u8>> {
        let mut result: Vec<Vec<u8>> = Vec::with_capacity(self.db.len());

        for value in self.db.values() {
            result.push(value.to_vec());
        }

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
    pub email: String, // unique index required
    pub phones: Vec<String>,     // possible empty list
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

    pub fn random() -> Person {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(100_000..=999_999);
        let id = format!("{:x}", n);
        let email = format!("{}@rcs.com", id);

        Person::new(&id, &email)
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
}
