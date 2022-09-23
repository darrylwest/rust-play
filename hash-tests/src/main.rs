
use log::info;
use hash_tests::{Database, Person};
use anyhow::Result;

fn main() -> Result<()> {
    log4rs::init_file("config/rolling.yaml", Default::default()).unwrap();

    info!("hash tests");

    let count: usize = 6;

    let mut db = Database::init();
    let list = create_models(count);

    for person in &list {
        let value = person.email.as_bytes().to_vec();
        let _ = db.put(&person.id, &value);
    }

    info!("list: {:?}", &list);

    Ok(())
}

fn create_models(count: usize) -> Vec<Person> {
    let mut list: Vec<Person> = Vec::with_capacity(count);

    for _idx in 1..=count {
        list.push(Person::random());
    }
        
    list
}


/*
fn get_values(map: &HashMap<String, Model>) -> Vec<Model> {
    map.clone().into_values().collect()
}

    let mut map: HashMap<String, Model> = HashMap::with_capacity(count);
    let list = create_models(count);

    let model: &Model = &list[0];

    info!("model: {:?}", model);
    let key = model.id.to_string();

    for m in &list {
        let id = m.id.to_string();
        let model = m.copy();

        map.insert(id.to_string(), model);
    }

    info!("map: {:?}", map);

    let keys: Vec<String> = map.clone().into_keys().collect();
    info!("keys: {:?}", keys);


    let values = get_values(&map);
    info!("vals: {:?}", values);

    info!("sizes: map: {}, list: {}", map.len(), list.len());

    if let Some(m) = map.get(&key) {
        info!("get: {:?}", m);
    } else {
        panic!("could not get from map");
    }

    let new_model = Model::new(&key, &"new value".to_string());

    if let Some(old) = map.insert(key, new_model) {
        info!("old: {:?}", old);
    } else {
        panic!("could not update map");
    }
        
    info!("clone: {:?}", &map);
*/