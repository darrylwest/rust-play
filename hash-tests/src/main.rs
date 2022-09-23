use anyhow::Result;
use hash_tests::{Database, Person};
use log::info;
use std::fs::File;

fn main() -> Result<()> {
    log4rs::init_file("config/rolling.yaml", Default::default()).unwrap();

    info!("hash tests");

    let count: usize = 6;

    let list = Person::create_models(count);
    let mut db = Database::init();

    for person in &list {
        let json = serde_json::to_string(person)?;
        let _ = db.put(&person.id, &json);
    }

    info!("list: {:?}", &list);

    info!("keys: {:?}", db.keys());
    let values: Vec<String> = db
        .values()
        .into_iter()
        .map(|v| String::from_utf8(v).unwrap())
        .collect();
    for v in values {
        info!("{}", v);
    }

    let filename = "data/mydb.data";
    let buf = File::create(filename)?;
    let values = db.values();

    serde_json::to_writer(buf, &values)?;

    // let fin = File::open(filename)?;
    // let data = serde_json::from_reader(fin)?;
    // info!("{:?}", data);

    Ok(())
}
