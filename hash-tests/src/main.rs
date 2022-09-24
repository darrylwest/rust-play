use anyhow::Result;
use hash_tests::{Database, Person, Config};
use log::info;
use std::fs::File;

fn main() -> Result<()> {
    log4rs::init_file("config/rolling.yaml", Default::default()).unwrap();

    info!("hash tests");

    let count: usize = 6;

    let list = Person::create_models(count);
    let config = Config::default();
    let mut db = Database::init(config);

    for person in &list {
        let json = serde_json::to_vec(person)?;
        let _ = db.put(&person.id, json);
    }

    info!("list: {:?}", &list);

    info!("keys: {:?}", db.keys());
    let values: Vec<Person> = db
        .values()
        .into_iter()
        .map(|v| Person::from_json(v) )
        .collect();

    for v in &values {
        info!("{:?}", v);
    }

    let filename = "data/mydb.data";
    let buf = File::create(filename)?;

    serde_json::to_writer(buf, &values)?;

    // let content = std::fs::read_to_string(filename)?;
    // info!("json: {}", content);
    // let values = serde_json::from_str(&content)?;

    info!("values: {:?}", values);

    Ok(())
}
