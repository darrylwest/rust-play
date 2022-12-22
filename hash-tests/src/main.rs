use anyhow::Result;
use hash_tests::{Config, Database, Person};
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
    let values: Vec<Person> = db.values().into_iter().map(Person::from_json).collect();

    for v in &values {
        info!("{:?}", v);
    }

    info!("values: {:?}", values);

    let filename = "data/person-list.json";
    let buf = File::create(filename)?;

    serde_json::to_writer(buf, &values)?;

    let reader = File::open(filename)?;
    let parsed: Vec<Person> = serde_json::from_reader(reader)?;

    info!("parsed: {:?}", parsed);

    let filename = "data/db.data";
    info!("save db to file: {}", filename);
    db.save(filename)?;

    math_tests();

    Ok(())
}

// math tests
fn math_tests() {
    use malachite::num::arithmetic::traits::Factorial;
    use malachite::Natural;

    let n = 100;
    println!("factorial({});", n);
    println!("{}", Natural::factorial(n));
}
